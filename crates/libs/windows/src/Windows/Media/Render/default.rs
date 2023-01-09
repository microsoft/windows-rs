impl ::core::default::Default for AudioRenderCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AudioRenderCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioRenderCategory").field(&self.0).finish()
    }
}
