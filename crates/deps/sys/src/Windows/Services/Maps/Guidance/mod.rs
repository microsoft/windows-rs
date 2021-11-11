#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GuidanceAudioMeasurementSystem();
    fn GuidanceAudioNotificationKind();
    fn GuidanceAudioNotificationRequestedEventArgs();
    fn GuidanceAudioNotifications();
    fn GuidanceLaneInfo();
    fn GuidanceLaneMarkers();
    fn GuidanceManeuver();
    fn GuidanceManeuverKind();
    fn GuidanceMapMatchedCoordinate();
    fn GuidanceMode();
    fn GuidanceNavigator();
    fn GuidanceReroutedEventArgs();
    fn GuidanceRoadSegment();
    fn GuidanceRoadSignpost();
    fn GuidanceRoute();
    fn GuidanceTelemetryCollector();
    fn GuidanceUpdatedEventArgs();
    fn IGuidanceAudioNotificationRequestedEventArgs();
    fn IGuidanceLaneInfo();
    fn IGuidanceManeuver();
    fn IGuidanceMapMatchedCoordinate();
    fn IGuidanceNavigator();
    fn IGuidanceNavigator2();
    fn IGuidanceNavigatorStatics();
    fn IGuidanceNavigatorStatics2();
    fn IGuidanceReroutedEventArgs();
    fn IGuidanceRoadSegment();
    fn IGuidanceRoadSegment2();
    fn IGuidanceRoadSignpost();
    fn IGuidanceRoute();
    fn IGuidanceRouteStatics();
    fn IGuidanceTelemetryCollector();
    fn IGuidanceTelemetryCollectorStatics();
    fn IGuidanceUpdatedEventArgs();
}
