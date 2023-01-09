impl ::core::default::Default for AltitudeReferenceSystem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AltitudeReferenceSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AltitudeReferenceSystem").field(&self.0).finish()
    }
}
impl ::core::default::Default for BasicGeoposition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BasicGeoposition {
    fn eq(&self, other: &Self) -> bool {
        self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.Altitude == other.Altitude
    }
}
impl ::core::cmp::Eq for BasicGeoposition {}
impl ::core::fmt::Debug for BasicGeoposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BasicGeoposition").field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).finish()
    }
}
impl ::core::cmp::PartialEq for CivicAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CivicAddress {}
impl ::core::fmt::Debug for CivicAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CivicAddress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GeoboundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeoboundingBox {}
impl ::core::fmt::Debug for GeoboundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeoboundingBox").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Geocircle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geocircle {}
impl ::core::fmt::Debug for Geocircle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geocircle").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Geocoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geocoordinate {}
impl ::core::fmt::Debug for Geocoordinate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geocoordinate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GeocoordinateSatelliteData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeocoordinateSatelliteData {}
impl ::core::fmt::Debug for GeocoordinateSatelliteData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeocoordinateSatelliteData").field(&self.0).finish()
    }
}
impl ::core::default::Default for GeolocationAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GeolocationAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeolocationAccessStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Geolocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geolocator {}
impl ::core::fmt::Debug for Geolocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geolocator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Geopath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geopath {}
impl ::core::fmt::Debug for Geopath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geopath").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Geopoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geopoint {}
impl ::core::fmt::Debug for Geopoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geopoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Geoposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geoposition {}
impl ::core::fmt::Debug for Geoposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geoposition").field(&self.0).finish()
    }
}
impl ::core::default::Default for GeoshapeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GeoshapeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeoshapeType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Geovisit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Geovisit {}
impl ::core::fmt::Debug for Geovisit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Geovisit").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GeovisitMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitMonitor {}
impl ::core::fmt::Debug for GeovisitMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitMonitor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GeovisitStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitStateChangedEventArgs {}
impl ::core::fmt::Debug for GeovisitStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GeovisitTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitTriggerDetails {}
impl ::core::fmt::Debug for GeovisitTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGeoshape {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGeoshape {}
impl ::core::fmt::Debug for IGeoshape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGeoshape").field(&self.0).finish()
    }
}
impl ::core::default::Default for PositionAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PositionAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionAccuracy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PositionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PositionChangedEventArgs {}
impl ::core::fmt::Debug for PositionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for PositionSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PositionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionSource").field(&self.0).finish()
    }
}
impl ::core::default::Default for PositionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PositionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for StatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatusChangedEventArgs {}
impl ::core::fmt::Debug for StatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VenueData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VenueData {}
impl ::core::fmt::Debug for VenueData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VenueData").field(&self.0).finish()
    }
}
impl ::core::default::Default for VisitMonitoringScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VisitMonitoringScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisitMonitoringScope").field(&self.0).finish()
    }
}
impl ::core::default::Default for VisitStateChange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VisitStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisitStateChange").field(&self.0).finish()
    }
}
