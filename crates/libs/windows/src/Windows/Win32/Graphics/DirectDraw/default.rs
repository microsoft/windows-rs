#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ACCESSRECTLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ACCESSRECTLIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.rDest == other.rDest && self.lpOwner == other.lpOwner && self.lpSurfaceData == other.lpSurfaceData && self.dwFlags == other.dwFlags && self.lpHeapAliasInfo == other.lpHeapAliasInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ACCESSRECTLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ACCESSRECTLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESSRECTLIST").field("lpLink", &self.lpLink).field("rDest", &self.rDest).field("lpOwner", &self.lpOwner).field("lpSurfaceData", &self.lpSurfaceData).field("dwFlags", &self.dwFlags).field("lpHeapAliasInfo", &self.lpHeapAliasInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ATTACHLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ATTACHLIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpLink == other.lpLink && self.lpAttached == other.lpAttached && self.lpIAttached == other.lpIAttached
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ATTACHLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ATTACHLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATTACHLIST").field("dwFlags", &self.dwFlags).field("lpLink", &self.lpLink).field("lpAttached", &self.lpAttached).field("lpIAttached", &self.lpIAttached).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DBLNODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DBLNODE {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.prev == other.prev && self.object == other.object && self.object_int == other.object_int
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DBLNODE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DBLNODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBLNODE").field("next", &self.next).field("prev", &self.prev).field("object", &self.object).field("object_int", &self.object_int).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD32BITDRIVERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD32BITDRIVERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.szEntryPoint == other.szEntryPoint && self.dwContext == other.dwContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD32BITDRIVERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD32BITDRIVERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD32BITDRIVERDATA").field("szName", &self.szName).field("szEntryPoint", &self.szEntryPoint).field("dwContext", &self.dwContext).finish()
    }
}
impl ::core::default::Default for DDARGB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDARGB {
    fn eq(&self, other: &Self) -> bool {
        self.blue == other.blue && self.green == other.green && self.red == other.red && self.alpha == other.alpha
    }
}
impl ::core::cmp::Eq for DDARGB {}
impl ::core::fmt::Debug for DDARGB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDARGB").field("blue", &self.blue).field("green", &self.green).field("red", &self.red).field("alpha", &self.alpha).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDBLTBATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDBLTBATCH {
    fn eq(&self, other: &Self) -> bool {
        self.lprDest == other.lprDest && self.lpDDSSrc == other.lpDDSSrc && self.lprSrc == other.lprSrc && self.dwFlags == other.dwFlags && self.lpDDBltFx == other.lpDDBltFx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDBLTBATCH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDBLTBATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDBLTBATCH").field("lprDest", &self.lprDest).field("lpDDSSrc", &self.lpDDSSrc).field("lprSrc", &self.lprSrc).field("dwFlags", &self.dwFlags).field("lpDDBltFx", &self.lpDDBltFx).finish()
    }
}
impl ::core::default::Default for DDBLTFX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDBOBNEXTFIELDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDBOBNEXTFIELDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurface == other.lpSurface
    }
}
impl ::core::cmp::Eq for DDBOBNEXTFIELDINFO {}
impl ::core::fmt::Debug for DDBOBNEXTFIELDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDBOBNEXTFIELDINFO").field("lpSurface", &self.lpSurface).finish()
    }
}
impl ::core::default::Default for DDCAPS_DX1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDCAPS_DX1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
    }
}
impl ::core::cmp::Eq for DDCAPS_DX1 {}
impl ::core::fmt::Debug for DDCAPS_DX1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCAPS_DX1")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .finish()
    }
}
impl ::core::default::Default for DDCAPS_DX3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDCAPS_DX3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwReserved4 == other.dwReserved4
            && self.dwReserved5 == other.dwReserved5
            && self.dwReserved6 == other.dwReserved6
    }
}
impl ::core::cmp::Eq for DDCAPS_DX3 {}
impl ::core::fmt::Debug for DDCAPS_DX3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCAPS_DX3")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwReserved4", &self.dwReserved4)
            .field("dwReserved5", &self.dwReserved5)
            .field("dwReserved6", &self.dwReserved6)
            .finish()
    }
}
impl ::core::default::Default for DDCAPS_DX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDCAPS_DX5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwMaxVideoPorts == other.dwMaxVideoPorts
            && self.dwCurrVideoPorts == other.dwCurrVideoPorts
            && self.dwSVBCaps2 == other.dwSVBCaps2
            && self.dwNLVBCaps == other.dwNLVBCaps
            && self.dwNLVBCaps2 == other.dwNLVBCaps2
            && self.dwNLVBCKeyCaps == other.dwNLVBCKeyCaps
            && self.dwNLVBFXCaps == other.dwNLVBFXCaps
            && self.dwNLVBRops == other.dwNLVBRops
    }
}
impl ::core::cmp::Eq for DDCAPS_DX5 {}
impl ::core::fmt::Debug for DDCAPS_DX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCAPS_DX5")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwMaxVideoPorts", &self.dwMaxVideoPorts)
            .field("dwCurrVideoPorts", &self.dwCurrVideoPorts)
            .field("dwSVBCaps2", &self.dwSVBCaps2)
            .field("dwNLVBCaps", &self.dwNLVBCaps)
            .field("dwNLVBCaps2", &self.dwNLVBCaps2)
            .field("dwNLVBCKeyCaps", &self.dwNLVBCKeyCaps)
            .field("dwNLVBFXCaps", &self.dwNLVBFXCaps)
            .field("dwNLVBRops", &self.dwNLVBRops)
            .finish()
    }
}
impl ::core::default::Default for DDCAPS_DX6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDCAPS_DX7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDCOLORCONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDCOLORCONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.lBrightness == other.lBrightness && self.lContrast == other.lContrast && self.lHue == other.lHue && self.lSaturation == other.lSaturation && self.lSharpness == other.lSharpness && self.lGamma == other.lGamma && self.lColorEnable == other.lColorEnable && self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DDCOLORCONTROL {}
impl ::core::fmt::Debug for DDCOLORCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCOLORCONTROL").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("lBrightness", &self.lBrightness).field("lContrast", &self.lContrast).field("lHue", &self.lHue).field("lSaturation", &self.lSaturation).field("lSharpness", &self.lSharpness).field("lGamma", &self.lGamma).field("lColorEnable", &self.lColorEnable).field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::core::default::Default for DDCOLORKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDCOLORKEY {
    fn eq(&self, other: &Self) -> bool {
        self.dwColorSpaceLowValue == other.dwColorSpaceLowValue && self.dwColorSpaceHighValue == other.dwColorSpaceHighValue
    }
}
impl ::core::cmp::Eq for DDCOLORKEY {}
impl ::core::fmt::Debug for DDCOLORKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCOLORKEY").field("dwColorSpaceLowValue", &self.dwColorSpaceLowValue).field("dwColorSpaceHighValue", &self.dwColorSpaceHighValue).finish()
    }
}
impl ::core::default::Default for DDCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDCORECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDCORECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwMaxVideoPorts == other.dwMaxVideoPorts
            && self.dwCurrVideoPorts == other.dwCurrVideoPorts
            && self.dwSVBCaps2 == other.dwSVBCaps2
    }
}
impl ::core::cmp::Eq for DDCORECAPS {}
impl ::core::fmt::Debug for DDCORECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCORECAPS")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwMaxVideoPorts", &self.dwMaxVideoPorts)
            .field("dwCurrVideoPorts", &self.dwCurrVideoPorts)
            .field("dwSVBCaps2", &self.dwSVBCaps2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDDEVICEIDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDDEVICEIDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.szDriver == other.szDriver && self.szDescription == other.szDescription && self.liDriverVersion == other.liDriverVersion && self.dwVendorId == other.dwVendorId && self.dwDeviceId == other.dwDeviceId && self.dwSubSysId == other.dwSubSysId && self.dwRevision == other.dwRevision && self.guidDeviceIdentifier == other.guidDeviceIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDDEVICEIDENTIFIER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDDEVICEIDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDDEVICEIDENTIFIER").field("szDriver", &self.szDriver).field("szDescription", &self.szDescription).field("liDriverVersion", &self.liDriverVersion).field("dwVendorId", &self.dwVendorId).field("dwDeviceId", &self.dwDeviceId).field("dwSubSysId", &self.dwSubSysId).field("dwRevision", &self.dwRevision).field("guidDeviceIdentifier", &self.guidDeviceIdentifier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDDEVICEIDENTIFIER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDDEVICEIDENTIFIER2 {
    fn eq(&self, other: &Self) -> bool {
        self.szDriver == other.szDriver && self.szDescription == other.szDescription && self.liDriverVersion == other.liDriverVersion && self.dwVendorId == other.dwVendorId && self.dwDeviceId == other.dwDeviceId && self.dwSubSysId == other.dwSubSysId && self.dwRevision == other.dwRevision && self.guidDeviceIdentifier == other.guidDeviceIdentifier && self.dwWHQLLevel == other.dwWHQLLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDDEVICEIDENTIFIER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDDEVICEIDENTIFIER2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDDEVICEIDENTIFIER2").field("szDriver", &self.szDriver).field("szDescription", &self.szDescription).field("liDriverVersion", &self.liDriverVersion).field("dwVendorId", &self.dwVendorId).field("dwDeviceId", &self.dwDeviceId).field("dwSubSysId", &self.dwSubSysId).field("dwRevision", &self.dwRevision).field("guidDeviceIdentifier", &self.guidDeviceIdentifier).field("dwWHQLLevel", &self.dwWHQLLevel).finish()
    }
}
impl ::core::default::Default for DDENABLEIRQINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDFLIPOVERLAYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDFLIPOVERLAYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpCurrentSurface == other.lpCurrentSurface && self.lpTargetSurface == other.lpTargetSurface && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DDFLIPOVERLAYINFO {}
impl ::core::fmt::Debug for DDFLIPOVERLAYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDFLIPOVERLAYINFO").field("lpCurrentSurface", &self.lpCurrentSurface).field("lpTargetSurface", &self.lpTargetSurface).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DDFLIPVIDEOPORTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDFLIPVIDEOPORTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData && self.lpCurrentSurface == other.lpCurrentSurface && self.lpTargetSurface == other.lpTargetSurface && self.dwFlipVPFlags == other.dwFlipVPFlags
    }
}
impl ::core::cmp::Eq for DDFLIPVIDEOPORTINFO {}
impl ::core::fmt::Debug for DDFLIPVIDEOPORTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDFLIPVIDEOPORTINFO").field("lpVideoPortData", &self.lpVideoPortData).field("lpCurrentSurface", &self.lpCurrentSurface).field("lpTargetSurface", &self.lpTargetSurface).field("dwFlipVPFlags", &self.dwFlipVPFlags).finish()
    }
}
impl ::core::default::Default for DDGAMMARAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGAMMARAMP {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for DDGAMMARAMP {}
impl ::core::fmt::Debug for DDGAMMARAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGAMMARAMP").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::core::default::Default for DDGETCURRENTAUTOFLIPININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGETCURRENTAUTOFLIPININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::core::cmp::Eq for DDGETCURRENTAUTOFLIPININFO {}
impl ::core::fmt::Debug for DDGETCURRENTAUTOFLIPININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETCURRENTAUTOFLIPININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::core::default::Default for DDGETCURRENTAUTOFLIPOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGETCURRENTAUTOFLIPOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
impl ::core::cmp::Eq for DDGETCURRENTAUTOFLIPOUTINFO {}
impl ::core::fmt::Debug for DDGETCURRENTAUTOFLIPOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETCURRENTAUTOFLIPOUTINFO").field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
impl ::core::default::Default for DDGETIRQINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGETIRQINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DDGETIRQINFO {}
impl ::core::fmt::Debug for DDGETIRQINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETIRQINFO").field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DDGETPOLARITYININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGETPOLARITYININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::core::cmp::Eq for DDGETPOLARITYININFO {}
impl ::core::fmt::Debug for DDGETPOLARITYININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETPOLARITYININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::core::default::Default for DDGETPOLARITYOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGETPOLARITYOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bPolarity == other.bPolarity
    }
}
impl ::core::cmp::Eq for DDGETPOLARITYOUTINFO {}
impl ::core::fmt::Debug for DDGETPOLARITYOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETPOLARITYOUTINFO").field("bPolarity", &self.bPolarity).finish()
    }
}
impl ::core::default::Default for DDGETPREVIOUSAUTOFLIPININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGETPREVIOUSAUTOFLIPININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::core::cmp::Eq for DDGETPREVIOUSAUTOFLIPININFO {}
impl ::core::fmt::Debug for DDGETPREVIOUSAUTOFLIPININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETPREVIOUSAUTOFLIPININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::core::default::Default for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
impl ::core::cmp::Eq for DDGETPREVIOUSAUTOFLIPOUTINFO {}
impl ::core::fmt::Debug for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETPREVIOUSAUTOFLIPOUTINFO").field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
impl ::core::default::Default for DDGETTRANSFERSTATUSOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDGETTRANSFERSTATUSOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwTransferID == other.dwTransferID
    }
}
impl ::core::cmp::Eq for DDGETTRANSFERSTATUSOUTINFO {}
impl ::core::fmt::Debug for DDGETTRANSFERSTATUSOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETTRANSFERSTATUSOUTINFO").field("dwTransferID", &self.dwTransferID).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHALDDRAWFNS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHALINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDHALMODEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDHALMODEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.lPitch == other.lPitch && self.dwBPP == other.dwBPP && self.wFlags == other.wFlags && self.wRefreshRate == other.wRefreshRate && self.dwRBitMask == other.dwRBitMask && self.dwGBitMask == other.dwGBitMask && self.dwBBitMask == other.dwBBitMask && self.dwAlphaBitMask == other.dwAlphaBitMask
    }
}
impl ::core::cmp::Eq for DDHALMODEINFO {}
impl ::core::fmt::Debug for DDHALMODEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHALMODEINFO").field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("lPitch", &self.lPitch).field("dwBPP", &self.dwBPP).field("wFlags", &self.wFlags).field("wRefreshRate", &self.wRefreshRate).field("dwRBitMask", &self.dwRBitMask).field("dwGBitMask", &self.dwGBitMask).field("dwBBitMask", &self.dwBBitMask).field("dwAlphaBitMask", &self.dwAlphaBitMask).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_ADDATTACHEDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_BEGINMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_BLTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CANCREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CANCREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_COLORCONTROLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATEMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATEPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATESURFACEEXDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDHAL_CREATESURFACEEXDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpDDLcl == other.lpDDLcl && self.lpDDSLcl == other.lpDDSLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDHAL_CREATESURFACEEXDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_CREATESURFACEEXDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_CREATESURFACEEXDATA").field("dwFlags", &self.dwFlags).field("lpDDLcl", &self.lpDDLcl).field("lpDDSLcl", &self.lpDDSLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDCOLORCONTROLCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDEXEBUFCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDKERNELCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDMISCELLANEOUSCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDMOTIONCOMPCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDPALETTECALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDSURFACECALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDVIDEOPORTCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYDDLOCALDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDHAL_DESTROYDDLOCALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pDDLcl == other.pDDLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDHAL_DESTROYDDLOCALDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DESTROYDDLOCALDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DESTROYDDLOCALDATA").field("dwFlags", &self.dwFlags).field("pDDLcl", &self.pDDLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYDRIVERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DRVSETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_ENDMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_FLIPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_FLIPTOGDISURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_FLIPVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETAVAILDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETBLTSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDHAL_GETDRIVERINFODATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDHAL_GETDRIVERINFODATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidInfo == other.guidInfo && self.dwExpectedSize == other.dwExpectedSize && self.lpvData == other.lpvData && self.dwActualSize == other.dwActualSize && self.ddRVal == other.ddRVal && self.dwContext == other.dwContext
    }
}
impl ::core::cmp::Eq for DDHAL_GETDRIVERINFODATA {}
impl ::core::fmt::Debug for DDHAL_GETDRIVERINFODATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETDRIVERINFODATA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidInfo", &self.guidInfo).field("dwExpectedSize", &self.dwExpectedSize).field("lpvData", &self.lpvData).field("dwActualSize", &self.dwActualSize).field("ddRVal", &self.ddRVal).field("dwContext", &self.dwContext).finish()
    }
}
impl ::core::default::Default for DDHAL_GETDRIVERSTATEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDHAL_GETHEAPALIGNMENTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETINTERNALMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETMOCOMPCOMPBUFFDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETMOCOMPFORMATSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETMOCOMPGUIDSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETSCANLINEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTBANDWIDTHDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTCONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTFIELDDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTINPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTLINEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTOUTPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTSIGNALDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_LOCKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_QUERYMOCOMPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_RENDERMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETCLIPLISTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETENTRIESDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETEXCLUSIVEMODEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETMODEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETOVERLAYPOSITIONDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SYNCSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDHAL_SYNCSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwSurfaceOffset == other.dwSurfaceOffset && self.fpLockPtr == other.fpLockPtr && self.lPitch == other.lPitch && self.dwOverlayOffset == other.dwOverlayOffset && self.dwOverlaySrcWidth == other.dwOverlaySrcWidth && self.dwOverlaySrcHeight == other.dwOverlaySrcHeight && self.dwOverlayDestWidth == other.dwOverlayDestWidth && self.dwOverlayDestHeight == other.dwOverlayDestHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDHAL_SYNCSURFACEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SYNCSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SYNCSURFACEDATA")
            .field("dwSize", &self.dwSize)
            .field("lpDD", &self.lpDD)
            .field("lpDDSurface", &self.lpDDSurface)
            .field("dwSurfaceOffset", &self.dwSurfaceOffset)
            .field("fpLockPtr", &self.fpLockPtr)
            .field("lPitch", &self.lPitch)
            .field("dwOverlayOffset", &self.dwOverlayOffset)
            .field("dwOverlaySrcWidth", &self.dwOverlaySrcWidth)
            .field("dwOverlaySrcHeight", &self.dwOverlaySrcHeight)
            .field("dwOverlayDestWidth", &self.dwOverlayDestWidth)
            .field("dwOverlayDestHeight", &self.dwOverlayDestHeight)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SYNCVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDHAL_SYNCVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDHAL_SYNCVIDEOPORTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SYNCVIDEOPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SYNCVIDEOPORTDATA").field("dwSize", &self.dwSize).field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwOriginOffset", &self.dwOriginOffset).field("dwHeight", &self.dwHeight).field("dwVBIHeight", &self.dwVBIHeight).field("dwDriverReserved1", &self.dwDriverReserved1).field("dwDriverReserved2", &self.dwDriverReserved2).field("dwDriverReserved3", &self.dwDriverReserved3).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_UNLOCKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_UPDATENONLOCALHEAPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_UPDATEOVERLAYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_UPDATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_VPORTCOLORDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_WAITFORVERTICALBLANKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_WAITFORVPORTSYNCDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDKERNELCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDKERNELCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCaps == other.dwCaps && self.dwIRQCaps == other.dwIRQCaps
    }
}
impl ::core::cmp::Eq for DDKERNELCAPS {}
impl ::core::fmt::Debug for DDKERNELCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDKERNELCAPS").field("dwSize", &self.dwSize).field("dwCaps", &self.dwCaps).field("dwIRQCaps", &self.dwIRQCaps).finish()
    }
}
impl ::core::default::Default for DDLOCKININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDLOCKININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData
    }
}
impl ::core::cmp::Eq for DDLOCKININFO {}
impl ::core::fmt::Debug for DDLOCKININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDLOCKININFO").field("lpSurfaceData", &self.lpSurfaceData).finish()
    }
}
impl ::core::default::Default for DDLOCKOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDLOCKOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfacePtr == other.dwSurfacePtr
    }
}
impl ::core::cmp::Eq for DDLOCKOUTINFO {}
impl ::core::fmt::Debug for DDLOCKOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDLOCKOUTINFO").field("dwSurfacePtr", &self.dwSurfacePtr).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDMCBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDMCBUFFERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpCompSurface == other.lpCompSurface && self.dwDataOffset == other.dwDataOffset && self.dwDataSize == other.dwDataSize && self.lpPrivate == other.lpPrivate
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDMCBUFFERINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDMCBUFFERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDMCBUFFERINFO").field("dwSize", &self.dwSize).field("lpCompSurface", &self.lpCompSurface).field("dwDataOffset", &self.dwDataOffset).field("dwDataSize", &self.dwDataSize).field("lpPrivate", &self.lpPrivate).finish()
    }
}
impl ::core::default::Default for DDMCCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDMOCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDMOCOMPBUFFERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpCompSurface == other.lpCompSurface && self.dwDataOffset == other.dwDataOffset && self.dwDataSize == other.dwDataSize && self.lpPrivate == other.lpPrivate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDMOCOMPBUFFERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDMOCOMPBUFFERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDMOCOMPBUFFERINFO").field("dwSize", &self.dwSize).field("lpCompSurface", &self.lpCompSurface).field("dwDataOffset", &self.dwDataOffset).field("dwDataSize", &self.dwDataSize).field("lpPrivate", &self.lpPrivate).finish()
    }
}
impl ::core::default::Default for DDMONITORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDMONITORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer && self.Product == other.Product && self.SerialNumber == other.SerialNumber && self.DeviceIdentifier == other.DeviceIdentifier && self.Mode640x480 == other.Mode640x480 && self.Mode800x600 == other.Mode800x600 && self.Mode1024x768 == other.Mode1024x768 && self.Mode1280x1024 == other.Mode1280x1024 && self.Mode1600x1200 == other.Mode1600x1200 && self.ModeReserved1 == other.ModeReserved1 && self.ModeReserved2 == other.ModeReserved2 && self.ModeReserved3 == other.ModeReserved3
    }
}
impl ::core::cmp::Eq for DDMONITORINFO {}
impl ::core::fmt::Debug for DDMONITORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDMONITORINFO")
            .field("Manufacturer", &self.Manufacturer)
            .field("Product", &self.Product)
            .field("SerialNumber", &self.SerialNumber)
            .field("DeviceIdentifier", &self.DeviceIdentifier)
            .field("Mode640x480", &self.Mode640x480)
            .field("Mode800x600", &self.Mode800x600)
            .field("Mode1024x768", &self.Mode1024x768)
            .field("Mode1280x1024", &self.Mode1280x1024)
            .field("Mode1600x1200", &self.Mode1600x1200)
            .field("ModeReserved1", &self.ModeReserved1)
            .field("ModeReserved2", &self.ModeReserved2)
            .field("ModeReserved3", &self.ModeReserved3)
            .finish()
    }
}
impl ::core::default::Default for DDMORESURFACECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDNONLOCALVIDMEMCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDNONLOCALVIDMEMCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwNLVBCaps == other.dwNLVBCaps && self.dwNLVBCaps2 == other.dwNLVBCaps2 && self.dwNLVBCKeyCaps == other.dwNLVBCKeyCaps && self.dwNLVBFXCaps == other.dwNLVBFXCaps && self.dwNLVBRops == other.dwNLVBRops
    }
}
impl ::core::cmp::Eq for DDNONLOCALVIDMEMCAPS {}
impl ::core::fmt::Debug for DDNONLOCALVIDMEMCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDNONLOCALVIDMEMCAPS").field("dwSize", &self.dwSize).field("dwNLVBCaps", &self.dwNLVBCaps).field("dwNLVBCaps2", &self.dwNLVBCaps2).field("dwNLVBCKeyCaps", &self.dwNLVBCKeyCaps).field("dwNLVBFXCaps", &self.dwNLVBFXCaps).field("dwNLVBRops", &self.dwNLVBRops).finish()
    }
}
impl ::core::default::Default for DDNTCORECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDNTCORECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwMaxVideoPorts == other.dwMaxVideoPorts
            && self.dwCurrVideoPorts == other.dwCurrVideoPorts
            && self.dwSVBCaps2 == other.dwSVBCaps2
    }
}
impl ::core::cmp::Eq for DDNTCORECAPS {}
impl ::core::fmt::Debug for DDNTCORECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDNTCORECAPS")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwMaxVideoPorts", &self.dwMaxVideoPorts)
            .field("dwCurrVideoPorts", &self.dwCurrVideoPorts)
            .field("dwSVBCaps2", &self.dwSVBCaps2)
            .finish()
    }
}
impl ::core::default::Default for DDOPTSURFACEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDOSCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDOSCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCaps == other.dwCaps
    }
}
impl ::core::cmp::Eq for DDOSCAPS {}
impl ::core::fmt::Debug for DDOSCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDOSCAPS").field("dwCaps", &self.dwCaps).finish()
    }
}
impl ::core::default::Default for DDOVERLAYFX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDPIXELFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDMOTIONCOMP_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDMOTIONCOMP_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDMOTIONCOMP_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDMOTIONCOMP_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDMOTIONCOMP_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDMOTIONCOMP_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWCLIPPER_GBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_GBL {
    fn eq(&self, other: &Self) -> bool {
        self.dwRefCnt == other.dwRefCnt && self.dwFlags == other.dwFlags && self.lpDD == other.lpDD && self.dwProcessId == other.dwProcessId && self.dwReserved1 == other.dwReserved1 && self.hWnd == other.hWnd && self.lpStaticClipList == other.lpStaticClipList
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWCLIPPER_GBL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWCLIPPER_GBL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWCLIPPER_GBL").field("dwRefCnt", &self.dwRefCnt).field("dwFlags", &self.dwFlags).field("lpDD", &self.lpDD).field("dwProcessId", &self.dwProcessId).field("dwReserved1", &self.dwReserved1).field("hWnd", &self.hWnd).field("lpStaticClipList", &self.lpStaticClipList).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWCLIPPER_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWCLIPPER_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWCLIPPER_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWCLIPPER_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWCLIPPER_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpClipMore == other.lpClipMore && self.lpGbl == other.lpGbl && self.lpDD_lcl == other.lpDD_lcl && self.dwLocalRefCnt == other.dwLocalRefCnt && self.pUnkOuter == other.pUnkOuter && self.lpDD_int == other.lpDD_int && self.dwReserved1 == other.dwReserved1 && self.pAddrefedThisOwner == other.pAddrefedThisOwner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWCLIPPER_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWCLIPPER_LCL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWCLIPPER_LCL").field("lpClipMore", &self.lpClipMore).field("lpGbl", &self.lpGbl).field("lpDD_lcl", &self.lpDD_lcl).field("dwLocalRefCnt", &self.dwLocalRefCnt).field("pUnkOuter", &self.pUnkOuter).field("lpDD_int", &self.lpDD_int).field("dwReserved1", &self.dwReserved1).field("pAddrefedThisOwner", &self.pAddrefedThisOwner).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWPALETTE_GBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWPALETTE_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWPALETTE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWPALETTE_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWPALETTE_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWPALETTE_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWPALETTE_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWPALETTE_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpPalMore == other.lpPalMore && self.lpGbl == other.lpGbl && self.dwUnused0 == other.dwUnused0 && self.dwLocalRefCnt == other.dwLocalRefCnt && self.pUnkOuter == other.pUnkOuter && self.lpDD_lcl == other.lpDD_lcl && self.dwReserved1 == other.dwReserved1 && self.dwDDRAWReserved1 == other.dwDDRAWReserved1 && self.dwDDRAWReserved2 == other.dwDDRAWReserved2 && self.dwDDRAWReserved3 == other.dwDDRAWReserved3
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWPALETTE_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWPALETTE_LCL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWPALETTE_LCL").field("lpPalMore", &self.lpPalMore).field("lpGbl", &self.lpGbl).field("dwUnused0", &self.dwUnused0).field("dwLocalRefCnt", &self.dwLocalRefCnt).field("pUnkOuter", &self.pUnkOuter).field("lpDD_lcl", &self.lpDD_lcl).field("dwReserved1", &self.dwReserved1).field("dwDDRAWReserved1", &self.dwDDRAWReserved1).field("dwDDRAWReserved2", &self.dwDDRAWReserved2).field("dwDDRAWReserved3", &self.dwDDRAWReserved3).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL_MORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWSURFACE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWSURFACE_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWSURFACE_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWSURFACE_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_MORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDVIDEOPORT_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDVIDEOPORT_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt && self.dwFlags == other.dwFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDVIDEOPORT_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDVIDEOPORT_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDVIDEOPORT_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDVIDEOPORT_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDVIDEOPORT_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD
            && self.ddvpDesc == other.ddvpDesc
            && self.ddvpInfo == other.ddvpInfo
            && self.lpSurface == other.lpSurface
            && self.lpVBISurface == other.lpVBISurface
            && self.lpFlipInts == other.lpFlipInts
            && self.dwNumAutoflip == other.dwNumAutoflip
            && self.dwProcessID == other.dwProcessID
            && self.dwStateFlags == other.dwStateFlags
            && self.dwFlags == other.dwFlags
            && self.dwRefCnt == other.dwRefCnt
            && self.fpLastFlip == other.fpLastFlip
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.hDDVideoPort == other.hDDVideoPort
            && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip
            && self.lpVBIDesc == other.lpVBIDesc
            && self.lpVideoDesc == other.lpVideoDesc
            && self.lpVBIInfo == other.lpVBIInfo
            && self.lpVideoInfo == other.lpVideoInfo
            && self.dwVBIProcessID == other.dwVBIProcessID
            && self.lpVPNotify == other.lpVPNotify
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDVIDEOPORT_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDVIDEOPORT_LCL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDVIDEOPORT_LCL")
            .field("lpDD", &self.lpDD)
            .field("ddvpDesc", &self.ddvpDesc)
            .field("ddvpInfo", &self.ddvpInfo)
            .field("lpSurface", &self.lpSurface)
            .field("lpVBISurface", &self.lpVBISurface)
            .field("lpFlipInts", &self.lpFlipInts)
            .field("dwNumAutoflip", &self.dwNumAutoflip)
            .field("dwProcessID", &self.dwProcessID)
            .field("dwStateFlags", &self.dwStateFlags)
            .field("dwFlags", &self.dwFlags)
            .field("dwRefCnt", &self.dwRefCnt)
            .field("fpLastFlip", &self.fpLastFlip)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("hDDVideoPort", &self.hDDVideoPort)
            .field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip)
            .field("lpVBIDesc", &self.lpVBIDesc)
            .field("lpVideoDesc", &self.lpVideoDesc)
            .field("lpVBIInfo", &self.lpVBIInfo)
            .field("lpVideoInfo", &self.lpVideoInfo)
            .field("dwVBIProcessID", &self.dwVBIProcessID)
            .field("lpVPNotify", &self.lpVPNotify)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DIRECTDRAW_GBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DIRECTDRAW_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DIRECTDRAW_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DIRECTDRAW_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DIRECTDRAW_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DIRECTDRAW_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DIRECTDRAW_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDRGBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDRGBA {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue && self.alpha == other.alpha
    }
}
impl ::core::cmp::Eq for DDRGBA {}
impl ::core::fmt::Debug for DDRGBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRGBA").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).field("alpha", &self.alpha).finish()
    }
}
impl ::core::default::Default for DDSCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDSCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCaps == other.dwCaps
    }
}
impl ::core::cmp::Eq for DDSCAPS {}
impl ::core::fmt::Debug for DDSCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSCAPS").field("dwCaps", &self.dwCaps).finish()
    }
}
impl ::core::default::Default for DDSCAPS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDSCAPSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDSETSTATEININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDSETSTATEININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData && self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::core::cmp::Eq for DDSETSTATEININFO {}
impl ::core::fmt::Debug for DDSETSTATEININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSETSTATEININFO").field("lpSurfaceData", &self.lpSurfaceData).field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDSETSTATEOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDSETSTATEOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bSoftwareAutoflip == other.bSoftwareAutoflip && self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDSETSTATEOUTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDSETSTATEOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSETSTATEOUTINFO").field("bSoftwareAutoflip", &self.bSoftwareAutoflip).field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
impl ::core::default::Default for DDSKIPNEXTFIELDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDSKIPNEXTFIELDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData && self.dwSkipFlags == other.dwSkipFlags
    }
}
impl ::core::cmp::Eq for DDSKIPNEXTFIELDINFO {}
impl ::core::fmt::Debug for DDSKIPNEXTFIELDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSKIPNEXTFIELDINFO").field("lpVideoPortData", &self.lpVideoPortData).field("dwSkipFlags", &self.dwSkipFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDSTEREOMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDSTEREOMODE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwHeight == other.dwHeight && self.dwWidth == other.dwWidth && self.dwBpp == other.dwBpp && self.dwRefreshRate == other.dwRefreshRate && self.bSupported == other.bSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDSTEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDSTEREOMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSTEREOMODE").field("dwSize", &self.dwSize).field("dwHeight", &self.dwHeight).field("dwWidth", &self.dwWidth).field("dwBpp", &self.dwBpp).field("dwRefreshRate", &self.dwRefreshRate).field("bSupported", &self.bSupported).finish()
    }
}
impl ::core::default::Default for DDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.ddsCaps == other.ddsCaps
            && self.dwSurfaceOffset == other.dwSurfaceOffset
            && self.fpLockPtr == other.fpLockPtr
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.lPitch == other.lPitch
            && self.dwOverlayFlags == other.dwOverlayFlags
            && self.dwOverlayOffset == other.dwOverlayOffset
            && self.dwOverlaySrcWidth == other.dwOverlaySrcWidth
            && self.dwOverlaySrcHeight == other.dwOverlaySrcHeight
            && self.dwOverlayDestWidth == other.dwOverlayDestWidth
            && self.dwOverlayDestHeight == other.dwOverlayDestHeight
            && self.dwVideoPortId == other.dwVideoPortId
            && self.dwFormatFlags == other.dwFormatFlags
            && self.dwFormatFourCC == other.dwFormatFourCC
            && self.dwFormatBitCount == other.dwFormatBitCount
            && self.dwRBitMask == other.dwRBitMask
            && self.dwGBitMask == other.dwGBitMask
            && self.dwBBitMask == other.dwBBitMask
            && self.dwDriverReserved1 == other.dwDriverReserved1
            && self.dwDriverReserved2 == other.dwDriverReserved2
            && self.dwDriverReserved3 == other.dwDriverReserved3
            && self.dwDriverReserved4 == other.dwDriverReserved4
    }
}
impl ::core::cmp::Eq for DDSURFACEDATA {}
impl ::core::fmt::Debug for DDSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSURFACEDATA")
            .field("ddsCaps", &self.ddsCaps)
            .field("dwSurfaceOffset", &self.dwSurfaceOffset)
            .field("fpLockPtr", &self.fpLockPtr)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("lPitch", &self.lPitch)
            .field("dwOverlayFlags", &self.dwOverlayFlags)
            .field("dwOverlayOffset", &self.dwOverlayOffset)
            .field("dwOverlaySrcWidth", &self.dwOverlaySrcWidth)
            .field("dwOverlaySrcHeight", &self.dwOverlaySrcHeight)
            .field("dwOverlayDestWidth", &self.dwOverlayDestWidth)
            .field("dwOverlayDestHeight", &self.dwOverlayDestHeight)
            .field("dwVideoPortId", &self.dwVideoPortId)
            .field("dwFormatFlags", &self.dwFormatFlags)
            .field("dwFormatFourCC", &self.dwFormatFourCC)
            .field("dwFormatBitCount", &self.dwFormatBitCount)
            .field("dwRBitMask", &self.dwRBitMask)
            .field("dwGBitMask", &self.dwGBitMask)
            .field("dwBBitMask", &self.dwBBitMask)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("dwDriverReserved4", &self.dwDriverReserved4)
            .finish()
    }
}
impl ::core::default::Default for DDSURFACEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDSURFACEDESC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DDTRANSFERININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDTRANSFERININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData && self.dwStartLine == other.dwStartLine && self.dwEndLine == other.dwEndLine && self.dwTransferID == other.dwTransferID && self.dwTransferFlags == other.dwTransferFlags && self.lpDestMDL == other.lpDestMDL
    }
}
impl ::core::cmp::Eq for DDTRANSFERININFO {}
impl ::core::fmt::Debug for DDTRANSFERININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDTRANSFERININFO").field("lpSurfaceData", &self.lpSurfaceData).field("dwStartLine", &self.dwStartLine).field("dwEndLine", &self.dwEndLine).field("dwTransferID", &self.dwTransferID).field("dwTransferFlags", &self.dwTransferFlags).field("lpDestMDL", &self.lpDestMDL).finish()
    }
}
impl ::core::default::Default for DDTRANSFEROUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDTRANSFEROUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwBufferPolarity == other.dwBufferPolarity
    }
}
impl ::core::cmp::Eq for DDTRANSFEROUTINFO {}
impl ::core::fmt::Debug for DDTRANSFEROUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDTRANSFEROUTINFO").field("dwBufferPolarity", &self.dwBufferPolarity).finish()
    }
}
impl ::core::default::Default for DDVERSIONDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDVERSIONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwHALVersion == other.dwHALVersion && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for DDVERSIONDATA {}
impl ::core::fmt::Debug for DDVERSIONDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVERSIONDATA").field("dwHALVersion", &self.dwHALVersion).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::core::default::Default for DDVIDEOPORTBANDWIDTH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDVIDEOPORTBANDWIDTH {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCaps == other.dwCaps && self.dwOverlay == other.dwOverlay && self.dwColorkey == other.dwColorkey && self.dwYInterpolate == other.dwYInterpolate && self.dwYInterpAndColorkey == other.dwYInterpAndColorkey && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTBANDWIDTH {}
impl ::core::fmt::Debug for DDVIDEOPORTBANDWIDTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTBANDWIDTH").field("dwSize", &self.dwSize).field("dwCaps", &self.dwCaps).field("dwOverlay", &self.dwOverlay).field("dwColorkey", &self.dwColorkey).field("dwYInterpolate", &self.dwYInterpolate).field("dwYInterpAndColorkey", &self.dwYInterpAndColorkey).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::core::default::Default for DDVIDEOPORTCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDVIDEOPORTCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.dwMaxWidth == other.dwMaxWidth
            && self.dwMaxVBIWidth == other.dwMaxVBIWidth
            && self.dwMaxHeight == other.dwMaxHeight
            && self.dwVideoPortID == other.dwVideoPortID
            && self.dwCaps == other.dwCaps
            && self.dwFX == other.dwFX
            && self.dwNumAutoFlipSurfaces == other.dwNumAutoFlipSurfaces
            && self.dwAlignVideoPortBoundary == other.dwAlignVideoPortBoundary
            && self.dwAlignVideoPortPrescaleWidth == other.dwAlignVideoPortPrescaleWidth
            && self.dwAlignVideoPortCropBoundary == other.dwAlignVideoPortCropBoundary
            && self.dwAlignVideoPortCropWidth == other.dwAlignVideoPortCropWidth
            && self.dwPreshrinkXStep == other.dwPreshrinkXStep
            && self.dwPreshrinkYStep == other.dwPreshrinkYStep
            && self.dwNumVBIAutoFlipSurfaces == other.dwNumVBIAutoFlipSurfaces
            && self.dwNumPreferredAutoflip == other.dwNumPreferredAutoflip
            && self.wNumFilterTapsX == other.wNumFilterTapsX
            && self.wNumFilterTapsY == other.wNumFilterTapsY
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTCAPS {}
impl ::core::fmt::Debug for DDVIDEOPORTCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwMaxWidth", &self.dwMaxWidth)
            .field("dwMaxVBIWidth", &self.dwMaxVBIWidth)
            .field("dwMaxHeight", &self.dwMaxHeight)
            .field("dwVideoPortID", &self.dwVideoPortID)
            .field("dwCaps", &self.dwCaps)
            .field("dwFX", &self.dwFX)
            .field("dwNumAutoFlipSurfaces", &self.dwNumAutoFlipSurfaces)
            .field("dwAlignVideoPortBoundary", &self.dwAlignVideoPortBoundary)
            .field("dwAlignVideoPortPrescaleWidth", &self.dwAlignVideoPortPrescaleWidth)
            .field("dwAlignVideoPortCropBoundary", &self.dwAlignVideoPortCropBoundary)
            .field("dwAlignVideoPortCropWidth", &self.dwAlignVideoPortCropWidth)
            .field("dwPreshrinkXStep", &self.dwPreshrinkXStep)
            .field("dwPreshrinkYStep", &self.dwPreshrinkYStep)
            .field("dwNumVBIAutoFlipSurfaces", &self.dwNumVBIAutoFlipSurfaces)
            .field("dwNumPreferredAutoflip", &self.dwNumPreferredAutoflip)
            .field("wNumFilterTapsX", &self.wNumFilterTapsX)
            .field("wNumFilterTapsY", &self.wNumFilterTapsY)
            .finish()
    }
}
impl ::core::default::Default for DDVIDEOPORTCONNECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDVIDEOPORTCONNECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwPortWidth == other.dwPortWidth && self.guidTypeID == other.guidTypeID && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTCONNECT {}
impl ::core::fmt::Debug for DDVIDEOPORTCONNECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTCONNECT").field("dwSize", &self.dwSize).field("dwPortWidth", &self.dwPortWidth).field("guidTypeID", &self.guidTypeID).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::core::default::Default for DDVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwVideoPortId == other.dwVideoPortId && self.dwVPFlags == other.dwVPFlags && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTDATA {}
impl ::core::fmt::Debug for DDVIDEOPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTDATA").field("dwVideoPortId", &self.dwVideoPortId).field("dwVPFlags", &self.dwVPFlags).field("dwOriginOffset", &self.dwOriginOffset).field("dwHeight", &self.dwHeight).field("dwVBIHeight", &self.dwVBIHeight).field("dwDriverReserved1", &self.dwDriverReserved1).field("dwDriverReserved2", &self.dwDriverReserved2).field("dwDriverReserved3", &self.dwDriverReserved3).finish()
    }
}
impl ::core::default::Default for DDVIDEOPORTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDVIDEOPORTDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFieldWidth == other.dwFieldWidth && self.dwVBIWidth == other.dwVBIWidth && self.dwFieldHeight == other.dwFieldHeight && self.dwMicrosecondsPerField == other.dwMicrosecondsPerField && self.dwMaxPixelsPerSecond == other.dwMaxPixelsPerSecond && self.dwVideoPortID == other.dwVideoPortID && self.dwReserved1 == other.dwReserved1 && self.VideoPortType == other.VideoPortType && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTDESC {}
impl ::core::fmt::Debug for DDVIDEOPORTDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTDESC")
            .field("dwSize", &self.dwSize)
            .field("dwFieldWidth", &self.dwFieldWidth)
            .field("dwVBIWidth", &self.dwVBIWidth)
            .field("dwFieldHeight", &self.dwFieldHeight)
            .field("dwMicrosecondsPerField", &self.dwMicrosecondsPerField)
            .field("dwMaxPixelsPerSecond", &self.dwMaxPixelsPerSecond)
            .field("dwVideoPortID", &self.dwVideoPortID)
            .field("dwReserved1", &self.dwReserved1)
            .field("VideoPortType", &self.VideoPortType)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDVIDEOPORTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDVIDEOPORTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwOriginX == other.dwOriginX && self.dwOriginY == other.dwOriginY && self.dwVPFlags == other.dwVPFlags && self.rCrop == other.rCrop && self.dwPrescaleWidth == other.dwPrescaleWidth && self.dwPrescaleHeight == other.dwPrescaleHeight && self.lpddpfInputFormat == other.lpddpfInputFormat && self.lpddpfVBIInputFormat == other.lpddpfVBIInputFormat && self.lpddpfVBIOutputFormat == other.lpddpfVBIOutputFormat && self.dwVBIHeight == other.dwVBIHeight && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDVIDEOPORTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDVIDEOPORTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTINFO")
            .field("dwSize", &self.dwSize)
            .field("dwOriginX", &self.dwOriginX)
            .field("dwOriginY", &self.dwOriginY)
            .field("dwVPFlags", &self.dwVPFlags)
            .field("rCrop", &self.rCrop)
            .field("dwPrescaleWidth", &self.dwPrescaleWidth)
            .field("dwPrescaleHeight", &self.dwPrescaleHeight)
            .field("lpddpfInputFormat", &self.lpddpfInputFormat)
            .field("lpddpfVBIInputFormat", &self.lpddpfVBIInputFormat)
            .field("lpddpfVBIOutputFormat", &self.lpddpfVBIOutputFormat)
            .field("dwVBIHeight", &self.dwVBIHeight)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
