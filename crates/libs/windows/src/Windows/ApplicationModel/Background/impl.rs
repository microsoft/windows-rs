#[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivitySensorTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn SubscribedActivities(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SupportedActivities(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>>;
    fn MinimumReportInterval(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivitySensorTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IActivitySensorTrigger";
}
#[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivitySensorTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorTrigger_Vtbl {
        unsafe extern "system" fn SubscribedActivities<Impl: IActivitySensorTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribedActivities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportInterval<Impl: IActivitySensorTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedActivities<Impl: IActivitySensorTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedActivities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IActivitySensorTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivitySensorTrigger, BASE_OFFSET>(),
            SubscribedActivities: SubscribedActivities::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            SupportedActivities: SupportedActivities::<Impl, IMPL_OFFSET>,
            MinimumReportInterval: MinimumReportInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorTriggerFactory_Impl: Sized {
    fn Create(&mut self, reportintervalinmilliseconds: u32) -> ::windows::core::Result<ActivitySensorTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivitySensorTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IActivitySensorTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IActivitySensorTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivitySensorTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivitySensorTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IActivitySensorTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportintervalinmilliseconds: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(reportintervalinmilliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IActivitySensorTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivitySensorTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAlarmApplicationManagerStatics_Impl: Sized {
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>>;
    fn GetAccessStatus(&mut self) -> ::windows::core::Result<AlarmAccessStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAlarmApplicationManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAlarmApplicationManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAlarmApplicationManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlarmApplicationManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlarmApplicationManagerStatics_Vtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IAlarmApplicationManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAccessStatus<Impl: IAlarmApplicationManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AlarmAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAlarmApplicationManagerStatics, BASE_OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            GetAccessStatus: GetAccessStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlarmApplicationManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn SetProviderInfo(&mut self, value: &::core::option::Option<AppBroadcastTriggerProviderInfo>) -> ::windows::core::Result<()>;
    fn ProviderInfo(&mut self) -> ::windows::core::Result<AppBroadcastTriggerProviderInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAppBroadcastTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastTrigger_Vtbl {
        unsafe extern "system" fn SetProviderInfo<Impl: IAppBroadcastTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderInfo(&*(&value as *const <AppBroadcastTriggerProviderInfo as ::windows::core::Abi>::Abi as *const <AppBroadcastTriggerProviderInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProviderInfo<Impl: IAppBroadcastTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastTrigger, BASE_OFFSET>(),
            SetProviderInfo: SetProviderInfo::<Impl, IMPL_OFFSET>,
            ProviderInfo: ProviderInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerFactory_Impl: Sized {
    fn CreateAppBroadcastTrigger(&mut self, providerkey: &::windows::core::HSTRING) -> ::windows::core::Result<AppBroadcastTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAppBroadcastTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastTriggerFactory_Vtbl {
        unsafe extern "system" fn CreateAppBroadcastTrigger<Impl: IAppBroadcastTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAppBroadcastTrigger(&*(&providerkey as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastTriggerFactory, BASE_OFFSET>(),
            CreateAppBroadcastTrigger: CreateAppBroadcastTrigger::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastTriggerProviderInfo_Impl: Sized {
    fn SetDisplayNameResource(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayNameResource(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLogoResource(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogoResource(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVideoKeyFrameInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn VideoKeyFrameInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetMaxVideoBitrate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxVideoWidth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoWidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxVideoHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoHeight(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastTriggerProviderInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAppBroadcastTriggerProviderInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastTriggerProviderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastTriggerProviderInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastTriggerProviderInfo_Vtbl {
        unsafe extern "system" fn SetDisplayNameResource<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayNameResource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayNameResource<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayNameResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogoResource<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogoResource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogoResource<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogoResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoKeyFrameInterval<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoKeyFrameInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoKeyFrameInterval<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoKeyFrameInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxVideoBitrate<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxVideoBitrate(value).into()
        }
        unsafe extern "system" fn MaxVideoBitrate<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxVideoBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxVideoWidth<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxVideoWidth(value).into()
        }
        unsafe extern "system" fn MaxVideoWidth<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxVideoWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxVideoHeight<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxVideoHeight(value).into()
        }
        unsafe extern "system" fn MaxVideoHeight<Impl: IAppBroadcastTriggerProviderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxVideoHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastTriggerProviderInfo, BASE_OFFSET>(),
            SetDisplayNameResource: SetDisplayNameResource::<Impl, IMPL_OFFSET>,
            DisplayNameResource: DisplayNameResource::<Impl, IMPL_OFFSET>,
            SetLogoResource: SetLogoResource::<Impl, IMPL_OFFSET>,
            LogoResource: LogoResource::<Impl, IMPL_OFFSET>,
            SetVideoKeyFrameInterval: SetVideoKeyFrameInterval::<Impl, IMPL_OFFSET>,
            VideoKeyFrameInterval: VideoKeyFrameInterval::<Impl, IMPL_OFFSET>,
            SetMaxVideoBitrate: SetMaxVideoBitrate::<Impl, IMPL_OFFSET>,
            MaxVideoBitrate: MaxVideoBitrate::<Impl, IMPL_OFFSET>,
            SetMaxVideoWidth: SetMaxVideoWidth::<Impl, IMPL_OFFSET>,
            MaxVideoWidth: MaxVideoWidth::<Impl, IMPL_OFFSET>,
            SetMaxVideoHeight: SetMaxVideoHeight::<Impl, IMPL_OFFSET>,
            MaxVideoHeight: MaxVideoHeight::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastTriggerProviderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IApplicationTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn RequestAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>;
    fn RequestAsyncWithArguments(&mut self, arguments: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IApplicationTrigger";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IApplicationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationTrigger_Vtbl {
        unsafe extern "system" fn RequestAsync<Impl: IApplicationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAsyncWithArguments<Impl: IApplicationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAsyncWithArguments(&*(&arguments as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationTrigger, BASE_OFFSET>(),
            RequestAsync: RequestAsync::<Impl, IMPL_OFFSET>,
            RequestAsyncWithArguments: RequestAsyncWithArguments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IApplicationTriggerDetails_Impl: Sized {
    fn Arguments(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IApplicationTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IApplicationTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationTriggerDetails_Vtbl {
        unsafe extern "system" fn Arguments<Impl: IApplicationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationTriggerDetails, BASE_OFFSET>(), Arguments: Arguments::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreNotificationTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAppointmentStoreNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreNotificationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppointmentStoreNotificationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppointmentStoreNotificationTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppointmentStoreNotificationTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppointmentStoreNotificationTrigger as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundCondition_Impl: Sized {}
impl ::windows::core::RuntimeName for IBackgroundCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundCondition";
}
impl IBackgroundCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCondition_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundCondition, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBackgroundExecutionManagerStatics_Impl: Sized {
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>;
    fn RequestAccessForApplicationAsync(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>;
    fn RemoveAccess(&mut self) -> ::windows::core::Result<()>;
    fn RemoveAccessForApplication(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetAccessStatus(&mut self) -> ::windows::core::Result<BackgroundAccessStatus>;
    fn GetAccessStatusForApplication(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundAccessStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundExecutionManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundExecutionManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBackgroundExecutionManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundExecutionManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundExecutionManagerStatics_Vtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IBackgroundExecutionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessForApplicationAsync<Impl: IBackgroundExecutionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessForApplicationAsync(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccess<Impl: IBackgroundExecutionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccess().into()
        }
        unsafe extern "system" fn RemoveAccessForApplication<Impl: IBackgroundExecutionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessForApplication(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAccessStatus<Impl: IBackgroundExecutionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessStatusForApplication<Impl: IBackgroundExecutionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessStatusForApplication(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundExecutionManagerStatics, BASE_OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            RequestAccessForApplicationAsync: RequestAccessForApplicationAsync::<Impl, IMPL_OFFSET>,
            RemoveAccess: RemoveAccess::<Impl, IMPL_OFFSET>,
            RemoveAccessForApplication: RemoveAccessForApplication::<Impl, IMPL_OFFSET>,
            GetAccessStatus: GetAccessStatus::<Impl, IMPL_OFFSET>,
            GetAccessStatusForApplication: GetAccessStatusForApplication::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundExecutionManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBackgroundExecutionManagerStatics2_Impl: Sized {
    fn RequestAccessKindAsync(&mut self, requestedaccess: BackgroundAccessRequestKind, reason: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundExecutionManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundExecutionManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBackgroundExecutionManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundExecutionManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundExecutionManagerStatics2_Vtbl {
        unsafe extern "system" fn RequestAccessKindAsync<Impl: IBackgroundExecutionManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessKindAsync(requestedaccess, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundExecutionManagerStatics2, BASE_OFFSET>(),
            RequestAccessKindAsync: RequestAccessKindAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundExecutionManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBackgroundExecutionManagerStatics3_Impl: Sized {
    fn RequestAccessKindForModernStandbyAsync(&mut self, requestedaccess: BackgroundAccessRequestKind, reason: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetAccessStatusForModernStandby(&mut self) -> ::windows::core::Result<BackgroundAccessStatus>;
    fn GetAccessStatusForModernStandbyForApplication(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundAccessStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundExecutionManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundExecutionManagerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBackgroundExecutionManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundExecutionManagerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundExecutionManagerStatics3_Vtbl {
        unsafe extern "system" fn RequestAccessKindForModernStandbyAsync<Impl: IBackgroundExecutionManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessKindForModernStandbyAsync(requestedaccess, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessStatusForModernStandby<Impl: IBackgroundExecutionManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessStatusForModernStandby() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessStatusForModernStandbyForApplication<Impl: IBackgroundExecutionManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessStatusForModernStandbyForApplication(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundExecutionManagerStatics3, BASE_OFFSET>(),
            RequestAccessKindForModernStandbyAsync: RequestAccessKindForModernStandbyAsync::<Impl, IMPL_OFFSET>,
            GetAccessStatusForModernStandby: GetAccessStatusForModernStandby::<Impl, IMPL_OFFSET>,
            GetAccessStatusForModernStandbyForApplication: GetAccessStatusForModernStandbyForApplication::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundExecutionManagerStatics3 as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTask_Impl: Sized {
    fn Run(&mut self, taskinstance: &::core::option::Option<IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBackgroundTask {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTask";
}
impl IBackgroundTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTask_Vtbl {
        unsafe extern "system" fn Run<Impl: IBackgroundTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Run(&*(&taskinstance as *const <IBackgroundTaskInstance as ::windows::core::Abi>::Abi as *const <IBackgroundTaskInstance as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTask, BASE_OFFSET>(), Run: Run::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder_Impl: Sized {
    fn SetTaskEntryPoint(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TaskEntryPoint(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTrigger(&mut self, trigger: &::core::option::Option<IBackgroundTrigger>) -> ::windows::core::Result<()>;
    fn AddCondition(&mut self, condition: &::core::option::Option<IBackgroundCondition>) -> ::windows::core::Result<()>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Register(&mut self) -> ::windows::core::Result<BackgroundTaskRegistration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskBuilder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskBuilder_Vtbl {
        unsafe extern "system" fn SetTaskEntryPoint<Impl: IBackgroundTaskBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskEntryPoint(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TaskEntryPoint<Impl: IBackgroundTaskBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskEntryPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrigger<Impl: IBackgroundTaskBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trigger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrigger(&*(&trigger as *const <IBackgroundTrigger as ::windows::core::Abi>::Abi as *const <IBackgroundTrigger as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddCondition<Impl: IBackgroundTaskBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCondition(&*(&condition as *const <IBackgroundCondition as ::windows::core::Abi>::Abi as *const <IBackgroundCondition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetName<Impl: IBackgroundTaskBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IBackgroundTaskBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: IBackgroundTaskBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskBuilder, BASE_OFFSET>(),
            SetTaskEntryPoint: SetTaskEntryPoint::<Impl, IMPL_OFFSET>,
            TaskEntryPoint: TaskEntryPoint::<Impl, IMPL_OFFSET>,
            SetTrigger: SetTrigger::<Impl, IMPL_OFFSET>,
            AddCondition: AddCondition::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Register: Register::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder2_Impl: Sized + IBackgroundTaskBuilder_Impl {
    fn SetCancelOnConditionLoss(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CancelOnConditionLoss(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder2";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskBuilder2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskBuilder2_Vtbl {
        unsafe extern "system" fn SetCancelOnConditionLoss<Impl: IBackgroundTaskBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancelOnConditionLoss(value).into()
        }
        unsafe extern "system" fn CancelOnConditionLoss<Impl: IBackgroundTaskBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelOnConditionLoss() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskBuilder2, BASE_OFFSET>(),
            SetCancelOnConditionLoss: SetCancelOnConditionLoss::<Impl, IMPL_OFFSET>,
            CancelOnConditionLoss: CancelOnConditionLoss::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskBuilder2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder3_Impl: Sized + IBackgroundTaskBuilder_Impl {
    fn SetIsNetworkRequested(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsNetworkRequested(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder3";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskBuilder3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskBuilder3_Vtbl {
        unsafe extern "system" fn SetIsNetworkRequested<Impl: IBackgroundTaskBuilder3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsNetworkRequested(value).into()
        }
        unsafe extern "system" fn IsNetworkRequested<Impl: IBackgroundTaskBuilder3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNetworkRequested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskBuilder3, BASE_OFFSET>(),
            SetIsNetworkRequested: SetIsNetworkRequested::<Impl, IMPL_OFFSET>,
            IsNetworkRequested: IsNetworkRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskBuilder3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder4_Impl: Sized + IBackgroundTaskBuilder_Impl {
    fn TaskGroup(&mut self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
    fn SetTaskGroup(&mut self, value: &::core::option::Option<BackgroundTaskRegistrationGroup>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder4 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder4";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskBuilder4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskBuilder4_Vtbl {
        unsafe extern "system" fn TaskGroup<Impl: IBackgroundTaskBuilder4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskGroup<Impl: IBackgroundTaskBuilder4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskGroup(&*(&value as *const <BackgroundTaskRegistrationGroup as ::windows::core::Abi>::Abi as *const <BackgroundTaskRegistrationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskBuilder4, BASE_OFFSET>(),
            TaskGroup: TaskGroup::<Impl, IMPL_OFFSET>,
            SetTaskGroup: SetTaskGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskBuilder4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder5_Impl: Sized {
    fn SetTaskEntryPointClsid(&mut self, taskentrypoint: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder5 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder5";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskBuilder5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskBuilder5_Vtbl {
        unsafe extern "system" fn SetTaskEntryPointClsid<Impl: IBackgroundTaskBuilder5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskentrypoint: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskEntryPointClsid(&*(&taskentrypoint as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskBuilder5, BASE_OFFSET>(),
            SetTaskEntryPointClsid: SetTaskEntryPointClsid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskBuilder5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskCompletedEventArgs_Impl: Sized {
    fn InstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CheckResult(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskCompletedEventArgs_Vtbl {
        unsafe extern "system" fn InstanceId<Impl: IBackgroundTaskCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckResult<Impl: IBackgroundTaskCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckResult().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskCompletedEventArgs, BASE_OFFSET>(),
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
            CheckResult: CheckResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IBackgroundTaskDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance_Impl: Sized {
    fn InstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Task(&mut self) -> ::windows::core::Result<BackgroundTaskRegistration>;
    fn Progress(&mut self) -> ::windows::core::Result<u32>;
    fn SetProgress(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn TriggerDetails(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Canceled(&mut self, cancelhandler: &::core::option::Option<BackgroundTaskCanceledEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SuspendedCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<BackgroundTaskDeferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskInstance_Vtbl {
        unsafe extern "system" fn InstanceId<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Task<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgress<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(value).into()
        }
        unsafe extern "system" fn TriggerDetails<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Canceled<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancelhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Canceled(&*(&cancelhandler as *const <BackgroundTaskCanceledEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskCanceledEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanceled(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuspendedCount<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuspendedCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance, BASE_OFFSET>(),
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
            Task: Task::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            SetProgress: SetProgress::<Impl, IMPL_OFFSET>,
            TriggerDetails: TriggerDetails::<Impl, IMPL_OFFSET>,
            Canceled: Canceled::<Impl, IMPL_OFFSET>,
            RemoveCanceled: RemoveCanceled::<Impl, IMPL_OFFSET>,
            SuspendedCount: SuspendedCount::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance2_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn GetThrottleCount(&mut self, counter: BackgroundTaskThrottleCounter) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance2";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskInstance2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskInstance2_Vtbl {
        unsafe extern "system" fn GetThrottleCount<Impl: IBackgroundTaskInstance2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThrottleCount(counter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance2, BASE_OFFSET>(),
            GetThrottleCount: GetThrottleCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait IBackgroundTaskInstance4_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance4 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance4";
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl IBackgroundTaskInstance4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskInstance4_Vtbl {
        unsafe extern "system" fn User<Impl: IBackgroundTaskInstance4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance4, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskProgressEventArgs_Impl: Sized {
    fn InstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Progress(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskProgressEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskProgressEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskProgressEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskProgressEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskProgressEventArgs_Vtbl {
        unsafe extern "system" fn InstanceId<Impl: IBackgroundTaskProgressEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IBackgroundTaskProgressEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskProgressEventArgs, BASE_OFFSET>(),
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskProgressEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration_Impl: Sized {
    fn TaskId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Progress(&mut self, handler: &::core::option::Option<BackgroundTaskProgressEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgress(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&mut self, handler: &::core::option::Option<BackgroundTaskCompletedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unregister(&mut self, canceltask: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistration_Vtbl {
        unsafe extern "system" fn TaskId<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress(&*(&handler as *const <BackgroundTaskProgressEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskProgressEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProgress<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProgress(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <BackgroundTaskCompletedEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskCompletedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unregister<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(canceltask).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration, BASE_OFFSET>(),
            TaskId: TaskId::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            RemoveProgress: RemoveProgress::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
            Unregister: Unregister::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration2_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn Trigger(&mut self) -> ::windows::core::Result<IBackgroundTrigger>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration2";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistration2_Vtbl {
        unsafe extern "system" fn Trigger<Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration2, BASE_OFFSET>(), Trigger: Trigger::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration3_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn TaskGroup(&mut self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration3";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistration3_Vtbl {
        unsafe extern "system" fn TaskGroup<Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration3, BASE_OFFSET>(), TaskGroup: TaskGroup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundTaskRegistrationGroup_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BackgroundActivated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllTasks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, BackgroundTaskRegistration>>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistrationGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistrationGroup";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundTaskRegistrationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistrationGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistrationGroup_Vtbl {
        unsafe extern "system" fn Id<Impl: IBackgroundTaskRegistrationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IBackgroundTaskRegistrationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundActivated<Impl: IBackgroundTaskRegistrationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundActivated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackgroundActivated<Impl: IBackgroundTaskRegistrationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBackgroundActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllTasks<Impl: IBackgroundTaskRegistrationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllTasks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistrationGroup, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            BackgroundActivated: BackgroundActivated::<Impl, IMPL_OFFSET>,
            RemoveBackgroundActivated: RemoveBackgroundActivated::<Impl, IMPL_OFFSET>,
            AllTasks: AllTasks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistrationGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationGroupFactory_Impl: Sized {
    fn Create(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
    fn CreateWithName(&mut self, id: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistrationGroupFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistrationGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskRegistrationGroupFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistrationGroupFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistrationGroupFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IBackgroundTaskRegistrationGroupFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithName<Impl: IBackgroundTaskRegistrationGroupFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithName(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistrationGroupFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithName: CreateWithName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistrationGroupFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundTaskRegistrationStatics_Impl: Sized {
    fn AllTasks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, IBackgroundTaskRegistration>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistrationStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistrationStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundTaskRegistrationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistrationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistrationStatics_Vtbl {
        unsafe extern "system" fn AllTasks<Impl: IBackgroundTaskRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllTasks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistrationStatics, BASE_OFFSET>(),
            AllTasks: AllTasks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistrationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBackgroundTaskRegistrationStatics2_Impl: Sized {
    fn AllTaskGroups(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BackgroundTaskRegistrationGroup>>;
    fn GetTaskGroup(&mut self, groupid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistrationStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistrationStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBackgroundTaskRegistrationStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistrationStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistrationStatics2_Vtbl {
        unsafe extern "system" fn AllTaskGroups<Impl: IBackgroundTaskRegistrationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllTaskGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTaskGroup<Impl: IBackgroundTaskRegistrationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, groupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTaskGroup(&*(&groupid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistrationStatics2, BASE_OFFSET>(),
            AllTaskGroups: AllTaskGroups::<Impl, IMPL_OFFSET>,
            GetTaskGroup: GetTaskGroup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistrationStatics2 as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTrigger_Impl: Sized {}
impl ::windows::core::RuntimeName for IBackgroundTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTrigger";
}
impl IBackgroundTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundWorkCostStatics_Impl: Sized {
    fn CurrentBackgroundWorkCost(&mut self) -> ::windows::core::Result<BackgroundWorkCostValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundWorkCostStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundWorkCostStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundWorkCostStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundWorkCostStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundWorkCostStatics_Vtbl {
        unsafe extern "system" fn CurrentBackgroundWorkCost<Impl: IBackgroundWorkCostStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundWorkCostValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentBackgroundWorkCost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundWorkCostStatics, BASE_OFFSET>(),
            CurrentBackgroundWorkCost: CurrentBackgroundWorkCost::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundWorkCostStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementPublisherTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn Advertisement(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement>;
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBluetoothLEAdvertisementPublisherTrigger";
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisherTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherTrigger_Vtbl {
        unsafe extern "system" fn Advertisement<Impl: IBluetoothLEAdvertisementPublisherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advertisement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAdvertisementPublisherTrigger, BASE_OFFSET>(),
            Advertisement: Advertisement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisherTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementPublisherTrigger2_Impl: Sized {
    fn PreferredTransmitPowerLevelInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetPreferredTransmitPowerLevelInDBm(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn UseExtendedFormat(&mut self) -> ::windows::core::Result<bool>;
    fn SetUseExtendedFormat(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAnonymous(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAnonymous(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeTransmitPowerLevel(&mut self) -> ::windows::core::Result<bool>;
    fn SetIncludeTransmitPowerLevel(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTrigger2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBluetoothLEAdvertisementPublisherTrigger2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisherTrigger2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherTrigger2_Vtbl {
        unsafe extern "system" fn PreferredTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredTransmitPowerLevelInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredTransmitPowerLevelInDBm(&*(&value as *const <super::super::Foundation::IReference<i16> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i16> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UseExtendedFormat<Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseExtendedFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseExtendedFormat<Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseExtendedFormat(value).into()
        }
        unsafe extern "system" fn IsAnonymous<Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAnonymous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAnonymous<Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAnonymous(value).into()
        }
        unsafe extern "system" fn IncludeTransmitPowerLevel<Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeTransmitPowerLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeTransmitPowerLevel<Impl: IBluetoothLEAdvertisementPublisherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeTransmitPowerLevel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAdvertisementPublisherTrigger2, BASE_OFFSET>(),
            PreferredTransmitPowerLevelInDBm: PreferredTransmitPowerLevelInDBm::<Impl, IMPL_OFFSET>,
            SetPreferredTransmitPowerLevelInDBm: SetPreferredTransmitPowerLevelInDBm::<Impl, IMPL_OFFSET>,
            UseExtendedFormat: UseExtendedFormat::<Impl, IMPL_OFFSET>,
            SetUseExtendedFormat: SetUseExtendedFormat::<Impl, IMPL_OFFSET>,
            IsAnonymous: IsAnonymous::<Impl, IMPL_OFFSET>,
            SetIsAnonymous: SetIsAnonymous::<Impl, IMPL_OFFSET>,
            IncludeTransmitPowerLevel: IncludeTransmitPowerLevel::<Impl, IMPL_OFFSET>,
            SetIncludeTransmitPowerLevel: SetIncludeTransmitPowerLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisherTrigger2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth", feature = "Devices_Bluetooth_Advertisement", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementWatcherTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn MinSamplingInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MaxSamplingInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MinOutOfRangeTimeout(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MaxOutOfRangeTimeout(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SignalStrengthFilter(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>;
    fn SetSignalStrengthFilter(&mut self, value: &::core::option::Option<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>) -> ::windows::core::Result<()>;
    fn AdvertisementFilter(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>;
    fn SetAdvertisementFilter(&mut self, value: &::core::option::Option<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth", feature = "Devices_Bluetooth_Advertisement", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBluetoothLEAdvertisementWatcherTrigger";
}
#[cfg(all(feature = "Devices_Bluetooth", feature = "Devices_Bluetooth_Advertisement", feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementWatcherTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementWatcherTrigger_Vtbl {
        unsafe extern "system" fn MinSamplingInterval<Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinSamplingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSamplingInterval<Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSamplingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinOutOfRangeTimeout<Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinOutOfRangeTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxOutOfRangeTimeout<Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxOutOfRangeTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalStrengthFilter<Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalStrengthFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalStrengthFilter<Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalStrengthFilter(&*(&value as *const <super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdvertisementFilter<Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisementFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisementFilter<Impl: IBluetoothLEAdvertisementWatcherTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvertisementFilter(&*(&value as *const <super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAdvertisementWatcherTrigger, BASE_OFFSET>(),
            MinSamplingInterval: MinSamplingInterval::<Impl, IMPL_OFFSET>,
            MaxSamplingInterval: MaxSamplingInterval::<Impl, IMPL_OFFSET>,
            MinOutOfRangeTimeout: MinOutOfRangeTimeout::<Impl, IMPL_OFFSET>,
            MaxOutOfRangeTimeout: MaxOutOfRangeTimeout::<Impl, IMPL_OFFSET>,
            SignalStrengthFilter: SignalStrengthFilter::<Impl, IMPL_OFFSET>,
            SetSignalStrengthFilter: SetSignalStrengthFilter::<Impl, IMPL_OFFSET>,
            AdvertisementFilter: AdvertisementFilter::<Impl, IMPL_OFFSET>,
            SetAdvertisementFilter: SetAdvertisementFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementWatcherTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherTrigger2_Impl: Sized {
    fn AllowExtendedAdvertisements(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowExtendedAdvertisements(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherTrigger2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBluetoothLEAdvertisementWatcherTrigger2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementWatcherTrigger2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcherTrigger2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementWatcherTrigger2_Vtbl {
        unsafe extern "system" fn AllowExtendedAdvertisements<Impl: IBluetoothLEAdvertisementWatcherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowExtendedAdvertisements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowExtendedAdvertisements<Impl: IBluetoothLEAdvertisementWatcherTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowExtendedAdvertisements(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAdvertisementWatcherTrigger2, BASE_OFFSET>(),
            AllowExtendedAdvertisements: AllowExtendedAdvertisements::<Impl, IMPL_OFFSET>,
            SetAllowExtendedAdvertisements: SetAllowExtendedAdvertisements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementWatcherTrigger2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICachedFileUpdaterTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICachedFileUpdaterTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ICachedFileUpdaterTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileUpdaterTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICachedFileUpdaterTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICachedFileUpdaterTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICachedFileUpdaterTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Provider", feature = "implement_exclusive"))]
pub trait ICachedFileUpdaterTriggerDetails_Impl: Sized {
    fn UpdateTarget(&mut self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileTarget>;
    fn UpdateRequest(&mut self) -> ::windows::core::Result<super::super::Storage::Provider::FileUpdateRequest>;
    fn CanRequestUserInput(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Storage_Provider", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICachedFileUpdaterTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICachedFileUpdaterTriggerDetails";
}
#[cfg(all(feature = "Storage_Provider", feature = "implement_exclusive"))]
impl ICachedFileUpdaterTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileUpdaterTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICachedFileUpdaterTriggerDetails_Vtbl {
        unsafe extern "system" fn UpdateTarget<Impl: ICachedFileUpdaterTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Storage::Provider::CachedFileTarget) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateRequest<Impl: ICachedFileUpdaterTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRequestUserInput<Impl: ICachedFileUpdaterTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRequestUserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICachedFileUpdaterTriggerDetails, BASE_OFFSET>(),
            UpdateTarget: UpdateTarget::<Impl, IMPL_OFFSET>,
            UpdateRequest: UpdateRequest::<Impl, IMPL_OFFSET>,
            CanRequestUserInput: CanRequestUserInput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICachedFileUpdaterTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageNotificationTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IChatMessageNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageNotificationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageNotificationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageNotificationTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageNotificationTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageNotificationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageReceivedNotificationTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageReceivedNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IChatMessageReceivedNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageReceivedNotificationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChatMessageReceivedNotificationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChatMessageReceivedNotificationTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IChatMessageReceivedNotificationTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChatMessageReceivedNotificationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommunicationBlockingAppSetAsActiveTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommunicationBlockingAppSetAsActiveTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICommunicationBlockingAppSetAsActiveTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ICommunicationBlockingAppSetAsActiveTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommunicationBlockingAppSetAsActiveTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommunicationBlockingAppSetAsActiveTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICommunicationBlockingAppSetAsActiveTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommunicationBlockingAppSetAsActiveTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreNotificationTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IContactStoreNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IContactStoreNotificationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactStoreNotificationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactStoreNotificationTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContactStoreNotificationTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactStoreNotificationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContentPrefetchTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn WaitInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentPrefetchTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IContentPrefetchTrigger";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContentPrefetchTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetchTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPrefetchTrigger_Vtbl {
        unsafe extern "system" fn WaitInterval<Impl: IContentPrefetchTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContentPrefetchTrigger, BASE_OFFSET>(),
            WaitInterval: WaitInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPrefetchTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContentPrefetchTriggerFactory_Impl: Sized {
    fn Create(&mut self, waitinterval: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ContentPrefetchTrigger>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContentPrefetchTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IContentPrefetchTriggerFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContentPrefetchTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContentPrefetchTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContentPrefetchTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IContentPrefetchTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitinterval: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&waitinterval as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContentPrefetchTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContentPrefetchTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSystemEventTrigger_Impl: Sized {
    fn TriggerId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recurrence(&mut self) -> ::windows::core::Result<CustomSystemEventTriggerRecurrence>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomSystemEventTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICustomSystemEventTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomSystemEventTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomSystemEventTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomSystemEventTrigger_Vtbl {
        unsafe extern "system" fn TriggerId<Impl: ICustomSystemEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recurrence<Impl: ICustomSystemEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CustomSystemEventTriggerRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recurrence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomSystemEventTrigger, BASE_OFFSET>(),
            TriggerId: TriggerId::<Impl, IMPL_OFFSET>,
            Recurrence: Recurrence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomSystemEventTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSystemEventTriggerFactory_Impl: Sized {
    fn Create(&mut self, triggerid: &::windows::core::HSTRING, recurrence: CustomSystemEventTriggerRecurrence) -> ::windows::core::Result<CustomSystemEventTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomSystemEventTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICustomSystemEventTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomSystemEventTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomSystemEventTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomSystemEventTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ICustomSystemEventTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recurrence: CustomSystemEventTriggerRecurrence, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&triggerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), recurrence) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICustomSystemEventTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomSystemEventTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceConnectionChangeTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanMaintainConnection(&mut self) -> ::windows::core::Result<bool>;
    fn MaintainConnection(&mut self) -> ::windows::core::Result<bool>;
    fn SetMaintainConnection(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceConnectionChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceConnectionChangeTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceConnectionChangeTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceConnectionChangeTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceConnectionChangeTrigger_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IDeviceConnectionChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanMaintainConnection<Impl: IDeviceConnectionChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMaintainConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaintainConnection<Impl: IDeviceConnectionChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaintainConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintainConnection<Impl: IDeviceConnectionChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaintainConnection(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceConnectionChangeTrigger, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            CanMaintainConnection: CanMaintainConnection::<Impl, IMPL_OFFSET>,
            MaintainConnection: MaintainConnection::<Impl, IMPL_OFFSET>,
            SetMaintainConnection: SetMaintainConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceConnectionChangeTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDeviceConnectionChangeTriggerStatics_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceConnectionChangeTriggerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceConnectionChangeTriggerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDeviceConnectionChangeTriggerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceConnectionChangeTriggerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceConnectionChangeTriggerStatics_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IDeviceConnectionChangeTriggerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceConnectionChangeTriggerStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceConnectionChangeTriggerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDeviceManufacturerNotificationTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn TriggerQualifier(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OneShot(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceManufacturerNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceManufacturerNotificationTrigger";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IDeviceManufacturerNotificationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceManufacturerNotificationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceManufacturerNotificationTrigger_Vtbl {
        unsafe extern "system" fn TriggerQualifier<Impl: IDeviceManufacturerNotificationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerQualifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OneShot<Impl: IDeviceManufacturerNotificationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceManufacturerNotificationTrigger, BASE_OFFSET>(),
            TriggerQualifier: TriggerQualifier::<Impl, IMPL_OFFSET>,
            OneShot: OneShot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceManufacturerNotificationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDeviceManufacturerNotificationTriggerFactory_Impl: Sized {
    fn Create(&mut self, triggerqualifier: &::windows::core::HSTRING, oneshot: bool) -> ::windows::core::Result<DeviceManufacturerNotificationTrigger>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceManufacturerNotificationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceManufacturerNotificationTriggerFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IDeviceManufacturerNotificationTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceManufacturerNotificationTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceManufacturerNotificationTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IDeviceManufacturerNotificationTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerqualifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&triggerqualifier as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceManufacturerNotificationTriggerFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceManufacturerNotificationTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDeviceServicingTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn RequestAsyncSimple(&mut self, deviceid: &::windows::core::HSTRING, expectedduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
    fn RequestAsyncWithArguments(&mut self, deviceid: &::windows::core::HSTRING, expectedduration: &super::super::Foundation::TimeSpan, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceServicingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceServicingTrigger";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDeviceServicingTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceServicingTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceServicingTrigger_Vtbl {
        unsafe extern "system" fn RequestAsyncSimple<Impl: IDeviceServicingTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAsyncSimple(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&expectedduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAsyncWithArguments<Impl: IDeviceServicingTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAsyncWithArguments(
                &*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&expectedduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceServicingTrigger, BASE_OFFSET>(),
            RequestAsyncSimple: RequestAsyncSimple::<Impl, IMPL_OFFSET>,
            RequestAsyncWithArguments: RequestAsyncWithArguments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceServicingTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDeviceUseTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn RequestAsyncSimple(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
    fn RequestAsyncWithArguments(&mut self, deviceid: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceUseTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceUseTrigger";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDeviceUseTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceUseTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceUseTrigger_Vtbl {
        unsafe extern "system" fn RequestAsyncSimple<Impl: IDeviceUseTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAsyncSimple(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAsyncWithArguments<Impl: IDeviceUseTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAsyncWithArguments(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceUseTrigger, BASE_OFFSET>(),
            RequestAsyncSimple: RequestAsyncSimple::<Impl, IMPL_OFFSET>,
            RequestAsyncWithArguments: RequestAsyncWithArguments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceUseTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceWatcherTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceWatcherTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcherTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceWatcherTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceWatcherTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceWatcherTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailStoreNotificationTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IEmailStoreNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailStoreNotificationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailStoreNotificationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailStoreNotificationTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IEmailStoreNotificationTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailStoreNotificationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
pub trait IGattCharacteristicNotificationTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn Characteristic(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattCharacteristicNotificationTrigger";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl IGattCharacteristicNotificationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicNotificationTrigger_Vtbl {
        unsafe extern "system" fn Characteristic<Impl: IGattCharacteristicNotificationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicNotificationTrigger, BASE_OFFSET>(),
            Characteristic: Characteristic::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicNotificationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "implement_exclusive"))]
pub trait IGattCharacteristicNotificationTrigger2_Impl: Sized {
    fn EventTriggeringMode(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode>;
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTrigger2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattCharacteristicNotificationTrigger2";
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "implement_exclusive"))]
impl IGattCharacteristicNotificationTrigger2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTrigger2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicNotificationTrigger2_Vtbl {
        unsafe extern "system" fn EventTriggeringMode<Impl: IGattCharacteristicNotificationTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventTriggeringMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicNotificationTrigger2, BASE_OFFSET>(),
            EventTriggeringMode: EventTriggeringMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicNotificationTrigger2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
pub trait IGattCharacteristicNotificationTriggerFactory_Impl: Sized {
    fn Create(&mut self, characteristic: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>) -> ::windows::core::Result<GattCharacteristicNotificationTrigger>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattCharacteristicNotificationTriggerFactory";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl IGattCharacteristicNotificationTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicNotificationTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IGattCharacteristicNotificationTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characteristic: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&characteristic as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicNotificationTriggerFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicNotificationTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
pub trait IGattCharacteristicNotificationTriggerFactory2_Impl: Sized {
    fn CreateWithEventTriggeringMode(&mut self, characteristic: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::Result<GattCharacteristicNotificationTrigger>;
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerFactory2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattCharacteristicNotificationTriggerFactory2";
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl IGattCharacteristicNotificationTriggerFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTriggerFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicNotificationTriggerFactory2_Vtbl {
        unsafe extern "system" fn CreateWithEventTriggeringMode<Impl: IGattCharacteristicNotificationTriggerFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characteristic: ::windows::core::RawPtr, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithEventTriggeringMode(&*(&characteristic as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic as ::windows::core::DefaultType>::DefaultType), eventtriggeringmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicNotificationTriggerFactory2, BASE_OFFSET>(),
            CreateWithEventTriggeringMode: CreateWithEventTriggeringMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicNotificationTriggerFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
pub trait IGattServiceProviderTrigger_Impl: Sized {
    fn TriggerId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService>;
    fn SetAdvertisingParameters(&mut self, value: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>) -> ::windows::core::Result<()>;
    fn AdvertisingParameters(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattServiceProviderTrigger";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl IGattServiceProviderTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderTrigger_Vtbl {
        unsafe extern "system" fn TriggerId<Impl: IGattServiceProviderTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Impl: IGattServiceProviderTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisingParameters<Impl: IGattServiceProviderTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvertisingParameters(&*(&value as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdvertisingParameters<Impl: IGattServiceProviderTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisingParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderTrigger, BASE_OFFSET>(),
            TriggerId: TriggerId::<Impl, IMPL_OFFSET>,
            Service: Service::<Impl, IMPL_OFFSET>,
            SetAdvertisingParameters: SetAdvertisingParameters::<Impl, IMPL_OFFSET>,
            AdvertisingParameters: AdvertisingParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth", feature = "implement_exclusive"))]
pub trait IGattServiceProviderTriggerResult_Impl: Sized {
    fn Trigger(&mut self) -> ::windows::core::Result<GattServiceProviderTrigger>;
    fn Error(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothError>;
}
#[cfg(all(feature = "Devices_Bluetooth", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderTriggerResult {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattServiceProviderTriggerResult";
}
#[cfg(all(feature = "Devices_Bluetooth", feature = "implement_exclusive"))]
impl IGattServiceProviderTriggerResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderTriggerResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderTriggerResult_Vtbl {
        unsafe extern "system" fn Trigger<Impl: IGattServiceProviderTriggerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IGattServiceProviderTriggerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::BluetoothError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderTriggerResult, BASE_OFFSET>(),
            Trigger: Trigger::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderTriggerResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattServiceProviderTriggerStatics_Impl: Sized {
    fn CreateAsync(&mut self, triggerid: &::windows::core::HSTRING, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderTriggerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattServiceProviderTriggerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattServiceProviderTriggerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderTriggerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderTriggerStatics_Vtbl {
        unsafe extern "system" fn CreateAsync<Impl: IGattServiceProviderTriggerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&triggerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderTriggerStatics, BASE_OFFSET>(),
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderTriggerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IGeovisitTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn MonitoringScope(&mut self) -> ::windows::core::Result<super::super::Devices::Geolocation::VisitMonitoringScope>;
    fn SetMonitoringScope(&mut self, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeovisitTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGeovisitTrigger";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IGeovisitTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeovisitTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeovisitTrigger_Vtbl {
        unsafe extern "system" fn MonitoringScope<Impl: IGeovisitTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonitoringScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitoringScope<Impl: IGeovisitTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonitoringScope(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeovisitTrigger, BASE_OFFSET>(),
            MonitoringScope: MonitoringScope::<Impl, IMPL_OFFSET>,
            SetMonitoringScope: SetMonitoringScope::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeovisitTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocationTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn TriggerType(&mut self) -> ::windows::core::Result<LocationTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ILocationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ILocationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocationTrigger_Vtbl {
        unsafe extern "system" fn TriggerType<Impl: ILocationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LocationTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILocationTrigger, BASE_OFFSET>(), TriggerType: TriggerType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocationTriggerFactory_Impl: Sized {
    fn Create(&mut self, triggertype: LocationTriggerType) -> ::windows::core::Result<LocationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ILocationTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILocationTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocationTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ILocationTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggertype: LocationTriggerType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(triggertype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILocationTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMaintenanceTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn FreshnessTime(&mut self) -> ::windows::core::Result<u32>;
    fn OneShot(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMaintenanceTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IMaintenanceTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IMaintenanceTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMaintenanceTrigger_Vtbl {
        unsafe extern "system" fn FreshnessTime<Impl: IMaintenanceTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreshnessTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OneShot<Impl: IMaintenanceTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMaintenanceTrigger, BASE_OFFSET>(),
            FreshnessTime: FreshnessTime::<Impl, IMPL_OFFSET>,
            OneShot: OneShot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMaintenanceTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMaintenanceTriggerFactory_Impl: Sized {
    fn Create(&mut self, freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<MaintenanceTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMaintenanceTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IMaintenanceTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMaintenanceTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMaintenanceTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IMaintenanceTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(freshnesstime, oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMaintenanceTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMaintenanceTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaProcessingTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn RequestAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>;
    fn RequestAsyncWithArguments(&mut self, arguments: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaProcessingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IMediaProcessingTrigger";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaProcessingTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProcessingTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaProcessingTrigger_Vtbl {
        unsafe extern "system" fn RequestAsync<Impl: IMediaProcessingTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAsyncWithArguments<Impl: IMediaProcessingTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAsyncWithArguments(&*(&arguments as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaProcessingTrigger, BASE_OFFSET>(),
            RequestAsync: RequestAsync::<Impl, IMPL_OFFSET>,
            RequestAsyncWithArguments: RequestAsyncWithArguments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProcessingTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorHotspotAuthenticationTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorHotspotAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.INetworkOperatorHotspotAuthenticationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorHotspotAuthenticationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorHotspotAuthenticationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorHotspotAuthenticationTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorHotspotAuthenticationTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorHotspotAuthenticationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorNotificationTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn NetworkAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.INetworkOperatorNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorNotificationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorNotificationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorNotificationTrigger_Vtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: INetworkOperatorNotificationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorNotificationTrigger, BASE_OFFSET>(),
            NetworkAccountId: NetworkAccountId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorNotificationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorNotificationTriggerFactory_Impl: Sized {
    fn Create(&mut self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<NetworkOperatorNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorNotificationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.INetworkOperatorNotificationTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorNotificationTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorNotificationTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorNotificationTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: INetworkOperatorNotificationTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorNotificationTriggerFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorNotificationTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Calls_Background", feature = "implement_exclusive"))]
pub trait IPhoneTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn OneShot(&mut self) -> ::windows::core::Result<bool>;
    fn TriggerType(&mut self) -> ::windows::core::Result<super::Calls::Background::PhoneTriggerType>;
}
#[cfg(all(feature = "ApplicationModel_Calls_Background", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IPhoneTrigger";
}
#[cfg(all(feature = "ApplicationModel_Calls_Background", feature = "implement_exclusive"))]
impl IPhoneTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneTrigger_Vtbl {
        unsafe extern "system" fn OneShot<Impl: IPhoneTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TriggerType<Impl: IPhoneTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Calls::Background::PhoneTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneTrigger, BASE_OFFSET>(),
            OneShot: OneShot::<Impl, IMPL_OFFSET>,
            TriggerType: TriggerType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Calls_Background", feature = "implement_exclusive"))]
pub trait IPhoneTriggerFactory_Impl: Sized {
    fn Create(&mut self, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool) -> ::windows::core::Result<PhoneTrigger>;
}
#[cfg(all(feature = "ApplicationModel_Calls_Background", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPhoneTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IPhoneTriggerFactory";
}
#[cfg(all(feature = "ApplicationModel_Calls_Background", feature = "implement_exclusive"))]
impl IPhoneTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPhoneTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(r#type, oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationTriggerFactory_Impl: Sized {
    fn Create(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<PushNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IPushNotificationTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPushNotificationTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPushNotificationTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IPushNotificationTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPushNotificationTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPushNotificationTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageAvailableTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRcsEndUserMessageAvailableTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IRcsEndUserMessageAvailableTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IRcsEndUserMessageAvailableTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRcsEndUserMessageAvailableTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRcsEndUserMessageAvailableTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRcsEndUserMessageAvailableTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRcsEndUserMessageAvailableTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
pub trait IRfcommConnectionTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn InboundConnection(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation>;
    fn OutboundConnection(&mut self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation>;
    fn AllowMultipleConnections(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowMultipleConnections(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ProtectionLevel(&mut self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketProtectionLevel>;
    fn SetProtectionLevel(&mut self, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::Result<()>;
    fn RemoteHostName(&mut self) -> ::windows::core::Result<super::super::Networking::HostName>;
    fn SetRemoteHostName(&mut self, value: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommConnectionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IRfcommConnectionTrigger";
}
#[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl IRfcommConnectionTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommConnectionTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommConnectionTrigger_Vtbl {
        unsafe extern "system" fn InboundConnection<Impl: IRfcommConnectionTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutboundConnection<Impl: IRfcommConnectionTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowMultipleConnections<Impl: IRfcommConnectionTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowMultipleConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowMultipleConnections<Impl: IRfcommConnectionTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowMultipleConnections(value).into()
        }
        unsafe extern "system" fn ProtectionLevel<Impl: IRfcommConnectionTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectionLevel<Impl: IRfcommConnectionTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectionLevel(value).into()
        }
        unsafe extern "system" fn RemoteHostName<Impl: IRfcommConnectionTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteHostName<Impl: IRfcommConnectionTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteHostName(&*(&value as *const <super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommConnectionTrigger, BASE_OFFSET>(),
            InboundConnection: InboundConnection::<Impl, IMPL_OFFSET>,
            OutboundConnection: OutboundConnection::<Impl, IMPL_OFFSET>,
            AllowMultipleConnections: AllowMultipleConnections::<Impl, IMPL_OFFSET>,
            SetAllowMultipleConnections: SetAllowMultipleConnections::<Impl, IMPL_OFFSET>,
            ProtectionLevel: ProtectionLevel::<Impl, IMPL_OFFSET>,
            SetProtectionLevel: SetProtectionLevel::<Impl, IMPL_OFFSET>,
            RemoteHostName: RemoteHostName::<Impl, IMPL_OFFSET>,
            SetRemoteHostName: SetRemoteHostName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommConnectionTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISecondaryAuthenticationFactorAuthenticationTrigger";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorAuthenticationTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorAuthenticationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorDataThresholdTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorDataThresholdTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISensorDataThresholdTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorDataThresholdTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataThresholdTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorDataThresholdTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISensorDataThresholdTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorDataThresholdTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Sensors", feature = "implement_exclusive"))]
pub trait ISensorDataThresholdTriggerFactory_Impl: Sized {
    fn Create(&mut self, threshold: &::core::option::Option<super::super::Devices::Sensors::ISensorDataThreshold>) -> ::windows::core::Result<SensorDataThresholdTrigger>;
}
#[cfg(all(feature = "Devices_Sensors", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISensorDataThresholdTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISensorDataThresholdTriggerFactory";
}
#[cfg(all(feature = "Devices_Sensors", feature = "implement_exclusive"))]
impl ISensorDataThresholdTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataThresholdTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorDataThresholdTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISensorDataThresholdTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&threshold as *const <super::super::Devices::Sensors::ISensorDataThreshold as ::windows::core::Abi>::Abi as *const <super::super::Devices::Sensors::ISensorDataThreshold as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISensorDataThresholdTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorDataThresholdTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_SmartCards", feature = "implement_exclusive"))]
pub trait ISmartCardTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn TriggerType(&mut self) -> ::windows::core::Result<super::super::Devices::SmartCards::SmartCardTriggerType>;
}
#[cfg(all(feature = "Devices_SmartCards", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISmartCardTrigger";
}
#[cfg(all(feature = "Devices_SmartCards", feature = "implement_exclusive"))]
impl ISmartCardTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardTrigger_Vtbl {
        unsafe extern "system" fn TriggerType<Impl: ISmartCardTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardTrigger, BASE_OFFSET>(), TriggerType: TriggerType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_SmartCards", feature = "implement_exclusive"))]
pub trait ISmartCardTriggerFactory_Impl: Sized {
    fn Create(&mut self, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::Result<SmartCardTrigger>;
}
#[cfg(all(feature = "Devices_SmartCards", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmartCardTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISmartCardTriggerFactory";
}
#[cfg(all(feature = "Devices_SmartCards", feature = "implement_exclusive"))]
impl ISmartCardTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmartCardTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmartCardTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISmartCardTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(triggertype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISmartCardTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmartCardTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
pub trait ISmsMessageReceivedTriggerFactory_Impl: Sized {
    fn Create(&mut self, filterrules: &::core::option::Option<super::super::Devices::Sms::SmsFilterRules>) -> ::windows::core::Result<SmsMessageReceivedTrigger>;
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISmsMessageReceivedTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISmsMessageReceivedTriggerFactory";
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
impl ISmsMessageReceivedTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmsMessageReceivedTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmsMessageReceivedTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISmsMessageReceivedTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filterrules: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&filterrules as *const <super::super::Devices::Sms::SmsFilterRules as ::windows::core::Abi>::Abi as *const <super::super::Devices::Sms::SmsFilterRules as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISmsMessageReceivedTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmsMessageReceivedTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityTrigger_Impl: Sized {
    fn IsWakeFromLowPowerSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISocketActivityTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISocketActivityTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ISocketActivityTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityTrigger_Vtbl {
        unsafe extern "system" fn IsWakeFromLowPowerSupported<Impl: ISocketActivityTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWakeFromLowPowerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocketActivityTrigger, BASE_OFFSET>(),
            IsWakeFromLowPowerSupported: IsWakeFromLowPowerSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IStorageLibraryChangeTrackerTriggerFactory_Impl: Sized {
    fn Create(&mut self, tracker: &::core::option::Option<super::super::Storage::StorageLibraryChangeTracker>) -> ::windows::core::Result<StorageLibraryChangeTrackerTrigger>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTrackerTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IStorageLibraryChangeTrackerTriggerFactory";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IStorageLibraryChangeTrackerTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTrackerTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryChangeTrackerTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IStorageLibraryChangeTrackerTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracker: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&tracker as *const <super::super::Storage::StorageLibraryChangeTracker as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageLibraryChangeTracker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryChangeTrackerTriggerFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryChangeTrackerTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryContentChangedTrigger_Impl: Sized + IBackgroundTrigger_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryContentChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IStorageLibraryContentChangedTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryContentChangedTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryContentChangedTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryContentChangedTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryContentChangedTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryContentChangedTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IStorageLibraryContentChangedTriggerStatics_Impl: Sized {
    fn Create(&mut self, storagelibrary: &::core::option::Option<super::super::Storage::StorageLibrary>) -> ::windows::core::Result<StorageLibraryContentChangedTrigger>;
    fn CreateFromLibraries(&mut self, storagelibraries: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary>>) -> ::windows::core::Result<StorageLibraryContentChangedTrigger>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibraryContentChangedTriggerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IStorageLibraryContentChangedTriggerStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IStorageLibraryContentChangedTriggerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryContentChangedTriggerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryContentChangedTriggerStatics_Vtbl {
        unsafe extern "system" fn Create<Impl: IStorageLibraryContentChangedTriggerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagelibrary: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&storagelibrary as *const <super::super::Storage::StorageLibrary as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageLibrary as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLibraries<Impl: IStorageLibraryContentChangedTriggerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagelibraries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLibraries(&*(&storagelibraries as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryContentChangedTriggerStatics, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateFromLibraries: CreateFromLibraries::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryContentChangedTriggerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemCondition_Impl: Sized + IBackgroundCondition_Impl {
    fn ConditionType(&mut self) -> ::windows::core::Result<SystemConditionType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISystemCondition";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemCondition_Vtbl {
        unsafe extern "system" fn ConditionType<Impl: ISystemCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConditionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemCondition, BASE_OFFSET>(), ConditionType: ConditionType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemConditionFactory_Impl: Sized {
    fn Create(&mut self, conditiontype: SystemConditionType) -> ::windows::core::Result<SystemCondition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemConditionFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISystemConditionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemConditionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemConditionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemConditionFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISystemConditionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditiontype: SystemConditionType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(conditiontype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemConditionFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemConditionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn OneShot(&mut self) -> ::windows::core::Result<bool>;
    fn TriggerType(&mut self) -> ::windows::core::Result<SystemTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISystemTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemTrigger_Vtbl {
        unsafe extern "system" fn OneShot<Impl: ISystemTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TriggerType<Impl: ISystemTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemTrigger, BASE_OFFSET>(),
            OneShot: OneShot::<Impl, IMPL_OFFSET>,
            TriggerType: TriggerType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemTriggerFactory_Impl: Sized {
    fn Create(&mut self, triggertype: SystemTriggerType, oneshot: bool) -> ::windows::core::Result<SystemTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISystemTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISystemTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggertype: SystemTriggerType, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(triggertype, oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeTrigger_Impl: Sized + IBackgroundTrigger_Impl {
    fn FreshnessTime(&mut self) -> ::windows::core::Result<u32>;
    fn OneShot(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ITimeTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ITimeTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeTrigger_Vtbl {
        unsafe extern "system" fn FreshnessTime<Impl: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreshnessTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OneShot<Impl: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimeTrigger, BASE_OFFSET>(),
            FreshnessTime: FreshnessTime::<Impl, IMPL_OFFSET>,
            OneShot: OneShot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeTriggerFactory_Impl: Sized {
    fn Create(&mut self, freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<TimeTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimeTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ITimeTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITimeTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ITimeTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(freshnesstime, oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITimeTriggerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationActionTriggerFactory_Impl: Sized {
    fn Create(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotificationActionTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationActionTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IToastNotificationActionTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationActionTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationActionTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationActionTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IToastNotificationActionTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationActionTriggerFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationActionTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryChangedTriggerFactory_Impl: Sized {
    fn Create(&mut self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotificationHistoryChangedTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationHistoryChangedTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IToastNotificationHistoryChangedTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationHistoryChangedTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationHistoryChangedTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToastNotificationHistoryChangedTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IToastNotificationHistoryChangedTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationHistoryChangedTriggerFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationHistoryChangedTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
pub trait IUserNotificationChangedTriggerFactory_Impl: Sized {
    fn Create(&mut self, notificationkinds: super::super::UI::Notifications::NotificationKinds) -> ::windows::core::Result<UserNotificationChangedTrigger>;
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserNotificationChangedTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IUserNotificationChangedTriggerFactory";
}
#[cfg(all(feature = "UI_Notifications", feature = "implement_exclusive"))]
impl IUserNotificationChangedTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserNotificationChangedTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserNotificationChangedTriggerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IUserNotificationChangedTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationkinds: super::super::UI::Notifications::NotificationKinds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(notificationkinds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserNotificationChangedTriggerFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserNotificationChangedTriggerFactory as ::windows::core::Interface>::IID
    }
}
