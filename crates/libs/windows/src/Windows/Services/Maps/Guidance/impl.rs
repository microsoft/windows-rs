#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceAudioNotificationRequestedEventArgsImpl: Sized {
    fn AudioNotification(&self) -> ::windows::core::Result<GuidanceAudioNotificationKind>;
    fn AudioFilePaths(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn AudioText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceLaneInfoImpl: Sized {
    fn LaneMarkers(&self) -> ::windows::core::Result<GuidanceLaneMarkers>;
    fn IsOnRoute(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceManeuverImpl: Sized {
    fn StartLocation(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint>;
    fn DistanceFromRouteStart(&self) -> ::windows::core::Result<i32>;
    fn DistanceFromPreviousManeuver(&self) -> ::windows::core::Result<i32>;
    fn DepartureRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NextRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DepartureShortRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NextShortRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<GuidanceManeuverKind>;
    fn StartAngle(&self) -> ::windows::core::Result<i32>;
    fn EndAngle(&self) -> ::windows::core::Result<i32>;
    fn RoadSignpost(&self) -> ::windows::core::Result<GuidanceRoadSignpost>;
    fn InstructionText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceMapMatchedCoordinateImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint>;
    fn CurrentHeading(&self) -> ::windows::core::Result<f64>;
    fn CurrentSpeed(&self) -> ::windows::core::Result<f64>;
    fn IsOnStreet(&self) -> ::windows::core::Result<bool>;
    fn Road(&self) -> ::windows::core::Result<GuidanceRoadSegment>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceNavigatorImpl: Sized {
    fn StartNavigating(&self, route: &::core::option::Option<GuidanceRoute>) -> ::windows::core::Result<()>;
    fn StartSimulating(&self, route: &::core::option::Option<GuidanceRoute>, speedinmeterspersecond: i32) -> ::windows::core::Result<()>;
    fn StartTracking(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn RepeatLastAudioNotification(&self) -> ::windows::core::Result<()>;
    fn AudioMeasurementSystem(&self) -> ::windows::core::Result<GuidanceAudioMeasurementSystem>;
    fn SetAudioMeasurementSystem(&self, value: GuidanceAudioMeasurementSystem) -> ::windows::core::Result<()>;
    fn AudioNotifications(&self) -> ::windows::core::Result<GuidanceAudioNotifications>;
    fn SetAudioNotifications(&self, value: GuidanceAudioNotifications) -> ::windows::core::Result<()>;
    fn GuidanceUpdated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGuidanceUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DestinationReached(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDestinationReached(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Rerouting(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRerouting(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Rerouted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRerouted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RerouteFailed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRerouteFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UserLocationLost(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserLocationLost(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UserLocationRestored(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserLocationRestored(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetGuidanceVoice(&self, voiceid: i32, voicefolder: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UpdateUserLocation(&self, userlocation: &::core::option::Option<super::super::super::Devices::Geolocation::Geocoordinate>) -> ::windows::core::Result<()>;
    fn UpdateUserLocationWithPositionOverride(&self, userlocation: &::core::option::Option<super::super::super::Devices::Geolocation::Geocoordinate>, positionoverride: &super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceNavigator2Impl: Sized {
    fn AudioNotificationRequested(&self, value: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioNotificationRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsGuidanceAudioMuted(&self) -> ::windows::core::Result<bool>;
    fn SetIsGuidanceAudioMuted(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceNavigatorStaticsImpl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<GuidanceNavigator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceNavigatorStatics2Impl: Sized {
    fn UseAppProvidedVoice(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceReroutedEventArgsImpl: Sized {
    fn Route(&self) -> ::windows::core::Result<GuidanceRoute>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceRoadSegmentImpl: Sized {
    fn RoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShortRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SpeedLimit(&self) -> ::windows::core::Result<f64>;
    fn TravelTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Path(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopath>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsHighway(&self) -> ::windows::core::Result<bool>;
    fn IsTunnel(&self) -> ::windows::core::Result<bool>;
    fn IsTollRoad(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceRoadSegment2Impl: Sized {
    fn IsScenic(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceRoadSignpostImpl: Sized {
    fn ExitNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Exit(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::super::UI::Color>;
    fn ExitDirections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceRouteImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Distance(&self) -> ::windows::core::Result<i32>;
    fn Maneuvers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceManeuver>>;
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::GeoboundingBox>;
    fn Path(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopath>;
    fn RoadSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceRoadSegment>>;
    fn ConvertToMapRoute(&self) -> ::windows::core::Result<super::MapRoute>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceRouteStaticsImpl: Sized {
    fn CanCreateFromMapRoute(&self, maproute: &::core::option::Option<super::MapRoute>) -> ::windows::core::Result<bool>;
    fn TryCreateFromMapRoute(&self, maproute: &::core::option::Option<super::MapRoute>) -> ::windows::core::Result<GuidanceRoute>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceTelemetryCollectorImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ClearLocalData(&self) -> ::windows::core::Result<()>;
    fn SpeedTrigger(&self) -> ::windows::core::Result<f64>;
    fn SetSpeedTrigger(&self, value: f64) -> ::windows::core::Result<()>;
    fn UploadFrequency(&self) -> ::windows::core::Result<i32>;
    fn SetUploadFrequency(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceTelemetryCollectorStaticsImpl: Sized {
    fn GetCurrent(&self) -> ::windows::core::Result<GuidanceTelemetryCollector>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGuidanceUpdatedEventArgsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<GuidanceMode>;
    fn NextManeuver(&self) -> ::windows::core::Result<GuidanceManeuver>;
    fn NextManeuverDistance(&self) -> ::windows::core::Result<i32>;
    fn AfterNextManeuver(&self) -> ::windows::core::Result<GuidanceManeuver>;
    fn AfterNextManeuverDistance(&self) -> ::windows::core::Result<i32>;
    fn DistanceToDestination(&self) -> ::windows::core::Result<i32>;
    fn ElapsedDistance(&self) -> ::windows::core::Result<i32>;
    fn ElapsedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn TimeToDestination(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn RoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Route(&self) -> ::windows::core::Result<GuidanceRoute>;
    fn CurrentLocation(&self) -> ::windows::core::Result<GuidanceMapMatchedCoordinate>;
    fn IsNewManeuver(&self) -> ::windows::core::Result<bool>;
    fn LaneInfo(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceLaneInfo>>;
}
