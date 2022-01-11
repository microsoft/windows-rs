#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeofenceImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn DwellTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MonitoredStates(&self) -> ::windows::core::Result<MonitoredGeofenceStates>;
    fn Geoshape(&self) -> ::windows::core::Result<super::IGeoshape>;
    fn SingleUse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeofence {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.IGeofence";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeofenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeofenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeofenceVtbl {
        unsafe extern "system" fn StartTime<Impl: IGeofenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IGeofenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DwellTime<Impl: IGeofenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DwellTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IGeofenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MonitoredStates<Impl: IGeofenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MonitoredGeofenceStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonitoredStates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Geoshape<Impl: IGeofenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Geoshape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SingleUse<Impl: IGeofenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SingleUse() {
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
            ::windows::core::GetRuntimeClassName::<IGeofence>,
            ::windows::core::GetTrustLevel,
            StartTime::<Impl, IMPL_OFFSET>,
            Duration::<Impl, IMPL_OFFSET>,
            DwellTime::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            MonitoredStates::<Impl, IMPL_OFFSET>,
            Geoshape::<Impl, IMPL_OFFSET>,
            SingleUse::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeofence as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeofenceFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStates(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStatesAndDwellTime(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: &super::super::super::Foundation::TimeSpan, starttime: &super::super::super::Foundation::DateTime, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<Geofence>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeofenceFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.IGeofenceFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeofenceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeofenceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeofenceFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeofenceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&geoshape as *const <super::IGeoshape as ::windows::core::Abi>::Abi as *const <super::IGeoshape as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithMonitorStates<Impl: IGeofenceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithMonitorStates(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&geoshape as *const <super::IGeoshape as ::windows::core::Abi>::Abi as *const <super::IGeoshape as ::windows::core::DefaultType>::DefaultType), monitoredstates, singleuse) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithMonitorStatesAndDwellTime<Impl: IGeofenceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithMonitorStatesAndDwellTime(
                &*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&geoshape as *const <super::IGeoshape as ::windows::core::Abi>::Abi as *const <super::IGeoshape as ::windows::core::DefaultType>::DefaultType),
                monitoredstates,
                singleuse,
                &*(&dwelltime as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration<Impl: IGeofenceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, starttime: super::super::super::Foundation::DateTime, duration: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithMonitorStatesDwellTimeStartTimeAndDuration(
                &*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&geoshape as *const <super::IGeoshape as ::windows::core::Abi>::Abi as *const <super::IGeoshape as ::windows::core::DefaultType>::DefaultType),
                monitoredstates,
                singleuse,
                &*(&dwelltime as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&starttime as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                &*(&duration as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IGeofenceFactory>,
            ::windows::core::GetTrustLevel,
            Create::<Impl, IMPL_OFFSET>,
            CreateWithMonitorStates::<Impl, IMPL_OFFSET>,
            CreateWithMonitorStatesAndDwellTime::<Impl, IMPL_OFFSET>,
            CreateWithMonitorStatesDwellTimeStartTimeAndDuration::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeofenceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGeofenceMonitorImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GeofenceMonitorStatus>;
    fn Geofences(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Geofence>>;
    fn LastKnownGeoposition(&self) -> ::windows::core::Result<super::Geoposition>;
    fn GeofenceStateChanged(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGeofenceStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReadReports(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>>;
    fn StatusChanged(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeofenceMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.IGeofenceMonitor";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGeofenceMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeofenceMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeofenceMonitorVtbl {
        unsafe extern "system" fn Status<Impl: IGeofenceMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeofenceMonitorStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Geofences<Impl: IGeofenceMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Geofences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastKnownGeoposition<Impl: IGeofenceMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastKnownGeoposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GeofenceStateChanged<Impl: IGeofenceMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeofenceStateChanged(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGeofenceStateChanged<Impl: IGeofenceMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGeofenceStateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadReports<Impl: IGeofenceMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusChanged<Impl: IGeofenceMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IGeofenceMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGeofenceMonitor>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, IMPL_OFFSET>,
            Geofences::<Impl, IMPL_OFFSET>,
            LastKnownGeoposition::<Impl, IMPL_OFFSET>,
            GeofenceStateChanged::<Impl, IMPL_OFFSET>,
            RemoveGeofenceStateChanged::<Impl, IMPL_OFFSET>,
            ReadReports::<Impl, IMPL_OFFSET>,
            StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeofenceMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceMonitorStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<GeofenceMonitor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeofenceMonitorStatics {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.IGeofenceMonitorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeofenceMonitorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeofenceMonitorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeofenceMonitorStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IGeofenceMonitorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGeofenceMonitorStatics>, ::windows::core::GetTrustLevel, Current::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeofenceMonitorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceStateChangeReportImpl: Sized {
    fn NewState(&self) -> ::windows::core::Result<GeofenceState>;
    fn Geofence(&self) -> ::windows::core::Result<Geofence>;
    fn Geoposition(&self) -> ::windows::core::Result<super::Geoposition>;
    fn RemovalReason(&self) -> ::windows::core::Result<GeofenceRemovalReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeofenceStateChangeReport {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.IGeofenceStateChangeReport";
}
#[cfg(feature = "implement_exclusive")]
impl IGeofenceStateChangeReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeofenceStateChangeReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeofenceStateChangeReportVtbl {
        unsafe extern "system" fn NewState<Impl: IGeofenceStateChangeReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeofenceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Geofence<Impl: IGeofenceStateChangeReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Geofence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Geoposition<Impl: IGeofenceStateChangeReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Geoposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovalReason<Impl: IGeofenceStateChangeReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeofenceRemovalReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovalReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGeofenceStateChangeReport>, ::windows::core::GetTrustLevel, NewState::<Impl, IMPL_OFFSET>, Geofence::<Impl, IMPL_OFFSET>, Geoposition::<Impl, IMPL_OFFSET>, RemovalReason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeofenceStateChangeReport as ::windows::core::Interface>::IID
    }
}
