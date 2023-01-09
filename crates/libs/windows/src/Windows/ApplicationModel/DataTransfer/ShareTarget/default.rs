impl ::core::cmp::PartialEq for QuickLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuickLink {}
impl ::core::fmt::Debug for QuickLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuickLink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ShareOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareOperation {}
impl ::core::fmt::Debug for ShareOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareOperation").field(&self.0).finish()
    }
}
