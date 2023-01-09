impl ::core::cmp::PartialEq for PnpObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PnpObject {}
impl ::core::fmt::Debug for PnpObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PnpObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PnpObjectCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PnpObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObjectCollection").field(&self.0).finish()
    }
}
impl ::core::default::Default for PnpObjectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PnpObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObjectType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PnpObjectUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PnpObjectUpdate {}
impl ::core::fmt::Debug for PnpObjectUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObjectUpdate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PnpObjectWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PnpObjectWatcher {}
impl ::core::fmt::Debug for PnpObjectWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PnpObjectWatcher").field(&self.0).finish()
    }
}
