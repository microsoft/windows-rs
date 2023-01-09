impl ::core::cmp::PartialEq for InstalledDesktopApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InstalledDesktopApp {}
impl ::core::fmt::Debug for InstalledDesktopApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InstalledDesktopApp").field(&self.0).finish()
    }
}
