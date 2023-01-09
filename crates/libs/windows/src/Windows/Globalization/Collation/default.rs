impl ::core::cmp::PartialEq for CharacterGrouping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterGrouping {}
impl ::core::fmt::Debug for CharacterGrouping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterGrouping").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CharacterGroupings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterGroupings {}
impl ::core::fmt::Debug for CharacterGroupings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterGroupings").field(&self.0).finish()
    }
}
