#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGuidanceAudioNotificationRequestedEventArgsImpl: Sized {
    fn AudioNotification(&mut self) -> ::windows::core::Result<GuidanceAudioNotificationKind>;
    fn AudioFilePaths(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn AudioText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceAudioNotificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceAudioNotificationRequestedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGuidanceAudioNotificationRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceAudioNotificationRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceAudioNotificationRequestedEventArgsVtbl {
        unsafe extern "system" fn AudioNotification<Impl: IGuidanceAudioNotificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotificationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFilePaths<Impl: IGuidanceAudioNotificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFilePaths() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioText<Impl: IGuidanceAudioNotificationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceAudioNotificationRequestedEventArgs, BASE_OFFSET>(),
            AudioNotification: AudioNotification::<Impl, IMPL_OFFSET>,
            AudioFilePaths: AudioFilePaths::<Impl, IMPL_OFFSET>,
            AudioText: AudioText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceAudioNotificationRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceLaneInfoImpl: Sized {
    fn LaneMarkers(&mut self) -> ::windows::core::Result<GuidanceLaneMarkers>;
    fn IsOnRoute(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceLaneInfo {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceLaneInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceLaneInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceLaneInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceLaneInfoVtbl {
        unsafe extern "system" fn LaneMarkers<Impl: IGuidanceLaneInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceLaneMarkers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaneMarkers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOnRoute<Impl: IGuidanceLaneInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOnRoute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceLaneInfo, BASE_OFFSET>(),
            LaneMarkers: LaneMarkers::<Impl, IMPL_OFFSET>,
            IsOnRoute: IsOnRoute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceLaneInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IGuidanceManeuverImpl: Sized {
    fn StartLocation(&mut self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint>;
    fn DistanceFromRouteStart(&mut self) -> ::windows::core::Result<i32>;
    fn DistanceFromPreviousManeuver(&mut self) -> ::windows::core::Result<i32>;
    fn DepartureRoadName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NextRoadName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DepartureShortRoadName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NextShortRoadName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&mut self) -> ::windows::core::Result<GuidanceManeuverKind>;
    fn StartAngle(&mut self) -> ::windows::core::Result<i32>;
    fn EndAngle(&mut self) -> ::windows::core::Result<i32>;
    fn RoadSignpost(&mut self) -> ::windows::core::Result<GuidanceRoadSignpost>;
    fn InstructionText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceManeuver {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceManeuver";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IGuidanceManeuverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceManeuverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceManeuverVtbl {
        unsafe extern "system" fn StartLocation<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistanceFromRouteStart<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DistanceFromRouteStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistanceFromPreviousManeuver<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DistanceFromPreviousManeuver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepartureRoadName<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DepartureRoadName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRoadName<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextRoadName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepartureShortRoadName<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DepartureShortRoadName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextShortRoadName<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextShortRoadName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceManeuverKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAngle<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAngle<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadSignpost<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadSignpost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstructionText<Impl: IGuidanceManeuverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstructionText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceManeuver, BASE_OFFSET>(),
            StartLocation: StartLocation::<Impl, IMPL_OFFSET>,
            DistanceFromRouteStart: DistanceFromRouteStart::<Impl, IMPL_OFFSET>,
            DistanceFromPreviousManeuver: DistanceFromPreviousManeuver::<Impl, IMPL_OFFSET>,
            DepartureRoadName: DepartureRoadName::<Impl, IMPL_OFFSET>,
            NextRoadName: NextRoadName::<Impl, IMPL_OFFSET>,
            DepartureShortRoadName: DepartureShortRoadName::<Impl, IMPL_OFFSET>,
            NextShortRoadName: NextShortRoadName::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            StartAngle: StartAngle::<Impl, IMPL_OFFSET>,
            EndAngle: EndAngle::<Impl, IMPL_OFFSET>,
            RoadSignpost: RoadSignpost::<Impl, IMPL_OFFSET>,
            InstructionText: InstructionText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceManeuver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait IGuidanceMapMatchedCoordinateImpl: Sized {
    fn Location(&mut self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint>;
    fn CurrentHeading(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentSpeed(&mut self) -> ::windows::core::Result<f64>;
    fn IsOnStreet(&mut self) -> ::windows::core::Result<bool>;
    fn Road(&mut self) -> ::windows::core::Result<GuidanceRoadSegment>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceMapMatchedCoordinate {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceMapMatchedCoordinate";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl IGuidanceMapMatchedCoordinateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceMapMatchedCoordinateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceMapMatchedCoordinateVtbl {
        unsafe extern "system" fn Location<Impl: IGuidanceMapMatchedCoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Location() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHeading<Impl: IGuidanceMapMatchedCoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHeading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSpeed<Impl: IGuidanceMapMatchedCoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOnStreet<Impl: IGuidanceMapMatchedCoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOnStreet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Road<Impl: IGuidanceMapMatchedCoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Road() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceMapMatchedCoordinate, BASE_OFFSET>(),
            Location: Location::<Impl, IMPL_OFFSET>,
            CurrentHeading: CurrentHeading::<Impl, IMPL_OFFSET>,
            CurrentSpeed: CurrentSpeed::<Impl, IMPL_OFFSET>,
            IsOnStreet: IsOnStreet::<Impl, IMPL_OFFSET>,
            Road: Road::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceMapMatchedCoordinate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGuidanceNavigatorImpl: Sized {
    fn StartNavigating(&mut self, route: &::core::option::Option<GuidanceRoute>) -> ::windows::core::Result<()>;
    fn StartSimulating(&mut self, route: &::core::option::Option<GuidanceRoute>, speedinmeterspersecond: i32) -> ::windows::core::Result<()>;
    fn StartTracking(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn RepeatLastAudioNotification(&mut self) -> ::windows::core::Result<()>;
    fn AudioMeasurementSystem(&mut self) -> ::windows::core::Result<GuidanceAudioMeasurementSystem>;
    fn SetAudioMeasurementSystem(&mut self, value: GuidanceAudioMeasurementSystem) -> ::windows::core::Result<()>;
    fn AudioNotifications(&mut self) -> ::windows::core::Result<GuidanceAudioNotifications>;
    fn SetAudioNotifications(&mut self, value: GuidanceAudioNotifications) -> ::windows::core::Result<()>;
    fn GuidanceUpdated(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGuidanceUpdated(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DestinationReached(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDestinationReached(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Rerouting(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRerouting(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Rerouted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRerouted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RerouteFailed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRerouteFailed(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UserLocationLost(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserLocationLost(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UserLocationRestored(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserLocationRestored(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetGuidanceVoice(&mut self, voiceid: i32, voicefolder: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdateUserLocation(&mut self, userlocation: &::core::option::Option<super::super::super::Devices::Geolocation::Geocoordinate>) -> ::windows::core::Result<()>;
    fn UpdateUserLocationWithPositionOverride(&mut self, userlocation: &::core::option::Option<super::super::super::Devices::Geolocation::Geocoordinate>, positionoverride: &super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceNavigator {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceNavigator";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IGuidanceNavigatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceNavigatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceNavigatorVtbl {
        unsafe extern "system" fn StartNavigating<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, route: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartNavigating(&*(&route as *const <GuidanceRoute as ::windows::core::Abi>::Abi as *const <GuidanceRoute as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartSimulating<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, route: ::windows::core::RawPtr, speedinmeterspersecond: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartSimulating(&*(&route as *const <GuidanceRoute as ::windows::core::Abi>::Abi as *const <GuidanceRoute as ::windows::core::DefaultType>::DefaultType), speedinmeterspersecond).into()
        }
        unsafe extern "system" fn StartTracking<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTracking().into()
        }
        unsafe extern "system" fn Pause<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Stop<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn RepeatLastAudioNotification<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RepeatLastAudioNotification().into()
        }
        unsafe extern "system" fn AudioMeasurementSystem<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioMeasurementSystem) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioMeasurementSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioMeasurementSystem<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GuidanceAudioMeasurementSystem) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioMeasurementSystem(value).into()
        }
        unsafe extern "system" fn AudioNotifications<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotifications) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioNotifications<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GuidanceAudioNotifications) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioNotifications(value).into()
        }
        unsafe extern "system" fn GuidanceUpdated<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GuidanceUpdated(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGuidanceUpdated<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGuidanceUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DestinationReached<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationReached(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDestinationReached<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDestinationReached(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rerouting<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rerouting(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRerouting<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRerouting(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rerouted<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rerouted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRerouted<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRerouted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RerouteFailed<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RerouteFailed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRerouteFailed<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRerouteFailed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserLocationLost<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserLocationLost(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserLocationLost<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserLocationLost(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserLocationRestored<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserLocationRestored(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserLocationRestored<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserLocationRestored(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetGuidanceVoice<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voiceid: i32, voicefolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGuidanceVoice(voiceid, &*(&voicefolder as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateUserLocation<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userlocation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateUserLocation(&*(&userlocation as *const <super::super::super::Devices::Geolocation::Geocoordinate as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::Geocoordinate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateUserLocationWithPositionOverride<Impl: IGuidanceNavigatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userlocation: ::windows::core::RawPtr, positionoverride: super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .UpdateUserLocationWithPositionOverride(&*(&userlocation as *const <super::super::super::Devices::Geolocation::Geocoordinate as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::Geocoordinate as ::windows::core::DefaultType>::DefaultType), &*(&positionoverride as *const <super::super::super::Devices::Geolocation::BasicGeoposition as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::BasicGeoposition as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceNavigator, BASE_OFFSET>(),
            StartNavigating: StartNavigating::<Impl, IMPL_OFFSET>,
            StartSimulating: StartSimulating::<Impl, IMPL_OFFSET>,
            StartTracking: StartTracking::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            RepeatLastAudioNotification: RepeatLastAudioNotification::<Impl, IMPL_OFFSET>,
            AudioMeasurementSystem: AudioMeasurementSystem::<Impl, IMPL_OFFSET>,
            SetAudioMeasurementSystem: SetAudioMeasurementSystem::<Impl, IMPL_OFFSET>,
            AudioNotifications: AudioNotifications::<Impl, IMPL_OFFSET>,
            SetAudioNotifications: SetAudioNotifications::<Impl, IMPL_OFFSET>,
            GuidanceUpdated: GuidanceUpdated::<Impl, IMPL_OFFSET>,
            RemoveGuidanceUpdated: RemoveGuidanceUpdated::<Impl, IMPL_OFFSET>,
            DestinationReached: DestinationReached::<Impl, IMPL_OFFSET>,
            RemoveDestinationReached: RemoveDestinationReached::<Impl, IMPL_OFFSET>,
            Rerouting: Rerouting::<Impl, IMPL_OFFSET>,
            RemoveRerouting: RemoveRerouting::<Impl, IMPL_OFFSET>,
            Rerouted: Rerouted::<Impl, IMPL_OFFSET>,
            RemoveRerouted: RemoveRerouted::<Impl, IMPL_OFFSET>,
            RerouteFailed: RerouteFailed::<Impl, IMPL_OFFSET>,
            RemoveRerouteFailed: RemoveRerouteFailed::<Impl, IMPL_OFFSET>,
            UserLocationLost: UserLocationLost::<Impl, IMPL_OFFSET>,
            RemoveUserLocationLost: RemoveUserLocationLost::<Impl, IMPL_OFFSET>,
            UserLocationRestored: UserLocationRestored::<Impl, IMPL_OFFSET>,
            RemoveUserLocationRestored: RemoveUserLocationRestored::<Impl, IMPL_OFFSET>,
            SetGuidanceVoice: SetGuidanceVoice::<Impl, IMPL_OFFSET>,
            UpdateUserLocation: UpdateUserLocation::<Impl, IMPL_OFFSET>,
            UpdateUserLocationWithPositionOverride: UpdateUserLocationWithPositionOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceNavigator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGuidanceNavigator2Impl: Sized {
    fn AudioNotificationRequested(&mut self, value: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioNotificationRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsGuidanceAudioMuted(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsGuidanceAudioMuted(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceNavigator2 {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceNavigator2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGuidanceNavigator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceNavigator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceNavigator2Vtbl {
        unsafe extern "system" fn AudioNotificationRequested<Impl: IGuidanceNavigator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioNotificationRequested(&*(&value as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioNotificationRequested<Impl: IGuidanceNavigator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioNotificationRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsGuidanceAudioMuted<Impl: IGuidanceNavigator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGuidanceAudioMuted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsGuidanceAudioMuted<Impl: IGuidanceNavigator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsGuidanceAudioMuted(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceNavigator2, BASE_OFFSET>(),
            AudioNotificationRequested: AudioNotificationRequested::<Impl, IMPL_OFFSET>,
            RemoveAudioNotificationRequested: RemoveAudioNotificationRequested::<Impl, IMPL_OFFSET>,
            IsGuidanceAudioMuted: IsGuidanceAudioMuted::<Impl, IMPL_OFFSET>,
            SetIsGuidanceAudioMuted: SetIsGuidanceAudioMuted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceNavigator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceNavigatorStaticsImpl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<GuidanceNavigator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceNavigatorStatics {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceNavigatorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceNavigatorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceNavigatorStaticsVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IGuidanceNavigatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceNavigatorStatics, BASE_OFFSET>(), GetCurrent: GetCurrent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceNavigatorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceNavigatorStatics2Impl: Sized {
    fn UseAppProvidedVoice(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceNavigatorStatics2 {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceNavigatorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceNavigatorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceNavigatorStatics2Vtbl {
        unsafe extern "system" fn UseAppProvidedVoice<Impl: IGuidanceNavigatorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseAppProvidedVoice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceNavigatorStatics2, BASE_OFFSET>(),
            UseAppProvidedVoice: UseAppProvidedVoice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceNavigatorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceReroutedEventArgsImpl: Sized {
    fn Route(&mut self) -> ::windows::core::Result<GuidanceRoute>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceReroutedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceReroutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceReroutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceReroutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceReroutedEventArgsVtbl {
        unsafe extern "system" fn Route<Impl: IGuidanceReroutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Route() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceReroutedEventArgs, BASE_OFFSET>(), Route: Route::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceReroutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGuidanceRoadSegmentImpl: Sized {
    fn RoadName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShortRoadName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SpeedLimit(&mut self) -> ::windows::core::Result<f64>;
    fn TravelTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopath>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsHighway(&mut self) -> ::windows::core::Result<bool>;
    fn IsTunnel(&mut self) -> ::windows::core::Result<bool>;
    fn IsTollRoad(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceRoadSegment {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceRoadSegment";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IGuidanceRoadSegmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRoadSegmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRoadSegmentVtbl {
        unsafe extern "system" fn RoadName<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShortRoadName<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShortRoadName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeedLimit<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeedLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TravelTime<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TravelTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsHighway<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHighway() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTunnel<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTunnel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTollRoad<Impl: IGuidanceRoadSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTollRoad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceRoadSegment, BASE_OFFSET>(),
            RoadName: RoadName::<Impl, IMPL_OFFSET>,
            ShortRoadName: ShortRoadName::<Impl, IMPL_OFFSET>,
            SpeedLimit: SpeedLimit::<Impl, IMPL_OFFSET>,
            TravelTime: TravelTime::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            IsHighway: IsHighway::<Impl, IMPL_OFFSET>,
            IsTunnel: IsTunnel::<Impl, IMPL_OFFSET>,
            IsTollRoad: IsTollRoad::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceRoadSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceRoadSegment2Impl: Sized {
    fn IsScenic(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceRoadSegment2 {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceRoadSegment2";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceRoadSegment2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRoadSegment2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRoadSegment2Vtbl {
        unsafe extern "system" fn IsScenic<Impl: IGuidanceRoadSegment2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScenic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceRoadSegment2, BASE_OFFSET>(), IsScenic: IsScenic::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceRoadSegment2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
pub trait IGuidanceRoadSignpostImpl: Sized {
    fn ExitNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Exit(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn ForegroundColor(&mut self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn ExitDirections(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceRoadSignpost {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceRoadSignpost";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI", feature = "implement_exclusive"))]
impl IGuidanceRoadSignpostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRoadSignpostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRoadSignpostVtbl {
        unsafe extern "system" fn ExitNumber<Impl: IGuidanceRoadSignpostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exit<Impl: IGuidanceRoadSignpostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundColor<Impl: IGuidanceRoadSignpostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForegroundColor<Impl: IGuidanceRoadSignpostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitDirections<Impl: IGuidanceRoadSignpostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitDirections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceRoadSignpost, BASE_OFFSET>(),
            ExitNumber: ExitNumber::<Impl, IMPL_OFFSET>,
            Exit: Exit::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            ForegroundColor: ForegroundColor::<Impl, IMPL_OFFSET>,
            ExitDirections: ExitDirections::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceRoadSignpost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGuidanceRouteImpl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Distance(&mut self) -> ::windows::core::Result<i32>;
    fn Maneuvers(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceManeuver>>;
    fn BoundingBox(&mut self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::GeoboundingBox>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopath>;
    fn RoadSegments(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceRoadSegment>>;
    fn ConvertToMapRoute(&mut self) -> ::windows::core::Result<super::MapRoute>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceRoute {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceRoute";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGuidanceRouteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRouteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRouteVtbl {
        unsafe extern "system" fn Duration<Impl: IGuidanceRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Distance<Impl: IGuidanceRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Distance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maneuvers<Impl: IGuidanceRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maneuvers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingBox<Impl: IGuidanceRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IGuidanceRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadSegments<Impl: IGuidanceRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadSegments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertToMapRoute<Impl: IGuidanceRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertToMapRoute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceRoute, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            Distance: Distance::<Impl, IMPL_OFFSET>,
            Maneuvers: Maneuvers::<Impl, IMPL_OFFSET>,
            BoundingBox: BoundingBox::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            RoadSegments: RoadSegments::<Impl, IMPL_OFFSET>,
            ConvertToMapRoute: ConvertToMapRoute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceRoute as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceRouteStaticsImpl: Sized {
    fn CanCreateFromMapRoute(&mut self, maproute: &::core::option::Option<super::MapRoute>) -> ::windows::core::Result<bool>;
    fn TryCreateFromMapRoute(&mut self, maproute: &::core::option::Option<super::MapRoute>) -> ::windows::core::Result<GuidanceRoute>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceRouteStatics {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceRouteStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceRouteStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRouteStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRouteStaticsVtbl {
        unsafe extern "system" fn CanCreateFromMapRoute<Impl: IGuidanceRouteStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maproute: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCreateFromMapRoute(&*(&maproute as *const <super::MapRoute as ::windows::core::Abi>::Abi as *const <super::MapRoute as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFromMapRoute<Impl: IGuidanceRouteStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maproute: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFromMapRoute(&*(&maproute as *const <super::MapRoute as ::windows::core::Abi>::Abi as *const <super::MapRoute as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceRouteStatics, BASE_OFFSET>(),
            CanCreateFromMapRoute: CanCreateFromMapRoute::<Impl, IMPL_OFFSET>,
            TryCreateFromMapRoute: TryCreateFromMapRoute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceRouteStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceTelemetryCollectorImpl: Sized {
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ClearLocalData(&mut self) -> ::windows::core::Result<()>;
    fn SpeedTrigger(&mut self) -> ::windows::core::Result<f64>;
    fn SetSpeedTrigger(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn UploadFrequency(&mut self) -> ::windows::core::Result<i32>;
    fn SetUploadFrequency(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceTelemetryCollector {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceTelemetryCollector";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceTelemetryCollectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceTelemetryCollectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceTelemetryCollectorVtbl {
        unsafe extern "system" fn Enabled<Impl: IGuidanceTelemetryCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IGuidanceTelemetryCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn ClearLocalData<Impl: IGuidanceTelemetryCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearLocalData().into()
        }
        unsafe extern "system" fn SpeedTrigger<Impl: IGuidanceTelemetryCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeedTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeedTrigger<Impl: IGuidanceTelemetryCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeedTrigger(value).into()
        }
        unsafe extern "system" fn UploadFrequency<Impl: IGuidanceTelemetryCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UploadFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUploadFrequency<Impl: IGuidanceTelemetryCollectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUploadFrequency(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceTelemetryCollector, BASE_OFFSET>(),
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            ClearLocalData: ClearLocalData::<Impl, IMPL_OFFSET>,
            SpeedTrigger: SpeedTrigger::<Impl, IMPL_OFFSET>,
            SetSpeedTrigger: SetSpeedTrigger::<Impl, IMPL_OFFSET>,
            UploadFrequency: UploadFrequency::<Impl, IMPL_OFFSET>,
            SetUploadFrequency: SetUploadFrequency::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceTelemetryCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceTelemetryCollectorStaticsImpl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<GuidanceTelemetryCollector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceTelemetryCollectorStatics {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceTelemetryCollectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceTelemetryCollectorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceTelemetryCollectorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceTelemetryCollectorStaticsVtbl {
        unsafe extern "system" fn GetCurrent<Impl: IGuidanceTelemetryCollectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceTelemetryCollectorStatics, BASE_OFFSET>(),
            GetCurrent: GetCurrent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceTelemetryCollectorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGuidanceUpdatedEventArgsImpl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<GuidanceMode>;
    fn NextManeuver(&mut self) -> ::windows::core::Result<GuidanceManeuver>;
    fn NextManeuverDistance(&mut self) -> ::windows::core::Result<i32>;
    fn AfterNextManeuver(&mut self) -> ::windows::core::Result<GuidanceManeuver>;
    fn AfterNextManeuverDistance(&mut self) -> ::windows::core::Result<i32>;
    fn DistanceToDestination(&mut self) -> ::windows::core::Result<i32>;
    fn ElapsedDistance(&mut self) -> ::windows::core::Result<i32>;
    fn ElapsedTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn TimeToDestination(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn RoadName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Route(&mut self) -> ::windows::core::Result<GuidanceRoute>;
    fn CurrentLocation(&mut self) -> ::windows::core::Result<GuidanceMapMatchedCoordinate>;
    fn IsNewManeuver(&mut self) -> ::windows::core::Result<bool>;
    fn LaneInfo(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceLaneInfo>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceUpdatedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceUpdatedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGuidanceUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceUpdatedEventArgsVtbl {
        unsafe extern "system" fn Mode<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextManeuver<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextManeuver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextManeuverDistance<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextManeuverDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AfterNextManeuver<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AfterNextManeuver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AfterNextManeuverDistance<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AfterNextManeuverDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistanceToDestination<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DistanceToDestination() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedDistance<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElapsedDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeToDestination<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeToDestination() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoadName<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoadName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Route<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Route() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocation<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNewManeuver<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNewManeuver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaneInfo<Impl: IGuidanceUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaneInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidanceUpdatedEventArgs, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            NextManeuver: NextManeuver::<Impl, IMPL_OFFSET>,
            NextManeuverDistance: NextManeuverDistance::<Impl, IMPL_OFFSET>,
            AfterNextManeuver: AfterNextManeuver::<Impl, IMPL_OFFSET>,
            AfterNextManeuverDistance: AfterNextManeuverDistance::<Impl, IMPL_OFFSET>,
            DistanceToDestination: DistanceToDestination::<Impl, IMPL_OFFSET>,
            ElapsedDistance: ElapsedDistance::<Impl, IMPL_OFFSET>,
            ElapsedTime: ElapsedTime::<Impl, IMPL_OFFSET>,
            TimeToDestination: TimeToDestination::<Impl, IMPL_OFFSET>,
            RoadName: RoadName::<Impl, IMPL_OFFSET>,
            Route: Route::<Impl, IMPL_OFFSET>,
            CurrentLocation: CurrentLocation::<Impl, IMPL_OFFSET>,
            IsNewManeuver: IsNewManeuver::<Impl, IMPL_OFFSET>,
            LaneInfo: LaneInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidanceUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