impl ::core::default::Default for DDVIDEOPORTNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDVIDEOPORTNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.ApproximateTimeStamp == other.ApproximateTimeStamp && self.lField == other.lField && self.dwSurfaceIndex == other.dwSurfaceIndex && self.lDone == other.lDone
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTNOTIFY {}
impl ::core::fmt::Debug for DDVIDEOPORTNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTNOTIFY").field("ApproximateTimeStamp", &self.ApproximateTimeStamp).field("lField", &self.lField).field("dwSurfaceIndex", &self.dwSurfaceIndex).field("lDone", &self.lDone).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDVIDEOPORTSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDVIDEOPORTSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bInUse == other.bInUse && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.VideoPortType == other.VideoPortType && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDVIDEOPORTSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDVIDEOPORTSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTSTATUS").field("dwSize", &self.dwSize).field("bInUse", &self.bInUse).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("VideoPortType", &self.VideoPortType).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_ADDATTACHEDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_ADDATTACHEDSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpSurfAttached == other.lpSurfAttached && self.ddRVal == other.ddRVal && self.AddAttachedSurface == other.AddAttachedSurface
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_ADDATTACHEDSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_ADDATTACHEDSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_ADDATTACHEDSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpSurfAttached", &self.lpSurfAttached).field("ddRVal", &self.ddRVal).field("AddAttachedSurface", &self.AddAttachedSurface).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_ATTACHLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_ATTACHLIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.lpAttached == other.lpAttached
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_ATTACHLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_ATTACHLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_ATTACHLIST").field("lpLink", &self.lpLink).field("lpAttached", &self.lpAttached).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_BEGINMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_BEGINMOCOMPFRAMEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpDestSurface == other.lpDestSurface && self.dwInputDataSize == other.dwInputDataSize && self.lpInputData == other.lpInputData && self.dwOutputDataSize == other.dwOutputDataSize && self.lpOutputData == other.lpOutputData && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_BEGINMOCOMPFRAMEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_BEGINMOCOMPFRAMEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_BEGINMOCOMPFRAMEDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpDestSurface", &self.lpDestSurface).field("dwInputDataSize", &self.dwInputDataSize).field("lpInputData", &self.lpInputData).field("dwOutputDataSize", &self.dwOutputDataSize).field("lpOutputData", &self.lpOutputData).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_BLTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DD_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_CANCREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_CANCREATESURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurfaceDesc == other.lpDDSurfaceDesc && self.bIsDifferentPixelFormat == other.bIsDifferentPixelFormat && self.ddRVal == other.ddRVal && self.CanCreateSurface == other.CanCreateSurface
    }
}
impl ::core::cmp::Eq for DD_CANCREATESURFACEDATA {}
impl ::core::fmt::Debug for DD_CANCREATESURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CANCREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("bIsDifferentPixelFormat", &self.bIsDifferentPixelFormat).field("ddRVal", &self.ddRVal).field("CanCreateSurface", &self.CanCreateSurface).finish()
    }
}
impl ::core::default::Default for DD_CANCREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_CANCREATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDVideoPortDesc == other.lpDDVideoPortDesc && self.ddRVal == other.ddRVal && self.CanCreateVideoPort == other.CanCreateVideoPort
    }
}
impl ::core::cmp::Eq for DD_CANCREATEVPORTDATA {}
impl ::core::fmt::Debug for DD_CANCREATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CANCREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("ddRVal", &self.ddRVal).field("CanCreateVideoPort", &self.CanCreateVideoPort).finish()
    }
}
impl ::core::default::Default for DD_CLIPPER_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_CLIPPER_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DD_CLIPPER_GLOBAL {}
impl ::core::fmt::Debug for DD_CLIPPER_GLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CLIPPER_GLOBAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::core::default::Default for DD_CLIPPER_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_CLIPPER_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DD_CLIPPER_LOCAL {}
impl ::core::fmt::Debug for DD_CLIPPER_LOCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CLIPPER_LOCAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_COLORCONTROLCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_COLORCONTROLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_COLORCONTROLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpColorData == other.lpColorData && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.ColorControl == other.ColorControl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_COLORCONTROLDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_COLORCONTROLDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_COLORCONTROLDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpColorData", &self.lpColorData).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("ColorControl", &self.ColorControl).finish()
    }
}
impl ::core::default::Default for DD_CREATEMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DD_CREATEPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DD_CREATEPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.lpColorTable == other.lpColorTable && self.ddRVal == other.ddRVal && self.CreatePalette == other.CreatePalette && self.is_excl == other.is_excl
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DD_CREATEPALETTEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DD_CREATEPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CREATEPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("lpColorTable", &self.lpColorTable).field("ddRVal", &self.ddRVal).field("CreatePalette", &self.CreatePalette).field("is_excl", &self.is_excl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_CREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_CREATESURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurfaceDesc == other.lpDDSurfaceDesc && self.lplpSList == other.lplpSList && self.dwSCnt == other.dwSCnt && self.ddRVal == other.ddRVal && self.CreateSurface == other.CreateSurface
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_CREATESURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_CREATESURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("lplpSList", &self.lplpSList).field("dwSCnt", &self.dwSCnt).field("ddRVal", &self.ddRVal).field("CreateSurface", &self.CreateSurface).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_CREATESURFACEEXDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_CREATESURFACEEXDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpDDLcl == other.lpDDLcl && self.lpDDSLcl == other.lpDDSLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_CREATESURFACEEXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_CREATESURFACEEXDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CREATESURFACEEXDATA").field("dwFlags", &self.dwFlags).field("lpDDLcl", &self.lpDDLcl).field("lpDDSLcl", &self.lpDDSLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_CREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_CREATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDVideoPortDesc == other.lpDDVideoPortDesc && self.lpVideoPort == other.lpVideoPort && self.ddRVal == other.ddRVal && self.CreateVideoPort == other.CreateVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_CREATEVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_CREATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).field("CreateVideoPort", &self.CreateVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_D3DBUFCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_DESTROYDDLOCALDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_DESTROYDDLOCALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pDDLcl == other.pDDLcl && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_DESTROYDDLOCALDATA {}
impl ::core::fmt::Debug for DD_DESTROYDDLOCALDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYDDLOCALDATA").field("dwFlags", &self.dwFlags).field("pDDLcl", &self.pDDLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::core::default::Default for DD_DESTROYMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_DESTROYMOCOMPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_DESTROYMOCOMPDATA {}
impl ::core::fmt::Debug for DD_DESTROYMOCOMPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYMOCOMPDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::core::default::Default for DD_DESTROYPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_DESTROYPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.ddRVal == other.ddRVal && self.DestroyPalette == other.DestroyPalette
    }
}
impl ::core::cmp::Eq for DD_DESTROYPALETTEDATA {}
impl ::core::fmt::Debug for DD_DESTROYPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).field("DestroyPalette", &self.DestroyPalette).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_DESTROYSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_DESTROYSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.DestroySurface == other.DestroySurface
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_DESTROYSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_DESTROYSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("DestroySurface", &self.DestroySurface).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_DESTROYVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_DESTROYVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.ddRVal == other.ddRVal && self.DestroyVideoPort == other.DestroyVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_DESTROYVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_DESTROYVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).field("DestroyVideoPort", &self.DestroyVideoPort).finish()
    }
}
impl ::core::default::Default for DD_DIRECTDRAW_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_DIRECTDRAW_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dhpdev == other.dhpdev && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.lpDDVideoPortCaps == other.lpDDVideoPortCaps
    }
}
impl ::core::cmp::Eq for DD_DIRECTDRAW_GLOBAL {}
impl ::core::fmt::Debug for DD_DIRECTDRAW_GLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DIRECTDRAW_GLOBAL").field("dhpdev", &self.dhpdev).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("lpDDVideoPortCaps", &self.lpDDVideoPortCaps).finish()
    }
}
impl ::core::default::Default for DD_DIRECTDRAW_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_DIRECTDRAW_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.lpGbl == other.lpGbl
    }
}
impl ::core::cmp::Eq for DD_DIRECTDRAW_LOCAL {}
impl ::core::fmt::Debug for DD_DIRECTDRAW_LOCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DIRECTDRAW_LOCAL").field("lpGbl", &self.lpGbl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_DRVSETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_DRVSETCOLORKEYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ckNew == other.ckNew && self.ddRVal == other.ddRVal && self.SetColorKey == other.SetColorKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_DRVSETCOLORKEYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_DRVSETCOLORKEYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DRVSETCOLORKEYDATA").field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).field("SetColorKey", &self.SetColorKey).finish()
    }
}
impl ::core::default::Default for DD_ENDMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_ENDMOCOMPFRAMEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpInputData == other.lpInputData && self.dwInputDataSize == other.dwInputDataSize && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_ENDMOCOMPFRAMEDATA {}
impl ::core::fmt::Debug for DD_ENDMOCOMPFRAMEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_ENDMOCOMPFRAMEDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpInputData", &self.lpInputData).field("dwInputDataSize", &self.dwInputDataSize).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_FLIPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_FLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpSurfCurr == other.lpSurfCurr && self.lpSurfTarg == other.lpSurfTarg && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.Flip == other.Flip && self.lpSurfCurrLeft == other.lpSurfCurrLeft && self.lpSurfTargLeft == other.lpSurfTargLeft
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_FLIPDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_FLIPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_FLIPDATA").field("lpDD", &self.lpDD).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("Flip", &self.Flip).field("lpSurfCurrLeft", &self.lpSurfCurrLeft).field("lpSurfTargLeft", &self.lpSurfTargLeft).finish()
    }
}
impl ::core::default::Default for DD_FLIPTOGDISURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_FLIPTOGDISURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwToGDI == other.dwToGDI && self.dwReserved == other.dwReserved && self.ddRVal == other.ddRVal && self.FlipToGDISurface == other.FlipToGDISurface
    }
}
impl ::core::cmp::Eq for DD_FLIPTOGDISURFACEDATA {}
impl ::core::fmt::Debug for DD_FLIPTOGDISURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_FLIPTOGDISURFACEDATA").field("lpDD", &self.lpDD).field("dwToGDI", &self.dwToGDI).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).field("FlipToGDISurface", &self.FlipToGDISurface).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_FLIPVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_FLIPVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lpSurfCurr == other.lpSurfCurr && self.lpSurfTarg == other.lpSurfTarg && self.ddRVal == other.ddRVal && self.FlipVideoPort == other.FlipVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_FLIPVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_FLIPVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_FLIPVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("ddRVal", &self.ddRVal).field("FlipVideoPort", &self.FlipVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_FREEDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_FREEDRIVERMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.FreeDriverMemory == other.FreeDriverMemory
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_FREEDRIVERMEMORYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_FREEDRIVERMEMORYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_FREEDRIVERMEMORYDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("FreeDriverMemory", &self.FreeDriverMemory).finish()
    }
}
impl ::core::default::Default for DD_GETAVAILDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_GETAVAILDRIVERMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.DDSCaps == other.DDSCaps && self.dwTotal == other.dwTotal && self.dwFree == other.dwFree && self.ddRVal == other.ddRVal && self.GetAvailDriverMemory == other.GetAvailDriverMemory
    }
}
impl ::core::cmp::Eq for DD_GETAVAILDRIVERMEMORYDATA {}
impl ::core::fmt::Debug for DD_GETAVAILDRIVERMEMORYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETAVAILDRIVERMEMORYDATA").field("lpDD", &self.lpDD).field("DDSCaps", &self.DDSCaps).field("dwTotal", &self.dwTotal).field("dwFree", &self.dwFree).field("ddRVal", &self.ddRVal).field("GetAvailDriverMemory", &self.GetAvailDriverMemory).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETBLTSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETBLTSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.GetBltStatus == other.GetBltStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETBLTSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETBLTSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETBLTSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("GetBltStatus", &self.GetBltStatus).finish()
    }
}
impl ::core::default::Default for DD_GETDRIVERINFODATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_GETDRIVERINFODATA {
    fn eq(&self, other: &Self) -> bool {
        self.dhpdev == other.dhpdev && self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidInfo == other.guidInfo && self.dwExpectedSize == other.dwExpectedSize && self.lpvData == other.lpvData && self.dwActualSize == other.dwActualSize && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_GETDRIVERINFODATA {}
impl ::core::fmt::Debug for DD_GETDRIVERINFODATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETDRIVERINFODATA").field("dhpdev", &self.dhpdev).field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidInfo", &self.guidInfo).field("dwExpectedSize", &self.dwExpectedSize).field("lpvData", &self.lpvData).field("dwActualSize", &self.dwActualSize).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::core::default::Default for DD_GETDRIVERSTATEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETFLIPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.GetFlipStatus == other.GetFlipStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETFLIPSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETFLIPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("GetFlipStatus", &self.GetFlipStatus).finish()
    }
}
impl ::core::default::Default for DD_GETHEAPALIGNMENTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_GETINTERNALMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_GETMOCOMPCOMPBUFFDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_GETMOCOMPFORMATSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_GETMOCOMPFORMATSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpGuid == other.lpGuid && self.dwNumFormats == other.dwNumFormats && self.lpFormats == other.lpFormats && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_GETMOCOMPFORMATSDATA {}
impl ::core::fmt::Debug for DD_GETMOCOMPFORMATSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETMOCOMPFORMATSDATA").field("lpDD", &self.lpDD).field("lpGuid", &self.lpGuid).field("dwNumFormats", &self.dwNumFormats).field("lpFormats", &self.lpFormats).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::core::default::Default for DD_GETMOCOMPGUIDSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_GETMOCOMPGUIDSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwNumGuids == other.dwNumGuids && self.lpGuids == other.lpGuids && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_GETMOCOMPGUIDSDATA {}
impl ::core::fmt::Debug for DD_GETMOCOMPGUIDSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETMOCOMPGUIDSDATA").field("lpDD", &self.lpDD).field("dwNumGuids", &self.dwNumGuids).field("lpGuids", &self.lpGuids).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::core::default::Default for DD_GETSCANLINEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_GETSCANLINEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwScanLine == other.dwScanLine && self.ddRVal == other.ddRVal && self.GetScanLine == other.GetScanLine
    }
}
impl ::core::cmp::Eq for DD_GETSCANLINEDATA {}
impl ::core::fmt::Debug for DD_GETSCANLINEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETSCANLINEDATA").field("lpDD", &self.lpDD).field("dwScanLine", &self.dwScanLine).field("ddRVal", &self.ddRVal).field("GetScanLine", &self.GetScanLine).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTBANDWIDTHDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTBANDWIDTHDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lpddpfFormat == other.lpddpfFormat && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwFlags == other.dwFlags && self.lpBandwidth == other.lpBandwidth && self.ddRVal == other.ddRVal && self.GetVideoPortBandwidth == other.GetVideoPortBandwidth
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTBANDWIDTHDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTBANDWIDTHDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTBANDWIDTHDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lpddpfFormat", &self.lpddpfFormat).field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("dwFlags", &self.dwFlags).field("lpBandwidth", &self.lpBandwidth).field("ddRVal", &self.ddRVal).field("GetVideoPortBandwidth", &self.GetVideoPortBandwidth).finish()
    }
}
impl ::core::default::Default for DD_GETVPORTCONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_GETVPORTCONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwPortId == other.dwPortId && self.lpConnect == other.lpConnect && self.dwNumEntries == other.dwNumEntries && self.ddRVal == other.ddRVal && self.GetVideoPortConnectInfo == other.GetVideoPortConnectInfo
    }
}
impl ::core::cmp::Eq for DD_GETVPORTCONNECTDATA {}
impl ::core::fmt::Debug for DD_GETVPORTCONNECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTCONNECTDATA").field("lpDD", &self.lpDD).field("dwPortId", &self.dwPortId).field("lpConnect", &self.lpConnect).field("dwNumEntries", &self.dwNumEntries).field("ddRVal", &self.ddRVal).field("GetVideoPortConnectInfo", &self.GetVideoPortConnectInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTFIELDDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTFIELDDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.bField == other.bField && self.ddRVal == other.ddRVal && self.GetVideoPortField == other.GetVideoPortField
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTFIELDDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTFIELDDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTFIELDDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("bField", &self.bField).field("ddRVal", &self.ddRVal).field("GetVideoPortField", &self.GetVideoPortField).finish()
    }
}
impl ::core::default::Default for DD_GETVPORTFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_GETVPORTFLIPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.fpSurface == other.fpSurface && self.ddRVal == other.ddRVal && self.GetVideoPortFlipStatus == other.GetVideoPortFlipStatus
    }
}
impl ::core::cmp::Eq for DD_GETVPORTFLIPSTATUSDATA {}
impl ::core::fmt::Debug for DD_GETVPORTFLIPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("fpSurface", &self.fpSurface).field("ddRVal", &self.ddRVal).field("GetVideoPortFlipStatus", &self.GetVideoPortFlipStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTINPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTINPUTFORMATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpddpfFormat == other.lpddpfFormat && self.dwNumFormats == other.dwNumFormats && self.ddRVal == other.ddRVal && self.GetVideoPortInputFormats == other.GetVideoPortInputFormats
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTINPUTFORMATDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTINPUTFORMATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTINPUTFORMATDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpddpfFormat", &self.lpddpfFormat).field("dwNumFormats", &self.dwNumFormats).field("ddRVal", &self.ddRVal).field("GetVideoPortInputFormats", &self.GetVideoPortInputFormats).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTLINEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTLINEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwLine == other.dwLine && self.ddRVal == other.ddRVal && self.GetVideoPortLine == other.GetVideoPortLine
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTLINEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTLINEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTLINEDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwLine", &self.dwLine).field("ddRVal", &self.ddRVal).field("GetVideoPortLine", &self.GetVideoPortLine).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTOUTPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTOUTPUTFORMATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpddpfInputFormat == other.lpddpfInputFormat && self.lpddpfOutputFormats == other.lpddpfOutputFormats && self.dwNumFormats == other.dwNumFormats && self.ddRVal == other.ddRVal && self.GetVideoPortInputFormats == other.GetVideoPortInputFormats
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTOUTPUTFORMATDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTOUTPUTFORMATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTOUTPUTFORMATDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpddpfInputFormat", &self.lpddpfInputFormat).field("lpddpfOutputFormats", &self.lpddpfOutputFormats).field("dwNumFormats", &self.dwNumFormats).field("ddRVal", &self.ddRVal).field("GetVideoPortInputFormats", &self.GetVideoPortInputFormats).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTSIGNALDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTSIGNALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwStatus == other.dwStatus && self.ddRVal == other.ddRVal && self.GetVideoSignalStatus == other.GetVideoSignalStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTSIGNALDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTSIGNALDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTSIGNALDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwStatus", &self.dwStatus).field("ddRVal", &self.ddRVal).field("GetVideoSignalStatus", &self.GetVideoSignalStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_HALINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_HALINFO_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_KERNELCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_LOCKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_LOCKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.bHasRect == other.bHasRect && self.rArea == other.rArea && self.lpSurfData == other.lpSurfData && self.ddRVal == other.ddRVal && self.Lock == other.Lock && self.dwFlags == other.dwFlags && self.fpProcess == other.fpProcess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_LOCKDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_LOCKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_LOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("bHasRect", &self.bHasRect).field("rArea", &self.rArea).field("lpSurfData", &self.lpSurfData).field("ddRVal", &self.ddRVal).field("Lock", &self.Lock).field("dwFlags", &self.dwFlags).field("fpProcess", &self.fpProcess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_MAPMEMORYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_MAPMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.bMap == other.bMap && self.hProcess == other.hProcess && self.fpProcess == other.fpProcess && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_MAPMEMORYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_MAPMEMORYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_MAPMEMORYDATA").field("lpDD", &self.lpDD).field("bMap", &self.bMap).field("hProcess", &self.hProcess).field("fpProcess", &self.fpProcess).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_MISCELLANEOUS2CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_MISCELLANEOUSCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_MORECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_MORECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwAlphaCaps == other.dwAlphaCaps && self.dwSVBAlphaCaps == other.dwSVBAlphaCaps && self.dwVSBAlphaCaps == other.dwVSBAlphaCaps && self.dwSSBAlphaCaps == other.dwSSBAlphaCaps && self.dwFilterCaps == other.dwFilterCaps && self.dwSVBFilterCaps == other.dwSVBFilterCaps && self.dwVSBFilterCaps == other.dwVSBFilterCaps && self.dwSSBFilterCaps == other.dwSSBFilterCaps
    }
}
impl ::core::cmp::Eq for DD_MORECAPS {}
impl ::core::fmt::Debug for DD_MORECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_MORECAPS").field("dwSize", &self.dwSize).field("dwAlphaCaps", &self.dwAlphaCaps).field("dwSVBAlphaCaps", &self.dwSVBAlphaCaps).field("dwVSBAlphaCaps", &self.dwVSBAlphaCaps).field("dwSSBAlphaCaps", &self.dwSSBAlphaCaps).field("dwFilterCaps", &self.dwFilterCaps).field("dwSVBFilterCaps", &self.dwSVBFilterCaps).field("dwVSBFilterCaps", &self.dwVSBFilterCaps).field("dwSSBFilterCaps", &self.dwSSBFilterCaps).finish()
    }
}
impl ::core::default::Default for DD_MORESURFACECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_MOTIONCOMPCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_MOTIONCOMP_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_NONLOCALVIDMEMCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_NONLOCALVIDMEMCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwNLVBCaps == other.dwNLVBCaps && self.dwNLVBCaps2 == other.dwNLVBCaps2 && self.dwNLVBCKeyCaps == other.dwNLVBCKeyCaps && self.dwNLVBFXCaps == other.dwNLVBFXCaps && self.dwNLVBRops == other.dwNLVBRops
    }
}
impl ::core::cmp::Eq for DD_NONLOCALVIDMEMCAPS {}
impl ::core::fmt::Debug for DD_NONLOCALVIDMEMCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_NONLOCALVIDMEMCAPS").field("dwSize", &self.dwSize).field("dwNLVBCaps", &self.dwNLVBCaps).field("dwNLVBCaps2", &self.dwNLVBCaps2).field("dwNLVBCKeyCaps", &self.dwNLVBCKeyCaps).field("dwNLVBFXCaps", &self.dwNLVBFXCaps).field("dwNLVBRops", &self.dwNLVBRops).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_NTCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_NTPRIVATEDRIVERCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_NTPRIVATEDRIVERCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwPrivateCaps == other.dwPrivateCaps
    }
}
impl ::core::cmp::Eq for DD_NTPRIVATEDRIVERCAPS {}
impl ::core::fmt::Debug for DD_NTPRIVATEDRIVERCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_NTPRIVATEDRIVERCAPS").field("dwSize", &self.dwSize).field("dwPrivateCaps", &self.dwPrivateCaps).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for DD_PALETTECALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DD_PALETTE_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_PALETTE_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DD_PALETTE_GLOBAL {}
impl ::core::fmt::Debug for DD_PALETTE_GLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_PALETTE_GLOBAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::core::default::Default for DD_PALETTE_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_PALETTE_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved0 == other.dwReserved0 && self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DD_PALETTE_LOCAL {}
impl ::core::fmt::Debug for DD_PALETTE_LOCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_PALETTE_LOCAL").field("dwReserved0", &self.dwReserved0).field("dwReserved1", &self.dwReserved1).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_QUERYMOCOMPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_QUERYMOCOMPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpSurface == other.lpSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_QUERYMOCOMPSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_QUERYMOCOMPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_QUERYMOCOMPSTATUSDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpSurface", &self.lpSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_RENDERMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_RENDERMOCOMPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.dwNumBuffers == other.dwNumBuffers && self.lpBufferInfo == other.lpBufferInfo && self.dwFunction == other.dwFunction && self.lpInputData == other.lpInputData && self.dwInputDataSize == other.dwInputDataSize && self.lpOutputData == other.lpOutputData && self.dwOutputDataSize == other.dwOutputDataSize && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_RENDERMOCOMPDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_RENDERMOCOMPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_RENDERMOCOMPDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("dwNumBuffers", &self.dwNumBuffers).field("lpBufferInfo", &self.lpBufferInfo).field("dwFunction", &self.dwFunction).field("lpInputData", &self.lpInputData).field("dwInputDataSize", &self.dwInputDataSize).field("lpOutputData", &self.lpOutputData).field("dwOutputDataSize", &self.dwOutputDataSize).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SETCLIPLISTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SETCLIPLISTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.SetClipList == other.SetClipList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SETCLIPLISTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SETCLIPLISTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETCLIPLISTDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("SetClipList", &self.SetClipList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SETCOLORKEYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ckNew == other.ckNew && self.ddRVal == other.ddRVal && self.SetColorKey == other.SetColorKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SETCOLORKEYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SETCOLORKEYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETCOLORKEYDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).field("SetColorKey", &self.SetColorKey).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for DD_SETENTRIESDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for DD_SETENTRIESDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.dwBase == other.dwBase && self.dwNumEntries == other.dwNumEntries && self.lpEntries == other.lpEntries && self.ddRVal == other.ddRVal && self.SetEntries == other.SetEntries
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for DD_SETENTRIESDATA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for DD_SETENTRIESDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETENTRIESDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("dwBase", &self.dwBase).field("dwNumEntries", &self.dwNumEntries).field("lpEntries", &self.lpEntries).field("ddRVal", &self.ddRVal).field("SetEntries", &self.SetEntries).finish()
    }
}
impl ::core::default::Default for DD_SETEXCLUSIVEMODEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_SETEXCLUSIVEMODEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwEnterExcl == other.dwEnterExcl && self.dwReserved == other.dwReserved && self.ddRVal == other.ddRVal && self.SetExclusiveMode == other.SetExclusiveMode
    }
}
impl ::core::cmp::Eq for DD_SETEXCLUSIVEMODEDATA {}
impl ::core::fmt::Debug for DD_SETEXCLUSIVEMODEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETEXCLUSIVEMODEDATA").field("lpDD", &self.lpDD).field("dwEnterExcl", &self.dwEnterExcl).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).field("SetExclusiveMode", &self.SetExclusiveMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SETOVERLAYPOSITIONDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SETOVERLAYPOSITIONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSrcSurface == other.lpDDSrcSurface && self.lpDDDestSurface == other.lpDDDestSurface && self.lXPos == other.lXPos && self.lYPos == other.lYPos && self.ddRVal == other.ddRVal && self.SetOverlayPosition == other.SetOverlayPosition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SETOVERLAYPOSITIONDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SETOVERLAYPOSITIONDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETOVERLAYPOSITIONDATA").field("lpDD", &self.lpDD).field("lpDDSrcSurface", &self.lpDDSrcSurface).field("lpDDDestSurface", &self.lpDDDestSurface).field("lXPos", &self.lXPos).field("lYPos", &self.lYPos).field("ddRVal", &self.ddRVal).field("SetOverlayPosition", &self.SetOverlayPosition).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SETPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SETPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpDDPalette == other.lpDDPalette && self.ddRVal == other.ddRVal && self.SetPalette == other.SetPalette && self.Attach == other.Attach
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SETPALETTEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SETPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).field("SetPalette", &self.SetPalette).field("Attach", &self.Attach).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_STEREOMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_STEREOMODE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwHeight == other.dwHeight && self.dwWidth == other.dwWidth && self.dwBpp == other.dwBpp && self.dwRefreshRate == other.dwRefreshRate && self.bSupported == other.bSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_STEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_STEREOMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_STEREOMODE").field("dwSize", &self.dwSize).field("dwHeight", &self.dwHeight).field("dwWidth", &self.dwWidth).field("dwBpp", &self.dwBpp).field("dwRefreshRate", &self.dwRefreshRate).field("bSupported", &self.bSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACECALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SURFACE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpLcl == other.lpLcl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SURFACE_INT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SURFACE_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SURFACE_INT").field("lpLcl", &self.lpLcl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_MORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SYNCSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SYNCSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwSurfaceOffset == other.dwSurfaceOffset && self.fpLockPtr == other.fpLockPtr && self.lPitch == other.lPitch && self.dwOverlayOffset == other.dwOverlayOffset && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.dwDriverReserved4 == other.dwDriverReserved4 && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SYNCSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SYNCSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SYNCSURFACEDATA")
            .field("lpDD", &self.lpDD)
            .field("lpDDSurface", &self.lpDDSurface)
            .field("dwSurfaceOffset", &self.dwSurfaceOffset)
            .field("fpLockPtr", &self.fpLockPtr)
            .field("lPitch", &self.lPitch)
            .field("dwOverlayOffset", &self.dwOverlayOffset)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("dwDriverReserved4", &self.dwDriverReserved4)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SYNCVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SYNCVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SYNCVIDEOPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SYNCVIDEOPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SYNCVIDEOPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwOriginOffset", &self.dwOriginOffset).field("dwHeight", &self.dwHeight).field("dwVBIHeight", &self.dwVBIHeight).field("dwDriverReserved1", &self.dwDriverReserved1).field("dwDriverReserved2", &self.dwDriverReserved2).field("dwDriverReserved3", &self.dwDriverReserved3).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_UNLOCKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_UNLOCKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.Unlock == other.Unlock
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_UNLOCKDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_UNLOCKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_UNLOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("Unlock", &self.Unlock).finish()
    }
}
impl ::core::default::Default for DD_UPDATENONLOCALHEAPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_UPDATENONLOCALHEAPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwHeap == other.dwHeap && self.fpGARTLin == other.fpGARTLin && self.fpGARTDev == other.fpGARTDev && self.ulPolicyMaxBytes == other.ulPolicyMaxBytes && self.ddRVal == other.ddRVal && self.UpdateNonLocalHeap == other.UpdateNonLocalHeap
    }
}
impl ::core::cmp::Eq for DD_UPDATENONLOCALHEAPDATA {}
impl ::core::fmt::Debug for DD_UPDATENONLOCALHEAPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_UPDATENONLOCALHEAPDATA").field("lpDD", &self.lpDD).field("dwHeap", &self.dwHeap).field("fpGARTLin", &self.fpGARTLin).field("fpGARTDev", &self.fpGARTDev).field("ulPolicyMaxBytes", &self.ulPolicyMaxBytes).field("ddRVal", &self.ddRVal).field("UpdateNonLocalHeap", &self.UpdateNonLocalHeap).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_UPDATEOVERLAYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_UPDATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_UPDATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lplpDDSurface == other.lplpDDSurface && self.lplpDDVBISurface == other.lplpDDVBISurface && self.lpVideoInfo == other.lpVideoInfo && self.dwFlags == other.dwFlags && self.dwNumAutoflip == other.dwNumAutoflip && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip && self.ddRVal == other.ddRVal && self.UpdateVideoPort == other.UpdateVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_UPDATEVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_UPDATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_UPDATEVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lplpDDSurface", &self.lplpDDSurface).field("lplpDDVBISurface", &self.lplpDDVBISurface).field("lpVideoInfo", &self.lpVideoInfo).field("dwFlags", &self.dwFlags).field("dwNumAutoflip", &self.dwNumAutoflip).field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip).field("ddRVal", &self.ddRVal).field("UpdateVideoPort", &self.UpdateVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_VIDEOPORTCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_VIDEOPORT_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_VIDEOPORT_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.ddvpDesc == other.ddvpDesc && self.ddvpInfo == other.ddvpInfo && self.lpSurface == other.lpSurface && self.lpVBISurface == other.lpVBISurface && self.dwNumAutoflip == other.dwNumAutoflip && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_VIDEOPORT_LOCAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_VIDEOPORT_LOCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_VIDEOPORT_LOCAL").field("lpDD", &self.lpDD).field("ddvpDesc", &self.ddvpDesc).field("ddvpInfo", &self.ddvpInfo).field("lpSurface", &self.lpSurface).field("lpVBISurface", &self.lpVBISurface).field("dwNumAutoflip", &self.dwNumAutoflip).field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_VPORTCOLORDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_VPORTCOLORDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpColorData == other.lpColorData && self.ddRVal == other.ddRVal && self.ColorControl == other.ColorControl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_VPORTCOLORDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_VPORTCOLORDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_VPORTCOLORDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpColorData", &self.lpColorData).field("ddRVal", &self.ddRVal).field("ColorControl", &self.ColorControl).finish()
    }
}
impl ::core::default::Default for DD_WAITFORVERTICALBLANKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DD_WAITFORVERTICALBLANKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwFlags == other.dwFlags && self.bIsInVB == other.bIsInVB && self.hEvent == other.hEvent && self.ddRVal == other.ddRVal && self.WaitForVerticalBlank == other.WaitForVerticalBlank
    }
}
impl ::core::cmp::Eq for DD_WAITFORVERTICALBLANKDATA {}
impl ::core::fmt::Debug for DD_WAITFORVERTICALBLANKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_WAITFORVERTICALBLANKDATA").field("lpDD", &self.lpDD).field("dwFlags", &self.dwFlags).field("bIsInVB", &self.bIsInVB).field("hEvent", &self.hEvent).field("ddRVal", &self.ddRVal).field("WaitForVerticalBlank", &self.WaitForVerticalBlank).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_WAITFORVPORTSYNCDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_WAITFORVPORTSYNCDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.dwLine == other.dwLine && self.dwTimeOut == other.dwTimeOut && self.ddRVal == other.ddRVal && self.UpdateVideoPort == other.UpdateVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_WAITFORVPORTSYNCDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_WAITFORVPORTSYNCDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_WAITFORVPORTSYNCDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("dwLine", &self.dwLine).field("dwTimeOut", &self.dwTimeOut).field("ddRVal", &self.ddRVal).field("UpdateVideoPort", &self.UpdateVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXAPI_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DX_IRQDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DX_IRQDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwIrqFlags == other.dwIrqFlags
    }
}
impl ::core::cmp::Eq for DX_IRQDATA {}
impl ::core::fmt::Debug for DX_IRQDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DX_IRQDATA").field("dwIrqFlags", &self.dwIrqFlags).finish()
    }
}
impl ::core::default::Default for HEAPALIAS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HEAPALIAS {
    fn eq(&self, other: &Self) -> bool {
        self.fpVidMem == other.fpVidMem && self.lpAlias == other.lpAlias && self.dwAliasSize == other.dwAliasSize
    }
}
impl ::core::cmp::Eq for HEAPALIAS {}
impl ::core::fmt::Debug for HEAPALIAS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAPALIAS").field("fpVidMem", &self.fpVidMem).field("lpAlias", &self.lpAlias).field("dwAliasSize", &self.dwAliasSize).finish()
    }
}
impl ::core::default::Default for HEAPALIASINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HEAPALIASINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwRefCnt == other.dwRefCnt && self.dwFlags == other.dwFlags && self.dwNumHeaps == other.dwNumHeaps && self.lpAliases == other.lpAliases
    }
}
impl ::core::cmp::Eq for HEAPALIASINFO {}
impl ::core::fmt::Debug for HEAPALIASINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAPALIASINFO").field("dwRefCnt", &self.dwRefCnt).field("dwFlags", &self.dwFlags).field("dwNumHeaps", &self.dwNumHeaps).field("lpAliases", &self.lpAliases).finish()
    }
}
impl ::core::default::Default for HEAPALIGNMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IDDVideoPortContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDDVideoPortContainer {}
impl ::core::fmt::Debug for IDDVideoPortContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDDVideoPortContainer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDraw {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDraw {}
impl ::core::fmt::Debug for IDirectDraw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDraw").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDraw2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDraw2 {}
impl ::core::fmt::Debug for IDirectDraw2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDraw2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDraw4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDraw4 {}
impl ::core::fmt::Debug for IDirectDraw4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDraw4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDraw7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDraw7 {}
impl ::core::fmt::Debug for IDirectDraw7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDraw7").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawClipper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawClipper {}
impl ::core::fmt::Debug for IDirectDrawClipper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawClipper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawColorControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawColorControl {}
impl ::core::fmt::Debug for IDirectDrawColorControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawColorControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawGammaControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawGammaControl {}
impl ::core::fmt::Debug for IDirectDrawGammaControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawGammaControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawKernel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawKernel {}
impl ::core::fmt::Debug for IDirectDrawKernel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawKernel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawPalette {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawPalette {}
impl ::core::fmt::Debug for IDirectDrawPalette {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawPalette").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface {}
impl ::core::fmt::Debug for IDirectDrawSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawSurface2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface2 {}
impl ::core::fmt::Debug for IDirectDrawSurface2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawSurface3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface3 {}
impl ::core::fmt::Debug for IDirectDrawSurface3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawSurface4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface4 {}
impl ::core::fmt::Debug for IDirectDrawSurface4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawSurface7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface7 {}
impl ::core::fmt::Debug for IDirectDrawSurface7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface7").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawSurfaceKernel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurfaceKernel {}
impl ::core::fmt::Debug for IDirectDrawSurfaceKernel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurfaceKernel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawVideoPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawVideoPort {}
impl ::core::fmt::Debug for IDirectDrawVideoPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawVideoPort").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectDrawVideoPortNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawVideoPortNotify {}
impl ::core::fmt::Debug for IDirectDrawVideoPortNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawVideoPortNotify").field(&self.0).finish()
    }
}
impl ::core::default::Default for IUNKNOWN_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IUNKNOWN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.lpGuid == other.lpGuid && self.lpIUnknown == other.lpIUnknown
    }
}
impl ::core::cmp::Eq for IUNKNOWN_LIST {}
impl ::core::fmt::Debug for IUNKNOWN_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IUNKNOWN_LIST").field("lpLink", &self.lpLink).field("lpGuid", &self.lpGuid).field("lpIUnknown", &self.lpIUnknown).finish()
    }
}
impl ::core::default::Default for MDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MDL {
    fn eq(&self, other: &Self) -> bool {
        self.MdlNext == other.MdlNext && self.MdlSize == other.MdlSize && self.MdlFlags == other.MdlFlags && self.Process == other.Process && self.lpMappedSystemVa == other.lpMappedSystemVa && self.lpStartVa == other.lpStartVa && self.ByteCount == other.ByteCount && self.ByteOffset == other.ByteOffset
    }
}
impl ::core::cmp::Eq for MDL {}
impl ::core::fmt::Debug for MDL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDL").field("MdlNext", &self.MdlNext).field("MdlSize", &self.MdlSize).field("MdlFlags", &self.MdlFlags).field("Process", &self.Process).field("lpMappedSystemVa", &self.lpMappedSystemVa).field("lpStartVa", &self.lpStartVa).field("ByteCount", &self.ByteCount).field("ByteOffset", &self.ByteOffset).finish()
    }
}
impl ::core::default::Default for PROCESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.dwProcessId == other.dwProcessId && self.dwRefCnt == other.dwRefCnt && self.dwAlphaDepth == other.dwAlphaDepth && self.dwZDepth == other.dwZDepth
    }
}
impl ::core::cmp::Eq for PROCESS_LIST {}
impl ::core::fmt::Debug for PROCESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_LIST").field("lpLink", &self.lpLink).field("dwProcessId", &self.dwProcessId).field("dwRefCnt", &self.dwRefCnt).field("dwAlphaDepth", &self.dwAlphaDepth).field("dwZDepth", &self.dwZDepth).finish()
    }
}
impl ::core::default::Default for SURFACEALIGNMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEOMEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VIDEOMEMORYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDMEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDMEMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VMEMHEAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VMEML {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VMEML {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.ptr == other.ptr && self.size == other.size && self.bDiscardable == other.bDiscardable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VMEML {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VMEML {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VMEML").field("next", &self.next).field("ptr", &self.ptr).field("size", &self.size).field("bDiscardable", &self.bDiscardable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VMEMR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VMEMR {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.prev == other.prev && self.pUp == other.pUp && self.pDown == other.pDown && self.pLeft == other.pLeft && self.pRight == other.pRight && self.ptr == other.ptr && self.size == other.size && self.x == other.x && self.y == other.y && self.cx == other.cx && self.cy == other.cy && self.flags == other.flags && self.pBits == other.pBits && self.bDiscardable == other.bDiscardable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VMEMR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VMEMR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VMEMR").field("next", &self.next).field("prev", &self.prev).field("pUp", &self.pUp).field("pDown", &self.pDown).field("pLeft", &self.pLeft).field("pRight", &self.pRight).field("ptr", &self.ptr).field("size", &self.size).field("x", &self.x).field("y", &self.y).field("cx", &self.cx).field("cy", &self.cy).field("flags", &self.flags).field("pBits", &self.pBits).field("bDiscardable", &self.bDiscardable).finish()
    }
}
