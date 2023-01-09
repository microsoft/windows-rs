#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMRPIXELFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EMRPIXELFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.pfd == other.pfd
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EMRPIXELFORMAT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMRPIXELFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPIXELFORMAT").field("emr", &self.emr).field("pfd", &self.pfd).finish()
    }
}
impl ::core::default::Default for GLYPHMETRICSFLOAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GLYPHMETRICSFLOAT {
    fn eq(&self, other: &Self) -> bool {
        self.gmfBlackBoxX == other.gmfBlackBoxX && self.gmfBlackBoxY == other.gmfBlackBoxY && self.gmfptGlyphOrigin == other.gmfptGlyphOrigin && self.gmfCellIncX == other.gmfCellIncX && self.gmfCellIncY == other.gmfCellIncY
    }
}
impl ::core::cmp::Eq for GLYPHMETRICSFLOAT {}
impl ::core::fmt::Debug for GLYPHMETRICSFLOAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHMETRICSFLOAT").field("gmfBlackBoxX", &self.gmfBlackBoxX).field("gmfBlackBoxY", &self.gmfBlackBoxY).field("gmfptGlyphOrigin", &self.gmfptGlyphOrigin).field("gmfCellIncX", &self.gmfCellIncX).field("gmfCellIncY", &self.gmfCellIncY).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LAYERPLANEDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LAYERPLANEDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.nSize == other.nSize
            && self.nVersion == other.nVersion
            && self.dwFlags == other.dwFlags
            && self.iPixelType == other.iPixelType
            && self.cColorBits == other.cColorBits
            && self.cRedBits == other.cRedBits
            && self.cRedShift == other.cRedShift
            && self.cGreenBits == other.cGreenBits
            && self.cGreenShift == other.cGreenShift
            && self.cBlueBits == other.cBlueBits
            && self.cBlueShift == other.cBlueShift
            && self.cAlphaBits == other.cAlphaBits
            && self.cAlphaShift == other.cAlphaShift
            && self.cAccumBits == other.cAccumBits
            && self.cAccumRedBits == other.cAccumRedBits
            && self.cAccumGreenBits == other.cAccumGreenBits
            && self.cAccumBlueBits == other.cAccumBlueBits
            && self.cAccumAlphaBits == other.cAccumAlphaBits
            && self.cDepthBits == other.cDepthBits
            && self.cStencilBits == other.cStencilBits
            && self.cAuxBuffers == other.cAuxBuffers
            && self.iLayerPlane == other.iLayerPlane
            && self.bReserved == other.bReserved
            && self.crTransparent == other.crTransparent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LAYERPLANEDESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LAYERPLANEDESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LAYERPLANEDESCRIPTOR")
            .field("nSize", &self.nSize)
            .field("nVersion", &self.nVersion)
            .field("dwFlags", &self.dwFlags)
            .field("iPixelType", &self.iPixelType)
            .field("cColorBits", &self.cColorBits)
            .field("cRedBits", &self.cRedBits)
            .field("cRedShift", &self.cRedShift)
            .field("cGreenBits", &self.cGreenBits)
            .field("cGreenShift", &self.cGreenShift)
            .field("cBlueBits", &self.cBlueBits)
            .field("cBlueShift", &self.cBlueShift)
            .field("cAlphaBits", &self.cAlphaBits)
            .field("cAlphaShift", &self.cAlphaShift)
            .field("cAccumBits", &self.cAccumBits)
            .field("cAccumRedBits", &self.cAccumRedBits)
            .field("cAccumGreenBits", &self.cAccumGreenBits)
            .field("cAccumBlueBits", &self.cAccumBlueBits)
            .field("cAccumAlphaBits", &self.cAccumAlphaBits)
            .field("cDepthBits", &self.cDepthBits)
            .field("cStencilBits", &self.cStencilBits)
            .field("cAuxBuffers", &self.cAuxBuffers)
            .field("iLayerPlane", &self.iLayerPlane)
            .field("bReserved", &self.bReserved)
            .field("crTransparent", &self.crTransparent)
            .finish()
    }
}
impl ::core::default::Default for PFD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PFD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PFD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PFD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PFD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PFD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PFD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PFD_LAYER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PFD_LAYER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFD_LAYER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PFD_PIXEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PFD_PIXEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFD_PIXEL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PIXELFORMATDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PIXELFORMATDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.nSize == other.nSize
            && self.nVersion == other.nVersion
            && self.dwFlags == other.dwFlags
            && self.iPixelType == other.iPixelType
            && self.cColorBits == other.cColorBits
            && self.cRedBits == other.cRedBits
            && self.cRedShift == other.cRedShift
            && self.cGreenBits == other.cGreenBits
            && self.cGreenShift == other.cGreenShift
            && self.cBlueBits == other.cBlueBits
            && self.cBlueShift == other.cBlueShift
            && self.cAlphaBits == other.cAlphaBits
            && self.cAlphaShift == other.cAlphaShift
            && self.cAccumBits == other.cAccumBits
            && self.cAccumRedBits == other.cAccumRedBits
            && self.cAccumGreenBits == other.cAccumGreenBits
            && self.cAccumBlueBits == other.cAccumBlueBits
            && self.cAccumAlphaBits == other.cAccumAlphaBits
            && self.cDepthBits == other.cDepthBits
            && self.cStencilBits == other.cStencilBits
            && self.cAuxBuffers == other.cAuxBuffers
            && self.iLayerType == other.iLayerType
            && self.bReserved == other.bReserved
            && self.dwLayerMask == other.dwLayerMask
            && self.dwVisibleMask == other.dwVisibleMask
            && self.dwDamageMask == other.dwDamageMask
    }
}
impl ::core::cmp::Eq for PIXELFORMATDESCRIPTOR {}
impl ::core::fmt::Debug for PIXELFORMATDESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PIXELFORMATDESCRIPTOR")
            .field("nSize", &self.nSize)
            .field("nVersion", &self.nVersion)
            .field("dwFlags", &self.dwFlags)
            .field("iPixelType", &self.iPixelType)
            .field("cColorBits", &self.cColorBits)
            .field("cRedBits", &self.cRedBits)
            .field("cRedShift", &self.cRedShift)
            .field("cGreenBits", &self.cGreenBits)
            .field("cGreenShift", &self.cGreenShift)
            .field("cBlueBits", &self.cBlueBits)
            .field("cBlueShift", &self.cBlueShift)
            .field("cAlphaBits", &self.cAlphaBits)
            .field("cAlphaShift", &self.cAlphaShift)
            .field("cAccumBits", &self.cAccumBits)
            .field("cAccumRedBits", &self.cAccumRedBits)
            .field("cAccumGreenBits", &self.cAccumGreenBits)
            .field("cAccumBlueBits", &self.cAccumBlueBits)
            .field("cAccumAlphaBits", &self.cAccumAlphaBits)
            .field("cDepthBits", &self.cDepthBits)
            .field("cStencilBits", &self.cStencilBits)
            .field("cAuxBuffers", &self.cAuxBuffers)
            .field("iLayerType", &self.iLayerType)
            .field("bReserved", &self.bReserved)
            .field("dwLayerMask", &self.dwLayerMask)
            .field("dwVisibleMask", &self.dwVisibleMask)
            .field("dwDamageMask", &self.dwDamageMask)
            .finish()
    }
}
impl ::core::default::Default for POINTFLOAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POINTFLOAT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTFLOAT {}
impl ::core::fmt::Debug for POINTFLOAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTFLOAT").field("x", &self.x).field("y", &self.y).finish()
    }
}
