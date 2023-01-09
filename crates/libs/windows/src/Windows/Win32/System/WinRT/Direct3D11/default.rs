impl ::core::cmp::PartialEq for IDirect3DDxgiInterfaceAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDxgiInterfaceAccess {}
impl ::core::fmt::Debug for IDirect3DDxgiInterfaceAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDxgiInterfaceAccess").field(&self.0).finish()
    }
}
