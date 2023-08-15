#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceAudioNotificationRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceAudioNotificationRequestedEventArgs {
    type Vtable = IGuidanceAudioNotificationRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGuidanceAudioNotificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceAudioNotificationRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca2aa24a_c7c2_4d4c_9d7c_499576bceddb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceAudioNotificationRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AudioNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotificationKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AudioFilePaths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AudioFilePaths: usize,
    pub AudioText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceLaneInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceLaneInfo {
    type Vtable = IGuidanceLaneInfo_Vtbl;
}
impl ::core::clone::Clone for IGuidanceLaneInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceLaneInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8404d114_6581_43b7_ac15_c9079bf90df1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceLaneInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LaneMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceLaneMarkers) -> ::windows_core::HRESULT,
    pub IsOnRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceManeuver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceManeuver {
    type Vtable = IGuidanceManeuver_Vtbl;
}
impl ::core::clone::Clone for IGuidanceManeuver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceManeuver {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc09326c_ecc9_4928_a2a1_7232b99b94a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceManeuver_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub StartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    StartLocation: usize,
    pub DistanceFromRouteStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub DistanceFromPreviousManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub DepartureRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NextRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DepartureShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NextShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceManeuverKind) -> ::windows_core::HRESULT,
    pub StartAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub EndAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub RoadSignpost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InstructionText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceMapMatchedCoordinate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceMapMatchedCoordinate {
    type Vtable = IGuidanceMapMatchedCoordinate_Vtbl;
}
impl ::core::clone::Clone for IGuidanceMapMatchedCoordinate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceMapMatchedCoordinate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7acb168_2912_4a99_aff1_798609b981fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceMapMatchedCoordinate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub CurrentHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CurrentSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub IsOnStreet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Road: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceNavigator {
    type Vtable = IGuidanceNavigator_Vtbl;
}
impl ::core::clone::Clone for IGuidanceNavigator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceNavigator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08f17ef7_8e3f_4d9a_be8a_108f9a012c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartNavigating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, route: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartSimulating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, route: *mut ::core::ffi::c_void, speedinmeterspersecond: i32) -> ::windows_core::HRESULT,
    pub StartTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RepeatLastAudioNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AudioMeasurementSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioMeasurementSystem) -> ::windows_core::HRESULT,
    pub SetAudioMeasurementSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GuidanceAudioMeasurementSystem) -> ::windows_core::HRESULT,
    pub AudioNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotifications) -> ::windows_core::HRESULT,
    pub SetAudioNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GuidanceAudioNotifications) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GuidanceUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GuidanceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGuidanceUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGuidanceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub DestinationReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DestinationReached: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDestinationReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDestinationReached: usize,
    #[cfg(feature = "Foundation")]
    pub Rerouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rerouting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouting: usize,
    #[cfg(feature = "Foundation")]
    pub Rerouted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rerouted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouted: usize,
    #[cfg(feature = "Foundation")]
    pub RerouteFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RerouteFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouteFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouteFailed: usize,
    #[cfg(feature = "Foundation")]
    pub UserLocationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserLocationLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserLocationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserLocationLost: usize,
    #[cfg(feature = "Foundation")]
    pub UserLocationRestored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserLocationRestored: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserLocationRestored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserLocationRestored: usize,
    pub SetGuidanceVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voiceid: i32, voicefolder: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub UpdateUserLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userlocation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    UpdateUserLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub UpdateUserLocationWithPositionOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userlocation: *mut ::core::ffi::c_void, positionoverride: super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    UpdateUserLocationWithPositionOverride: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceNavigator2 {
    type Vtable = IGuidanceNavigator2_Vtbl;
}
impl ::core::clone::Clone for IGuidanceNavigator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceNavigator2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cdc50d1_041c_4bf3_b633_a101fc2f6b57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigator2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AudioNotificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioNotificationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioNotificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioNotificationRequested: usize,
    pub IsGuidanceAudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsGuidanceAudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceNavigatorStatics {
    type Vtable = IGuidanceNavigatorStatics_Vtbl;
}
impl ::core::clone::Clone for IGuidanceNavigatorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceNavigatorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00fd9513_4456_4e66_a143_3add6be08426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceNavigatorStatics2 {
    type Vtable = IGuidanceNavigatorStatics2_Vtbl;
}
impl ::core::clone::Clone for IGuidanceNavigatorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceNavigatorStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54c5c3e2_7784_4c85_8c95_d0c6efb43965);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UseAppProvidedVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceReroutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceReroutedEventArgs {
    type Vtable = IGuidanceReroutedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGuidanceReroutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceReroutedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x115d4008_d528_454e_bb94_a50341d2c9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceReroutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRoadSegment {
    type Vtable = IGuidanceRoadSegment_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRoadSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceRoadSegment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb32758a6_be78_4c63_afe7_6c2957479b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SpeedLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TravelTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TravelTime: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsHighway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTunnel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTollRoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSegment2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRoadSegment2 {
    type Vtable = IGuidanceRoadSegment2_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRoadSegment2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceRoadSegment2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2474a61d_1723_49f1_895b_47a2c4aa9c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsScenic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSignpost(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRoadSignpost {
    type Vtable = IGuidanceRoadSignpost_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRoadSignpost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceRoadSignpost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1a728b6_f77a_4742_8312_53300f9845f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSignpost_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExitNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Exit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExitDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExitDirections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoute(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRoute {
    type Vtable = IGuidanceRoute_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRoute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceRoute {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a14545d_801a_40bd_a286_afb2010cce6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoute_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub Distance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Maneuvers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Maneuvers: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RoadSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RoadSegments: usize,
    pub ConvertToMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRouteStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceRouteStatics {
    type Vtable = IGuidanceRouteStatics_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRouteStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceRouteStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf56d926a_55ed_49c1_b09c_4b8223b50db3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRouteStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanCreateFromMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maproute: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryCreateFromMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maproute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceTelemetryCollector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceTelemetryCollector {
    type Vtable = IGuidanceTelemetryCollector_Vtbl;
}
impl ::core::clone::Clone for IGuidanceTelemetryCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceTelemetryCollector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb1f8da5_b878_4d92_98dd_347d23d38262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ClearLocalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SpeedTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSpeedTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub UploadFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetUploadFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceTelemetryCollectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceTelemetryCollectorStatics {
    type Vtable = IGuidanceTelemetryCollectorStatics_Vtbl;
}
impl ::core::clone::Clone for IGuidanceTelemetryCollectorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceTelemetryCollectorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36532047_f160_44fb_b578_94577ca05990);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGuidanceUpdatedEventArgs {
    type Vtable = IGuidanceUpdatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGuidanceUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGuidanceUpdatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfdac160b_9e8d_4de3_a9fa_b06321d18db9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceMode) -> ::windows_core::HRESULT,
    pub NextManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NextManeuverDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub AfterNextManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AfterNextManeuverDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub DistanceToDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ElapsedDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ElapsedTime: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToDestination: usize,
    pub RoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsNewManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub LaneInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaneInfo: usize,
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceAudioNotificationRequestedEventArgs(::windows_core::IUnknown);
impl GuidanceAudioNotificationRequestedEventArgs {
    pub fn AudioNotification(&self) -> ::windows_core::Result<GuidanceAudioNotificationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioNotification)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AudioFilePaths(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFilePaths)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AudioText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceAudioNotificationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceAudioNotificationRequestedEventArgs {}
impl ::core::fmt::Debug for GuidanceAudioNotificationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotificationRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceAudioNotificationRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs;{ca2aa24a-c7c2-4d4c-9d7c-499576bceddb})");
}
impl ::core::clone::Clone for GuidanceAudioNotificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceAudioNotificationRequestedEventArgs {
    type Vtable = IGuidanceAudioNotificationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceAudioNotificationRequestedEventArgs {
    const IID: ::windows_core::GUID = <IGuidanceAudioNotificationRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceAudioNotificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(GuidanceAudioNotificationRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceAudioNotificationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceAudioNotificationRequestedEventArgs {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceLaneInfo(::windows_core::IUnknown);
impl GuidanceLaneInfo {
    pub fn LaneMarkers(&self) -> ::windows_core::Result<GuidanceLaneMarkers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaneMarkers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOnRoute(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOnRoute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceLaneInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceLaneInfo {}
impl ::core::fmt::Debug for GuidanceLaneInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceLaneInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceLaneInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceLaneInfo;{8404d114-6581-43b7-ac15-c9079bf90df1})");
}
impl ::core::clone::Clone for GuidanceLaneInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceLaneInfo {
    type Vtable = IGuidanceLaneInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceLaneInfo {
    const IID: ::windows_core::GUID = <IGuidanceLaneInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceLaneInfo {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceLaneInfo";
}
::windows_core::imp::interface_hierarchy!(GuidanceLaneInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceLaneInfo {}
unsafe impl ::core::marker::Sync for GuidanceLaneInfo {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceManeuver(::windows_core::IUnknown);
impl GuidanceManeuver {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn StartLocation(&self) -> ::windows_core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DistanceFromRouteStart(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DistanceFromRouteStart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DistanceFromPreviousManeuver(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DistanceFromPreviousManeuver)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DepartureRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DepartureRoadName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextRoadName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DepartureShortRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DepartureShortRoadName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextShortRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextShortRoadName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<GuidanceManeuverKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAngle(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAngle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EndAngle(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndAngle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoadSignpost(&self) -> ::windows_core::Result<GuidanceRoadSignpost> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoadSignpost)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstructionText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstructionText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceManeuver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceManeuver {}
impl ::core::fmt::Debug for GuidanceManeuver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceManeuver").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceManeuver {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceManeuver;{fc09326c-ecc9-4928-a2a1-7232b99b94a1})");
}
impl ::core::clone::Clone for GuidanceManeuver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceManeuver {
    type Vtable = IGuidanceManeuver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceManeuver {
    const IID: ::windows_core::GUID = <IGuidanceManeuver as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceManeuver {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceManeuver";
}
::windows_core::imp::interface_hierarchy!(GuidanceManeuver, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceManeuver {}
unsafe impl ::core::marker::Sync for GuidanceManeuver {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceMapMatchedCoordinate(::windows_core::IUnknown);
impl GuidanceMapMatchedCoordinate {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows_core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Location)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentHeading(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentHeading)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentSpeed(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSpeed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOnStreet(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOnStreet)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Road(&self) -> ::windows_core::Result<GuidanceRoadSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Road)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceMapMatchedCoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceMapMatchedCoordinate {}
impl ::core::fmt::Debug for GuidanceMapMatchedCoordinate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceMapMatchedCoordinate").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceMapMatchedCoordinate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate;{b7acb168-2912-4a99-aff1-798609b981fe})");
}
impl ::core::clone::Clone for GuidanceMapMatchedCoordinate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceMapMatchedCoordinate {
    type Vtable = IGuidanceMapMatchedCoordinate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceMapMatchedCoordinate {
    const IID: ::windows_core::GUID = <IGuidanceMapMatchedCoordinate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceMapMatchedCoordinate {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate";
}
::windows_core::imp::interface_hierarchy!(GuidanceMapMatchedCoordinate, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceMapMatchedCoordinate {}
unsafe impl ::core::marker::Sync for GuidanceMapMatchedCoordinate {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceNavigator(::windows_core::IUnknown);
impl GuidanceNavigator {
    pub fn StartNavigating<P0>(&self, route: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<GuidanceRoute>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartNavigating)(::windows_core::Interface::as_raw(this), route.into_param().abi()).ok() }
    }
    pub fn StartSimulating<P0>(&self, route: P0, speedinmeterspersecond: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<GuidanceRoute>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartSimulating)(::windows_core::Interface::as_raw(this), route.into_param().abi(), speedinmeterspersecond).ok() }
    }
    pub fn StartTracking(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartTracking)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RepeatLastAudioNotification(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RepeatLastAudioNotification)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AudioMeasurementSystem(&self) -> ::windows_core::Result<GuidanceAudioMeasurementSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioMeasurementSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioMeasurementSystem(&self, value: GuidanceAudioMeasurementSystem) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioMeasurementSystem)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioNotifications(&self) -> ::windows_core::Result<GuidanceAudioNotifications> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioNotifications)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioNotifications(&self, value: GuidanceAudioNotifications) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioNotifications)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GuidanceUpdated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GuidanceUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGuidanceUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGuidanceUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DestinationReached<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DestinationReached)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDestinationReached(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDestinationReached)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Rerouting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rerouting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRerouting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRerouting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Rerouted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rerouted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRerouted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRerouted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RerouteFailed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RerouteFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRerouteFailed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRerouteFailed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserLocationLost<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserLocationLost)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserLocationLost(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserLocationLost)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserLocationRestored<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserLocationRestored)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserLocationRestored(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserLocationRestored)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetGuidanceVoice(&self, voiceid: i32, voicefolder: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGuidanceVoice)(::windows_core::Interface::as_raw(this), voiceid, ::core::mem::transmute_copy(voicefolder)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn UpdateUserLocation<P0>(&self, userlocation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Devices::Geolocation::Geocoordinate>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateUserLocation)(::windows_core::Interface::as_raw(this), userlocation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn UpdateUserLocationWithPositionOverride<P0>(&self, userlocation: P0, positionoverride: super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Devices::Geolocation::Geocoordinate>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateUserLocationWithPositionOverride)(::windows_core::Interface::as_raw(this), userlocation.into_param().abi(), positionoverride).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioNotificationRequested<P0>(&self, value: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IGuidanceNavigator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioNotificationRequested)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioNotificationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGuidanceNavigator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioNotificationRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsGuidanceAudioMuted(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGuidanceNavigator2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGuidanceAudioMuted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsGuidanceAudioMuted(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGuidanceNavigator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsGuidanceAudioMuted)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrent() -> ::windows_core::Result<GuidanceNavigator> {
        Self::IGuidanceNavigatorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UseAppProvidedVoice() -> ::windows_core::Result<bool> {
        Self::IGuidanceNavigatorStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UseAppProvidedVoice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGuidanceNavigatorStatics<R, F: FnOnce(&IGuidanceNavigatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GuidanceNavigator, IGuidanceNavigatorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGuidanceNavigatorStatics2<R, F: FnOnce(&IGuidanceNavigatorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GuidanceNavigator, IGuidanceNavigatorStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GuidanceNavigator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceNavigator {}
impl ::core::fmt::Debug for GuidanceNavigator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceNavigator").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceNavigator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceNavigator;{08f17ef7-8e3f-4d9a-be8a-108f9a012c67})");
}
impl ::core::clone::Clone for GuidanceNavigator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceNavigator {
    type Vtable = IGuidanceNavigator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceNavigator {
    const IID: ::windows_core::GUID = <IGuidanceNavigator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceNavigator {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceNavigator";
}
::windows_core::imp::interface_hierarchy!(GuidanceNavigator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceNavigator {}
unsafe impl ::core::marker::Sync for GuidanceNavigator {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceReroutedEventArgs(::windows_core::IUnknown);
impl GuidanceReroutedEventArgs {
    pub fn Route(&self) -> ::windows_core::Result<GuidanceRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Route)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceReroutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceReroutedEventArgs {}
impl ::core::fmt::Debug for GuidanceReroutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceReroutedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceReroutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs;{115d4008-d528-454e-bb94-a50341d2c9f1})");
}
impl ::core::clone::Clone for GuidanceReroutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceReroutedEventArgs {
    type Vtable = IGuidanceReroutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceReroutedEventArgs {
    const IID: ::windows_core::GUID = <IGuidanceReroutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceReroutedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(GuidanceReroutedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceReroutedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceReroutedEventArgs {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceRoadSegment(::windows_core::IUnknown);
impl GuidanceRoadSegment {
    pub fn RoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoadName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShortRoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShortRoadName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SpeedLimit(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpeedLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TravelTime(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TravelTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows_core::Result<super::super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsHighway(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHighway)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTunnel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTunnel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTollRoad(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTollRoad)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsScenic(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGuidanceRoadSegment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsScenic)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceRoadSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoadSegment {}
impl ::core::fmt::Debug for GuidanceRoadSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoadSegment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceRoadSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoadSegment;{b32758a6-be78-4c63-afe7-6c2957479b3e})");
}
impl ::core::clone::Clone for GuidanceRoadSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceRoadSegment {
    type Vtable = IGuidanceRoadSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceRoadSegment {
    const IID: ::windows_core::GUID = <IGuidanceRoadSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceRoadSegment {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoadSegment";
}
::windows_core::imp::interface_hierarchy!(GuidanceRoadSegment, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceRoadSegment {}
unsafe impl ::core::marker::Sync for GuidanceRoadSegment {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceRoadSignpost(::windows_core::IUnknown);
impl GuidanceRoadSignpost {
    pub fn ExitNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Exit(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Exit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ForegroundColor(&self) -> ::windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExitDirections(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDirections)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceRoadSignpost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoadSignpost {}
impl ::core::fmt::Debug for GuidanceRoadSignpost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoadSignpost").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceRoadSignpost {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoadSignpost;{f1a728b6-f77a-4742-8312-53300f9845f0})");
}
impl ::core::clone::Clone for GuidanceRoadSignpost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceRoadSignpost {
    type Vtable = IGuidanceRoadSignpost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceRoadSignpost {
    const IID: ::windows_core::GUID = <IGuidanceRoadSignpost as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceRoadSignpost {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoadSignpost";
}
::windows_core::imp::interface_hierarchy!(GuidanceRoadSignpost, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceRoadSignpost {}
unsafe impl ::core::marker::Sync for GuidanceRoadSignpost {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceRoute(::windows_core::IUnknown);
impl GuidanceRoute {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Distance(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Distance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Maneuvers(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceManeuver>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Maneuvers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn BoundingBox(&self) -> ::windows_core::Result<super::super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundingBox)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows_core::Result<super::super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RoadSegments(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceRoadSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoadSegments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConvertToMapRoute(&self) -> ::windows_core::Result<super::MapRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertToMapRoute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanCreateFromMapRoute<P0>(maproute: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::MapRoute>,
    {
        Self::IGuidanceRouteStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanCreateFromMapRoute)(::windows_core::Interface::as_raw(this), maproute.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn TryCreateFromMapRoute<P0>(maproute: P0) -> ::windows_core::Result<GuidanceRoute>
    where
        P0: ::windows_core::IntoParam<super::MapRoute>,
    {
        Self::IGuidanceRouteStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromMapRoute)(::windows_core::Interface::as_raw(this), maproute.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGuidanceRouteStatics<R, F: FnOnce(&IGuidanceRouteStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GuidanceRoute, IGuidanceRouteStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GuidanceRoute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoute {}
impl ::core::fmt::Debug for GuidanceRoute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoute").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceRoute {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoute;{3a14545d-801a-40bd-a286-afb2010cce6c})");
}
impl ::core::clone::Clone for GuidanceRoute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceRoute {
    type Vtable = IGuidanceRoute_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceRoute {
    const IID: ::windows_core::GUID = <IGuidanceRoute as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceRoute {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoute";
}
::windows_core::imp::interface_hierarchy!(GuidanceRoute, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceRoute {}
unsafe impl ::core::marker::Sync for GuidanceRoute {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceTelemetryCollector(::windows_core::IUnknown);
impl GuidanceTelemetryCollector {
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClearLocalData(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearLocalData)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SpeedTrigger(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpeedTrigger)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpeedTrigger(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpeedTrigger)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UploadFrequency(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UploadFrequency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUploadFrequency(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUploadFrequency)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrent() -> ::windows_core::Result<GuidanceTelemetryCollector> {
        Self::IGuidanceTelemetryCollectorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGuidanceTelemetryCollectorStatics<R, F: FnOnce(&IGuidanceTelemetryCollectorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GuidanceTelemetryCollector, IGuidanceTelemetryCollectorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GuidanceTelemetryCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceTelemetryCollector {}
impl ::core::fmt::Debug for GuidanceTelemetryCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceTelemetryCollector").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceTelemetryCollector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceTelemetryCollector;{db1f8da5-b878-4d92-98dd-347d23d38262})");
}
impl ::core::clone::Clone for GuidanceTelemetryCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceTelemetryCollector {
    type Vtable = IGuidanceTelemetryCollector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceTelemetryCollector {
    const IID: ::windows_core::GUID = <IGuidanceTelemetryCollector as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceTelemetryCollector {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceTelemetryCollector";
}
::windows_core::imp::interface_hierarchy!(GuidanceTelemetryCollector, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceTelemetryCollector {}
unsafe impl ::core::marker::Sync for GuidanceTelemetryCollector {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceUpdatedEventArgs(::windows_core::IUnknown);
impl GuidanceUpdatedEventArgs {
    pub fn Mode(&self) -> ::windows_core::Result<GuidanceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextManeuver(&self) -> ::windows_core::Result<GuidanceManeuver> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextManeuver)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextManeuverDistance(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextManeuverDistance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AfterNextManeuver(&self) -> ::windows_core::Result<GuidanceManeuver> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AfterNextManeuver)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AfterNextManeuverDistance(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AfterNextManeuverDistance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DistanceToDestination(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DistanceToDestination)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ElapsedDistance(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElapsedDistance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ElapsedTime(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElapsedTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToDestination(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeToDestination)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoadName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoadName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Route(&self) -> ::windows_core::Result<GuidanceRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Route)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentLocation(&self) -> ::windows_core::Result<GuidanceMapMatchedCoordinate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsNewManeuver(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNewManeuver)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaneInfo(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceLaneInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaneInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceUpdatedEventArgs {}
impl ::core::fmt::Debug for GuidanceUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceUpdatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceUpdatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs;{fdac160b-9e8d-4de3-a9fa-b06321d18db9})");
}
impl ::core::clone::Clone for GuidanceUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GuidanceUpdatedEventArgs {
    type Vtable = IGuidanceUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GuidanceUpdatedEventArgs {
    const IID: ::windows_core::GUID = <IGuidanceUpdatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GuidanceUpdatedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(GuidanceUpdatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceUpdatedEventArgs {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceAudioMeasurementSystem(pub i32);
impl GuidanceAudioMeasurementSystem {
    pub const Meters: Self = Self(0i32);
    pub const MilesAndYards: Self = Self(1i32);
    pub const MilesAndFeet: Self = Self(2i32);
}
impl ::core::marker::Copy for GuidanceAudioMeasurementSystem {}
impl ::core::clone::Clone for GuidanceAudioMeasurementSystem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioMeasurementSystem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GuidanceAudioMeasurementSystem {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GuidanceAudioMeasurementSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioMeasurementSystem").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceAudioMeasurementSystem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioMeasurementSystem;i4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceAudioNotificationKind(pub i32);
impl GuidanceAudioNotificationKind {
    pub const Maneuver: Self = Self(0i32);
    pub const Route: Self = Self(1i32);
    pub const Gps: Self = Self(2i32);
    pub const SpeedLimit: Self = Self(3i32);
    pub const Traffic: Self = Self(4i32);
    pub const TrafficCamera: Self = Self(5i32);
}
impl ::core::marker::Copy for GuidanceAudioNotificationKind {}
impl ::core::clone::Clone for GuidanceAudioNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioNotificationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GuidanceAudioNotificationKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GuidanceAudioNotificationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotificationKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceAudioNotificationKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioNotificationKind;i4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceAudioNotifications(pub u32);
impl GuidanceAudioNotifications {
    pub const None: Self = Self(0u32);
    pub const Maneuver: Self = Self(1u32);
    pub const Route: Self = Self(2u32);
    pub const Gps: Self = Self(4u32);
    pub const SpeedLimit: Self = Self(8u32);
    pub const Traffic: Self = Self(16u32);
    pub const TrafficCamera: Self = Self(32u32);
}
impl ::core::marker::Copy for GuidanceAudioNotifications {}
impl ::core::clone::Clone for GuidanceAudioNotifications {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioNotifications {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GuidanceAudioNotifications {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GuidanceAudioNotifications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotifications").field(&self.0).finish()
    }
}
impl GuidanceAudioNotifications {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GuidanceAudioNotifications {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GuidanceAudioNotifications {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GuidanceAudioNotifications {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GuidanceAudioNotifications {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GuidanceAudioNotifications {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for GuidanceAudioNotifications {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioNotifications;u4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceLaneMarkers(pub u32);
impl GuidanceLaneMarkers {
    pub const None: Self = Self(0u32);
    pub const LightRight: Self = Self(1u32);
    pub const Right: Self = Self(2u32);
    pub const HardRight: Self = Self(4u32);
    pub const Straight: Self = Self(8u32);
    pub const UTurnLeft: Self = Self(16u32);
    pub const HardLeft: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const LightLeft: Self = Self(128u32);
    pub const UTurnRight: Self = Self(256u32);
    pub const Unknown: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for GuidanceLaneMarkers {}
impl ::core::clone::Clone for GuidanceLaneMarkers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceLaneMarkers {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GuidanceLaneMarkers {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GuidanceLaneMarkers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceLaneMarkers").field(&self.0).finish()
    }
}
impl GuidanceLaneMarkers {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GuidanceLaneMarkers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GuidanceLaneMarkers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GuidanceLaneMarkers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GuidanceLaneMarkers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GuidanceLaneMarkers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for GuidanceLaneMarkers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceLaneMarkers;u4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceManeuverKind(pub i32);
impl GuidanceManeuverKind {
    pub const None: Self = Self(0i32);
    pub const GoStraight: Self = Self(1i32);
    pub const UTurnRight: Self = Self(2i32);
    pub const UTurnLeft: Self = Self(3i32);
    pub const TurnKeepRight: Self = Self(4i32);
    pub const TurnLightRight: Self = Self(5i32);
    pub const TurnRight: Self = Self(6i32);
    pub const TurnHardRight: Self = Self(7i32);
    pub const KeepMiddle: Self = Self(8i32);
    pub const TurnKeepLeft: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLeft: Self = Self(11i32);
    pub const TurnHardLeft: Self = Self(12i32);
    pub const FreewayEnterRight: Self = Self(13i32);
    pub const FreewayEnterLeft: Self = Self(14i32);
    pub const FreewayLeaveRight: Self = Self(15i32);
    pub const FreewayLeaveLeft: Self = Self(16i32);
    pub const FreewayKeepRight: Self = Self(17i32);
    pub const FreewayKeepLeft: Self = Self(18i32);
    pub const TrafficCircleRight1: Self = Self(19i32);
    pub const TrafficCircleRight2: Self = Self(20i32);
    pub const TrafficCircleRight3: Self = Self(21i32);
    pub const TrafficCircleRight4: Self = Self(22i32);
    pub const TrafficCircleRight5: Self = Self(23i32);
    pub const TrafficCircleRight6: Self = Self(24i32);
    pub const TrafficCircleRight7: Self = Self(25i32);
    pub const TrafficCircleRight8: Self = Self(26i32);
    pub const TrafficCircleRight9: Self = Self(27i32);
    pub const TrafficCircleRight10: Self = Self(28i32);
    pub const TrafficCircleRight11: Self = Self(29i32);
    pub const TrafficCircleRight12: Self = Self(30i32);
    pub const TrafficCircleLeft1: Self = Self(31i32);
    pub const TrafficCircleLeft2: Self = Self(32i32);
    pub const TrafficCircleLeft3: Self = Self(33i32);
    pub const TrafficCircleLeft4: Self = Self(34i32);
    pub const TrafficCircleLeft5: Self = Self(35i32);
    pub const TrafficCircleLeft6: Self = Self(36i32);
    pub const TrafficCircleLeft7: Self = Self(37i32);
    pub const TrafficCircleLeft8: Self = Self(38i32);
    pub const TrafficCircleLeft9: Self = Self(39i32);
    pub const TrafficCircleLeft10: Self = Self(40i32);
    pub const TrafficCircleLeft11: Self = Self(41i32);
    pub const TrafficCircleLeft12: Self = Self(42i32);
    pub const Start: Self = Self(43i32);
    pub const End: Self = Self(44i32);
    pub const TakeFerry: Self = Self(45i32);
    pub const PassTransitStation: Self = Self(46i32);
    pub const LeaveTransitStation: Self = Self(47i32);
}
impl ::core::marker::Copy for GuidanceManeuverKind {}
impl ::core::clone::Clone for GuidanceManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceManeuverKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GuidanceManeuverKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GuidanceManeuverKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceManeuverKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceManeuverKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceManeuverKind;i4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceMode(pub i32);
impl GuidanceMode {
    pub const None: Self = Self(0i32);
    pub const Simulation: Self = Self(1i32);
    pub const Navigation: Self = Self(2i32);
    pub const Tracking: Self = Self(3i32);
}
impl ::core::marker::Copy for GuidanceMode {}
impl ::core::clone::Clone for GuidanceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GuidanceMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GuidanceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GuidanceMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceMode;i4)");
}
