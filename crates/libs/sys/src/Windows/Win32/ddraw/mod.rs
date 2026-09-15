windows_link::link!("ddraw.dll" "system" fn DirectDrawCreate(lpguid : *mut windows_sys::core::GUID, lplpdd : *mut LPDIRECTDRAW, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ddraw.dll" "system" fn DirectDrawCreateClipper(dwflags : u32, lplpddclipper : *mut LPDIRECTDRAWCLIPPER, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ddraw.dll" "system" fn DirectDrawCreateEx(lpguid : *mut windows_sys::core::GUID, lplpdd : *mut *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateA(lpcallback : LPDDENUMCALLBACKA, lpcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateExA(lpcallback : LPDDENUMCALLBACKEXA, lpcontext : *mut core::ffi::c_void, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateExW(lpcallback : LPDDENUMCALLBACKEXW, lpcontext : *mut core::ffi::c_void, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateW(lpcallback : LPDDENUMCALLBACKW, lpcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const CLSID_DirectDraw: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7b70ee0_4340_11cf_b063_0020afc2cd35);
pub const CLSID_DirectDraw7: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3c305196_50db_11d3_9cfe_00c04fd930c5);
pub const CLSID_DirectDrawClipper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x593817a0_7db3_11cf_a2de_00aa00b93356);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DDARGB {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}
pub const DDBD_1: i32 = 16384;
pub const DDBD_16: i32 = 1024;
pub const DDBD_2: i32 = 8192;
pub const DDBD_24: i32 = 512;
pub const DDBD_32: i32 = 256;
pub const DDBD_4: i32 = 4096;
pub const DDBD_8: i32 = 2048;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DDBLTBATCH {
    pub lprDest: super::LPRECT,
    pub lpDDSSrc: LPDIRECTDRAWSURFACE,
    pub lprSrc: super::LPRECT,
    pub dwFlags: u32,
    pub lpDDBltFx: LPDDBLTFX,
}
pub const DDBLTFAST_DESTCOLORKEY: i32 = 2;
pub const DDBLTFAST_DONOTWAIT: i32 = 32;
pub const DDBLTFAST_NOCOLORKEY: i32 = 0;
pub const DDBLTFAST_SRCCOLORKEY: i32 = 1;
pub const DDBLTFAST_WAIT: i32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDBLTFX {
    pub dwSize: u32,
    pub dwDDFX: u32,
    pub dwROP: u32,
    pub dwDDROP: u32,
    pub dwRotationAngle: u32,
    pub dwZBufferOpCode: u32,
    pub dwZBufferLow: u32,
    pub dwZBufferHigh: u32,
    pub dwZBufferBaseDest: u32,
    pub dwZDestConstBitDepth: u32,
    pub Anonymous: DDBLTFX_0,
    pub dwZSrcConstBitDepth: u32,
    pub Anonymous2: DDBLTFX_1,
    pub dwAlphaEdgeBlendBitDepth: u32,
    pub dwAlphaEdgeBlend: u32,
    pub dwReserved: u32,
    pub dwAlphaDestConstBitDepth: u32,
    pub Anonymous3: DDBLTFX_2,
    pub dwAlphaSrcConstBitDepth: u32,
    pub Anonymous4: DDBLTFX_3,
    pub Anonymous5: DDBLTFX_4,
    pub ddckDestColorkey: DDCOLORKEY,
    pub ddckSrcColorkey: DDCOLORKEY,
}
impl Default for DDBLTFX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDBLTFX_0 {
    pub dwZDestConst: u32,
    pub lpDDSZBufferDest: LPDIRECTDRAWSURFACE,
}
impl Default for DDBLTFX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDBLTFX_1 {
    pub dwZSrcConst: u32,
    pub lpDDSZBufferSrc: LPDIRECTDRAWSURFACE,
}
impl Default for DDBLTFX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDBLTFX_2 {
    pub dwAlphaDestConst: u32,
    pub lpDDSAlphaDest: LPDIRECTDRAWSURFACE,
}
impl Default for DDBLTFX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDBLTFX_3 {
    pub dwAlphaSrcConst: u32,
    pub lpDDSAlphaSrc: LPDIRECTDRAWSURFACE,
}
impl Default for DDBLTFX_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDBLTFX_4 {
    pub dwFillColor: u32,
    pub dwFillDepth: u32,
    pub dwFillPixel: u32,
    pub lpDDSPattern: LPDIRECTDRAWSURFACE,
}
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
#[derive(Clone, Copy)]
pub struct DDCAPS_DX1 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
}
impl Default for DDCAPS_DX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX3 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwReserved4: u32,
    pub dwReserved5: u32,
    pub dwReserved6: u32,
}
impl Default for DDCAPS_DX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX5 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
}
impl Default for DDCAPS_DX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX6 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsOldCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
    pub ddsCaps: DDSCAPS2,
}
impl Default for DDCAPS_DX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDCAPS_DX7 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsOldCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
    pub ddsCaps: DDSCAPS2,
}
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
#[derive(Clone, Copy, Default)]
pub struct DDCOLORCONTROL {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lBrightness: i32,
    pub lContrast: i32,
    pub lHue: i32,
    pub lSaturation: i32,
    pub lSharpness: i32,
    pub lGamma: i32,
    pub lColorEnable: i32,
    pub dwReserved1: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DDCOLORKEY {
    pub dwColorSpaceLowValue: u32,
    pub dwColorSpaceHighValue: u32,
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DDDEVICEIDENTIFIER {
    pub szDriver: [i8; 512],
    pub szDescription: [i8; 512],
    pub liDriverVersion: super::LARGE_INTEGER,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: windows_sys::core::GUID,
}
#[cfg(feature = "winnt")]
impl Default for DDDEVICEIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DDDEVICEIDENTIFIER2 {
    pub szDriver: [i8; 512],
    pub szDescription: [i8; 512],
    pub liDriverVersion: super::LARGE_INTEGER,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: windows_sys::core::GUID,
    pub dwWHQLLevel: u32,
}
#[cfg(feature = "winnt")]
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
#[derive(Clone, Copy)]
pub struct DDGAMMARAMP {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}
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
#[derive(Clone, Copy)]
pub struct DDOPTSURFACEDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ddSCaps: DDSCAPS2,
    pub ddOSCaps: DDOSCAPS,
    pub guid: windows_sys::core::GUID,
    pub dwCompressionRatio: u32,
}
impl Default for DDOPTSURFACEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DDOSCAPS {
    pub dwCaps: u32,
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
#[derive(Clone, Copy)]
pub struct DDOVERLAYFX {
    pub dwSize: u32,
    pub dwAlphaEdgeBlendBitDepth: u32,
    pub dwAlphaEdgeBlend: u32,
    pub dwReserved: u32,
    pub dwAlphaDestConstBitDepth: u32,
    pub Anonymous: DDOVERLAYFX_0,
    pub dwAlphaSrcConstBitDepth: u32,
    pub Anonymous2: DDOVERLAYFX_1,
    pub dckDestColorkey: DDCOLORKEY,
    pub dckSrcColorkey: DDCOLORKEY,
    pub dwDDFX: u32,
    pub dwFlags: u32,
}
impl Default for DDOVERLAYFX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDOVERLAYFX_0 {
    pub dwAlphaDestConst: u32,
    pub lpDDSAlphaDest: LPDIRECTDRAWSURFACE,
}
impl Default for DDOVERLAYFX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDOVERLAYFX_1 {
    pub dwAlphaSrcConst: u32,
    pub lpDDSAlphaSrc: LPDIRECTDRAWSURFACE,
}
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
#[derive(Clone, Copy, Default)]
pub struct DDRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DDSCAPS {
    pub dwCaps: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDSCAPS2 {
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCaps3: u32,
    pub Anonymous: DDSCAPS2_0,
}
impl Default for DDSCAPS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDSCAPS2_0 {
    pub dwCaps4: u32,
    pub dwVolumeDepth: u32,
}
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
#[derive(Clone, Copy)]
pub struct DDSCAPSEX {
    pub dwCaps2: u32,
    pub dwCaps3: u32,
    pub Anonymous: DDSCAPSEX_0,
}
impl Default for DDSCAPSEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DDSCAPSEX_0 {
    pub dwCaps4: u32,
    pub dwVolumeDepth: u32,
}
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
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub struct DDSURFACEDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub Anonymous: DDSURFACEDESC_0,
    pub dwBackBufferCount: u32,
    pub Anonymous2: DDSURFACEDESC_1,
    pub dwAlphaBitDepth: u32,
    pub dwReserved: u32,
    pub lpSurface: *mut core::ffi::c_void,
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub ddpfPixelFormat: super::DDPIXELFORMAT,
    pub ddsCaps: DDSCAPS,
}
#[cfg(feature = "ksmedia")]
impl Default for DDSURFACEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC_0 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
#[cfg(feature = "ksmedia")]
impl Default for DDSURFACEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC_1 {
    pub dwMipMapCount: u32,
    pub dwZBufferBitDepth: u32,
    pub dwRefreshRate: u32,
}
#[cfg(feature = "ksmedia")]
impl Default for DDSURFACEDESC_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub struct DDSURFACEDESC2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub Anonymous: DDSURFACEDESC2_0,
    pub Anonymous2: DDSURFACEDESC2_1,
    pub Anonymous3: DDSURFACEDESC2_2,
    pub dwAlphaBitDepth: u32,
    pub dwReserved: u32,
    pub lpSurface: *mut core::ffi::c_void,
    pub Anonymous4: DDSURFACEDESC2_3,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub Anonymous5: DDSURFACEDESC2_4,
    pub ddsCaps: DDSCAPS2,
    pub dwTextureStage: u32,
}
#[cfg(feature = "ksmedia")]
impl Default for DDSURFACEDESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_0 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
#[cfg(feature = "ksmedia")]
impl Default for DDSURFACEDESC2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_1 {
    pub dwBackBufferCount: u32,
    pub dwDepth: u32,
}
#[cfg(feature = "ksmedia")]
impl Default for DDSURFACEDESC2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_2 {
    pub dwMipMapCount: u32,
    pub dwRefreshRate: u32,
    pub dwSrcVBHandle: u32,
}
#[cfg(feature = "ksmedia")]
impl Default for DDSURFACEDESC2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_3 {
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub dwEmptyFaceColor: u32,
}
#[cfg(feature = "ksmedia")]
impl Default for DDSURFACEDESC2_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_4 {
    pub ddpfPixelFormat: super::DDPIXELFORMAT,
    pub dwFVF: u32,
}
#[cfg(feature = "ksmedia")]
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
pub const FOURCC_DXT1: u32 = 827611204;
pub const FOURCC_DXT2: u32 = 844388420;
pub const FOURCC_DXT3: u32 = 861165636;
pub const FOURCC_DXT4: u32 = 877942852;
pub const FOURCC_DXT5: u32 = 894720068;
#[cfg(feature = "windef")]
pub type LPCLIPPERCALLBACK = Option<unsafe extern "system" fn(lpddclipper: LPDIRECTDRAWCLIPPER, hwnd: super::HWND, code: u32, lpcontext: *mut core::ffi::c_void) -> u32>;
pub type LPDDARGB = *mut DDARGB;
#[cfg(feature = "windef")]
pub type LPDDBLTBATCH = *mut DDBLTBATCH;
pub type LPDDBLTFX = *mut DDBLTFX;
pub type LPDDCAPS = *mut DDCAPS;
pub type LPDDCAPS_DX1 = *mut DDCAPS_DX1;
pub type LPDDCAPS_DX3 = *mut DDCAPS_DX3;
pub type LPDDCAPS_DX5 = *mut DDCAPS_DX5;
pub type LPDDCAPS_DX6 = *mut DDCAPS_DX6;
pub type LPDDCAPS_DX7 = *mut DDCAPS_DX7;
pub type LPDDCOLORCONTROL = *mut DDCOLORCONTROL;
pub type LPDDCOLORKEY = *mut DDCOLORKEY;
#[cfg(feature = "winnt")]
pub type LPDDDEVICEIDENTIFIER = *mut DDDEVICEIDENTIFIER;
#[cfg(feature = "winnt")]
pub type LPDDDEVICEIDENTIFIER2 = *mut DDDEVICEIDENTIFIER2;
pub type LPDDENUMCALLBACK = LPDDENUMCALLBACKA;
pub type LPDDENUMCALLBACKA = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "windef")]
pub type LPDDENUMCALLBACKEX = LPDDENUMCALLBACKEXA;
#[cfg(feature = "windef")]
pub type LPDDENUMCALLBACKEXA = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: *mut core::ffi::c_void, param4: super::HMONITOR) -> windows_sys::core::BOOL>;
#[cfg(feature = "windef")]
pub type LPDDENUMCALLBACKEXW = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: *mut core::ffi::c_void, param4: super::HMONITOR) -> windows_sys::core::BOOL>;
pub type LPDDENUMCALLBACKW = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMMODESCALLBACK = Option<unsafe extern "system" fn(param0: LPDDSURFACEDESC, param1: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMMODESCALLBACK2 = Option<unsafe extern "system" fn(param0: LPDDSURFACEDESC2, param1: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMSURFACESCALLBACK = Option<unsafe extern "system" fn(param0: LPDIRECTDRAWSURFACE, param1: LPDDSURFACEDESC, param2: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMSURFACESCALLBACK2 = Option<unsafe extern "system" fn(param0: LPDIRECTDRAWSURFACE4, param1: LPDDSURFACEDESC2, param2: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMSURFACESCALLBACK7 = Option<unsafe extern "system" fn(param0: LPDIRECTDRAWSURFACE7, param1: LPDDSURFACEDESC2, param2: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type LPDDFXROP = *mut _DDFXROP;
pub type LPDDGAMMARAMP = *mut DDGAMMARAMP;
pub type LPDDOSCAPS = *mut DDOSCAPS;
pub type LPDDOVERLAYFX = *mut DDOVERLAYFX;
pub type LPDDRGBA = *mut DDRGBA;
pub type LPDDSCAPS = *mut DDSCAPS;
pub type LPDDSCAPS2 = *mut DDSCAPS2;
pub type LPDDSCAPSEX = *mut DDSCAPSEX;
#[cfg(feature = "ksmedia")]
pub type LPDDSURFACEDESC = *mut DDSURFACEDESC;
#[cfg(feature = "ksmedia")]
pub type LPDDSURFACEDESC2 = *mut DDSURFACEDESC2;
pub type LPDIRECTDRAW = *mut core::ffi::c_void;
pub type LPDIRECTDRAW2 = *mut core::ffi::c_void;
pub type LPDIRECTDRAW4 = *mut core::ffi::c_void;
pub type LPDIRECTDRAW7 = *mut core::ffi::c_void;
pub type LPDIRECTDRAWCLIPPER = *mut core::ffi::c_void;
pub type LPDIRECTDRAWCOLORCONTROL = *mut core::ffi::c_void;
#[cfg(feature = "windef")]
pub type LPDIRECTDRAWENUMERATEEX = LPDIRECTDRAWENUMERATEEXA;
#[cfg(feature = "windef")]
pub type LPDIRECTDRAWENUMERATEEXA = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: *mut core::ffi::c_void, dwflags: u32) -> windows_sys::core::HRESULT>;
#[cfg(feature = "windef")]
pub type LPDIRECTDRAWENUMERATEEXW = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXW, lpcontext: *mut core::ffi::c_void, dwflags: u32) -> windows_sys::core::HRESULT>;
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
