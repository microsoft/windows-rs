impl ::core::cmp::PartialEq for CompositorController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositorController {}
impl ::core::fmt::Debug for CompositorController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositorController").field(&self.0).finish()
    }
}
