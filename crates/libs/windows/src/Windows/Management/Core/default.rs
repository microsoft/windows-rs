impl ::core::cmp::PartialEq for ApplicationDataManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationDataManager {}
impl ::core::fmt::Debug for ApplicationDataManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataManager").field(&self.0).finish()
    }
}
