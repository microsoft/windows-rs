impl ::core::default::Default for LicenseProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LicenseProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseProtectionStatus").field(&self.0).finish()
    }
}
