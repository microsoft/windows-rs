impl ::core::default::Default for PowerSavingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PowerSavingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSavingMode").field(&self.0).finish()
    }
}
