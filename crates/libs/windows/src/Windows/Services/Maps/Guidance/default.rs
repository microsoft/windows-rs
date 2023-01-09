impl ::core::default::Default for GuidanceAudioMeasurementSystem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GuidanceAudioMeasurementSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioMeasurementSystem").field(&self.0).finish()
    }
}
impl ::core::default::Default for GuidanceAudioNotificationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GuidanceAudioNotificationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotificationKind").field(&self.0).finish()
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
impl ::core::default::Default for GuidanceAudioNotifications {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GuidanceAudioNotifications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotifications").field(&self.0).finish()
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
impl ::core::default::Default for GuidanceLaneMarkers {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GuidanceLaneMarkers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceLaneMarkers").field(&self.0).finish()
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
impl ::core::default::Default for GuidanceManeuverKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GuidanceManeuverKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceManeuverKind").field(&self.0).finish()
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
impl ::core::default::Default for GuidanceMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GuidanceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceMode").field(&self.0).finish()
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
