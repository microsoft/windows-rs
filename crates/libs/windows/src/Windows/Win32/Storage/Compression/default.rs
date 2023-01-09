impl ::core::default::Default for COMPRESS_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPRESS_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPRESS_ALGORITHM").field(&self.0).finish()
    }
}
impl ::core::default::Default for COMPRESS_ALLOCATION_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COMPRESS_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPRESS_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPRESS_INFORMATION_CLASS").field(&self.0).finish()
    }
}
