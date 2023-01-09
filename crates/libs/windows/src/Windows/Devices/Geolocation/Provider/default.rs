impl ::core::cmp::PartialEq for GeolocationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeolocationProvider {}
impl ::core::fmt::Debug for GeolocationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeolocationProvider").field(&self.0).finish()
    }
}
impl ::core::default::Default for LocationOverrideStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LocationOverrideStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocationOverrideStatus").field(&self.0).finish()
    }
}
