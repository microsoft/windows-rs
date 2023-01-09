impl ::core::default::Default for GameServiceGameOutcome {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GameServiceGameOutcome {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceGameOutcome").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GameServicePropertyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameServicePropertyCollection {}
impl ::core::fmt::Debug for GameServicePropertyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServicePropertyCollection").field(&self.0).finish()
    }
}
impl ::core::default::Default for GameServiceScoreKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GameServiceScoreKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceScoreKind").field(&self.0).finish()
    }
}
