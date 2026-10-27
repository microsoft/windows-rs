windows_link::link!("ddraw.dll" "system" fn DirectDrawCreate(lpguid : *mut windows_sys::core::GUID, lplpdd : *mut LPDIRECTDRAW, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ddraw.dll" "system" fn DirectDrawCreateClipper(dwflags : super::DWORD, lplpddclipper : *mut LPDIRECTDRAWCLIPPER, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("ddraw.dll" "system" fn DirectDrawCreateEx(lpguid : *mut windows_sys::core::GUID, lplpdd : *mut super::LPVOID, iid : *const super::IID, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateA(lpcallback : LPDDENUMCALLBACKA, lpcontext : super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateExA(lpcallback : LPDDENUMCALLBACKEXA, lpcontext : super::LPVOID, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateExW(lpcallback : LPDDENUMCALLBACKEXW, lpcontext : super::LPVOID, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateW(lpcallback : LPDDENUMCALLBACKW, lpcontext : super::LPVOID) -> windows_sys::core::HRESULT);
pub const CLSID_DirectDraw: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7b70ee0_4340_11cf_b063_0020afc2cd35);
pub const CLSID_DirectDraw7: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3c305196_50db_11d3_9cfe_00c04fd930c5);
pub const CLSID_DirectDrawClipper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x593817a0_7db3_11cf_a2de_00aa00b93356);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DDARGB {
    pub blue: super::BYTE,
    pub green: super::BYTE,
    pub red: super::BYTE,
    pub alpha: super::BYTE,
}
pub const DDBD_1: i32 = 16384;
pub const DDBD_16: i32 = 1024;
pub const DDBD_2: i32 = 8192;
pub const DDBD_24: i32 = 512;
pub const DDBD_32: i32 = 256;
pub const DDBD_4: i32 = 4096;
pub const DDBD_8: i32 = 2048;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DDBLTBATCH {
    pub lprDest: super::LPRECT,
    pub lpDDSSrc: LPDIRECTDRAWSURFACE,
    pub lprSrc: super::LPRECT,
    pub dwFlags: super::DWORD,
    pub lpDDBltFx: LPDDBLTFX,
}
pub const DDBLTFAST_DESTCOLORKEY: i32 = 2;
pub const DDBLTFAST_DONOTWAIT: i32 = 32;
pub const DDBLTFAST_NOCOLORKEY: i32 = 0;
pub const DDBLTFAST_SRCCOLORKEY: i32 = 1;
pub const DDBLTFAST_WAIT: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDBLTFX {
    pub dwSize: super::DWORD,
    pub dwDDFX: super::DWORD,
    pub dwROP: super::DWORD,
    pub dwDDROP: super::DWORD,
    pub dwRotationAngle: super::DWORD,
    pub dwZBufferOpCode: super::DWORD,
    pub dwZBufferLow: super::DWORD,
    pub dwZBufferHigh: super::DWORD,
    pub dwZBufferBaseDest: super::DWORD,
    pub dwZDestConstBitDepth: super::DWORD,
    pub Anonymous: DDBLTFX_0,
    pub dwZSrcConstBitDepth: super::DWORD,
    pub Anonymous2: DDBLTFX_1,
    pub dwAlphaEdgeBlendBitDepth: super::DWORD,
    pub dwAlphaEdgeBlend: super::DWORD,
    pub dwReserved: super::DWORD,
    pub dwAlphaDestConstBitDepth: super::DWORD,
    pub Anonymous3: DDBLTFX_2,
    pub dwAlphaSrcConstBitDepth: super::DWORD,
    pub Anonymous4: DDBLTFX_3,
    pub Anonymous5: DDBLTFX_4,
    pub ddckDestColorkey: DDCOLORKEY,
    pub ddckSrcColorkey: DDCOLORKEY,
}
#[cfg(feature = "shared")]
impl Default for DDBLTFX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDBLTFX_0 {
    pub dwZDestConst: super::DWORD,
    pub lpDDSZBufferDest: LPDIRECTDRAWSURFACE,
}
#[cfg(feature = "shared")]
impl Default for DDBLTFX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDBLTFX_1 {
    pub dwZSrcConst: super::DWORD,
    pub lpDDSZBufferSrc: LPDIRECTDRAWSURFACE,
}
#[cfg(feature = "shared")]
impl Default for DDBLTFX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDBLTFX_2 {
    pub dwAlphaDestConst: super::DWORD,
    pub lpDDSAlphaDest: LPDIRECTDRAWSURFACE,
}
#[cfg(feature = "shared")]
impl Default for DDBLTFX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDBLTFX_3 {
    pub dwAlphaSrcConst: super::DWORD,
    pub lpDDSAlphaSrc: LPDIRECTDRAWSURFACE,
}
#[cfg(feature = "shared")]
impl Default for DDBLTFX_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDBLTFX_4 {
    pub dwFillColor: super::DWORD,
    pub dwFillDepth: super::DWORD,
    pub dwFillPixel: super::DWORD,
    pub lpDDSPattern: LPDIRECTDRAWSURFACE,
}
#[cfg(feature = "shared")]
impl Default for DDBLTFX_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDBLTFX_ARITHSTRETCHY: i32 = 1;
pub const DDBLTFX_MIRRORLEFTRIGHT: i32 = 2;
pub const DDBLTFX_MIRRORUPDOWN: i32 = 4;
pub const DDBLTFX_NOTEARING: i32 = 8;
pub const DDBLTFX_ROTATE180: i32 = 16;
pub const DDBLTFX_ROTATE270: i32 = 32;
pub const DDBLTFX_ROTATE90: i32 = 64;
pub const DDBLTFX_ZBUFFERBASEDEST: i32 = 256;
pub const DDBLTFX_ZBUFFERRANGE: i32 = 128;
pub const DDBLT_ALPHADEST: i32 = 1;
pub const DDBLT_ALPHADESTCONSTOVERRIDE: i32 = 2;
pub const DDBLT_ALPHADESTNEG: i32 = 4;
pub const DDBLT_ALPHADESTSURFACEOVERRIDE: i32 = 8;
pub const DDBLT_ALPHAEDGEBLEND: i32 = 16;
pub const DDBLT_ALPHASRC: i32 = 32;
pub const DDBLT_ALPHASRCCONSTOVERRIDE: i32 = 64;
pub const DDBLT_ALPHASRCNEG: i32 = 128;
pub const DDBLT_ALPHASRCSURFACEOVERRIDE: i32 = 256;
pub const DDBLT_ASYNC: i32 = 512;
pub const DDBLT_COLORFILL: i32 = 1024;
pub const DDBLT_DDFX: i32 = 2048;
pub const DDBLT_DDROPS: i32 = 4096;
pub const DDBLT_DEPTHFILL: i32 = 33554432;
pub const DDBLT_DONOTWAIT: i32 = 134217728;
pub const DDBLT_EXTENDED_FLAGS: i32 = 1073741824;
pub const DDBLT_EXTENDED_LINEAR_CONTENT: i32 = 4;
pub const DDBLT_KEYDEST: i32 = 8192;
pub const DDBLT_KEYDESTOVERRIDE: i32 = 16384;
pub const DDBLT_KEYSRC: i32 = 32768;
pub const DDBLT_KEYSRCOVERRIDE: i32 = 65536;
pub const DDBLT_LAST_PRESENTATION: i32 = 536870912;
pub const DDBLT_PRESENTATION: i32 = 268435456;
pub const DDBLT_ROP: i32 = 131072;
pub const DDBLT_ROTATIONANGLE: i32 = 262144;
pub const DDBLT_WAIT: i32 = 16777216;
pub const DDBLT_ZBUFFER: i32 = 524288;
pub const DDBLT_ZBUFFERDESTCONSTOVERRIDE: i32 = 1048576;
pub const DDBLT_ZBUFFERDESTOVERRIDE: i32 = 2097152;
pub const DDBLT_ZBUFFERSRCCONSTOVERRIDE: i32 = 4194304;
pub const DDBLT_ZBUFFERSRCOVERRIDE: i32 = 8388608;
#[cfg(feature = "shared")]
pub type DDCAPS = DDCAPS_DX7;
pub const DDCAPS2_AUTOFLIPOVERLAY: i32 = 8;
pub const DDCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824;
pub const DDCAPS2_CANBOBHARDWARE: i32 = 16384;
pub const DDCAPS2_CANBOBINTERLEAVED: i32 = 16;
pub const DDCAPS2_CANBOBNONINTERLEAVED: i32 = 32;
pub const DDCAPS2_CANCALIBRATEGAMMA: i32 = 1048576;
pub const DDCAPS2_CANDROPZ16BIT: i32 = 256;
pub const DDCAPS2_CANFLIPODDEVEN: i32 = 8192;
pub const DDCAPS2_CANMANAGERESOURCE: i32 = 268435456;
pub const DDCAPS2_CANMANAGETEXTURE: i32 = 8388608;
pub const DDCAPS2_CANRENDERWINDOWED: i32 = 524288;
pub const DDCAPS2_CANSHARERESOURCE: u32 = 2147483648;
pub const DDCAPS2_CERTIFIED: i32 = 1;
pub const DDCAPS2_COLORCONTROLOVERLAY: i32 = 64;
pub const DDCAPS2_COLORCONTROLPRIMARY: i32 = 128;
pub const DDCAPS2_COPYFOURCC: i32 = 32768;
pub const DDCAPS2_DYNAMICTEXTURES: i32 = 536870912;
pub const DDCAPS2_FLIPINTERVAL: i32 = 2097152;
pub const DDCAPS2_FLIPNOVSYNC: i32 = 4194304;
pub const DDCAPS2_NO2DDURING3DSCENE: i32 = 2;
pub const DDCAPS2_NONLOCALVIDMEM: i32 = 512;
pub const DDCAPS2_NONLOCALVIDMEMCAPS: i32 = 1024;
pub const DDCAPS2_NOPAGELOCKREQUIRED: i32 = 2048;
pub const DDCAPS2_PRIMARYGAMMA: i32 = 131072;
pub const DDCAPS2_RESERVED1: i32 = 134217728;
pub const DDCAPS2_STEREO: i32 = 33554432;
pub const DDCAPS2_SYSTONONLOCAL_AS_SYSTOLOCAL: i32 = 67108864;
pub const DDCAPS2_TEXMANINNONLOCALVIDMEM: i32 = 16777216;
pub const DDCAPS2_VIDEOPORT: i32 = 4;
pub const DDCAPS2_WIDESURFACES: i32 = 4096;
pub const DDCAPS_3D: i32 = 1;
pub const DDCAPS_ALIGNBOUNDARYDEST: i32 = 2;
pub const DDCAPS_ALIGNBOUNDARYSRC: i32 = 8;
pub const DDCAPS_ALIGNSIZEDEST: i32 = 4;
pub const DDCAPS_ALIGNSIZESRC: i32 = 16;
pub const DDCAPS_ALIGNSTRIDE: i32 = 32;
pub const DDCAPS_ALPHA: i32 = 8388608;
pub const DDCAPS_BANKSWITCHED: i32 = 134217728;
pub const DDCAPS_BLT: i32 = 64;
pub const DDCAPS_BLTCOLORFILL: i32 = 67108864;
pub const DDCAPS_BLTDEPTHFILL: i32 = 268435456;
pub const DDCAPS_BLTFOURCC: i32 = 256;
pub const DDCAPS_BLTQUEUE: i32 = 128;
pub const DDCAPS_BLTSTRETCH: i32 = 512;
pub const DDCAPS_CANBLTSYSMEM: u32 = 2147483648;
pub const DDCAPS_CANCLIP: i32 = 536870912;
pub const DDCAPS_CANCLIPSTRETCHED: i32 = 1073741824;
pub const DDCAPS_COLORKEY: i32 = 4194304;
pub const DDCAPS_COLORKEYHWASSIST: i32 = 16777216;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX1 {
    pub dwSize: super::DWORD,
    pub dwCaps: super::DWORD,
    pub dwCaps2: super::DWORD,
    pub dwCKeyCaps: super::DWORD,
    pub dwFXCaps: super::DWORD,
    pub dwFXAlphaCaps: super::DWORD,
    pub dwPalCaps: super::DWORD,
    pub dwSVCaps: super::DWORD,
    pub dwAlphaBltConstBitDepths: super::DWORD,
    pub dwAlphaBltPixelBitDepths: super::DWORD,
    pub dwAlphaBltSurfaceBitDepths: super::DWORD,
    pub dwAlphaOverlayConstBitDepths: super::DWORD,
    pub dwAlphaOverlayPixelBitDepths: super::DWORD,
    pub dwAlphaOverlaySurfaceBitDepths: super::DWORD,
    pub dwZBufferBitDepths: super::DWORD,
    pub dwVidMemTotal: super::DWORD,
    pub dwVidMemFree: super::DWORD,
    pub dwMaxVisibleOverlays: super::DWORD,
    pub dwCurrVisibleOverlays: super::DWORD,
    pub dwNumFourCCCodes: super::DWORD,
    pub dwAlignBoundarySrc: super::DWORD,
    pub dwAlignSizeSrc: super::DWORD,
    pub dwAlignBoundaryDest: super::DWORD,
    pub dwAlignSizeDest: super::DWORD,
    pub dwAlignStrideAlign: super::DWORD,
    pub dwRops: [super::DWORD; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: super::DWORD,
    pub dwMaxOverlayStretch: super::DWORD,
    pub dwMinLiveVideoStretch: super::DWORD,
    pub dwMaxLiveVideoStretch: super::DWORD,
    pub dwMinHwCodecStretch: super::DWORD,
    pub dwMaxHwCodecStretch: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub dwReserved3: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for DDCAPS_DX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX3 {
    pub dwSize: super::DWORD,
    pub dwCaps: super::DWORD,
    pub dwCaps2: super::DWORD,
    pub dwCKeyCaps: super::DWORD,
    pub dwFXCaps: super::DWORD,
    pub dwFXAlphaCaps: super::DWORD,
    pub dwPalCaps: super::DWORD,
    pub dwSVCaps: super::DWORD,
    pub dwAlphaBltConstBitDepths: super::DWORD,
    pub dwAlphaBltPixelBitDepths: super::DWORD,
    pub dwAlphaBltSurfaceBitDepths: super::DWORD,
    pub dwAlphaOverlayConstBitDepths: super::DWORD,
    pub dwAlphaOverlayPixelBitDepths: super::DWORD,
    pub dwAlphaOverlaySurfaceBitDepths: super::DWORD,
    pub dwZBufferBitDepths: super::DWORD,
    pub dwVidMemTotal: super::DWORD,
    pub dwVidMemFree: super::DWORD,
    pub dwMaxVisibleOverlays: super::DWORD,
    pub dwCurrVisibleOverlays: super::DWORD,
    pub dwNumFourCCCodes: super::DWORD,
    pub dwAlignBoundarySrc: super::DWORD,
    pub dwAlignSizeSrc: super::DWORD,
    pub dwAlignBoundaryDest: super::DWORD,
    pub dwAlignSizeDest: super::DWORD,
    pub dwAlignStrideAlign: super::DWORD,
    pub dwRops: [super::DWORD; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: super::DWORD,
    pub dwMaxOverlayStretch: super::DWORD,
    pub dwMinLiveVideoStretch: super::DWORD,
    pub dwMaxLiveVideoStretch: super::DWORD,
    pub dwMinHwCodecStretch: super::DWORD,
    pub dwMaxHwCodecStretch: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub dwReserved3: super::DWORD,
    pub dwSVBCaps: super::DWORD,
    pub dwSVBCKeyCaps: super::DWORD,
    pub dwSVBFXCaps: super::DWORD,
    pub dwSVBRops: [super::DWORD; 8],
    pub dwVSBCaps: super::DWORD,
    pub dwVSBCKeyCaps: super::DWORD,
    pub dwVSBFXCaps: super::DWORD,
    pub dwVSBRops: [super::DWORD; 8],
    pub dwSSBCaps: super::DWORD,
    pub dwSSBCKeyCaps: super::DWORD,
    pub dwSSBFXCaps: super::DWORD,
    pub dwSSBRops: [super::DWORD; 8],
    pub dwReserved4: super::DWORD,
    pub dwReserved5: super::DWORD,
    pub dwReserved6: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for DDCAPS_DX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX5 {
    pub dwSize: super::DWORD,
    pub dwCaps: super::DWORD,
    pub dwCaps2: super::DWORD,
    pub dwCKeyCaps: super::DWORD,
    pub dwFXCaps: super::DWORD,
    pub dwFXAlphaCaps: super::DWORD,
    pub dwPalCaps: super::DWORD,
    pub dwSVCaps: super::DWORD,
    pub dwAlphaBltConstBitDepths: super::DWORD,
    pub dwAlphaBltPixelBitDepths: super::DWORD,
    pub dwAlphaBltSurfaceBitDepths: super::DWORD,
    pub dwAlphaOverlayConstBitDepths: super::DWORD,
    pub dwAlphaOverlayPixelBitDepths: super::DWORD,
    pub dwAlphaOverlaySurfaceBitDepths: super::DWORD,
    pub dwZBufferBitDepths: super::DWORD,
    pub dwVidMemTotal: super::DWORD,
    pub dwVidMemFree: super::DWORD,
    pub dwMaxVisibleOverlays: super::DWORD,
    pub dwCurrVisibleOverlays: super::DWORD,
    pub dwNumFourCCCodes: super::DWORD,
    pub dwAlignBoundarySrc: super::DWORD,
    pub dwAlignSizeSrc: super::DWORD,
    pub dwAlignBoundaryDest: super::DWORD,
    pub dwAlignSizeDest: super::DWORD,
    pub dwAlignStrideAlign: super::DWORD,
    pub dwRops: [super::DWORD; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: super::DWORD,
    pub dwMaxOverlayStretch: super::DWORD,
    pub dwMinLiveVideoStretch: super::DWORD,
    pub dwMaxLiveVideoStretch: super::DWORD,
    pub dwMinHwCodecStretch: super::DWORD,
    pub dwMaxHwCodecStretch: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub dwReserved3: super::DWORD,
    pub dwSVBCaps: super::DWORD,
    pub dwSVBCKeyCaps: super::DWORD,
    pub dwSVBFXCaps: super::DWORD,
    pub dwSVBRops: [super::DWORD; 8],
    pub dwVSBCaps: super::DWORD,
    pub dwVSBCKeyCaps: super::DWORD,
    pub dwVSBFXCaps: super::DWORD,
    pub dwVSBRops: [super::DWORD; 8],
    pub dwSSBCaps: super::DWORD,
    pub dwSSBCKeyCaps: super::DWORD,
    pub dwSSBFXCaps: super::DWORD,
    pub dwSSBRops: [super::DWORD; 8],
    pub dwMaxVideoPorts: super::DWORD,
    pub dwCurrVideoPorts: super::DWORD,
    pub dwSVBCaps2: super::DWORD,
    pub dwNLVBCaps: super::DWORD,
    pub dwNLVBCaps2: super::DWORD,
    pub dwNLVBCKeyCaps: super::DWORD,
    pub dwNLVBFXCaps: super::DWORD,
    pub dwNLVBRops: [super::DWORD; 8],
}
#[cfg(feature = "shared")]
impl Default for DDCAPS_DX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX6 {
    pub dwSize: super::DWORD,
    pub dwCaps: super::DWORD,
    pub dwCaps2: super::DWORD,
    pub dwCKeyCaps: super::DWORD,
    pub dwFXCaps: super::DWORD,
    pub dwFXAlphaCaps: super::DWORD,
    pub dwPalCaps: super::DWORD,
    pub dwSVCaps: super::DWORD,
    pub dwAlphaBltConstBitDepths: super::DWORD,
    pub dwAlphaBltPixelBitDepths: super::DWORD,
    pub dwAlphaBltSurfaceBitDepths: super::DWORD,
    pub dwAlphaOverlayConstBitDepths: super::DWORD,
    pub dwAlphaOverlayPixelBitDepths: super::DWORD,
    pub dwAlphaOverlaySurfaceBitDepths: super::DWORD,
    pub dwZBufferBitDepths: super::DWORD,
    pub dwVidMemTotal: super::DWORD,
    pub dwVidMemFree: super::DWORD,
    pub dwMaxVisibleOverlays: super::DWORD,
    pub dwCurrVisibleOverlays: super::DWORD,
    pub dwNumFourCCCodes: super::DWORD,
    pub dwAlignBoundarySrc: super::DWORD,
    pub dwAlignSizeSrc: super::DWORD,
    pub dwAlignBoundaryDest: super::DWORD,
    pub dwAlignSizeDest: super::DWORD,
    pub dwAlignStrideAlign: super::DWORD,
    pub dwRops: [super::DWORD; 8],
    pub ddsOldCaps: DDSCAPS,
    pub dwMinOverlayStretch: super::DWORD,
    pub dwMaxOverlayStretch: super::DWORD,
    pub dwMinLiveVideoStretch: super::DWORD,
    pub dwMaxLiveVideoStretch: super::DWORD,
    pub dwMinHwCodecStretch: super::DWORD,
    pub dwMaxHwCodecStretch: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub dwReserved3: super::DWORD,
    pub dwSVBCaps: super::DWORD,
    pub dwSVBCKeyCaps: super::DWORD,
    pub dwSVBFXCaps: super::DWORD,
    pub dwSVBRops: [super::DWORD; 8],
    pub dwVSBCaps: super::DWORD,
    pub dwVSBCKeyCaps: super::DWORD,
    pub dwVSBFXCaps: super::DWORD,
    pub dwVSBRops: [super::DWORD; 8],
    pub dwSSBCaps: super::DWORD,
    pub dwSSBCKeyCaps: super::DWORD,
    pub dwSSBFXCaps: super::DWORD,
    pub dwSSBRops: [super::DWORD; 8],
    pub dwMaxVideoPorts: super::DWORD,
    pub dwCurrVideoPorts: super::DWORD,
    pub dwSVBCaps2: super::DWORD,
    pub dwNLVBCaps: super::DWORD,
    pub dwNLVBCaps2: super::DWORD,
    pub dwNLVBCKeyCaps: super::DWORD,
    pub dwNLVBFXCaps: super::DWORD,
    pub dwNLVBRops: [super::DWORD; 8],
    pub ddsCaps: DDSCAPS2,
}
#[cfg(feature = "shared")]
impl Default for DDCAPS_DX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX7 {
    pub dwSize: super::DWORD,
    pub dwCaps: super::DWORD,
    pub dwCaps2: super::DWORD,
    pub dwCKeyCaps: super::DWORD,
    pub dwFXCaps: super::DWORD,
    pub dwFXAlphaCaps: super::DWORD,
    pub dwPalCaps: super::DWORD,
    pub dwSVCaps: super::DWORD,
    pub dwAlphaBltConstBitDepths: super::DWORD,
    pub dwAlphaBltPixelBitDepths: super::DWORD,
    pub dwAlphaBltSurfaceBitDepths: super::DWORD,
    pub dwAlphaOverlayConstBitDepths: super::DWORD,
    pub dwAlphaOverlayPixelBitDepths: super::DWORD,
    pub dwAlphaOverlaySurfaceBitDepths: super::DWORD,
    pub dwZBufferBitDepths: super::DWORD,
    pub dwVidMemTotal: super::DWORD,
    pub dwVidMemFree: super::DWORD,
    pub dwMaxVisibleOverlays: super::DWORD,
    pub dwCurrVisibleOverlays: super::DWORD,
    pub dwNumFourCCCodes: super::DWORD,
    pub dwAlignBoundarySrc: super::DWORD,
    pub dwAlignSizeSrc: super::DWORD,
    pub dwAlignBoundaryDest: super::DWORD,
    pub dwAlignSizeDest: super::DWORD,
    pub dwAlignStrideAlign: super::DWORD,
    pub dwRops: [super::DWORD; 8],
    pub ddsOldCaps: DDSCAPS,
    pub dwMinOverlayStretch: super::DWORD,
    pub dwMaxOverlayStretch: super::DWORD,
    pub dwMinLiveVideoStretch: super::DWORD,
    pub dwMaxLiveVideoStretch: super::DWORD,
    pub dwMinHwCodecStretch: super::DWORD,
    pub dwMaxHwCodecStretch: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub dwReserved3: super::DWORD,
    pub dwSVBCaps: super::DWORD,
    pub dwSVBCKeyCaps: super::DWORD,
    pub dwSVBFXCaps: super::DWORD,
    pub dwSVBRops: [super::DWORD; 8],
    pub dwVSBCaps: super::DWORD,
    pub dwVSBCKeyCaps: super::DWORD,
    pub dwVSBFXCaps: super::DWORD,
    pub dwVSBRops: [super::DWORD; 8],
    pub dwSSBCaps: super::DWORD,
    pub dwSSBCKeyCaps: super::DWORD,
    pub dwSSBFXCaps: super::DWORD,
    pub dwSSBRops: [super::DWORD; 8],
    pub dwMaxVideoPorts: super::DWORD,
    pub dwCurrVideoPorts: super::DWORD,
    pub dwSVBCaps2: super::DWORD,
    pub dwNLVBCaps: super::DWORD,
    pub dwNLVBCaps2: super::DWORD,
    pub dwNLVBCKeyCaps: super::DWORD,
    pub dwNLVBFXCaps: super::DWORD,
    pub dwNLVBRops: [super::DWORD; 8],
    pub ddsCaps: DDSCAPS2,
}
#[cfg(feature = "shared")]
impl Default for DDCAPS_DX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDCAPS_GDI: i32 = 1024;
pub const DDCAPS_NOHARDWARE: i32 = 33554432;
pub const DDCAPS_OVERLAY: i32 = 2048;
pub const DDCAPS_OVERLAYCANTCLIP: i32 = 4096;
pub const DDCAPS_OVERLAYFOURCC: i32 = 8192;
pub const DDCAPS_OVERLAYSTRETCH: i32 = 16384;
pub const DDCAPS_PALETTE: i32 = 32768;
pub const DDCAPS_PALETTEVSYNC: i32 = 65536;
pub const DDCAPS_READSCANLINE: i32 = 131072;
pub const DDCAPS_RESERVED1: i32 = 262144;
pub const DDCAPS_VBI: i32 = 524288;
pub const DDCAPS_ZBLTS: i32 = 1048576;
pub const DDCAPS_ZOVERLAYS: i32 = 2097152;
pub const DDCKEYCAPS_DESTBLT: i32 = 1;
pub const DDCKEYCAPS_DESTBLTCLRSPACE: i32 = 2;
pub const DDCKEYCAPS_DESTBLTCLRSPACEYUV: i32 = 4;
pub const DDCKEYCAPS_DESTBLTYUV: i32 = 8;
pub const DDCKEYCAPS_DESTOVERLAY: i32 = 16;
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACE: i32 = 32;
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACEYUV: i32 = 64;
pub const DDCKEYCAPS_DESTOVERLAYONEACTIVE: i32 = 128;
pub const DDCKEYCAPS_DESTOVERLAYYUV: i32 = 256;
pub const DDCKEYCAPS_NOCOSTOVERLAY: i32 = 262144;
pub const DDCKEYCAPS_SRCBLT: i32 = 512;
pub const DDCKEYCAPS_SRCBLTCLRSPACE: i32 = 1024;
pub const DDCKEYCAPS_SRCBLTCLRSPACEYUV: i32 = 2048;
pub const DDCKEYCAPS_SRCBLTYUV: i32 = 4096;
pub const DDCKEYCAPS_SRCOVERLAY: i32 = 8192;
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACE: i32 = 16384;
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACEYUV: i32 = 32768;
pub const DDCKEYCAPS_SRCOVERLAYONEACTIVE: i32 = 65536;
pub const DDCKEYCAPS_SRCOVERLAYYUV: i32 = 131072;
pub const DDCKEY_COLORSPACE: i32 = 1;
pub const DDCKEY_DESTBLT: i32 = 2;
pub const DDCKEY_DESTOVERLAY: i32 = 4;
pub const DDCKEY_SRCBLT: i32 = 8;
pub const DDCKEY_SRCOVERLAY: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DDCOLORCONTROL {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub lBrightness: super::LONG,
    pub lContrast: super::LONG,
    pub lHue: super::LONG,
    pub lSaturation: super::LONG,
    pub lSharpness: super::LONG,
    pub lGamma: super::LONG,
    pub lColorEnable: super::LONG,
    pub dwReserved1: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DDCOLORKEY {
    pub dwColorSpaceLowValue: super::DWORD,
    pub dwColorSpaceHighValue: super::DWORD,
}
pub const DDCOLOR_BRIGHTNESS: i32 = 1;
pub const DDCOLOR_COLORENABLE: i32 = 64;
pub const DDCOLOR_CONTRAST: i32 = 2;
pub const DDCOLOR_GAMMA: i32 = 32;
pub const DDCOLOR_HUE: i32 = 4;
pub const DDCOLOR_SATURATION: i32 = 8;
pub const DDCOLOR_SHARPNESS: i32 = 16;
pub const DDCREATE_EMULATIONONLY: i32 = 2;
pub const DDCREATE_HARDWAREONLY: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DDDEVICEIDENTIFIER {
    pub szDriver: [i8; 512],
    pub szDescription: [i8; 512],
    pub liDriverVersion: super::LARGE_INTEGER,
    pub dwVendorId: super::DWORD,
    pub dwDeviceId: super::DWORD,
    pub dwSubSysId: super::DWORD,
    pub dwRevision: super::DWORD,
    pub guidDeviceIdentifier: windows_sys::core::GUID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DDDEVICEIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DDDEVICEIDENTIFIER2 {
    pub szDriver: [i8; 512],
    pub szDescription: [i8; 512],
    pub liDriverVersion: super::LARGE_INTEGER,
    pub dwVendorId: super::DWORD,
    pub dwDeviceId: super::DWORD,
    pub dwSubSysId: super::DWORD,
    pub dwRevision: super::DWORD,
    pub guidDeviceIdentifier: windows_sys::core::GUID,
    pub dwWHQLLevel: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DDDEVICEIDENTIFIER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDEDM_REFRESHRATES: i32 = 1;
pub const DDEDM_STANDARDVGAMODES: i32 = 2;
pub const DDEM_MODEFAILED: i32 = 2;
pub const DDEM_MODEPASSED: i32 = 1;
pub const DDENUMOVERLAYZ_BACKTOFRONT: i32 = 0;
pub const DDENUMOVERLAYZ_FRONTTOBACK: i32 = 1;
pub const DDENUMRET_CANCEL: i32 = 0;
pub const DDENUMRET_OK: i32 = 1;
pub const DDENUMSURFACES_ALL: i32 = 1;
pub const DDENUMSURFACES_CANBECREATED: i32 = 8;
pub const DDENUMSURFACES_DOESEXIST: i32 = 16;
pub const DDENUMSURFACES_MATCH: i32 = 2;
pub const DDENUMSURFACES_NOMATCH: i32 = 4;
pub const DDENUM_ATTACHEDSECONDARYDEVICES: i32 = 1;
pub const DDENUM_DETACHEDSECONDARYDEVICES: i32 = 2;
pub const DDENUM_NONDISPLAYDEVICES: i32 = 4;
pub const DDERR_ALREADYINITIALIZED: windows_sys::core::HRESULT = 0x88760005_u32 as _;
pub const DDERR_BLTFASTCANTCLIP: windows_sys::core::HRESULT = 0x8876023E_u32 as _;
pub const DDERR_CANNOTATTACHSURFACE: windows_sys::core::HRESULT = 0x8876000A_u32 as _;
pub const DDERR_CANNOTDETACHSURFACE: windows_sys::core::HRESULT = 0x88760014_u32 as _;
pub const DDERR_CANTCREATEDC: windows_sys::core::HRESULT = 0x88760249_u32 as _;
pub const DDERR_CANTDUPLICATE: windows_sys::core::HRESULT = 0x88760247_u32 as _;
pub const DDERR_CANTLOCKSURFACE: windows_sys::core::HRESULT = 0x887601B3_u32 as _;
pub const DDERR_CANTPAGELOCK: windows_sys::core::HRESULT = 0x88760280_u32 as _;
pub const DDERR_CANTPAGEUNLOCK: windows_sys::core::HRESULT = 0x88760294_u32 as _;
pub const DDERR_CLIPPERISUSINGHWND: windows_sys::core::HRESULT = 0x88760237_u32 as _;
pub const DDERR_COLORKEYNOTSET: windows_sys::core::HRESULT = 0x88760190_u32 as _;
pub const DDERR_CURRENTLYNOTAVAIL: windows_sys::core::HRESULT = 0x88760028_u32 as _;
pub const DDERR_D3DNOTINITIALIZED: windows_sys::core::HRESULT = 0x887602B6_u32 as _;
pub const DDERR_DCALREADYCREATED: windows_sys::core::HRESULT = 0x8876026C_u32 as _;
pub const DDERR_DDSCAPSCOMPLEXREQUIRED: windows_sys::core::HRESULT = 0x8876021E_u32 as _;
pub const DDERR_DEVICEDOESNTOWNSURFACE: windows_sys::core::HRESULT = 0x887602BB_u32 as _;
pub const DDERR_DIRECTDRAWALREADYCREATED: windows_sys::core::HRESULT = 0x88760232_u32 as _;
pub const DDERR_EXCEPTION: windows_sys::core::HRESULT = 0x88760037_u32 as _;
pub const DDERR_EXCLUSIVEMODEALREADYSET: windows_sys::core::HRESULT = 0x88760245_u32 as _;
pub const DDERR_EXPIRED: windows_sys::core::HRESULT = 0x887602B3_u32 as _;
pub const DDERR_GENERIC: windows_sys::core::HRESULT = 0x80004005_u32 as _;
pub const DDERR_HEIGHTALIGN: windows_sys::core::HRESULT = 0x8876005A_u32 as _;
pub const DDERR_HWNDALREADYSET: windows_sys::core::HRESULT = 0x8876023B_u32 as _;
pub const DDERR_HWNDSUBCLASSED: windows_sys::core::HRESULT = 0x8876023A_u32 as _;
pub const DDERR_IMPLICITLYCREATED: windows_sys::core::HRESULT = 0x8876024C_u32 as _;
pub const DDERR_INCOMPATIBLEPRIMARY: windows_sys::core::HRESULT = 0x8876005F_u32 as _;
pub const DDERR_INVALIDCAPS: windows_sys::core::HRESULT = 0x88760064_u32 as _;
pub const DDERR_INVALIDCLIPLIST: windows_sys::core::HRESULT = 0x8876006E_u32 as _;
pub const DDERR_INVALIDDIRECTDRAWGUID: windows_sys::core::HRESULT = 0x88760231_u32 as _;
pub const DDERR_INVALIDMODE: windows_sys::core::HRESULT = 0x88760078_u32 as _;
pub const DDERR_INVALIDOBJECT: windows_sys::core::HRESULT = 0x88760082_u32 as _;
pub const DDERR_INVALIDPARAMS: windows_sys::core::HRESULT = 0x80070057_u32 as _;
pub const DDERR_INVALIDPIXELFORMAT: windows_sys::core::HRESULT = 0x88760091_u32 as _;
pub const DDERR_INVALIDPOSITION: windows_sys::core::HRESULT = 0x88760243_u32 as _;
pub const DDERR_INVALIDRECT: windows_sys::core::HRESULT = 0x88760096_u32 as _;
pub const DDERR_INVALIDSTREAM: windows_sys::core::HRESULT = 0x88760209_u32 as _;
pub const DDERR_INVALIDSURFACETYPE: windows_sys::core::HRESULT = 0x88760250_u32 as _;
pub const DDERR_LOCKEDSURFACES: windows_sys::core::HRESULT = 0x887600A0_u32 as _;
pub const DDERR_MOREDATA: windows_sys::core::HRESULT = 0x887602B2_u32 as _;
pub const DDERR_NEWMODE: windows_sys::core::HRESULT = 0x887602B5_u32 as _;
pub const DDERR_NO3D: windows_sys::core::HRESULT = 0x887600AA_u32 as _;
pub const DDERR_NOALPHAHW: windows_sys::core::HRESULT = 0x887600B4_u32 as _;
pub const DDERR_NOBLTHW: windows_sys::core::HRESULT = 0x8876023F_u32 as _;
pub const DDERR_NOCLIPLIST: windows_sys::core::HRESULT = 0x887600CD_u32 as _;
pub const DDERR_NOCLIPPERATTACHED: windows_sys::core::HRESULT = 0x88760238_u32 as _;
pub const DDERR_NOCOLORCONVHW: windows_sys::core::HRESULT = 0x887600D2_u32 as _;
pub const DDERR_NOCOLORKEY: windows_sys::core::HRESULT = 0x887600D7_u32 as _;
pub const DDERR_NOCOLORKEYHW: windows_sys::core::HRESULT = 0x887600DC_u32 as _;
pub const DDERR_NOCOOPERATIVELEVELSET: windows_sys::core::HRESULT = 0x887600D4_u32 as _;
pub const DDERR_NODC: windows_sys::core::HRESULT = 0x8876024A_u32 as _;
pub const DDERR_NODDROPSHW: windows_sys::core::HRESULT = 0x88760240_u32 as _;
pub const DDERR_NODIRECTDRAWHW: windows_sys::core::HRESULT = 0x88760233_u32 as _;
pub const DDERR_NODIRECTDRAWSUPPORT: windows_sys::core::HRESULT = 0x887600DE_u32 as _;
pub const DDERR_NODRIVERSUPPORT: windows_sys::core::HRESULT = 0x887602B9_u32 as _;
pub const DDERR_NOEMULATION: windows_sys::core::HRESULT = 0x88760235_u32 as _;
pub const DDERR_NOEXCLUSIVEMODE: windows_sys::core::HRESULT = 0x887600E1_u32 as _;
pub const DDERR_NOFLIPHW: windows_sys::core::HRESULT = 0x887600E6_u32 as _;
pub const DDERR_NOFOCUSWINDOW: windows_sys::core::HRESULT = 0x8876025A_u32 as _;
pub const DDERR_NOGDI: windows_sys::core::HRESULT = 0x887600F0_u32 as _;
pub const DDERR_NOHWND: windows_sys::core::HRESULT = 0x88760239_u32 as _;
pub const DDERR_NOMIPMAPHW: windows_sys::core::HRESULT = 0x8876024F_u32 as _;
pub const DDERR_NOMIRRORHW: windows_sys::core::HRESULT = 0x887600FA_u32 as _;
pub const DDERR_NOMONITORINFORMATION: windows_sys::core::HRESULT = 0x887602B8_u32 as _;
pub const DDERR_NONONLOCALVIDMEM: windows_sys::core::HRESULT = 0x88760276_u32 as _;
pub const DDERR_NOOPTIMIZEHW: windows_sys::core::HRESULT = 0x88760258_u32 as _;
pub const DDERR_NOOVERLAYDEST: windows_sys::core::HRESULT = 0x88760242_u32 as _;
pub const DDERR_NOOVERLAYHW: windows_sys::core::HRESULT = 0x88760104_u32 as _;
pub const DDERR_NOPALETTEATTACHED: windows_sys::core::HRESULT = 0x8876023C_u32 as _;
pub const DDERR_NOPALETTEHW: windows_sys::core::HRESULT = 0x8876023D_u32 as _;
pub const DDERR_NORASTEROPHW: windows_sys::core::HRESULT = 0x88760118_u32 as _;
pub const DDERR_NOROTATIONHW: windows_sys::core::HRESULT = 0x88760122_u32 as _;
pub const DDERR_NOSTEREOHARDWARE: windows_sys::core::HRESULT = 0x887600B5_u32 as _;
pub const DDERR_NOSTRETCHHW: windows_sys::core::HRESULT = 0x88760136_u32 as _;
pub const DDERR_NOSURFACELEFT: windows_sys::core::HRESULT = 0x887600B6_u32 as _;
pub const DDERR_NOT4BITCOLOR: windows_sys::core::HRESULT = 0x8876013C_u32 as _;
pub const DDERR_NOT4BITCOLORINDEX: windows_sys::core::HRESULT = 0x8876013D_u32 as _;
pub const DDERR_NOT8BITCOLOR: windows_sys::core::HRESULT = 0x88760140_u32 as _;
pub const DDERR_NOTAOVERLAYSURFACE: windows_sys::core::HRESULT = 0x88760244_u32 as _;
pub const DDERR_NOTEXTUREHW: windows_sys::core::HRESULT = 0x8876014A_u32 as _;
pub const DDERR_NOTFLIPPABLE: windows_sys::core::HRESULT = 0x88760246_u32 as _;
pub const DDERR_NOTFOUND: windows_sys::core::HRESULT = 0x887600FF_u32 as _;
pub const DDERR_NOTINITIALIZED: windows_sys::core::HRESULT = 0x800401F0_u32 as _;
pub const DDERR_NOTLOADED: windows_sys::core::HRESULT = 0x88760259_u32 as _;
pub const DDERR_NOTLOCKED: windows_sys::core::HRESULT = 0x88760248_u32 as _;
pub const DDERR_NOTONMIPMAPSUBLEVEL: windows_sys::core::HRESULT = 0x8876025B_u32 as _;
pub const DDERR_NOTPAGELOCKED: windows_sys::core::HRESULT = 0x887602A8_u32 as _;
pub const DDERR_NOTPALETTIZED: windows_sys::core::HRESULT = 0x8876024D_u32 as _;
pub const DDERR_NOVSYNCHW: windows_sys::core::HRESULT = 0x8876014F_u32 as _;
pub const DDERR_NOZBUFFERHW: windows_sys::core::HRESULT = 0x88760154_u32 as _;
pub const DDERR_NOZOVERLAYHW: windows_sys::core::HRESULT = 0x8876015E_u32 as _;
pub const DDERR_OUTOFCAPS: windows_sys::core::HRESULT = 0x88760168_u32 as _;
pub const DDERR_OUTOFMEMORY: windows_sys::core::HRESULT = 0x8007000E_u32 as _;
pub const DDERR_OUTOFVIDEOMEMORY: windows_sys::core::HRESULT = 0x8876017C_u32 as _;
pub const DDERR_OVERLAPPINGRECTS: windows_sys::core::HRESULT = 0x8876010E_u32 as _;
pub const DDERR_OVERLAYCANTCLIP: windows_sys::core::HRESULT = 0x8876017E_u32 as _;
pub const DDERR_OVERLAYCOLORKEYONLYONEACTIVE: windows_sys::core::HRESULT = 0x88760180_u32 as _;
pub const DDERR_OVERLAYNOTVISIBLE: windows_sys::core::HRESULT = 0x88760241_u32 as _;
pub const DDERR_PALETTEBUSY: windows_sys::core::HRESULT = 0x88760183_u32 as _;
pub const DDERR_PRIMARYSURFACEALREADYEXISTS: windows_sys::core::HRESULT = 0x88760234_u32 as _;
pub const DDERR_REGIONTOOSMALL: windows_sys::core::HRESULT = 0x88760236_u32 as _;
pub const DDERR_SURFACEALREADYATTACHED: windows_sys::core::HRESULT = 0x8876019A_u32 as _;
pub const DDERR_SURFACEALREADYDEPENDENT: windows_sys::core::HRESULT = 0x887601A4_u32 as _;
pub const DDERR_SURFACEBUSY: windows_sys::core::HRESULT = 0x887601AE_u32 as _;
pub const DDERR_SURFACEISOBSCURED: windows_sys::core::HRESULT = 0x887601B8_u32 as _;
pub const DDERR_SURFACELOST: windows_sys::core::HRESULT = 0x887601C2_u32 as _;
pub const DDERR_SURFACENOTATTACHED: windows_sys::core::HRESULT = 0x887601CC_u32 as _;
pub const DDERR_TESTFINISHED: windows_sys::core::HRESULT = 0x887602B4_u32 as _;
pub const DDERR_TOOBIGHEIGHT: windows_sys::core::HRESULT = 0x887601D6_u32 as _;
pub const DDERR_TOOBIGSIZE: windows_sys::core::HRESULT = 0x887601E0_u32 as _;
pub const DDERR_TOOBIGWIDTH: windows_sys::core::HRESULT = 0x887601EA_u32 as _;
pub const DDERR_UNSUPPORTED: windows_sys::core::HRESULT = 0x80004001_u32 as _;
pub const DDERR_UNSUPPORTEDFORMAT: windows_sys::core::HRESULT = 0x887601FE_u32 as _;
pub const DDERR_UNSUPPORTEDMASK: windows_sys::core::HRESULT = 0x88760208_u32 as _;
pub const DDERR_UNSUPPORTEDMODE: windows_sys::core::HRESULT = 0x8876024E_u32 as _;
pub const DDERR_VERTICALBLANKINPROGRESS: windows_sys::core::HRESULT = 0x88760219_u32 as _;
pub const DDERR_VIDEONOTACTIVE: windows_sys::core::HRESULT = 0x887602B7_u32 as _;
pub const DDERR_WASSTILLDRAWING: windows_sys::core::HRESULT = 0x8876021C_u32 as _;
pub const DDERR_WRONGMODE: windows_sys::core::HRESULT = 0x8876024B_u32 as _;
pub const DDERR_XALIGN: windows_sys::core::HRESULT = 0x88760230_u32 as _;
pub const DDFLIP_DONOTWAIT: i32 = 32;
pub const DDFLIP_EVEN: i32 = 2;
pub const DDFLIP_INTERVAL2: i32 = 33554432;
pub const DDFLIP_INTERVAL3: i32 = 50331648;
pub const DDFLIP_INTERVAL4: i32 = 67108864;
pub const DDFLIP_NOVSYNC: i32 = 8;
pub const DDFLIP_ODD: i32 = 4;
pub const DDFLIP_STEREO: i32 = 16;
pub const DDFLIP_WAIT: i32 = 1;
pub const DDFXALPHACAPS_BLTALPHAEDGEBLEND: i32 = 1;
pub const DDFXALPHACAPS_BLTALPHAPIXELS: i32 = 2;
pub const DDFXALPHACAPS_BLTALPHAPIXELSNEG: i32 = 4;
pub const DDFXALPHACAPS_BLTALPHASURFACES: i32 = 8;
pub const DDFXALPHACAPS_BLTALPHASURFACESNEG: i32 = 16;
pub const DDFXALPHACAPS_OVERLAYALPHAEDGEBLEND: i32 = 32;
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELS: i32 = 64;
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELSNEG: i32 = 128;
pub const DDFXALPHACAPS_OVERLAYALPHASURFACES: i32 = 256;
pub const DDFXALPHACAPS_OVERLAYALPHASURFACESNEG: i32 = 512;
pub const DDFXCAPS_BLTALPHA: i32 = 1;
pub const DDFXCAPS_BLTARITHSTRETCHY: i32 = 32;
pub const DDFXCAPS_BLTARITHSTRETCHYN: i32 = 16;
pub const DDFXCAPS_BLTFILTER: i32 = 32;
pub const DDFXCAPS_BLTMIRRORLEFTRIGHT: i32 = 64;
pub const DDFXCAPS_BLTMIRRORUPDOWN: i32 = 128;
pub const DDFXCAPS_BLTROTATION: i32 = 256;
pub const DDFXCAPS_BLTROTATION90: i32 = 512;
pub const DDFXCAPS_BLTSHRINKX: i32 = 1024;
pub const DDFXCAPS_BLTSHRINKXN: i32 = 2048;
pub const DDFXCAPS_BLTSHRINKY: i32 = 4096;
pub const DDFXCAPS_BLTSHRINKYN: i32 = 8192;
pub const DDFXCAPS_BLTSTRETCHX: i32 = 16384;
pub const DDFXCAPS_BLTSTRETCHXN: i32 = 32768;
pub const DDFXCAPS_BLTSTRETCHY: i32 = 65536;
pub const DDFXCAPS_BLTSTRETCHYN: i32 = 131072;
pub const DDFXCAPS_OVERLAYALPHA: i32 = 4;
pub const DDFXCAPS_OVERLAYARITHSTRETCHY: i32 = 262144;
pub const DDFXCAPS_OVERLAYARITHSTRETCHYN: i32 = 8;
pub const DDFXCAPS_OVERLAYDEINTERLACE: i32 = 536870912;
pub const DDFXCAPS_OVERLAYFILTER: i32 = 262144;
pub const DDFXCAPS_OVERLAYMIRRORLEFTRIGHT: i32 = 134217728;
pub const DDFXCAPS_OVERLAYMIRRORUPDOWN: i32 = 268435456;
pub const DDFXCAPS_OVERLAYSHRINKX: i32 = 524288;
pub const DDFXCAPS_OVERLAYSHRINKXN: i32 = 1048576;
pub const DDFXCAPS_OVERLAYSHRINKY: i32 = 2097152;
pub const DDFXCAPS_OVERLAYSHRINKYN: i32 = 4194304;
pub const DDFXCAPS_OVERLAYSTRETCHX: i32 = 8388608;
pub const DDFXCAPS_OVERLAYSTRETCHXN: i32 = 16777216;
pub const DDFXCAPS_OVERLAYSTRETCHY: i32 = 33554432;
pub const DDFXCAPS_OVERLAYSTRETCHYN: i32 = 67108864;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDGAMMARAMP {
    pub red: [super::WORD; 256],
    pub green: [super::WORD; 256],
    pub blue: [super::WORD; 256],
}
#[cfg(feature = "shared")]
impl Default for DDGAMMARAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDGBS_CANBLT: i32 = 1;
pub const DDGBS_ISBLTDONE: i32 = 2;
pub const DDGDI_GETHOSTIDENTIFIER: i32 = 1;
pub const DDGFS_CANFLIP: i32 = 1;
pub const DDGFS_ISFLIPDONE: i32 = 2;
pub const DDLOCK_DISCARDCONTENTS: i32 = 8192;
pub const DDLOCK_DONOTWAIT: i32 = 16384;
pub const DDLOCK_EVENT: i32 = 2;
pub const DDLOCK_HASVOLUMETEXTUREBOXRECT: i32 = 32768;
pub const DDLOCK_NODIRTYUPDATE: i32 = 65536;
pub const DDLOCK_NOOVERWRITE: i32 = 4096;
pub const DDLOCK_NOSYSLOCK: i32 = 2048;
pub const DDLOCK_OKTOSWAP: i32 = 8192;
pub const DDLOCK_READONLY: i32 = 16;
pub const DDLOCK_SURFACEMEMORYPTR: i32 = 0;
pub const DDLOCK_WAIT: i32 = 1;
pub const DDLOCK_WRITEONLY: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDOPTSURFACEDESC {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub ddSCaps: DDSCAPS2,
    pub ddOSCaps: DDOSCAPS,
    pub guid: windows_sys::core::GUID,
    pub dwCompressionRatio: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for DDOPTSURFACEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DDOSCAPS {
    pub dwCaps: super::DWORD,
}
pub const DDOSDCAPS_MONOLITHICMIPMAP: i32 = 4;
pub const DDOSDCAPS_OPTCOMPRESSED: i32 = 1;
pub const DDOSDCAPS_OPTREORDERED: i32 = 2;
pub const DDOSDCAPS_VALIDOSCAPS: i32 = 7;
pub const DDOSDCAPS_VALIDSCAPS: i32 = 805324800;
pub const DDOSD_ALL: i32 = 15;
pub const DDOSD_COMPRESSION_RATIO: i32 = 2;
pub const DDOSD_GUID: i32 = 1;
pub const DDOSD_OSCAPS: i32 = 8;
pub const DDOSD_SCAPS: i32 = 4;
pub const DDOVERFX_ARITHSTRETCHY: i32 = 1;
pub const DDOVERFX_DEINTERLACE: i32 = 8;
pub const DDOVERFX_MIRRORLEFTRIGHT: i32 = 2;
pub const DDOVERFX_MIRRORUPDOWN: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDOVERLAYFX {
    pub dwSize: super::DWORD,
    pub dwAlphaEdgeBlendBitDepth: super::DWORD,
    pub dwAlphaEdgeBlend: super::DWORD,
    pub dwReserved: super::DWORD,
    pub dwAlphaDestConstBitDepth: super::DWORD,
    pub Anonymous: DDOVERLAYFX_0,
    pub dwAlphaSrcConstBitDepth: super::DWORD,
    pub Anonymous2: DDOVERLAYFX_1,
    pub dckDestColorkey: DDCOLORKEY,
    pub dckSrcColorkey: DDCOLORKEY,
    pub dwDDFX: super::DWORD,
    pub dwFlags: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for DDOVERLAYFX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDOVERLAYFX_0 {
    pub dwAlphaDestConst: super::DWORD,
    pub lpDDSAlphaDest: LPDIRECTDRAWSURFACE,
}
#[cfg(feature = "shared")]
impl Default for DDOVERLAYFX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDOVERLAYFX_1 {
    pub dwAlphaSrcConst: super::DWORD,
    pub lpDDSAlphaSrc: LPDIRECTDRAWSURFACE,
}
#[cfg(feature = "shared")]
impl Default for DDOVERLAYFX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDOVERZ_INSERTINBACKOF: i32 = 5;
pub const DDOVERZ_INSERTINFRONTOF: i32 = 4;
pub const DDOVERZ_MOVEBACKWARD: i32 = 3;
pub const DDOVERZ_MOVEFORWARD: i32 = 2;
pub const DDOVERZ_SENDTOBACK: i32 = 1;
pub const DDOVERZ_SENDTOFRONT: i32 = 0;
pub const DDOVER_ADDDIRTYRECT: i32 = 32768;
pub const DDOVER_ALPHADEST: i32 = 1;
pub const DDOVER_ALPHADESTCONSTOVERRIDE: i32 = 2;
pub const DDOVER_ALPHADESTNEG: i32 = 4;
pub const DDOVER_ALPHADESTSURFACEOVERRIDE: i32 = 8;
pub const DDOVER_ALPHAEDGEBLEND: i32 = 16;
pub const DDOVER_ALPHASRC: i32 = 32;
pub const DDOVER_ALPHASRCCONSTOVERRIDE: i32 = 64;
pub const DDOVER_ALPHASRCNEG: i32 = 128;
pub const DDOVER_ALPHASRCSURFACEOVERRIDE: i32 = 256;
pub const DDOVER_ARGBSCALEFACTORS: i32 = 33554432;
pub const DDOVER_AUTOFLIP: i32 = 1048576;
pub const DDOVER_BOB: i32 = 2097152;
pub const DDOVER_BOBHARDWARE: i32 = 16777216;
pub const DDOVER_DDFX: i32 = 524288;
pub const DDOVER_DEGRADEARGBSCALING: i32 = 67108864;
pub const DDOVER_HIDE: i32 = 512;
pub const DDOVER_INTERLEAVED: i32 = 8388608;
pub const DDOVER_KEYDEST: i32 = 1024;
pub const DDOVER_KEYDESTOVERRIDE: i32 = 2048;
pub const DDOVER_KEYSRC: i32 = 4096;
pub const DDOVER_KEYSRCOVERRIDE: i32 = 8192;
pub const DDOVER_OVERRIDEBOBWEAVE: i32 = 4194304;
pub const DDOVER_REFRESHALL: i32 = 131072;
pub const DDOVER_REFRESHDIRTYRECTS: i32 = 65536;
pub const DDOVER_SHOW: i32 = 16384;
pub const DDPCAPS_1BIT: i32 = 256;
pub const DDPCAPS_2BIT: i32 = 512;
pub const DDPCAPS_4BIT: i32 = 1;
pub const DDPCAPS_8BIT: i32 = 4;
pub const DDPCAPS_8BITENTRIES: i32 = 2;
pub const DDPCAPS_ALLOW256: i32 = 64;
pub const DDPCAPS_ALPHA: i32 = 1024;
pub const DDPCAPS_INITIALIZE: i32 = 0;
pub const DDPCAPS_PRIMARYSURFACE: i32 = 16;
pub const DDPCAPS_PRIMARYSURFACELEFT: i32 = 32;
pub const DDPCAPS_VSYNC: i32 = 128;
pub const DDPF_ALPHA: i32 = 2;
pub const DDPF_ALPHAPIXELS: i32 = 1;
pub const DDPF_ALPHAPREMULT: i32 = 32768;
pub const DDPF_BUMPDUDV: i32 = 524288;
pub const DDPF_BUMPLUMINANCE: i32 = 262144;
pub const DDPF_COMPRESSED: i32 = 128;
pub const DDPF_LUMINANCE: i32 = 131072;
pub const DDPF_PALETTEINDEXED1: i32 = 2048;
pub const DDPF_PALETTEINDEXED2: i32 = 4096;
pub const DDPF_PALETTEINDEXED4: i32 = 8;
pub const DDPF_PALETTEINDEXED8: i32 = 32;
pub const DDPF_PALETTEINDEXEDTO8: i32 = 16;
pub const DDPF_RGB: i32 = 64;
pub const DDPF_RGBTOYUV: i32 = 256;
pub const DDPF_STENCILBUFFER: i32 = 16384;
pub const DDPF_YUV: i32 = 512;
pub const DDPF_ZBUFFER: i32 = 1024;
pub const DDPF_ZPIXELS: i32 = 8192;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DDRGBA {
    pub red: super::BYTE,
    pub green: super::BYTE,
    pub blue: super::BYTE,
    pub alpha: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DDSCAPS {
    pub dwCaps: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDSCAPS2 {
    pub dwCaps: super::DWORD,
    pub dwCaps2: super::DWORD,
    pub dwCaps3: super::DWORD,
    pub Anonymous: DDSCAPS2_0,
}
#[cfg(feature = "shared")]
impl Default for DDSCAPS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDSCAPS2_0 {
    pub dwCaps4: super::DWORD,
    pub dwVolumeDepth: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for DDSCAPS2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDSCAPS2_ADDITIONALPRIMARY: u32 = 2147483648;
pub const DDSCAPS2_CUBEMAP: i32 = 512;
pub const DDSCAPS2_CUBEMAP_ALLFACES: i32 = 64512;
pub const DDSCAPS2_CUBEMAP_NEGATIVEX: i32 = 2048;
pub const DDSCAPS2_CUBEMAP_NEGATIVEY: i32 = 8192;
pub const DDSCAPS2_CUBEMAP_NEGATIVEZ: i32 = 32768;
pub const DDSCAPS2_CUBEMAP_POSITIVEX: i32 = 1024;
pub const DDSCAPS2_CUBEMAP_POSITIVEY: i32 = 4096;
pub const DDSCAPS2_CUBEMAP_POSITIVEZ: i32 = 16384;
pub const DDSCAPS2_D3DTEXTUREMANAGE: i32 = 131072;
pub const DDSCAPS2_DISCARDBACKBUFFER: i32 = 268435456;
pub const DDSCAPS2_DONOTPERSIST: i32 = 262144;
pub const DDSCAPS2_ENABLEALPHACHANNEL: i32 = 536870912;
pub const DDSCAPS2_EXTENDEDFORMATPRIMARY: i32 = 1073741824;
pub const DDSCAPS2_HARDWAREDEINTERLACE: i32 = 0;
pub const DDSCAPS2_HINTANTIALIASING: i32 = 256;
pub const DDSCAPS2_HINTDYNAMIC: i32 = 4;
pub const DDSCAPS2_HINTSTATIC: i32 = 8;
pub const DDSCAPS2_MIPMAPSUBLEVEL: i32 = 65536;
pub const DDSCAPS2_NOTUSERLOCKABLE: i32 = 4194304;
pub const DDSCAPS2_NPATCHES: i32 = 33554432;
pub const DDSCAPS2_OPAQUE: i32 = 128;
pub const DDSCAPS2_POINTS: i32 = 8388608;
pub const DDSCAPS2_RESERVED1: i32 = 32;
pub const DDSCAPS2_RESERVED2: i32 = 64;
pub const DDSCAPS2_RESERVED3: i32 = 67108864;
pub const DDSCAPS2_RESERVED4: i32 = 2;
pub const DDSCAPS2_RTPATCHES: i32 = 16777216;
pub const DDSCAPS2_STEREOSURFACELEFT: i32 = 524288;
pub const DDSCAPS2_TEXTUREMANAGE: i32 = 16;
pub const DDSCAPS2_VOLUME: i32 = 2097152;
pub const DDSCAPS3_AUTOGENMIPMAP: i32 = 2048;
pub const DDSCAPS3_CREATESHAREDRESOURCE: i32 = 8192;
pub const DDSCAPS3_DMAP: i32 = 4096;
pub const DDSCAPS3_LIGHTWEIGHTMIPMAP: i32 = 1024;
pub const DDSCAPS3_MULTISAMPLE_MASK: i32 = 31;
pub const DDSCAPS3_MULTISAMPLE_QUALITY_MASK: i32 = 224;
pub const DDSCAPS3_MULTISAMPLE_QUALITY_SHIFT: i32 = 5;
pub const DDSCAPS3_OPENSHAREDRESOURCE: i32 = 32768;
pub const DDSCAPS3_READONLYRESOURCE: i32 = 16384;
pub const DDSCAPS3_RESERVED1: i32 = 256;
pub const DDSCAPS3_RESERVED2: i32 = 512;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DDSCAPSEX {
    pub dwCaps2: super::DWORD,
    pub dwCaps3: super::DWORD,
    pub Anonymous: DDSCAPSEX_0,
}
#[cfg(feature = "shared")]
impl Default for DDSCAPSEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DDSCAPSEX_0 {
    pub dwCaps4: super::DWORD,
    pub dwVolumeDepth: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for DDSCAPSEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDSCAPS_3DDEVICE: i32 = 8192;
pub const DDSCAPS_ALLOCONLOAD: i32 = 67108864;
pub const DDSCAPS_ALPHA: i32 = 2;
pub const DDSCAPS_BACKBUFFER: i32 = 4;
pub const DDSCAPS_COMPLEX: i32 = 8;
pub const DDSCAPS_FLIP: i32 = 16;
pub const DDSCAPS_FRONTBUFFER: i32 = 32;
pub const DDSCAPS_HWCODEC: i32 = 1048576;
pub const DDSCAPS_LIVEVIDEO: i32 = 524288;
pub const DDSCAPS_LOCALVIDMEM: i32 = 268435456;
pub const DDSCAPS_MIPMAP: i32 = 4194304;
pub const DDSCAPS_MODEX: i32 = 2097152;
pub const DDSCAPS_NONLOCALVIDMEM: i32 = 536870912;
pub const DDSCAPS_OFFSCREENPLAIN: i32 = 64;
pub const DDSCAPS_OPTIMIZED: u32 = 2147483648;
pub const DDSCAPS_OVERLAY: i32 = 128;
pub const DDSCAPS_OWNDC: i32 = 262144;
pub const DDSCAPS_PALETTE: i32 = 256;
pub const DDSCAPS_PRIMARYSURFACE: i32 = 512;
pub const DDSCAPS_PRIMARYSURFACELEFT: i32 = 0;
pub const DDSCAPS_RESERVED1: i32 = 1;
pub const DDSCAPS_RESERVED2: i32 = 8388608;
pub const DDSCAPS_RESERVED3: i32 = 1024;
pub const DDSCAPS_STANDARDVGAMODE: i32 = 1073741824;
pub const DDSCAPS_SYSTEMMEMORY: i32 = 2048;
pub const DDSCAPS_TEXTURE: i32 = 4096;
pub const DDSCAPS_VIDEOMEMORY: i32 = 16384;
pub const DDSCAPS_VIDEOPORT: i32 = 134217728;
pub const DDSCAPS_VISIBLE: i32 = 32768;
pub const DDSCAPS_WRITEONLY: i32 = 65536;
pub const DDSCAPS_ZBUFFER: i32 = 131072;
pub const DDSCL_ALLOWMODEX: i32 = 64;
pub const DDSCL_ALLOWREBOOT: i32 = 2;
pub const DDSCL_CREATEDEVICEWINDOW: i32 = 512;
pub const DDSCL_EXCLUSIVE: i32 = 16;
pub const DDSCL_FPUPRESERVE: i32 = 4096;
pub const DDSCL_FPUSETUP: i32 = 2048;
pub const DDSCL_FULLSCREEN: i32 = 1;
pub const DDSCL_MULTITHREADED: i32 = 1024;
pub const DDSCL_NORMAL: i32 = 8;
pub const DDSCL_NOWINDOWCHANGES: i32 = 4;
pub const DDSCL_SETDEVICEWINDOW: i32 = 256;
pub const DDSCL_SETFOCUSWINDOW: i32 = 128;
pub const DDSDM_STANDARDVGAMODE: i32 = 1;
pub const DDSD_ALL: i32 = 16775662;
pub const DDSD_ALPHABITDEPTH: i32 = 128;
pub const DDSD_BACKBUFFERCOUNT: i32 = 32;
pub const DDSD_CAPS: i32 = 1;
pub const DDSD_CKDESTBLT: i32 = 16384;
pub const DDSD_CKDESTOVERLAY: i32 = 8192;
pub const DDSD_CKSRCBLT: i32 = 65536;
pub const DDSD_CKSRCOVERLAY: i32 = 32768;
pub const DDSD_DEPTH: i32 = 8388608;
pub const DDSD_FVF: i32 = 2097152;
pub const DDSD_HEIGHT: i32 = 2;
pub const DDSD_LINEARSIZE: i32 = 524288;
pub const DDSD_LPSURFACE: i32 = 2048;
pub const DDSD_MIPMAPCOUNT: i32 = 131072;
pub const DDSD_PITCH: i32 = 8;
pub const DDSD_PIXELFORMAT: i32 = 4096;
pub const DDSD_REFRESHRATE: i32 = 262144;
pub const DDSD_SRCVBHANDLE: i32 = 4194304;
pub const DDSD_TEXTURESTAGE: i32 = 1048576;
pub const DDSD_WIDTH: i32 = 4;
pub const DDSD_ZBUFFERBITDEPTH: i32 = 64;
pub const DDSGR_CALIBRATE: i32 = 1;
pub const DDSMT_ISTESTREQUIRED: i32 = 1;
pub const DDSPD_IUNKNOWNPOINTER: i32 = 1;
pub const DDSPD_VOLATILE: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct DDSURFACEDESC {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwHeight: super::DWORD,
    pub dwWidth: super::DWORD,
    pub Anonymous: DDSURFACEDESC_0,
    pub dwBackBufferCount: super::DWORD,
    pub Anonymous2: DDSURFACEDESC_1,
    pub dwAlphaBitDepth: super::DWORD,
    pub dwReserved: super::DWORD,
    pub lpSurface: super::LPVOID,
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub ddpfPixelFormat: super::DDPIXELFORMAT,
    pub ddsCaps: DDSCAPS,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC_0 {
    pub lPitch: super::LONG,
    pub dwLinearSize: super::DWORD,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC_1 {
    pub dwMipMapCount: super::DWORD,
    pub dwZBufferBitDepth: super::DWORD,
    pub dwRefreshRate: super::DWORD,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct DDSURFACEDESC2 {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwHeight: super::DWORD,
    pub dwWidth: super::DWORD,
    pub Anonymous: DDSURFACEDESC2_0,
    pub Anonymous2: DDSURFACEDESC2_1,
    pub Anonymous3: DDSURFACEDESC2_2,
    pub dwAlphaBitDepth: super::DWORD,
    pub dwReserved: super::DWORD,
    pub lpSurface: super::LPVOID,
    pub Anonymous4: DDSURFACEDESC2_3,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub Anonymous5: DDSURFACEDESC2_4,
    pub ddsCaps: DDSCAPS2,
    pub dwTextureStage: super::DWORD,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_0 {
    pub lPitch: super::LONG,
    pub dwLinearSize: super::DWORD,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_1 {
    pub dwBackBufferCount: super::DWORD,
    pub dwDepth: super::DWORD,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_2 {
    pub dwMipMapCount: super::DWORD,
    pub dwRefreshRate: super::DWORD,
    pub dwSrcVBHandle: super::DWORD,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_3 {
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub dwEmptyFaceColor: super::DWORD,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC2_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_4 {
    pub ddpfPixelFormat: super::DDPIXELFORMAT,
    pub dwFVF: super::DWORD,
}
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
impl Default for DDSURFACEDESC2_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDSVCAPS_RESERVED1: i32 = 1;
pub const DDSVCAPS_RESERVED2: i32 = 2;
pub const DDSVCAPS_RESERVED3: i32 = 4;
pub const DDSVCAPS_RESERVED4: i32 = 8;
pub const DDSVCAPS_STEREOSEQUENTIAL: i32 = 16;
pub const DDWAITVB_BLOCKBEGIN: i32 = 1;
pub const DDWAITVB_BLOCKBEGINEVENT: i32 = 2;
pub const DDWAITVB_BLOCKEND: i32 = 4;
pub const DD_FALSE: windows_sys::core::HRESULT = 0x1_u32 as _;
pub const DD_OK: windows_sys::core::HRESULT = 0x0_u32 as _;
pub const DD_ROP_SPACE: i32 = 8;
pub const DIRECTDRAW_VERSION: i32 = 1792;
#[cfg(feature = "shared")]
pub const FOURCC_DXT1: super::DWORD = 827611204;
#[cfg(feature = "shared")]
pub const FOURCC_DXT2: super::DWORD = 844388420;
#[cfg(feature = "shared")]
pub const FOURCC_DXT3: super::DWORD = 861165636;
#[cfg(feature = "shared")]
pub const FOURCC_DXT4: super::DWORD = 877942852;
#[cfg(feature = "shared")]
pub const FOURCC_DXT5: super::DWORD = 894720068;
pub const IID_IDirectDraw: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c14db80_a733_11ce_a521_0020af0be560);
pub const IID_IDirectDraw2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3a6f3e0_2b43_11cf_a2de_00aa00b93356);
pub const IID_IDirectDraw4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9c59509a_39bd_11d1_8c4a_00c04fd930c5);
pub const IID_IDirectDraw7: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x15e65ec0_3b9c_11d2_b92f_00609797ea5b);
pub const IID_IDirectDrawClipper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c14db85_a733_11ce_a521_0020af0be560);
pub const IID_IDirectDrawColorControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b9f0ee0_0d7e_11d0_9b06_00a0c903a3b8);
pub const IID_IDirectDrawGammaControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69c11c3e_b46b_11d1_ad7a_00c04fc29b4e);
pub const IID_IDirectDrawPalette: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c14db84_a733_11ce_a521_0020af0be560);
pub const IID_IDirectDrawSurface: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c14db81_a733_11ce_a521_0020af0be560);
pub const IID_IDirectDrawSurface2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57805885_6eec_11cf_9441_a82303c10e27);
pub const IID_IDirectDrawSurface3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xda044e00_69b2_11d0_a1d5_00aa00b8dfbb);
pub const IID_IDirectDrawSurface4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0b2b8630_ad35_11d0_8ea6_00609797ea5b);
pub const IID_IDirectDrawSurface7: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06675a80_3b9b_11d2_b92f_00609797ea5b);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPCLIPPERCALLBACK = Option<unsafe extern "system" fn(lpddclipper: LPDIRECTDRAWCLIPPER, hwnd: super::HWND, code: super::DWORD, lpcontext: super::LPVOID) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type LPDDARGB = *mut DDARGB;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPDDBLTBATCH = *mut DDBLTBATCH;
#[cfg(feature = "shared")]
pub type LPDDBLTFX = *mut DDBLTFX;
#[cfg(feature = "shared")]
pub type LPDDCAPS = *mut DDCAPS_DX7;
#[cfg(feature = "shared")]
pub type LPDDCAPS_DX1 = *mut DDCAPS_DX1;
#[cfg(feature = "shared")]
pub type LPDDCAPS_DX3 = *mut DDCAPS_DX3;
#[cfg(feature = "shared")]
pub type LPDDCAPS_DX5 = *mut DDCAPS_DX5;
#[cfg(feature = "shared")]
pub type LPDDCAPS_DX6 = *mut DDCAPS_DX6;
#[cfg(feature = "shared")]
pub type LPDDCAPS_DX7 = *mut DDCAPS_DX7;
#[cfg(feature = "shared")]
pub type LPDDCOLORCONTROL = *mut DDCOLORCONTROL;
#[cfg(feature = "shared")]
pub type LPDDCOLORKEY = *mut DDCOLORKEY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPDDDEVICEIDENTIFIER = *mut DDDEVICEIDENTIFIER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPDDDEVICEIDENTIFIER2 = *mut DDDEVICEIDENTIFIER2;
#[cfg(feature = "minwindef")]
pub type LPDDENUMCALLBACK = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PSTR, param2: windows_sys::core::PSTR, param3: super::LPVOID) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type LPDDENUMCALLBACKA = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PSTR, param2: windows_sys::core::PSTR, param3: super::LPVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPDDENUMCALLBACKEX = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PSTR, param2: windows_sys::core::PSTR, param3: super::LPVOID, param4: super::HMONITOR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPDDENUMCALLBACKEXA = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PSTR, param2: windows_sys::core::PSTR, param3: super::LPVOID, param4: super::HMONITOR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPDDENUMCALLBACKEXW = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PWSTR, param2: windows_sys::core::PWSTR, param3: super::LPVOID, param4: super::HMONITOR) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type LPDDENUMCALLBACKW = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PWSTR, param2: windows_sys::core::PWSTR, param3: super::LPVOID) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
pub type LPDDENUMMODESCALLBACK = Option<unsafe extern "system" fn(param0: LPDDSURFACEDESC, param1: super::LPVOID) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
pub type LPDDENUMMODESCALLBACK2 = Option<unsafe extern "system" fn(param0: LPDDSURFACEDESC2, param1: super::LPVOID) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
pub type LPDDENUMSURFACESCALLBACK = Option<unsafe extern "system" fn(param0: LPDIRECTDRAWSURFACE, param1: LPDDSURFACEDESC, param2: super::LPVOID) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
pub type LPDDENUMSURFACESCALLBACK2 = Option<unsafe extern "system" fn(param0: LPDIRECTDRAWSURFACE4, param1: LPDDSURFACEDESC2, param2: super::LPVOID) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
pub type LPDDENUMSURFACESCALLBACK7 = Option<unsafe extern "system" fn(param0: LPDIRECTDRAWSURFACE7, param1: LPDDSURFACEDESC2, param2: super::LPVOID) -> windows_sys::core::HRESULT>;
pub type LPDDFXROP = *mut _DDFXROP;
#[cfg(feature = "shared")]
pub type LPDDGAMMARAMP = *mut DDGAMMARAMP;
#[cfg(feature = "shared")]
pub type LPDDOSCAPS = *mut DDOSCAPS;
#[cfg(feature = "shared")]
pub type LPDDOVERLAYFX = *mut DDOVERLAYFX;
#[cfg(feature = "shared")]
pub type LPDDRGBA = *mut DDRGBA;
#[cfg(feature = "shared")]
pub type LPDDSCAPS = *mut DDSCAPS;
#[cfg(feature = "shared")]
pub type LPDDSCAPS2 = *mut DDSCAPS2;
#[cfg(feature = "shared")]
pub type LPDDSCAPSEX = *mut DDSCAPSEX;
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
pub type LPDDSURFACEDESC = *mut DDSURFACEDESC;
#[cfg(all(feature = "ksmedia", feature = "minwindef", feature = "shared"))]
pub type LPDDSURFACEDESC2 = *mut DDSURFACEDESC2;
pub type LPDIRECTDRAW = *mut core::ffi::c_void;
pub type LPDIRECTDRAW2 = *mut core::ffi::c_void;
pub type LPDIRECTDRAW4 = *mut core::ffi::c_void;
pub type LPDIRECTDRAW7 = *mut core::ffi::c_void;
pub type LPDIRECTDRAWCLIPPER = *mut core::ffi::c_void;
pub type LPDIRECTDRAWCOLORCONTROL = *mut core::ffi::c_void;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPDIRECTDRAWENUMERATEEX = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: super::LPVOID, dwflags: super::DWORD) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPDIRECTDRAWENUMERATEEXA = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: super::LPVOID, dwflags: super::DWORD) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPDIRECTDRAWENUMERATEEXW = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXW, lpcontext: super::LPVOID, dwflags: super::DWORD) -> windows_sys::core::HRESULT>;
pub type LPDIRECTDRAWGAMMACONTROL = *mut core::ffi::c_void;
pub type LPDIRECTDRAWPALETTE = *mut core::ffi::c_void;
pub type LPDIRECTDRAWSURFACE = *mut core::ffi::c_void;
pub type LPDIRECTDRAWSURFACE2 = *mut core::ffi::c_void;
pub type LPDIRECTDRAWSURFACE3 = *mut core::ffi::c_void;
pub type LPDIRECTDRAWSURFACE4 = *mut core::ffi::c_void;
pub type LPDIRECTDRAWSURFACE7 = *mut core::ffi::c_void;
pub const MAX_DDDEVICEID_STRING: i32 = 512;
pub const REGSTR_KEY_DDHW_DESCRIPTION: windows_sys::core::PCSTR = windows_sys::core::s!("Description");
pub const REGSTR_KEY_DDHW_DRIVERNAME: windows_sys::core::PCSTR = windows_sys::core::s!("DriverName");
pub const REGSTR_PATH_DDHW: windows_sys::core::PCSTR = windows_sys::core::s!("Hardware\\DirectDrawDrivers");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _DDFXROP(pub u8);
pub const _FACDD: i32 = 2166;
