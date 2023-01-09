impl ::core::default::Default for Direct3DBindings {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Direct3DBindings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direct3DBindings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for Direct3DBindings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for Direct3DBindings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for Direct3DBindings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for Direct3DBindings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for Direct3DBindings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for Direct3DMultisampleDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Direct3DMultisampleDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Quality == other.Quality
    }
}
impl ::core::cmp::Eq for Direct3DMultisampleDescription {}
impl ::core::fmt::Debug for Direct3DMultisampleDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Direct3DMultisampleDescription").field("Count", &self.Count).field("Quality", &self.Quality).finish()
    }
}
impl ::core::default::Default for Direct3DSurfaceDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Direct3DSurfaceDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.MultisampleDescription == other.MultisampleDescription
    }
}
impl ::core::cmp::Eq for Direct3DSurfaceDescription {}
impl ::core::fmt::Debug for Direct3DSurfaceDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Direct3DSurfaceDescription").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("MultisampleDescription", &self.MultisampleDescription).finish()
    }
}
impl ::core::default::Default for Direct3DUsage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Direct3DUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direct3DUsage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice {}
impl ::core::fmt::Debug for IDirect3DDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSurface {}
impl ::core::fmt::Debug for IDirect3DSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSurface").field(&self.0).finish()
    }
}
