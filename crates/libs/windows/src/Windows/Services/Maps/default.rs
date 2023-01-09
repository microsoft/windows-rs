impl ::core::cmp::PartialEq for EnhancedWaypoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnhancedWaypoint {}
impl ::core::fmt::Debug for EnhancedWaypoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnhancedWaypoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ManeuverWarning {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManeuverWarning {}
impl ::core::fmt::Debug for ManeuverWarning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManeuverWarning").field(&self.0).finish()
    }
}
impl ::core::default::Default for ManeuverWarningKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ManeuverWarningKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManeuverWarningKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for ManeuverWarningSeverity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ManeuverWarningSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManeuverWarningSeverity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MapAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapAddress {}
impl ::core::fmt::Debug for MapAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapAddress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MapLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapLocation {}
impl ::core::fmt::Debug for MapLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocation").field(&self.0).finish()
    }
}
impl ::core::default::Default for MapLocationDesiredAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MapLocationDesiredAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocationDesiredAccuracy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MapLocationFinderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapLocationFinderResult {}
impl ::core::fmt::Debug for MapLocationFinderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocationFinderResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for MapLocationFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MapLocationFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocationFinderStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for MapManeuverNotices {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MapManeuverNotices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapManeuverNotices").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MapManeuverNotices {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MapManeuverNotices {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MapManeuverNotices {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MapManeuverNotices {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MapManeuverNotices {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for MapRoute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRoute {}
impl ::core::fmt::Debug for MapRoute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRoute").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MapRouteDrivingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteDrivingOptions {}
impl ::core::fmt::Debug for MapRouteDrivingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteDrivingOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MapRouteFinderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteFinderResult {}
impl ::core::fmt::Debug for MapRouteFinderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteFinderResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for MapRouteFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MapRouteFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteFinderStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MapRouteLeg {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteLeg {}
impl ::core::fmt::Debug for MapRouteLeg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteLeg").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MapRouteManeuver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MapRouteManeuver {}
impl ::core::fmt::Debug for MapRouteManeuver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteManeuver").field(&self.0).finish()
    }
}
impl ::core::default::Default for MapRouteManeuverKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MapRouteManeuverKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteManeuverKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for MapRouteOptimization {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MapRouteOptimization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteOptimization").field(&self.0).finish()
    }
}
impl ::core::default::Default for MapRouteRestrictions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MapRouteRestrictions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteRestrictions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MapRouteRestrictions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MapRouteRestrictions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MapRouteRestrictions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MapRouteRestrictions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MapRouteRestrictions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MapServiceDataUsagePreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MapServiceDataUsagePreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapServiceDataUsagePreference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PlaceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaceInfo {}
impl ::core::fmt::Debug for PlaceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaceInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PlaceInfoCreateOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaceInfoCreateOptions {}
impl ::core::fmt::Debug for PlaceInfoCreateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaceInfoCreateOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for TrafficCongestion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TrafficCongestion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TrafficCongestion").field(&self.0).finish()
    }
}
impl ::core::default::Default for WaypointKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WaypointKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WaypointKind").field(&self.0).finish()
    }
}
