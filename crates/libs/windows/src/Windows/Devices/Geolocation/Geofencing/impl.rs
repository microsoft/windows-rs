#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn DwellTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MonitoredStates(&self) -> ::windows::core::Result<MonitoredGeofenceStates>;
    fn Geoshape(&self) -> ::windows::core::Result<super::IGeoshape>;
    fn SingleUse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeofence {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.IGeofence";
}
#[cfg(feature = "implement_exclusive")]
impl IGeofenceVtbl {
    pub const fn new<Impl: IGeofenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeofenceVtbl {
        unsafe extern "system" fn StartTime<Impl: IGeofenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IGeofenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DwellTime<Impl: IGeofenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DwellTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IGeofenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MonitoredStates<Impl: IGeofenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MonitoredGeofenceStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MonitoredStates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Geoshape<Impl: IGeofenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Geoshape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SingleUse<Impl: IGeofenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SingleUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeofence>, base.5, StartTime::<Impl, OFFSET>, Duration::<Impl, OFFSET>, DwellTime::<Impl, OFFSET>, Id::<Impl, OFFSET>, MonitoredStates::<Impl, OFFSET>, Geoshape::<Impl, OFFSET>, SingleUse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStates(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStatesAndDwellTime(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: &super::super::super::Foundation::TimeSpan, starttime: &super::super::super::Foundation::DateTime, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<Geofence>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeofenceFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.IGeofenceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeofenceFactoryVtbl {
    pub const fn new<Impl: IGeofenceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeofenceFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeofenceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&geoshape as *const <super::IGeoshape as ::windows::core::Abi>::Abi as *const <super::IGeoshape as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithMonitorStates<Impl: IGeofenceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithMonitorStates(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&geoshape as *const <super::IGeoshape as ::windows::core::Abi>::Abi as *const <super::IGeoshape as ::windows::core::DefaultType>::DefaultType), monitoredstates, singleuse) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithMonitorStatesAndDwellTime<Impl: IGeofenceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration<Impl: IGeofenceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, geoshape: ::windows::core::RawPtr, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, starttime: super::super::super::Foundation::DateTime, duration: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeofenceFactory>, base.5, Create::<Impl, OFFSET>, CreateWithMonitorStates::<Impl, OFFSET>, CreateWithMonitorStatesAndDwellTime::<Impl, OFFSET>, CreateWithMonitorStatesDwellTimeStartTimeAndDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeofenceMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geofencing.IGeofenceMonitor";
}
#[cfg(feature = "implement_exclusive")]
impl IGeofenceMonitorVtbl {
    pub const fn new<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeofenceMonitorVtbl {
        unsafe extern "system" fn Status<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GeofenceMonitorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Geofences<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Geofences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastKnownGeoposition<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastKnownGeoposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GeofenceStateChanged<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GeofenceStateChanged(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGeofenceStateChanged<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGeofenceStateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadReports<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusChanged<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&eventhandler as *const <super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IGeofenceMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeofenceMonitor>, base.5, Status::<Impl, OFFSET>, Geofences::<Impl, OFFSET>, LastKnownGeoposition::<Impl, OFFSET>, GeofenceStateChanged::<Impl, OFFSET>, RemoveGeofenceStateChanged::<Impl, OFFSET>, ReadReports::<Impl, OFFSET>, StatusChanged::<Impl, OFFSET>, RemoveStatusChanged::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGeofenceMonitorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeofenceMonitorStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IGeofenceMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeofenceMonitorStatics>, base.5, Current::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGeofenceStateChangeReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeofenceStateChangeReportVtbl {
        unsafe extern "system" fn NewState<Impl: IGeofenceStateChangeReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GeofenceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Geofence<Impl: IGeofenceStateChangeReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Geofence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Geoposition<Impl: IGeofenceStateChangeReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Geoposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovalReason<Impl: IGeofenceStateChangeReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GeofenceRemovalReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemovalReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeofenceStateChangeReport>, base.5, NewState::<Impl, OFFSET>, Geofence::<Impl, OFFSET>, Geoposition::<Impl, OFFSET>, RemovalReason::<Impl, OFFSET>)
    }
}
