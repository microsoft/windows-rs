#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGuidanceAudioNotificationRequestedEventArgs_Impl: Sized {
    fn AudioNotification(&mut self) -> ::windows::core::Result<GuidanceAudioNotificationKind>;
    fn AudioFilePaths(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn AudioText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGuidanceAudioNotificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceAudioNotificationRequestedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGuidanceAudioNotificationRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceAudioNotificationRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceAudioNotificationRequestedEventArgs_Vtbl {
        unsafe extern "system" fn AudioNotification<Impl: IGuidanceAudioNotificationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotificationKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AudioFilePaths<Impl: IGuidanceAudioNotificationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AudioText<Impl: IGuidanceAudioNotificationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IGuidanceLaneInfo_Impl: Sized {
    fn LaneMarkers(&mut self) -> ::windows::core::Result<GuidanceLaneMarkers>;
    fn IsOnRoute(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceLaneInfo {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceLaneInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceLaneInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceLaneInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceLaneInfo_Vtbl {
        unsafe extern "system" fn LaneMarkers<Impl: IGuidanceLaneInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceLaneMarkers) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOnRoute<Impl: IGuidanceLaneInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IGuidanceManeuver_Impl: Sized {
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
impl IGuidanceManeuver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceManeuver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceManeuver_Vtbl {
        unsafe extern "system" fn StartLocation<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DistanceFromRouteStart<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DistanceFromPreviousManeuver<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DepartureRoadName<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NextRoadName<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DepartureShortRoadName<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NextShortRoadName<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceManeuverKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartAngle<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndAngle<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadSignpost<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InstructionText<Impl: IGuidanceManeuver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IGuidanceMapMatchedCoordinate_Impl: Sized {
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
impl IGuidanceMapMatchedCoordinate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceMapMatchedCoordinate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceMapMatchedCoordinate_Vtbl {
        unsafe extern "system" fn Location<Impl: IGuidanceMapMatchedCoordinate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentHeading<Impl: IGuidanceMapMatchedCoordinate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentSpeed<Impl: IGuidanceMapMatchedCoordinate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOnStreet<Impl: IGuidanceMapMatchedCoordinate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Road<Impl: IGuidanceMapMatchedCoordinate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGuidanceNavigator_Impl: Sized {
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
impl IGuidanceNavigator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceNavigator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceNavigator_Vtbl {
        unsafe extern "system" fn StartNavigating<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, route: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartNavigating(&*(&route as *const <GuidanceRoute as ::windows::core::Abi>::Abi as *const <GuidanceRoute as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartSimulating<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, route: ::windows::core::RawPtr, speedinmeterspersecond: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartSimulating(&*(&route as *const <GuidanceRoute as ::windows::core::Abi>::Abi as *const <GuidanceRoute as ::windows::core::DefaultType>::DefaultType), speedinmeterspersecond).into()
        }
        unsafe extern "system" fn StartTracking<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTracking().into()
        }
        unsafe extern "system" fn Pause<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Stop<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn RepeatLastAudioNotification<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RepeatLastAudioNotification().into()
        }
        unsafe extern "system" fn AudioMeasurementSystem<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioMeasurementSystem) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioMeasurementSystem<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GuidanceAudioMeasurementSystem) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioMeasurementSystem(value).into()
        }
        unsafe extern "system" fn AudioNotifications<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotifications) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAudioNotifications<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GuidanceAudioNotifications) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioNotifications(value).into()
        }
        unsafe extern "system" fn GuidanceUpdated<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveGuidanceUpdated<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGuidanceUpdated(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DestinationReached<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDestinationReached<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDestinationReached(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rerouting<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRerouting<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRerouting(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rerouted<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRerouted<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRerouted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RerouteFailed<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRerouteFailed<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRerouteFailed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserLocationLost<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUserLocationLost<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserLocationLost(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserLocationRestored<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUserLocationRestored<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserLocationRestored(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetGuidanceVoice<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voiceid: i32, voicefolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGuidanceVoice(voiceid, &*(&voicefolder as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateUserLocation<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userlocation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateUserLocation(&*(&userlocation as *const <super::super::super::Devices::Geolocation::Geocoordinate as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::Geocoordinate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateUserLocationWithPositionOverride<Impl: IGuidanceNavigator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userlocation: ::windows::core::RawPtr, positionoverride: super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::HRESULT {
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
pub trait IGuidanceNavigator2_Impl: Sized {
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
impl IGuidanceNavigator2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceNavigator2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceNavigator2_Vtbl {
        unsafe extern "system" fn AudioNotificationRequested<Impl: IGuidanceNavigator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAudioNotificationRequested<Impl: IGuidanceNavigator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioNotificationRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsGuidanceAudioMuted<Impl: IGuidanceNavigator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsGuidanceAudioMuted<Impl: IGuidanceNavigator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IGuidanceNavigatorStatics_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<GuidanceNavigator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceNavigatorStatics {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceNavigatorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceNavigatorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceNavigatorStatics_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IGuidanceNavigatorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGuidanceNavigatorStatics2_Impl: Sized {
    fn UseAppProvidedVoice(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceNavigatorStatics2 {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceNavigatorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceNavigatorStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceNavigatorStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceNavigatorStatics2_Vtbl {
        unsafe extern "system" fn UseAppProvidedVoice<Impl: IGuidanceNavigatorStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IGuidanceReroutedEventArgs_Impl: Sized {
    fn Route(&mut self) -> ::windows::core::Result<GuidanceRoute>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceReroutedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceReroutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceReroutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceReroutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceReroutedEventArgs_Vtbl {
        unsafe extern "system" fn Route<Impl: IGuidanceReroutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGuidanceRoadSegment_Impl: Sized {
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
impl IGuidanceRoadSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRoadSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRoadSegment_Vtbl {
        unsafe extern "system" fn RoadName<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShortRoadName<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SpeedLimit<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TravelTime<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Path<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsHighway<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTunnel<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTollRoad<Impl: IGuidanceRoadSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IGuidanceRoadSegment2_Impl: Sized {
    fn IsScenic(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceRoadSegment2 {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceRoadSegment2";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceRoadSegment2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRoadSegment2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRoadSegment2_Vtbl {
        unsafe extern "system" fn IsScenic<Impl: IGuidanceRoadSegment2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IGuidanceRoadSignpost_Impl: Sized {
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
impl IGuidanceRoadSignpost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRoadSignpost_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRoadSignpost_Vtbl {
        unsafe extern "system" fn ExitNumber<Impl: IGuidanceRoadSignpost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Exit<Impl: IGuidanceRoadSignpost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BackgroundColor<Impl: IGuidanceRoadSignpost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ForegroundColor<Impl: IGuidanceRoadSignpost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExitDirections<Impl: IGuidanceRoadSignpost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGuidanceRoute_Impl: Sized {
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
impl IGuidanceRoute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRoute_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRoute_Vtbl {
        unsafe extern "system" fn Duration<Impl: IGuidanceRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Distance<Impl: IGuidanceRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Maneuvers<Impl: IGuidanceRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BoundingBox<Impl: IGuidanceRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Path<Impl: IGuidanceRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadSegments<Impl: IGuidanceRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConvertToMapRoute<Impl: IGuidanceRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGuidanceRouteStatics_Impl: Sized {
    fn CanCreateFromMapRoute(&mut self, maproute: &::core::option::Option<super::MapRoute>) -> ::windows::core::Result<bool>;
    fn TryCreateFromMapRoute(&mut self, maproute: &::core::option::Option<super::MapRoute>) -> ::windows::core::Result<GuidanceRoute>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceRouteStatics {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceRouteStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceRouteStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceRouteStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceRouteStatics_Vtbl {
        unsafe extern "system" fn CanCreateFromMapRoute<Impl: IGuidanceRouteStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maproute: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryCreateFromMapRoute<Impl: IGuidanceRouteStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maproute: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGuidanceTelemetryCollector_Impl: Sized {
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
impl IGuidanceTelemetryCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceTelemetryCollector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceTelemetryCollector_Vtbl {
        unsafe extern "system" fn Enabled<Impl: IGuidanceTelemetryCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnabled<Impl: IGuidanceTelemetryCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn ClearLocalData<Impl: IGuidanceTelemetryCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearLocalData().into()
        }
        unsafe extern "system" fn SpeedTrigger<Impl: IGuidanceTelemetryCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSpeedTrigger<Impl: IGuidanceTelemetryCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeedTrigger(value).into()
        }
        unsafe extern "system" fn UploadFrequency<Impl: IGuidanceTelemetryCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUploadFrequency<Impl: IGuidanceTelemetryCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
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
pub trait IGuidanceTelemetryCollectorStatics_Impl: Sized {
    fn GetCurrent(&mut self) -> ::windows::core::Result<GuidanceTelemetryCollector>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidanceTelemetryCollectorStatics {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.IGuidanceTelemetryCollectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidanceTelemetryCollectorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceTelemetryCollectorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceTelemetryCollectorStatics_Vtbl {
        unsafe extern "system" fn GetCurrent<Impl: IGuidanceTelemetryCollectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGuidanceUpdatedEventArgs_Impl: Sized {
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
impl IGuidanceUpdatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidanceUpdatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidanceUpdatedEventArgs_Vtbl {
        unsafe extern "system" fn Mode<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GuidanceMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NextManeuver<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NextManeuverDistance<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AfterNextManeuver<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AfterNextManeuverDistance<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DistanceToDestination<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ElapsedDistance<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ElapsedTime<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimeToDestination<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoadName<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Route<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentLocation<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsNewManeuver<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaneInfo<Impl: IGuidanceUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
