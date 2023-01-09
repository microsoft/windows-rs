impl ::core::cmp::PartialEq for IGraphicsEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffect {}
impl ::core::fmt::Debug for IGraphicsEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffect").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGraphicsEffectSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffectSource {}
impl ::core::fmt::Debug for IGraphicsEffectSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffectSource").field(&self.0).finish()
    }
}
