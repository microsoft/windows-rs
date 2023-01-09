impl ::core::cmp::PartialEq for HolographicKeyboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HolographicKeyboard {}
impl ::core::fmt::Debug for HolographicKeyboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HolographicKeyboard").field(&self.0).finish()
    }
}
