#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CORRELATION_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CORRELATION_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Vector == other.Vector
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CORRELATION_VECTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CORRELATION_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CORRELATION_VECTOR").field("Version", &self.Version).field("Vector", &self.Vector).finish()
    }
}
