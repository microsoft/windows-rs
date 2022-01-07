#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn SubscribedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SupportedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivitySensorTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IActivitySensorTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IActivitySensorTriggerVtbl {
    pub const fn new<Impl: IActivitySensorTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActivitySensorTriggerVtbl {
        unsafe extern "system" fn SubscribedActivities<Impl: IActivitySensorTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubscribedActivities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportInterval<Impl: IActivitySensorTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedActivities<Impl: IActivitySensorTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedActivities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumReportInterval<Impl: IActivitySensorTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinimumReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActivitySensorTrigger>, base.5, SubscribedActivities::<Impl, OFFSET>, ReportInterval::<Impl, OFFSET>, SupportedActivities::<Impl, OFFSET>, MinimumReportInterval::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorTriggerFactoryImpl: Sized {
    fn Create(&self, reportintervalinmilliseconds: u32) -> ::windows::core::Result<ActivitySensorTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivitySensorTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IActivitySensorTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IActivitySensorTriggerFactoryVtbl {
    pub const fn new<Impl: IActivitySensorTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActivitySensorTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IActivitySensorTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reportintervalinmilliseconds: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(reportintervalinmilliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActivitySensorTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAlarmApplicationManagerStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>>;
    fn GetAccessStatus(&self) -> ::windows::core::Result<AlarmAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAlarmApplicationManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAlarmApplicationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAlarmApplicationManagerStaticsVtbl {
    pub const fn new<Impl: IAlarmApplicationManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAlarmApplicationManagerStaticsVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IAlarmApplicationManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessStatus<Impl: IAlarmApplicationManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AlarmAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAlarmApplicationManagerStatics>, base.5, RequestAccessAsync::<Impl, OFFSET>, GetAccessStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn SetProviderInfo(&self, value: &::core::option::Option<AppBroadcastTriggerProviderInfo>) -> ::windows::core::Result<()>;
    fn ProviderInfo(&self) -> ::windows::core::Result<AppBroadcastTriggerProviderInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAppBroadcastTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastTriggerVtbl {
    pub const fn new<Impl: IAppBroadcastTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastTriggerVtbl {
        unsafe extern "system" fn SetProviderInfo<Impl: IAppBroadcastTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProviderInfo(&*(&value as *const <AppBroadcastTriggerProviderInfo as ::windows::core::Abi>::Abi as *const <AppBroadcastTriggerProviderInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProviderInfo<Impl: IAppBroadcastTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProviderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastTrigger>, base.5, SetProviderInfo::<Impl, OFFSET>, ProviderInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerFactoryImpl: Sized {
    fn CreateAppBroadcastTrigger(&self, providerkey: &::windows::core::HSTRING) -> ::windows::core::Result<AppBroadcastTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAppBroadcastTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastTriggerFactoryVtbl {
    pub const fn new<Impl: IAppBroadcastTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastTriggerFactoryVtbl {
        unsafe extern "system" fn CreateAppBroadcastTrigger<Impl: IAppBroadcastTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAppBroadcastTrigger(&*(&providerkey as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBroadcastTriggerFactory>, base.5, CreateAppBroadcastTrigger::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerProviderInfoImpl: Sized {
    fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLogoResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogoResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVideoKeyFrameInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn VideoKeyFrameInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetMaxVideoBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetMaxVideoWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoWidth(&self) -> ::windows::core::Result<u32>;
    fn SetMaxVideoHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoHeight(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastTriggerProviderInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAppBroadcastTriggerProviderInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastTriggerProviderInfoVtbl {
    pub const fn new<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBroadcastTriggerProviderInfoVtbl {
        unsafe extern "system" fn SetDisplayNameResource<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayNameResource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayNameResource<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayNameResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogoResource<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLogoResource(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogoResource<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LogoResource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoKeyFrameInterval<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoKeyFrameInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VideoKeyFrameInterval<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoKeyFrameInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxVideoBitrate<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxVideoBitrate(value).into()
        }
        unsafe extern "system" fn MaxVideoBitrate<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxVideoBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxVideoWidth<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxVideoWidth(value).into()
        }
        unsafe extern "system" fn MaxVideoWidth<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxVideoWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxVideoHeight<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxVideoHeight(value).into()
        }
        unsafe extern "system" fn MaxVideoHeight<Impl: IAppBroadcastTriggerProviderInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxVideoHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppBroadcastTriggerProviderInfo>,
            base.5,
            SetDisplayNameResource::<Impl, OFFSET>,
            DisplayNameResource::<Impl, OFFSET>,
            SetLogoResource::<Impl, OFFSET>,
            LogoResource::<Impl, OFFSET>,
            SetVideoKeyFrameInterval::<Impl, OFFSET>,
            VideoKeyFrameInterval::<Impl, OFFSET>,
            SetMaxVideoBitrate::<Impl, OFFSET>,
            MaxVideoBitrate::<Impl, OFFSET>,
            SetMaxVideoWidth::<Impl, OFFSET>,
            MaxVideoWidth::<Impl, OFFSET>,
            SetMaxVideoHeight::<Impl, OFFSET>,
            MaxVideoHeight::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>;
    fn RequestAsyncWithArguments(&self, arguments: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IApplicationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationTriggerVtbl {
    pub const fn new<Impl: IApplicationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationTriggerVtbl {
        unsafe extern "system" fn RequestAsync<Impl: IApplicationTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAsyncWithArguments<Impl: IApplicationTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAsyncWithArguments(&*(&arguments as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationTrigger>, base.5, RequestAsync::<Impl, OFFSET>, RequestAsyncWithArguments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationTriggerDetailsImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IApplicationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationTriggerDetailsVtbl {
    pub const fn new<Impl: IApplicationTriggerDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationTriggerDetailsVtbl {
        unsafe extern "system" fn Arguments<Impl: IApplicationTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationTriggerDetails>, base.5, Arguments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppointmentStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IAppointmentStoreNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IAppointmentStoreNotificationTriggerVtbl {
    pub const fn new<Impl: IAppointmentStoreNotificationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppointmentStoreNotificationTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppointmentStoreNotificationTrigger>, base.5)
    }
}
pub trait IBackgroundConditionImpl: Sized {}
impl ::windows::core::RuntimeName for IBackgroundCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundCondition";
}
impl IBackgroundConditionVtbl {
    pub const fn new<Impl: IBackgroundConditionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundConditionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundCondition>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundExecutionManagerStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>;
    fn RequestAccessForApplicationAsync(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>;
    fn RemoveAccess(&self) -> ::windows::core::Result<()>;
    fn RemoveAccessForApplication(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetAccessStatus(&self) -> ::windows::core::Result<BackgroundAccessStatus>;
    fn GetAccessStatusForApplication(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundExecutionManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundExecutionManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundExecutionManagerStaticsVtbl {
    pub const fn new<Impl: IBackgroundExecutionManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundExecutionManagerStaticsVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IBackgroundExecutionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessForApplicationAsync<Impl: IBackgroundExecutionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessForApplicationAsync(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccess<Impl: IBackgroundExecutionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAccess().into()
        }
        unsafe extern "system" fn RemoveAccessForApplication<Impl: IBackgroundExecutionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAccessForApplication(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAccessStatus<Impl: IBackgroundExecutionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessStatusForApplication<Impl: IBackgroundExecutionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessStatusForApplication(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundExecutionManagerStatics>, base.5, RequestAccessAsync::<Impl, OFFSET>, RequestAccessForApplicationAsync::<Impl, OFFSET>, RemoveAccess::<Impl, OFFSET>, RemoveAccessForApplication::<Impl, OFFSET>, GetAccessStatus::<Impl, OFFSET>, GetAccessStatusForApplication::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundExecutionManagerStatics2Impl: Sized {
    fn RequestAccessKindAsync(&self, requestedaccess: BackgroundAccessRequestKind, reason: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundExecutionManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundExecutionManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundExecutionManagerStatics2Vtbl {
    pub const fn new<Impl: IBackgroundExecutionManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundExecutionManagerStatics2Vtbl {
        unsafe extern "system" fn RequestAccessKindAsync<Impl: IBackgroundExecutionManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessKindAsync(requestedaccess, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundExecutionManagerStatics2>, base.5, RequestAccessKindAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundExecutionManagerStatics3Impl: Sized {
    fn RequestAccessKindForModernStandbyAsync(&self, requestedaccess: BackgroundAccessRequestKind, reason: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetAccessStatusForModernStandby(&self) -> ::windows::core::Result<BackgroundAccessStatus>;
    fn GetAccessStatusForModernStandbyForApplication(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundExecutionManagerStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundExecutionManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundExecutionManagerStatics3Vtbl {
    pub const fn new<Impl: IBackgroundExecutionManagerStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundExecutionManagerStatics3Vtbl {
        unsafe extern "system" fn RequestAccessKindForModernStandbyAsync<Impl: IBackgroundExecutionManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessKindForModernStandbyAsync(requestedaccess, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessStatusForModernStandby<Impl: IBackgroundExecutionManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessStatusForModernStandby() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessStatusForModernStandbyForApplication<Impl: IBackgroundExecutionManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessStatusForModernStandbyForApplication(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundExecutionManagerStatics3>, base.5, RequestAccessKindForModernStandbyAsync::<Impl, OFFSET>, GetAccessStatusForModernStandby::<Impl, OFFSET>, GetAccessStatusForModernStandbyForApplication::<Impl, OFFSET>)
    }
}
pub trait IBackgroundTaskImpl: Sized {
    fn Run(&self, taskinstance: &::core::option::Option<IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBackgroundTask {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTask";
}
impl IBackgroundTaskVtbl {
    pub const fn new<Impl: IBackgroundTaskImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskVtbl {
        unsafe extern "system" fn Run<Impl: IBackgroundTaskImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Run(&*(&taskinstance as *const <IBackgroundTaskInstance as ::windows::core::Abi>::Abi as *const <IBackgroundTaskInstance as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTask>, base.5, Run::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilderImpl: Sized {
    fn SetTaskEntryPoint(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TaskEntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTrigger(&self, trigger: &::core::option::Option<IBackgroundTrigger>) -> ::windows::core::Result<()>;
    fn AddCondition(&self, condition: &::core::option::Option<IBackgroundCondition>) -> ::windows::core::Result<()>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Register(&self) -> ::windows::core::Result<BackgroundTaskRegistration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilderVtbl {
    pub const fn new<Impl: IBackgroundTaskBuilderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskBuilderVtbl {
        unsafe extern "system" fn SetTaskEntryPoint<Impl: IBackgroundTaskBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTaskEntryPoint(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TaskEntryPoint<Impl: IBackgroundTaskBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskEntryPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrigger<Impl: IBackgroundTaskBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, trigger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTrigger(&*(&trigger as *const <IBackgroundTrigger as ::windows::core::Abi>::Abi as *const <IBackgroundTrigger as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddCondition<Impl: IBackgroundTaskBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddCondition(&*(&condition as *const <IBackgroundCondition as ::windows::core::Abi>::Abi as *const <IBackgroundCondition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetName<Impl: IBackgroundTaskBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IBackgroundTaskBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: IBackgroundTaskBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Register() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskBuilder>, base.5, SetTaskEntryPoint::<Impl, OFFSET>, TaskEntryPoint::<Impl, OFFSET>, SetTrigger::<Impl, OFFSET>, AddCondition::<Impl, OFFSET>, SetName::<Impl, OFFSET>, Name::<Impl, OFFSET>, Register::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder2Impl: Sized + IBackgroundTaskBuilderImpl {
    fn SetCancelOnConditionLoss(&self, value: bool) -> ::windows::core::Result<()>;
    fn CancelOnConditionLoss(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder2";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder2Vtbl {
    pub const fn new<Impl: IBackgroundTaskBuilder2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskBuilder2Vtbl {
        unsafe extern "system" fn SetCancelOnConditionLoss<Impl: IBackgroundTaskBuilder2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCancelOnConditionLoss(value).into()
        }
        unsafe extern "system" fn CancelOnConditionLoss<Impl: IBackgroundTaskBuilder2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelOnConditionLoss() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskBuilder2>, base.5, SetCancelOnConditionLoss::<Impl, OFFSET>, CancelOnConditionLoss::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder3Impl: Sized + IBackgroundTaskBuilderImpl {
    fn SetIsNetworkRequested(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsNetworkRequested(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder3";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder3Vtbl {
    pub const fn new<Impl: IBackgroundTaskBuilder3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskBuilder3Vtbl {
        unsafe extern "system" fn SetIsNetworkRequested<Impl: IBackgroundTaskBuilder3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsNetworkRequested(value).into()
        }
        unsafe extern "system" fn IsNetworkRequested<Impl: IBackgroundTaskBuilder3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsNetworkRequested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskBuilder3>, base.5, SetIsNetworkRequested::<Impl, OFFSET>, IsNetworkRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder4Impl: Sized + IBackgroundTaskBuilderImpl {
    fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
    fn SetTaskGroup(&self, value: &::core::option::Option<BackgroundTaskRegistrationGroup>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder4 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder4";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder4Vtbl {
    pub const fn new<Impl: IBackgroundTaskBuilder4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskBuilder4Vtbl {
        unsafe extern "system" fn TaskGroup<Impl: IBackgroundTaskBuilder4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskGroup<Impl: IBackgroundTaskBuilder4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTaskGroup(&*(&value as *const <BackgroundTaskRegistrationGroup as ::windows::core::Abi>::Abi as *const <BackgroundTaskRegistrationGroup as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskBuilder4>, base.5, TaskGroup::<Impl, OFFSET>, SetTaskGroup::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder5Impl: Sized {
    fn SetTaskEntryPointClsid(&self, taskentrypoint: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskBuilder5 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskBuilder5";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskBuilder5Vtbl {
    pub const fn new<Impl: IBackgroundTaskBuilder5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskBuilder5Vtbl {
        unsafe extern "system" fn SetTaskEntryPointClsid<Impl: IBackgroundTaskBuilder5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskentrypoint: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTaskEntryPointClsid(&*(&taskentrypoint as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskBuilder5>, base.5, SetTaskEntryPointClsid::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskCompletedEventArgsImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CheckResult(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskCompletedEventArgsVtbl {
    pub const fn new<Impl: IBackgroundTaskCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskCompletedEventArgsVtbl {
        unsafe extern "system" fn InstanceId<Impl: IBackgroundTaskCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckResult<Impl: IBackgroundTaskCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CheckResult().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskCompletedEventArgs>, base.5, InstanceId::<Impl, OFFSET>, CheckResult::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskDeferralVtbl {
    pub const fn new<Impl: IBackgroundTaskDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IBackgroundTaskDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
pub trait IBackgroundTaskInstanceImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration>;
    fn Progress(&self) -> ::windows::core::Result<u32>;
    fn SetProgress(&self, value: u32) -> ::windows::core::Result<()>;
    fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Canceled(&self, cancelhandler: &::core::option::Option<BackgroundTaskCanceledEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SuspendedCount(&self) -> ::windows::core::Result<u32>;
    fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral>;
}
impl ::windows::core::RuntimeName for IBackgroundTaskInstance {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance";
}
impl IBackgroundTaskInstanceVtbl {
    pub const fn new<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskInstanceVtbl {
        unsafe extern "system" fn InstanceId<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Task<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgress<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProgress(value).into()
        }
        unsafe extern "system" fn TriggerDetails<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Canceled<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cancelhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Canceled(&*(&cancelhandler as *const <BackgroundTaskCanceledEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskCanceledEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCanceled(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuspendedCount<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuspendedCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBackgroundTaskInstanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskInstance>, base.5, InstanceId::<Impl, OFFSET>, Task::<Impl, OFFSET>, Progress::<Impl, OFFSET>, SetProgress::<Impl, OFFSET>, TriggerDetails::<Impl, OFFSET>, Canceled::<Impl, OFFSET>, RemoveCanceled::<Impl, OFFSET>, SuspendedCount::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
pub trait IBackgroundTaskInstance2Impl: Sized + IBackgroundTaskInstanceImpl {
    fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IBackgroundTaskInstance2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance2";
}
impl IBackgroundTaskInstance2Vtbl {
    pub const fn new<Impl: IBackgroundTaskInstance2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskInstance2Vtbl {
        unsafe extern "system" fn GetThrottleCount<Impl: IBackgroundTaskInstance2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetThrottleCount(counter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskInstance2>, base.5, GetThrottleCount::<Impl, OFFSET>)
    }
}
pub trait IBackgroundTaskInstance4Impl: Sized + IBackgroundTaskInstanceImpl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
impl ::windows::core::RuntimeName for IBackgroundTaskInstance4 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance4";
}
impl IBackgroundTaskInstance4Vtbl {
    pub const fn new<Impl: IBackgroundTaskInstance4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskInstance4Vtbl {
        unsafe extern "system" fn User<Impl: IBackgroundTaskInstance4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskInstance4>, base.5, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskProgressEventArgsImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Progress(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskProgressEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskProgressEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskProgressEventArgsVtbl {
    pub const fn new<Impl: IBackgroundTaskProgressEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskProgressEventArgsVtbl {
        unsafe extern "system" fn InstanceId<Impl: IBackgroundTaskProgressEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IBackgroundTaskProgressEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskProgressEventArgs>, base.5, InstanceId::<Impl, OFFSET>, Progress::<Impl, OFFSET>)
    }
}
pub trait IBackgroundTaskRegistrationImpl: Sized {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Progress(&self, handler: &::core::option::Option<BackgroundTaskProgressEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgress(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<BackgroundTaskCompletedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration";
}
impl IBackgroundTaskRegistrationVtbl {
    pub const fn new<Impl: IBackgroundTaskRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskRegistrationVtbl {
        unsafe extern "system" fn TaskId<Impl: IBackgroundTaskRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IBackgroundTaskRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IBackgroundTaskRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Progress(&*(&handler as *const <BackgroundTaskProgressEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskProgressEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProgress<Impl: IBackgroundTaskRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveProgress(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IBackgroundTaskRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <BackgroundTaskCompletedEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskCompletedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IBackgroundTaskRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unregister<Impl: IBackgroundTaskRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Unregister(canceltask).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskRegistration>, base.5, TaskId::<Impl, OFFSET>, Name::<Impl, OFFSET>, Progress::<Impl, OFFSET>, RemoveProgress::<Impl, OFFSET>, Completed::<Impl, OFFSET>, RemoveCompleted::<Impl, OFFSET>, Unregister::<Impl, OFFSET>)
    }
}
pub trait IBackgroundTaskRegistration2Impl: Sized + IBackgroundTaskRegistrationImpl {
    fn Trigger(&self) -> ::windows::core::Result<IBackgroundTrigger>;
}
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration2";
}
impl IBackgroundTaskRegistration2Vtbl {
    pub const fn new<Impl: IBackgroundTaskRegistration2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskRegistration2Vtbl {
        unsafe extern "system" fn Trigger<Impl: IBackgroundTaskRegistration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskRegistration2>, base.5, Trigger::<Impl, OFFSET>)
    }
}
pub trait IBackgroundTaskRegistration3Impl: Sized + IBackgroundTaskRegistrationImpl {
    fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration3";
}
impl IBackgroundTaskRegistration3Vtbl {
    pub const fn new<Impl: IBackgroundTaskRegistration3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskRegistration3Vtbl {
        unsafe extern "system" fn TaskGroup<Impl: IBackgroundTaskRegistration3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskRegistration3>, base.5, TaskGroup::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationGroupImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BackgroundActivated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllTasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, BackgroundTaskRegistration>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistrationGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistrationGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskRegistrationGroupVtbl {
    pub const fn new<Impl: IBackgroundTaskRegistrationGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskRegistrationGroupVtbl {
        unsafe extern "system" fn Id<Impl: IBackgroundTaskRegistrationGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IBackgroundTaskRegistrationGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundActivated<Impl: IBackgroundTaskRegistrationGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundActivated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackgroundActivated<Impl: IBackgroundTaskRegistrationGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBackgroundActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllTasks<Impl: IBackgroundTaskRegistrationGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllTasks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskRegistrationGroup>, base.5, Id::<Impl, OFFSET>, Name::<Impl, OFFSET>, BackgroundActivated::<Impl, OFFSET>, RemoveBackgroundActivated::<Impl, OFFSET>, AllTasks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationGroupFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
    fn CreateWithName(&self, id: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistrationGroupFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistrationGroupFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskRegistrationGroupFactoryVtbl {
    pub const fn new<Impl: IBackgroundTaskRegistrationGroupFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskRegistrationGroupFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IBackgroundTaskRegistrationGroupFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithName<Impl: IBackgroundTaskRegistrationGroupFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithName(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskRegistrationGroupFactory>, base.5, Create::<Impl, OFFSET>, CreateWithName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationStaticsImpl: Sized {
    fn AllTasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, IBackgroundTaskRegistration>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistrationStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistrationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskRegistrationStaticsVtbl {
    pub const fn new<Impl: IBackgroundTaskRegistrationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskRegistrationStaticsVtbl {
        unsafe extern "system" fn AllTasks<Impl: IBackgroundTaskRegistrationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllTasks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskRegistrationStatics>, base.5, AllTasks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationStatics2Impl: Sized {
    fn AllTaskGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BackgroundTaskRegistrationGroup>>;
    fn GetTaskGroup(&self, groupid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistrationStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistrationStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundTaskRegistrationStatics2Vtbl {
    pub const fn new<Impl: IBackgroundTaskRegistrationStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTaskRegistrationStatics2Vtbl {
        unsafe extern "system" fn AllTaskGroups<Impl: IBackgroundTaskRegistrationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllTaskGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTaskGroup<Impl: IBackgroundTaskRegistrationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, groupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTaskGroup(&*(&groupid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTaskRegistrationStatics2>, base.5, AllTaskGroups::<Impl, OFFSET>, GetTaskGroup::<Impl, OFFSET>)
    }
}
pub trait IBackgroundTriggerImpl: Sized {}
impl ::windows::core::RuntimeName for IBackgroundTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTrigger";
}
impl IBackgroundTriggerVtbl {
    pub const fn new<Impl: IBackgroundTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundWorkCostStaticsImpl: Sized {
    fn CurrentBackgroundWorkCost(&self) -> ::windows::core::Result<BackgroundWorkCostValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackgroundWorkCostStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundWorkCostStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBackgroundWorkCostStaticsVtbl {
    pub const fn new<Impl: IBackgroundWorkCostStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBackgroundWorkCostStaticsVtbl {
        unsafe extern "system" fn CurrentBackgroundWorkCost<Impl: IBackgroundWorkCostStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundWorkCostValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentBackgroundWorkCost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBackgroundWorkCostStatics>, base.5, CurrentBackgroundWorkCost::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn Advertisement(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBluetoothLEAdvertisementPublisherTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementPublisherTriggerVtbl {
    pub const fn new<Impl: IBluetoothLEAdvertisementPublisherTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEAdvertisementPublisherTriggerVtbl {
        unsafe extern "system" fn Advertisement<Impl: IBluetoothLEAdvertisementPublisherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Advertisement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisherTrigger>, base.5, Advertisement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherTrigger2Impl: Sized {
    fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetPreferredTransmitPowerLevelInDBm(&self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn UseExtendedFormat(&self) -> ::windows::core::Result<bool>;
    fn SetUseExtendedFormat(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAnonymous(&self) -> ::windows::core::Result<bool>;
    fn SetIsAnonymous(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeTransmitPowerLevel(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTrigger2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBluetoothLEAdvertisementPublisherTrigger2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementPublisherTrigger2Vtbl {
    pub const fn new<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEAdvertisementPublisherTrigger2Vtbl {
        unsafe extern "system" fn PreferredTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredTransmitPowerLevelInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreferredTransmitPowerLevelInDBm(&*(&value as *const <super::super::Foundation::IReference<i16> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i16> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UseExtendedFormat<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseExtendedFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseExtendedFormat<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUseExtendedFormat(value).into()
        }
        unsafe extern "system" fn IsAnonymous<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAnonymous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAnonymous<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsAnonymous(value).into()
        }
        unsafe extern "system" fn IncludeTransmitPowerLevel<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludeTransmitPowerLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeTransmitPowerLevel<Impl: IBluetoothLEAdvertisementPublisherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIncludeTransmitPowerLevel(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisherTrigger2>, base.5, PreferredTransmitPowerLevelInDBm::<Impl, OFFSET>, SetPreferredTransmitPowerLevelInDBm::<Impl, OFFSET>, UseExtendedFormat::<Impl, OFFSET>, SetUseExtendedFormat::<Impl, OFFSET>, IsAnonymous::<Impl, OFFSET>, SetIsAnonymous::<Impl, OFFSET>, IncludeTransmitPowerLevel::<Impl, OFFSET>, SetIncludeTransmitPowerLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn MinSamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MaxSamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MinOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MaxOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>;
    fn SetSignalStrengthFilter(&self, value: &::core::option::Option<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>) -> ::windows::core::Result<()>;
    fn AdvertisementFilter(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>;
    fn SetAdvertisementFilter(&self, value: &::core::option::Option<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBluetoothLEAdvertisementWatcherTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementWatcherTriggerVtbl {
    pub const fn new<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEAdvertisementWatcherTriggerVtbl {
        unsafe extern "system" fn MinSamplingInterval<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinSamplingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSamplingInterval<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSamplingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinOutOfRangeTimeout<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinOutOfRangeTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxOutOfRangeTimeout<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxOutOfRangeTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalStrengthFilter<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignalStrengthFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalStrengthFilter<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSignalStrengthFilter(&*(&value as *const <super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdvertisementFilter<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdvertisementFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisementFilter<Impl: IBluetoothLEAdvertisementWatcherTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAdvertisementFilter(&*(&value as *const <super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementWatcherTrigger>, base.5, MinSamplingInterval::<Impl, OFFSET>, MaxSamplingInterval::<Impl, OFFSET>, MinOutOfRangeTimeout::<Impl, OFFSET>, MaxOutOfRangeTimeout::<Impl, OFFSET>, SignalStrengthFilter::<Impl, OFFSET>, SetSignalStrengthFilter::<Impl, OFFSET>, AdvertisementFilter::<Impl, OFFSET>, SetAdvertisementFilter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherTrigger2Impl: Sized {
    fn AllowExtendedAdvertisements(&self) -> ::windows::core::Result<bool>;
    fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherTrigger2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBluetoothLEAdvertisementWatcherTrigger2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementWatcherTrigger2Vtbl {
    pub const fn new<Impl: IBluetoothLEAdvertisementWatcherTrigger2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEAdvertisementWatcherTrigger2Vtbl {
        unsafe extern "system" fn AllowExtendedAdvertisements<Impl: IBluetoothLEAdvertisementWatcherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowExtendedAdvertisements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowExtendedAdvertisements<Impl: IBluetoothLEAdvertisementWatcherTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowExtendedAdvertisements(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementWatcherTrigger2>, base.5, AllowExtendedAdvertisements::<Impl, OFFSET>, SetAllowExtendedAdvertisements::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICachedFileUpdaterTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICachedFileUpdaterTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ICachedFileUpdaterTriggerVtbl {
    pub const fn new<Impl: ICachedFileUpdaterTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICachedFileUpdaterTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICachedFileUpdaterTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterTriggerDetailsImpl: Sized {
    fn UpdateTarget(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileTarget>;
    fn UpdateRequest(&self) -> ::windows::core::Result<super::super::Storage::Provider::FileUpdateRequest>;
    fn CanRequestUserInput(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICachedFileUpdaterTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICachedFileUpdaterTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ICachedFileUpdaterTriggerDetailsVtbl {
    pub const fn new<Impl: ICachedFileUpdaterTriggerDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICachedFileUpdaterTriggerDetailsVtbl {
        unsafe extern "system" fn UpdateTarget<Impl: ICachedFileUpdaterTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Storage::Provider::CachedFileTarget) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateRequest<Impl: ICachedFileUpdaterTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRequestUserInput<Impl: ICachedFileUpdaterTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanRequestUserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICachedFileUpdaterTriggerDetails>, base.5, UpdateTarget::<Impl, OFFSET>, UpdateRequest::<Impl, OFFSET>, CanRequestUserInput::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IChatMessageNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageNotificationTriggerVtbl {
    pub const fn new<Impl: IChatMessageNotificationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChatMessageNotificationTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IChatMessageNotificationTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageReceivedNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChatMessageReceivedNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IChatMessageReceivedNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IChatMessageReceivedNotificationTriggerVtbl {
    pub const fn new<Impl: IChatMessageReceivedNotificationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChatMessageReceivedNotificationTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IChatMessageReceivedNotificationTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommunicationBlockingAppSetAsActiveTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommunicationBlockingAppSetAsActiveTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICommunicationBlockingAppSetAsActiveTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ICommunicationBlockingAppSetAsActiveTriggerVtbl {
    pub const fn new<Impl: ICommunicationBlockingAppSetAsActiveTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommunicationBlockingAppSetAsActiveTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommunicationBlockingAppSetAsActiveTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IContactStoreNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IContactStoreNotificationTriggerVtbl {
    pub const fn new<Impl: IContactStoreNotificationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactStoreNotificationTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactStoreNotificationTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPrefetchTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn WaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentPrefetchTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IContentPrefetchTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IContentPrefetchTriggerVtbl {
    pub const fn new<Impl: IContentPrefetchTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContentPrefetchTriggerVtbl {
        unsafe extern "system" fn WaitInterval<Impl: IContentPrefetchTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContentPrefetchTrigger>, base.5, WaitInterval::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPrefetchTriggerFactoryImpl: Sized {
    fn Create(&self, waitinterval: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ContentPrefetchTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContentPrefetchTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IContentPrefetchTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IContentPrefetchTriggerFactoryVtbl {
    pub const fn new<Impl: IContentPrefetchTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContentPrefetchTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IContentPrefetchTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waitinterval: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&waitinterval as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContentPrefetchTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSystemEventTriggerImpl: Sized {
    fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recurrence(&self) -> ::windows::core::Result<CustomSystemEventTriggerRecurrence>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomSystemEventTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICustomSystemEventTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomSystemEventTriggerVtbl {
    pub const fn new<Impl: ICustomSystemEventTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomSystemEventTriggerVtbl {
        unsafe extern "system" fn TriggerId<Impl: ICustomSystemEventTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recurrence<Impl: ICustomSystemEventTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CustomSystemEventTriggerRecurrence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Recurrence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomSystemEventTrigger>, base.5, TriggerId::<Impl, OFFSET>, Recurrence::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSystemEventTriggerFactoryImpl: Sized {
    fn Create(&self, triggerid: &::windows::core::HSTRING, recurrence: CustomSystemEventTriggerRecurrence) -> ::windows::core::Result<CustomSystemEventTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICustomSystemEventTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ICustomSystemEventTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICustomSystemEventTriggerFactoryVtbl {
    pub const fn new<Impl: ICustomSystemEventTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICustomSystemEventTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ICustomSystemEventTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triggerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recurrence: CustomSystemEventTriggerRecurrence, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&triggerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), recurrence) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICustomSystemEventTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceConnectionChangeTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanMaintainConnection(&self) -> ::windows::core::Result<bool>;
    fn MaintainConnection(&self) -> ::windows::core::Result<bool>;
    fn SetMaintainConnection(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceConnectionChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceConnectionChangeTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceConnectionChangeTriggerVtbl {
    pub const fn new<Impl: IDeviceConnectionChangeTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceConnectionChangeTriggerVtbl {
        unsafe extern "system" fn DeviceId<Impl: IDeviceConnectionChangeTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMaintainConnection<Impl: IDeviceConnectionChangeTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanMaintainConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaintainConnection<Impl: IDeviceConnectionChangeTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaintainConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintainConnection<Impl: IDeviceConnectionChangeTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaintainConnection(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceConnectionChangeTrigger>, base.5, DeviceId::<Impl, OFFSET>, CanMaintainConnection::<Impl, OFFSET>, MaintainConnection::<Impl, OFFSET>, SetMaintainConnection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceConnectionChangeTriggerStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceConnectionChangeTriggerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceConnectionChangeTriggerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceConnectionChangeTriggerStaticsVtbl {
    pub const fn new<Impl: IDeviceConnectionChangeTriggerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceConnectionChangeTriggerStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IDeviceConnectionChangeTriggerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceConnectionChangeTriggerStatics>, base.5, FromIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDeviceManufacturerNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn TriggerQualifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OneShot(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceManufacturerNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceManufacturerNotificationTrigger";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IDeviceManufacturerNotificationTriggerVtbl {
    pub const fn new<Impl: IDeviceManufacturerNotificationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceManufacturerNotificationTriggerVtbl {
        unsafe extern "system" fn TriggerQualifier<Impl: IDeviceManufacturerNotificationTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerQualifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OneShot<Impl: IDeviceManufacturerNotificationTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceManufacturerNotificationTrigger>, base.5, TriggerQualifier::<Impl, OFFSET>, OneShot::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDeviceManufacturerNotificationTriggerFactoryImpl: Sized {
    fn Create(&self, triggerqualifier: &::windows::core::HSTRING, oneshot: bool) -> ::windows::core::Result<DeviceManufacturerNotificationTrigger>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceManufacturerNotificationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceManufacturerNotificationTriggerFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IDeviceManufacturerNotificationTriggerFactoryVtbl {
    pub const fn new<Impl: IDeviceManufacturerNotificationTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceManufacturerNotificationTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDeviceManufacturerNotificationTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triggerqualifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&triggerqualifier as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceManufacturerNotificationTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceServicingTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn RequestAsyncSimple(&self, deviceid: &::windows::core::HSTRING, expectedduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
    fn RequestAsyncWithArguments(&self, deviceid: &::windows::core::HSTRING, expectedduration: &super::super::Foundation::TimeSpan, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceServicingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceServicingTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceServicingTriggerVtbl {
    pub const fn new<Impl: IDeviceServicingTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceServicingTriggerVtbl {
        unsafe extern "system" fn RequestAsyncSimple<Impl: IDeviceServicingTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAsyncSimple(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&expectedduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAsyncWithArguments<Impl: IDeviceServicingTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceServicingTrigger>, base.5, RequestAsyncSimple::<Impl, OFFSET>, RequestAsyncWithArguments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceUseTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn RequestAsyncSimple(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
    fn RequestAsyncWithArguments(&self, deviceid: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceUseTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceUseTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceUseTriggerVtbl {
    pub const fn new<Impl: IDeviceUseTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceUseTriggerVtbl {
        unsafe extern "system" fn RequestAsyncSimple<Impl: IDeviceUseTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAsyncSimple(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAsyncWithArguments<Impl: IDeviceUseTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAsyncWithArguments(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceUseTrigger>, base.5, RequestAsyncSimple::<Impl, OFFSET>, RequestAsyncWithArguments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IDeviceWatcherTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceWatcherTriggerVtbl {
    pub const fn new<Impl: IDeviceWatcherTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDeviceWatcherTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDeviceWatcherTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailStoreNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IEmailStoreNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailStoreNotificationTriggerVtbl {
    pub const fn new<Impl: IEmailStoreNotificationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEmailStoreNotificationTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEmailStoreNotificationTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn Characteristic(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattCharacteristicNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicNotificationTriggerVtbl {
    pub const fn new<Impl: IGattCharacteristicNotificationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGattCharacteristicNotificationTriggerVtbl {
        unsafe extern "system" fn Characteristic<Impl: IGattCharacteristicNotificationTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Characteristic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGattCharacteristicNotificationTrigger>, base.5, Characteristic::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTrigger2Impl: Sized {
    fn EventTriggeringMode(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTrigger2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattCharacteristicNotificationTrigger2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicNotificationTrigger2Vtbl {
    pub const fn new<Impl: IGattCharacteristicNotificationTrigger2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGattCharacteristicNotificationTrigger2Vtbl {
        unsafe extern "system" fn EventTriggeringMode<Impl: IGattCharacteristicNotificationTrigger2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EventTriggeringMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGattCharacteristicNotificationTrigger2>, base.5, EventTriggeringMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerFactoryImpl: Sized {
    fn Create(&self, characteristic: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>) -> ::windows::core::Result<GattCharacteristicNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattCharacteristicNotificationTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicNotificationTriggerFactoryVtbl {
    pub const fn new<Impl: IGattCharacteristicNotificationTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGattCharacteristicNotificationTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGattCharacteristicNotificationTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, characteristic: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&characteristic as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGattCharacteristicNotificationTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerFactory2Impl: Sized {
    fn CreateWithEventTriggeringMode(&self, characteristic: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::Result<GattCharacteristicNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerFactory2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattCharacteristicNotificationTriggerFactory2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicNotificationTriggerFactory2Vtbl {
    pub const fn new<Impl: IGattCharacteristicNotificationTriggerFactory2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGattCharacteristicNotificationTriggerFactory2Vtbl {
        unsafe extern "system" fn CreateWithEventTriggeringMode<Impl: IGattCharacteristicNotificationTriggerFactory2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, characteristic: ::windows::core::RawPtr, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithEventTriggeringMode(&*(&characteristic as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic as ::windows::core::DefaultType>::DefaultType), eventtriggeringmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGattCharacteristicNotificationTriggerFactory2>, base.5, CreateWithEventTriggeringMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderTriggerImpl: Sized {
    fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService>;
    fn SetAdvertisingParameters(&self, value: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>) -> ::windows::core::Result<()>;
    fn AdvertisingParameters(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattServiceProviderTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderTriggerVtbl {
    pub const fn new<Impl: IGattServiceProviderTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGattServiceProviderTriggerVtbl {
        unsafe extern "system" fn TriggerId<Impl: IGattServiceProviderTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Impl: IGattServiceProviderTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisingParameters<Impl: IGattServiceProviderTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAdvertisingParameters(&*(&value as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters as ::windows::core::Abi>::Abi as *const <super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdvertisingParameters<Impl: IGattServiceProviderTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdvertisingParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGattServiceProviderTrigger>, base.5, TriggerId::<Impl, OFFSET>, Service::<Impl, OFFSET>, SetAdvertisingParameters::<Impl, OFFSET>, AdvertisingParameters::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderTriggerResultImpl: Sized {
    fn Trigger(&self) -> ::windows::core::Result<GattServiceProviderTrigger>;
    fn Error(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderTriggerResult {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattServiceProviderTriggerResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderTriggerResultVtbl {
    pub const fn new<Impl: IGattServiceProviderTriggerResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGattServiceProviderTriggerResultVtbl {
        unsafe extern "system" fn Trigger<Impl: IGattServiceProviderTriggerResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IGattServiceProviderTriggerResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::BluetoothError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGattServiceProviderTriggerResult>, base.5, Trigger::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderTriggerStaticsImpl: Sized {
    fn CreateAsync(&self, triggerid: &::windows::core::HSTRING, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderTriggerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGattServiceProviderTriggerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderTriggerStaticsVtbl {
    pub const fn new<Impl: IGattServiceProviderTriggerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGattServiceProviderTriggerStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IGattServiceProviderTriggerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triggerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&triggerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGattServiceProviderTriggerStatics>, base.5, CreateAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn MonitoringScope(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::VisitMonitoringScope>;
    fn SetMonitoringScope(&self, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeovisitTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IGeovisitTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IGeovisitTriggerVtbl {
    pub const fn new<Impl: IGeovisitTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeovisitTriggerVtbl {
        unsafe extern "system" fn MonitoringScope<Impl: IGeovisitTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MonitoringScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitoringScope<Impl: IGeovisitTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMonitoringScope(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeovisitTrigger>, base.5, MonitoringScope::<Impl, OFFSET>, SetMonitoringScope::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn TriggerType(&self) -> ::windows::core::Result<LocationTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ILocationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ILocationTriggerVtbl {
    pub const fn new<Impl: ILocationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILocationTriggerVtbl {
        unsafe extern "system" fn TriggerType<Impl: ILocationTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LocationTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILocationTrigger>, base.5, TriggerType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocationTriggerFactoryImpl: Sized {
    fn Create(&self, triggertype: LocationTriggerType) -> ::windows::core::Result<LocationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ILocationTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILocationTriggerFactoryVtbl {
    pub const fn new<Impl: ILocationTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILocationTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ILocationTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triggertype: LocationTriggerType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(triggertype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILocationTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMaintenanceTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn FreshnessTime(&self) -> ::windows::core::Result<u32>;
    fn OneShot(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMaintenanceTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IMaintenanceTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IMaintenanceTriggerVtbl {
    pub const fn new<Impl: IMaintenanceTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMaintenanceTriggerVtbl {
        unsafe extern "system" fn FreshnessTime<Impl: IMaintenanceTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreshnessTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OneShot<Impl: IMaintenanceTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMaintenanceTrigger>, base.5, FreshnessTime::<Impl, OFFSET>, OneShot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMaintenanceTriggerFactoryImpl: Sized {
    fn Create(&self, freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<MaintenanceTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMaintenanceTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IMaintenanceTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMaintenanceTriggerFactoryVtbl {
    pub const fn new<Impl: IMaintenanceTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMaintenanceTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IMaintenanceTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(freshnesstime, oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMaintenanceTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProcessingTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>;
    fn RequestAsyncWithArguments(&self, arguments: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaProcessingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IMediaProcessingTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaProcessingTriggerVtbl {
    pub const fn new<Impl: IMediaProcessingTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaProcessingTriggerVtbl {
        unsafe extern "system" fn RequestAsync<Impl: IMediaProcessingTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAsyncWithArguments<Impl: IMediaProcessingTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAsyncWithArguments(&*(&arguments as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaProcessingTrigger>, base.5, RequestAsync::<Impl, OFFSET>, RequestAsyncWithArguments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorHotspotAuthenticationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorHotspotAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.INetworkOperatorHotspotAuthenticationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorHotspotAuthenticationTriggerVtbl {
    pub const fn new<Impl: INetworkOperatorHotspotAuthenticationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkOperatorHotspotAuthenticationTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkOperatorHotspotAuthenticationTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.INetworkOperatorNotificationTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorNotificationTriggerVtbl {
    pub const fn new<Impl: INetworkOperatorNotificationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkOperatorNotificationTriggerVtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: INetworkOperatorNotificationTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkOperatorNotificationTrigger>, base.5, NetworkAccountId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorNotificationTriggerFactoryImpl: Sized {
    fn Create(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<NetworkOperatorNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorNotificationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.INetworkOperatorNotificationTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorNotificationTriggerFactoryVtbl {
    pub const fn new<Impl: INetworkOperatorNotificationTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkOperatorNotificationTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: INetworkOperatorNotificationTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkOperatorNotificationTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn OneShot(&self) -> ::windows::core::Result<bool>;
    fn TriggerType(&self) -> ::windows::core::Result<super::Calls::Background::PhoneTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IPhoneTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneTriggerVtbl {
    pub const fn new<Impl: IPhoneTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneTriggerVtbl {
        unsafe extern "system" fn OneShot<Impl: IPhoneTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TriggerType<Impl: IPhoneTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Calls::Background::PhoneTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneTrigger>, base.5, OneShot::<Impl, OFFSET>, TriggerType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneTriggerFactoryImpl: Sized {
    fn Create(&self, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool) -> ::windows::core::Result<PhoneTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IPhoneTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneTriggerFactoryVtbl {
    pub const fn new<Impl: IPhoneTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPhoneTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPhoneTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(r#type, oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPhoneTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationTriggerFactoryImpl: Sized {
    fn Create(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<PushNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPushNotificationTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IPushNotificationTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPushNotificationTriggerFactoryVtbl {
    pub const fn new<Impl: IPushNotificationTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPushNotificationTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPushNotificationTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPushNotificationTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageAvailableTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRcsEndUserMessageAvailableTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IRcsEndUserMessageAvailableTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IRcsEndUserMessageAvailableTriggerVtbl {
    pub const fn new<Impl: IRcsEndUserMessageAvailableTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRcsEndUserMessageAvailableTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRcsEndUserMessageAvailableTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommConnectionTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn InboundConnection(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation>;
    fn OutboundConnection(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation>;
    fn AllowMultipleConnections(&self) -> ::windows::core::Result<bool>;
    fn SetAllowMultipleConnections(&self, value: bool) -> ::windows::core::Result<()>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketProtectionLevel>;
    fn SetProtectionLevel(&self, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::Result<()>;
    fn RemoteHostName(&self) -> ::windows::core::Result<super::super::Networking::HostName>;
    fn SetRemoteHostName(&self, value: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommConnectionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IRfcommConnectionTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommConnectionTriggerVtbl {
    pub const fn new<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRfcommConnectionTriggerVtbl {
        unsafe extern "system" fn InboundConnection<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InboundConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutboundConnection<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutboundConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowMultipleConnections<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowMultipleConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowMultipleConnections<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowMultipleConnections(value).into()
        }
        unsafe extern "system" fn ProtectionLevel<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectionLevel<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProtectionLevel(value).into()
        }
        unsafe extern "system" fn RemoteHostName<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteHostName<Impl: IRfcommConnectionTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRemoteHostName(&*(&value as *const <super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRfcommConnectionTrigger>, base.5, InboundConnection::<Impl, OFFSET>, OutboundConnection::<Impl, OFFSET>, AllowMultipleConnections::<Impl, OFFSET>, SetAllowMultipleConnections::<Impl, OFFSET>, ProtectionLevel::<Impl, OFFSET>, SetProtectionLevel::<Impl, OFFSET>, RemoteHostName::<Impl, OFFSET>, SetRemoteHostName::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISecondaryAuthenticationFactorAuthenticationTrigger";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationTriggerVtbl {
    pub const fn new<Impl: ISecondaryAuthenticationFactorAuthenticationTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISecondaryAuthenticationFactorAuthenticationTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISecondaryAuthenticationFactorAuthenticationTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorDataThresholdTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorDataThresholdTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISensorDataThresholdTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorDataThresholdTriggerVtbl {
    pub const fn new<Impl: ISensorDataThresholdTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISensorDataThresholdTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISensorDataThresholdTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorDataThresholdTriggerFactoryImpl: Sized {
    fn Create(&self, threshold: &::core::option::Option<super::super::Devices::Sensors::ISensorDataThreshold>) -> ::windows::core::Result<SensorDataThresholdTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISensorDataThresholdTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISensorDataThresholdTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISensorDataThresholdTriggerFactoryVtbl {
    pub const fn new<Impl: ISensorDataThresholdTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISensorDataThresholdTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISensorDataThresholdTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threshold: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&threshold as *const <super::super::Devices::Sensors::ISensorDataThreshold as ::windows::core::Abi>::Abi as *const <super::super::Devices::Sensors::ISensorDataThreshold as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISensorDataThresholdTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn TriggerType(&self) -> ::windows::core::Result<super::super::Devices::SmartCards::SmartCardTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISmartCardTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardTriggerVtbl {
    pub const fn new<Impl: ISmartCardTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISmartCardTriggerVtbl {
        unsafe extern "system" fn TriggerType<Impl: ISmartCardTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISmartCardTrigger>, base.5, TriggerType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardTriggerFactoryImpl: Sized {
    fn Create(&self, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::Result<SmartCardTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmartCardTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISmartCardTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISmartCardTriggerFactoryVtbl {
    pub const fn new<Impl: ISmartCardTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISmartCardTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISmartCardTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(triggertype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISmartCardTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsMessageReceivedTriggerFactoryImpl: Sized {
    fn Create(&self, filterrules: &::core::option::Option<super::super::Devices::Sms::SmsFilterRules>) -> ::windows::core::Result<SmsMessageReceivedTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmsMessageReceivedTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISmsMessageReceivedTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISmsMessageReceivedTriggerFactoryVtbl {
    pub const fn new<Impl: ISmsMessageReceivedTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISmsMessageReceivedTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISmsMessageReceivedTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filterrules: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&filterrules as *const <super::super::Devices::Sms::SmsFilterRules as ::windows::core::Abi>::Abi as *const <super::super::Devices::Sms::SmsFilterRules as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISmsMessageReceivedTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityTriggerImpl: Sized {
    fn IsWakeFromLowPowerSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISocketActivityTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISocketActivityTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ISocketActivityTriggerVtbl {
    pub const fn new<Impl: ISocketActivityTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISocketActivityTriggerVtbl {
        unsafe extern "system" fn IsWakeFromLowPowerSupported<Impl: ISocketActivityTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsWakeFromLowPowerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISocketActivityTrigger>, base.5, IsWakeFromLowPowerSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerTriggerFactoryImpl: Sized {
    fn Create(&self, tracker: &::core::option::Option<super::super::Storage::StorageLibraryChangeTracker>) -> ::windows::core::Result<StorageLibraryChangeTrackerTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTrackerTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IStorageLibraryChangeTrackerTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTrackerTriggerFactoryVtbl {
    pub const fn new<Impl: IStorageLibraryChangeTrackerTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStorageLibraryChangeTrackerTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IStorageLibraryChangeTrackerTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tracker: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&tracker as *const <super::super::Storage::StorageLibraryChangeTracker as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageLibraryChangeTracker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStorageLibraryChangeTrackerTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryContentChangedTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryContentChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IStorageLibraryContentChangedTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryContentChangedTriggerVtbl {
    pub const fn new<Impl: IStorageLibraryContentChangedTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStorageLibraryContentChangedTriggerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStorageLibraryContentChangedTrigger>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryContentChangedTriggerStaticsImpl: Sized {
    fn Create(&self, storagelibrary: &::core::option::Option<super::super::Storage::StorageLibrary>) -> ::windows::core::Result<StorageLibraryContentChangedTrigger>;
    fn CreateFromLibraries(&self, storagelibraries: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary>>) -> ::windows::core::Result<StorageLibraryContentChangedTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryContentChangedTriggerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IStorageLibraryContentChangedTriggerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryContentChangedTriggerStaticsVtbl {
    pub const fn new<Impl: IStorageLibraryContentChangedTriggerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStorageLibraryContentChangedTriggerStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IStorageLibraryContentChangedTriggerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storagelibrary: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&storagelibrary as *const <super::super::Storage::StorageLibrary as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageLibrary as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromLibraries<Impl: IStorageLibraryContentChangedTriggerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storagelibraries: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromLibraries(&*(&storagelibraries as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStorageLibraryContentChangedTriggerStatics>, base.5, Create::<Impl, OFFSET>, CreateFromLibraries::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemConditionImpl: Sized + IBackgroundConditionImpl {
    fn ConditionType(&self) -> ::windows::core::Result<SystemConditionType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISystemCondition";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemConditionVtbl {
    pub const fn new<Impl: ISystemConditionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemConditionVtbl {
        unsafe extern "system" fn ConditionType<Impl: ISystemConditionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SystemConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConditionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemCondition>, base.5, ConditionType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemConditionFactoryImpl: Sized {
    fn Create(&self, conditiontype: SystemConditionType) -> ::windows::core::Result<SystemCondition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemConditionFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISystemConditionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemConditionFactoryVtbl {
    pub const fn new<Impl: ISystemConditionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemConditionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISystemConditionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditiontype: SystemConditionType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(conditiontype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemConditionFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn OneShot(&self) -> ::windows::core::Result<bool>;
    fn TriggerType(&self) -> ::windows::core::Result<SystemTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISystemTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemTriggerVtbl {
    pub const fn new<Impl: ISystemTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemTriggerVtbl {
        unsafe extern "system" fn OneShot<Impl: ISystemTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TriggerType<Impl: ISystemTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SystemTriggerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TriggerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemTrigger>, base.5, OneShot::<Impl, OFFSET>, TriggerType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemTriggerFactoryImpl: Sized {
    fn Create(&self, triggertype: SystemTriggerType, oneshot: bool) -> ::windows::core::Result<SystemTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ISystemTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemTriggerFactoryVtbl {
    pub const fn new<Impl: ISystemTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISystemTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triggertype: SystemTriggerType, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(triggertype, oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn FreshnessTime(&self) -> ::windows::core::Result<u32>;
    fn OneShot(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ITimeTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl ITimeTriggerVtbl {
    pub const fn new<Impl: ITimeTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimeTriggerVtbl {
        unsafe extern "system" fn FreshnessTime<Impl: ITimeTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreshnessTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OneShot<Impl: ITimeTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OneShot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimeTrigger>, base.5, FreshnessTime::<Impl, OFFSET>, OneShot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeTriggerFactoryImpl: Sized {
    fn Create(&self, freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<TimeTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimeTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ITimeTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITimeTriggerFactoryVtbl {
    pub const fn new<Impl: ITimeTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimeTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITimeTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(freshnesstime, oneshot) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimeTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationActionTriggerFactoryImpl: Sized {
    fn Create(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotificationActionTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationActionTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IToastNotificationActionTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationActionTriggerFactoryVtbl {
    pub const fn new<Impl: IToastNotificationActionTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationActionTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IToastNotificationActionTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationActionTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryChangedTriggerFactoryImpl: Sized {
    fn Create(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotificationHistoryChangedTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToastNotificationHistoryChangedTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IToastNotificationHistoryChangedTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToastNotificationHistoryChangedTriggerFactoryVtbl {
    pub const fn new<Impl: IToastNotificationHistoryChangedTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToastNotificationHistoryChangedTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IToastNotificationHistoryChangedTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&applicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToastNotificationHistoryChangedTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationChangedTriggerFactoryImpl: Sized {
    fn Create(&self, notificationkinds: super::super::UI::Notifications::NotificationKinds) -> ::windows::core::Result<UserNotificationChangedTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserNotificationChangedTriggerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IUserNotificationChangedTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUserNotificationChangedTriggerFactoryVtbl {
    pub const fn new<Impl: IUserNotificationChangedTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserNotificationChangedTriggerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IUserNotificationChangedTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notificationkinds: super::super::UI::Notifications::NotificationKinds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(notificationkinds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserNotificationChangedTriggerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
