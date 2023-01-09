impl ::core::cmp::PartialEq for GpioPinProviderValueChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioPinProviderValueChangedEventArgs {}
impl ::core::fmt::Debug for GpioPinProviderValueChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinProviderValueChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGpioControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGpioControllerProvider {}
impl ::core::fmt::Debug for IGpioControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGpioControllerProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGpioPinProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGpioPinProvider {}
impl ::core::fmt::Debug for IGpioPinProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGpioPinProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGpioProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGpioProvider {}
impl ::core::fmt::Debug for IGpioProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGpioProvider").field(&self.0).finish()
    }
}
impl ::core::default::Default for ProviderGpioPinDriveMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ProviderGpioPinDriveMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderGpioPinDriveMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for ProviderGpioPinEdge {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ProviderGpioPinEdge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderGpioPinEdge").field(&self.0).finish()
    }
}
impl ::core::default::Default for ProviderGpioPinValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ProviderGpioPinValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderGpioPinValue").field(&self.0).finish()
    }
}
impl ::core::default::Default for ProviderGpioSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ProviderGpioSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderGpioSharingMode").field(&self.0).finish()
    }
}
