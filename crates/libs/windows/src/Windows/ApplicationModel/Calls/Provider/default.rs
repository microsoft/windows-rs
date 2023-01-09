#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PhoneCallOrigin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PhoneCallOrigin {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PhoneCallOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallOrigin").field(&self.0).finish()
    }
}
