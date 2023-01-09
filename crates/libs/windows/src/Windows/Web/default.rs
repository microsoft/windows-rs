impl ::core::cmp::PartialEq for IUriToStreamResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriToStreamResolver {}
impl ::core::fmt::Debug for IUriToStreamResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriToStreamResolver").field(&self.0).finish()
    }
}
impl ::core::default::Default for WebErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WebErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebErrorStatus").field(&self.0).finish()
    }
}
