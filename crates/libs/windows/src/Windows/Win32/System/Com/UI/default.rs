impl ::core::cmp::PartialEq for IDummyHICONIncluder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDummyHICONIncluder {}
impl ::core::fmt::Debug for IDummyHICONIncluder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDummyHICONIncluder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IThumbnailExtractor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThumbnailExtractor {}
impl ::core::fmt::Debug for IThumbnailExtractor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThumbnailExtractor").field(&self.0).finish()
    }
}
