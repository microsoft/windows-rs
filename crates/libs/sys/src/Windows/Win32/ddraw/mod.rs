windows_link::link!("ddraw.dll" "system" fn DirectDrawCreate(lpguid : *mut windows_sys::core::GUID, lplpdd : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ddraw.dll" "system" fn DirectDrawCreateClipper(dwflags : u32, lplpddclipper : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ddraw.dll" "system" fn DirectDrawCreateEx(lpguid : *mut windows_sys::core::GUID, lplpdd : *mut *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateA(lpcallback : LPDDENUMCALLBACKA, lpcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("ddraw.dll" "system" fn DirectDrawEnumerateExA(lpcallback : LPDDENUMCALLBACKEXA, lpcontext : *mut core::ffi::c_void, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
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
pub const DDBD_1: u32 = 16384;
pub const DDBD_16: u32 = 1024;
pub const DDBD_2: u32 = 8192;
pub const DDBD_24: u32 = 512;
pub const DDBD_32: u32 = 256;
pub const DDBD_4: u32 = 4096;
pub const DDBD_8: u32 = 2048;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct DDBLTBATCH {
    pub lprDest: super::windef::LPRECT,
    pub lpDDSSrc: *mut core::ffi::c_void,
    pub lprSrc: super::windef::LPRECT,
    pub dwFlags: u32,
    pub lpDDBltFx: LPDDBLTFX,
}
#[cfg(feature = "Win32_windef")]
impl Default for DDBLTBATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDBLTFAST_DESTCOLORKEY: u32 = 2;
pub const DDBLTFAST_DONOTWAIT: u32 = 32;
pub const DDBLTFAST_NOCOLORKEY: u32 = 0;
pub const DDBLTFAST_SRCCOLORKEY: u32 = 1;
pub const DDBLTFAST_WAIT: u32 = 16;
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
    pub lpDDSZBufferDest: *mut core::ffi::c_void,
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
    pub lpDDSZBufferSrc: *mut core::ffi::c_void,
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
    pub lpDDSAlphaDest: *mut core::ffi::c_void,
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
    pub lpDDSAlphaSrc: *mut core::ffi::c_void,
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
    pub lpDDSPattern: *mut core::ffi::c_void,
}
impl Default for DDBLTFX_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDBLTFX_ARITHSTRETCHY: u32 = 1;
pub const DDBLTFX_MIRRORLEFTRIGHT: u32 = 2;
pub const DDBLTFX_MIRRORUPDOWN: u32 = 4;
pub const DDBLTFX_NOTEARING: u32 = 8;
pub const DDBLTFX_ROTATE180: u32 = 16;
pub const DDBLTFX_ROTATE270: u32 = 32;
pub const DDBLTFX_ROTATE90: u32 = 64;
pub const DDBLTFX_ZBUFFERBASEDEST: u32 = 256;
pub const DDBLTFX_ZBUFFERRANGE: u32 = 128;
pub const DDBLT_ALPHADEST: u32 = 1;
pub const DDBLT_ALPHADESTCONSTOVERRIDE: u32 = 2;
pub const DDBLT_ALPHADESTNEG: u32 = 4;
pub const DDBLT_ALPHADESTSURFACEOVERRIDE: u32 = 8;
pub const DDBLT_ALPHAEDGEBLEND: u32 = 16;
pub const DDBLT_ALPHASRC: u32 = 32;
pub const DDBLT_ALPHASRCCONSTOVERRIDE: u32 = 64;
pub const DDBLT_ALPHASRCNEG: u32 = 128;
pub const DDBLT_ALPHASRCSURFACEOVERRIDE: u32 = 256;
pub const DDBLT_ASYNC: u32 = 512;
pub const DDBLT_COLORFILL: u32 = 1024;
pub const DDBLT_DDFX: u32 = 2048;
pub const DDBLT_DDROPS: u32 = 4096;
pub const DDBLT_DEPTHFILL: u32 = 33554432;
pub const DDBLT_DONOTWAIT: u32 = 134217728;
pub const DDBLT_EXTENDED_FLAGS: u32 = 1073741824;
pub const DDBLT_EXTENDED_LINEAR_CONTENT: u32 = 4;
pub const DDBLT_KEYDEST: u32 = 8192;
pub const DDBLT_KEYDESTOVERRIDE: u32 = 16384;
pub const DDBLT_KEYSRC: u32 = 32768;
pub const DDBLT_KEYSRCOVERRIDE: u32 = 65536;
pub const DDBLT_LAST_PRESENTATION: u32 = 536870912;
pub const DDBLT_PRESENTATION: u32 = 268435456;
pub const DDBLT_ROP: u32 = 131072;
pub const DDBLT_ROTATIONANGLE: u32 = 262144;
pub const DDBLT_WAIT: u32 = 16777216;
pub const DDBLT_ZBUFFER: u32 = 524288;
pub const DDBLT_ZBUFFERDESTCONSTOVERRIDE: u32 = 1048576;
pub const DDBLT_ZBUFFERDESTOVERRIDE: u32 = 2097152;
pub const DDBLT_ZBUFFERSRCCONSTOVERRIDE: u32 = 4194304;
pub const DDBLT_ZBUFFERSRCOVERRIDE: u32 = 8388608;
pub type DDCAPS = DDCAPS_DX7;
pub const DDCAPS2_AUTOFLIPOVERLAY: u32 = 8;
pub const DDCAPS2_CANAUTOGENMIPMAP: u32 = 1073741824;
pub const DDCAPS2_CANBOBHARDWARE: u32 = 16384;
pub const DDCAPS2_CANBOBINTERLEAVED: u32 = 16;
pub const DDCAPS2_CANBOBNONINTERLEAVED: u32 = 32;
pub const DDCAPS2_CANCALIBRATEGAMMA: u32 = 1048576;
pub const DDCAPS2_CANDROPZ16BIT: u32 = 256;
pub const DDCAPS2_CANFLIPODDEVEN: u32 = 8192;
pub const DDCAPS2_CANMANAGERESOURCE: u32 = 268435456;
pub const DDCAPS2_CANMANAGETEXTURE: u32 = 8388608;
pub const DDCAPS2_CANRENDERWINDOWED: u32 = 524288;
pub const DDCAPS2_CANSHARERESOURCE: u32 = 2147483648;
pub const DDCAPS2_CERTIFIED: u32 = 1;
pub const DDCAPS2_COLORCONTROLOVERLAY: u32 = 64;
pub const DDCAPS2_COLORCONTROLPRIMARY: u32 = 128;
pub const DDCAPS2_COPYFOURCC: u32 = 32768;
pub const DDCAPS2_DYNAMICTEXTURES: u32 = 536870912;
pub const DDCAPS2_FLIPINTERVAL: u32 = 2097152;
pub const DDCAPS2_FLIPNOVSYNC: u32 = 4194304;
pub const DDCAPS2_NO2DDURING3DSCENE: u32 = 2;
pub const DDCAPS2_NONLOCALVIDMEM: u32 = 512;
pub const DDCAPS2_NONLOCALVIDMEMCAPS: u32 = 1024;
pub const DDCAPS2_NOPAGELOCKREQUIRED: u32 = 2048;
pub const DDCAPS2_PRIMARYGAMMA: u32 = 131072;
pub const DDCAPS2_RESERVED1: u32 = 134217728;
pub const DDCAPS2_STEREO: u32 = 33554432;
pub const DDCAPS2_SYSTONONLOCAL_AS_SYSTOLOCAL: u32 = 67108864;
pub const DDCAPS2_TEXMANINNONLOCALVIDMEM: u32 = 16777216;
pub const DDCAPS2_VIDEOPORT: u32 = 4;
pub const DDCAPS2_WIDESURFACES: u32 = 4096;
pub const DDCAPS_3D: u32 = 1;
pub const DDCAPS_ALIGNBOUNDARYDEST: u32 = 2;
pub const DDCAPS_ALIGNBOUNDARYSRC: u32 = 8;
pub const DDCAPS_ALIGNSIZEDEST: u32 = 4;
pub const DDCAPS_ALIGNSIZESRC: u32 = 16;
pub const DDCAPS_ALIGNSTRIDE: u32 = 32;
pub const DDCAPS_ALPHA: u32 = 8388608;
pub const DDCAPS_BANKSWITCHED: u32 = 134217728;
pub const DDCAPS_BLT: u32 = 64;
pub const DDCAPS_BLTCOLORFILL: u32 = 67108864;
pub const DDCAPS_BLTDEPTHFILL: u32 = 268435456;
pub const DDCAPS_BLTFOURCC: u32 = 256;
pub const DDCAPS_BLTQUEUE: u32 = 128;
pub const DDCAPS_BLTSTRETCH: u32 = 512;
pub const DDCAPS_CANBLTSYSMEM: u32 = 2147483648;
pub const DDCAPS_CANCLIP: u32 = 536870912;
pub const DDCAPS_CANCLIPSTRETCHED: u32 = 1073741824;
pub const DDCAPS_COLORKEY: u32 = 4194304;
pub const DDCAPS_COLORKEYHWASSIST: u32 = 16777216;
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
pub const DDCAPS_GDI: u32 = 1024;
pub const DDCAPS_NOHARDWARE: u32 = 33554432;
pub const DDCAPS_OVERLAY: u32 = 2048;
pub const DDCAPS_OVERLAYCANTCLIP: u32 = 4096;
pub const DDCAPS_OVERLAYFOURCC: u32 = 8192;
pub const DDCAPS_OVERLAYSTRETCH: u32 = 16384;
pub const DDCAPS_PALETTE: u32 = 32768;
pub const DDCAPS_PALETTEVSYNC: u32 = 65536;
pub const DDCAPS_READSCANLINE: u32 = 131072;
pub const DDCAPS_RESERVED1: u32 = 262144;
pub const DDCAPS_VBI: u32 = 524288;
pub const DDCAPS_ZBLTS: u32 = 1048576;
pub const DDCAPS_ZOVERLAYS: u32 = 2097152;
pub const DDCKEYCAPS_DESTBLT: u32 = 1;
pub const DDCKEYCAPS_DESTBLTCLRSPACE: u32 = 2;
pub const DDCKEYCAPS_DESTBLTCLRSPACEYUV: u32 = 4;
pub const DDCKEYCAPS_DESTBLTYUV: u32 = 8;
pub const DDCKEYCAPS_DESTOVERLAY: u32 = 16;
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACE: u32 = 32;
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACEYUV: u32 = 64;
pub const DDCKEYCAPS_DESTOVERLAYONEACTIVE: u32 = 128;
pub const DDCKEYCAPS_DESTOVERLAYYUV: u32 = 256;
pub const DDCKEYCAPS_NOCOSTOVERLAY: u32 = 262144;
pub const DDCKEYCAPS_SRCBLT: u32 = 512;
pub const DDCKEYCAPS_SRCBLTCLRSPACE: u32 = 1024;
pub const DDCKEYCAPS_SRCBLTCLRSPACEYUV: u32 = 2048;
pub const DDCKEYCAPS_SRCBLTYUV: u32 = 4096;
pub const DDCKEYCAPS_SRCOVERLAY: u32 = 8192;
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACE: u32 = 16384;
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACEYUV: u32 = 32768;
pub const DDCKEYCAPS_SRCOVERLAYONEACTIVE: u32 = 65536;
pub const DDCKEYCAPS_SRCOVERLAYYUV: u32 = 131072;
pub const DDCKEY_COLORSPACE: u32 = 1;
pub const DDCKEY_DESTBLT: u32 = 2;
pub const DDCKEY_DESTOVERLAY: u32 = 4;
pub const DDCKEY_SRCBLT: u32 = 8;
pub const DDCKEY_SRCOVERLAY: u32 = 16;
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
pub const DDCOLOR_BRIGHTNESS: u32 = 1;
pub const DDCOLOR_COLORENABLE: u32 = 64;
pub const DDCOLOR_CONTRAST: u32 = 2;
pub const DDCOLOR_GAMMA: u32 = 32;
pub const DDCOLOR_HUE: u32 = 4;
pub const DDCOLOR_SATURATION: u32 = 8;
pub const DDCOLOR_SHARPNESS: u32 = 16;
pub const DDCREATE_EMULATIONONLY: u32 = 2;
pub const DDCREATE_HARDWAREONLY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDDEVICEIDENTIFIER {
    pub szDriver: [i8; 512],
    pub szDescription: [i8; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: windows_sys::core::GUID,
}
impl Default for DDDEVICEIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDDEVICEIDENTIFIER2 {
    pub szDriver: [i8; 512],
    pub szDescription: [i8; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: windows_sys::core::GUID,
    pub dwWHQLLevel: u32,
}
impl Default for DDDEVICEIDENTIFIER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDEDM_REFRESHRATES: u32 = 1;
pub const DDEDM_STANDARDVGAMODES: u32 = 2;
pub const DDEM_MODEFAILED: u32 = 2;
pub const DDEM_MODEPASSED: u32 = 1;
pub const DDENUMOVERLAYZ_BACKTOFRONT: u32 = 0;
pub const DDENUMOVERLAYZ_FRONTTOBACK: u32 = 1;
pub const DDENUMRET_CANCEL: u32 = 0;
pub const DDENUMRET_OK: u32 = 1;
pub const DDENUMSURFACES_ALL: u32 = 1;
pub const DDENUMSURFACES_CANBECREATED: u32 = 8;
pub const DDENUMSURFACES_DOESEXIST: u32 = 16;
pub const DDENUMSURFACES_MATCH: u32 = 2;
pub const DDENUMSURFACES_NOMATCH: u32 = 4;
pub const DDENUM_ATTACHEDSECONDARYDEVICES: u32 = 1;
pub const DDENUM_DETACHEDSECONDARYDEVICES: u32 = 2;
pub const DDENUM_NONDISPLAYDEVICES: u32 = 4;
pub const DDERR_ALREADYINITIALIZED: i32 = -2005532667;
pub const DDERR_BLTFASTCANTCLIP: i32 = -2005532098;
pub const DDERR_CANNOTATTACHSURFACE: i32 = -2005532662;
pub const DDERR_CANNOTDETACHSURFACE: i32 = -2005532652;
pub const DDERR_CANTCREATEDC: i32 = -2005532087;
pub const DDERR_CANTDUPLICATE: i32 = -2005532089;
pub const DDERR_CANTLOCKSURFACE: i32 = -2005532237;
pub const DDERR_CANTPAGELOCK: i32 = -2005532032;
pub const DDERR_CANTPAGEUNLOCK: i32 = -2005532012;
pub const DDERR_CLIPPERISUSINGHWND: i32 = -2005532105;
pub const DDERR_COLORKEYNOTSET: i32 = -2005532272;
pub const DDERR_CURRENTLYNOTAVAIL: i32 = -2005532632;
pub const DDERR_D3DNOTINITIALIZED: i32 = -2005531978;
pub const DDERR_DCALREADYCREATED: i32 = -2005532052;
pub const DDERR_DDSCAPSCOMPLEXREQUIRED: i32 = -2005532130;
pub const DDERR_DEVICEDOESNTOWNSURFACE: i32 = -2005531973;
pub const DDERR_DIRECTDRAWALREADYCREATED: i32 = -2005532110;
pub const DDERR_EXCEPTION: i32 = -2005532617;
pub const DDERR_EXCLUSIVEMODEALREADYSET: i32 = -2005532091;
pub const DDERR_EXPIRED: i32 = -2005531981;
pub const DDERR_GENERIC: i32 = -2147467259;
pub const DDERR_HEIGHTALIGN: i32 = -2005532582;
pub const DDERR_HWNDALREADYSET: i32 = -2005532101;
pub const DDERR_HWNDSUBCLASSED: i32 = -2005532102;
pub const DDERR_IMPLICITLYCREATED: i32 = -2005532084;
pub const DDERR_INCOMPATIBLEPRIMARY: i32 = -2005532577;
pub const DDERR_INVALIDCAPS: i32 = -2005532572;
pub const DDERR_INVALIDCLIPLIST: i32 = -2005532562;
pub const DDERR_INVALIDDIRECTDRAWGUID: i32 = -2005532111;
pub const DDERR_INVALIDMODE: i32 = -2005532552;
pub const DDERR_INVALIDOBJECT: i32 = -2005532542;
pub const DDERR_INVALIDPARAMS: i32 = -2147024809;
pub const DDERR_INVALIDPIXELFORMAT: i32 = -2005532527;
pub const DDERR_INVALIDPOSITION: i32 = -2005532093;
pub const DDERR_INVALIDRECT: i32 = -2005532522;
pub const DDERR_INVALIDSTREAM: i32 = -2005532151;
pub const DDERR_INVALIDSURFACETYPE: i32 = -2005532080;
pub const DDERR_LOCKEDSURFACES: i32 = -2005532512;
pub const DDERR_MOREDATA: i32 = -2005531982;
pub const DDERR_NEWMODE: i32 = -2005531979;
pub const DDERR_NO3D: i32 = -2005532502;
pub const DDERR_NOALPHAHW: i32 = -2005532492;
pub const DDERR_NOBLTHW: i32 = -2005532097;
pub const DDERR_NOCLIPLIST: i32 = -2005532467;
pub const DDERR_NOCLIPPERATTACHED: i32 = -2005532104;
pub const DDERR_NOCOLORCONVHW: i32 = -2005532462;
pub const DDERR_NOCOLORKEY: i32 = -2005532457;
pub const DDERR_NOCOLORKEYHW: i32 = -2005532452;
pub const DDERR_NOCOOPERATIVELEVELSET: i32 = -2005532460;
pub const DDERR_NODC: i32 = -2005532086;
pub const DDERR_NODDROPSHW: i32 = -2005532096;
pub const DDERR_NODIRECTDRAWHW: i32 = -2005532109;
pub const DDERR_NODIRECTDRAWSUPPORT: i32 = -2005532450;
pub const DDERR_NODRIVERSUPPORT: i32 = -2005531975;
pub const DDERR_NOEMULATION: i32 = -2005532107;
pub const DDERR_NOEXCLUSIVEMODE: i32 = -2005532447;
pub const DDERR_NOFLIPHW: i32 = -2005532442;
pub const DDERR_NOFOCUSWINDOW: i32 = -2005532070;
pub const DDERR_NOGDI: i32 = -2005532432;
pub const DDERR_NOHWND: i32 = -2005532103;
pub const DDERR_NOMIPMAPHW: i32 = -2005532081;
pub const DDERR_NOMIRRORHW: i32 = -2005532422;
pub const DDERR_NOMONITORINFORMATION: i32 = -2005531976;
pub const DDERR_NONONLOCALVIDMEM: i32 = -2005532042;
pub const DDERR_NOOPTIMIZEHW: i32 = -2005532072;
pub const DDERR_NOOVERLAYDEST: i32 = -2005532094;
pub const DDERR_NOOVERLAYHW: i32 = -2005532412;
pub const DDERR_NOPALETTEATTACHED: i32 = -2005532100;
pub const DDERR_NOPALETTEHW: i32 = -2005532099;
pub const DDERR_NORASTEROPHW: i32 = -2005532392;
pub const DDERR_NOROTATIONHW: i32 = -2005532382;
pub const DDERR_NOSTEREOHARDWARE: i32 = -2005532491;
pub const DDERR_NOSTRETCHHW: i32 = -2005532362;
pub const DDERR_NOSURFACELEFT: i32 = -2005532490;
pub const DDERR_NOT4BITCOLOR: i32 = -2005532356;
pub const DDERR_NOT4BITCOLORINDEX: i32 = -2005532355;
pub const DDERR_NOT8BITCOLOR: i32 = -2005532352;
pub const DDERR_NOTAOVERLAYSURFACE: i32 = -2005532092;
pub const DDERR_NOTEXTUREHW: i32 = -2005532342;
pub const DDERR_NOTFLIPPABLE: i32 = -2005532090;
pub const DDERR_NOTFOUND: i32 = -2005532417;
pub const DDERR_NOTINITIALIZED: i32 = -2147221008;
pub const DDERR_NOTLOADED: i32 = -2005532071;
pub const DDERR_NOTLOCKED: i32 = -2005532088;
pub const DDERR_NOTONMIPMAPSUBLEVEL: i32 = -2005532069;
pub const DDERR_NOTPAGELOCKED: i32 = -2005531992;
pub const DDERR_NOTPALETTIZED: i32 = -2005532083;
pub const DDERR_NOVSYNCHW: i32 = -2005532337;
pub const DDERR_NOZBUFFERHW: i32 = -2005532332;
pub const DDERR_NOZOVERLAYHW: i32 = -2005532322;
pub const DDERR_OUTOFCAPS: i32 = -2005532312;
pub const DDERR_OUTOFMEMORY: i32 = -2147024882;
pub const DDERR_OUTOFVIDEOMEMORY: i32 = -2005532292;
pub const DDERR_OVERLAPPINGRECTS: i32 = -2005532402;
pub const DDERR_OVERLAYCANTCLIP: i32 = -2005532290;
pub const DDERR_OVERLAYCOLORKEYONLYONEACTIVE: i32 = -2005532288;
pub const DDERR_OVERLAYNOTVISIBLE: i32 = -2005532095;
pub const DDERR_PALETTEBUSY: i32 = -2005532285;
pub const DDERR_PRIMARYSURFACEALREADYEXISTS: i32 = -2005532108;
pub const DDERR_REGIONTOOSMALL: i32 = -2005532106;
pub const DDERR_SURFACEALREADYATTACHED: i32 = -2005532262;
pub const DDERR_SURFACEALREADYDEPENDENT: i32 = -2005532252;
pub const DDERR_SURFACEBUSY: i32 = -2005532242;
pub const DDERR_SURFACEISOBSCURED: i32 = -2005532232;
pub const DDERR_SURFACELOST: i32 = -2005532222;
pub const DDERR_SURFACENOTATTACHED: i32 = -2005532212;
pub const DDERR_TESTFINISHED: i32 = -2005531980;
pub const DDERR_TOOBIGHEIGHT: i32 = -2005532202;
pub const DDERR_TOOBIGSIZE: i32 = -2005532192;
pub const DDERR_TOOBIGWIDTH: i32 = -2005532182;
pub const DDERR_UNSUPPORTED: i32 = -2147467263;
pub const DDERR_UNSUPPORTEDFORMAT: i32 = -2005532162;
pub const DDERR_UNSUPPORTEDMASK: i32 = -2005532152;
pub const DDERR_UNSUPPORTEDMODE: i32 = -2005532082;
pub const DDERR_VERTICALBLANKINPROGRESS: i32 = -2005532135;
pub const DDERR_VIDEONOTACTIVE: i32 = -2005531977;
pub const DDERR_WASSTILLDRAWING: i32 = -2005532132;
pub const DDERR_WRONGMODE: i32 = -2005532085;
pub const DDERR_XALIGN: i32 = -2005532112;
pub const DDFLIP_DONOTWAIT: u32 = 32;
pub const DDFLIP_EVEN: u32 = 2;
pub const DDFLIP_INTERVAL2: u32 = 33554432;
pub const DDFLIP_INTERVAL3: u32 = 50331648;
pub const DDFLIP_INTERVAL4: u32 = 67108864;
pub const DDFLIP_NOVSYNC: u32 = 8;
pub const DDFLIP_ODD: u32 = 4;
pub const DDFLIP_STEREO: u32 = 16;
pub const DDFLIP_WAIT: u32 = 1;
pub const DDFXALPHACAPS_BLTALPHAEDGEBLEND: u32 = 1;
pub const DDFXALPHACAPS_BLTALPHAPIXELS: u32 = 2;
pub const DDFXALPHACAPS_BLTALPHAPIXELSNEG: u32 = 4;
pub const DDFXALPHACAPS_BLTALPHASURFACES: u32 = 8;
pub const DDFXALPHACAPS_BLTALPHASURFACESNEG: u32 = 16;
pub const DDFXALPHACAPS_OVERLAYALPHAEDGEBLEND: u32 = 32;
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELS: u32 = 64;
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELSNEG: u32 = 128;
pub const DDFXALPHACAPS_OVERLAYALPHASURFACES: u32 = 256;
pub const DDFXALPHACAPS_OVERLAYALPHASURFACESNEG: u32 = 512;
pub const DDFXCAPS_BLTALPHA: u32 = 1;
pub const DDFXCAPS_BLTARITHSTRETCHY: u32 = 32;
pub const DDFXCAPS_BLTARITHSTRETCHYN: u32 = 16;
pub const DDFXCAPS_BLTFILTER: u32 = 32;
pub const DDFXCAPS_BLTMIRRORLEFTRIGHT: u32 = 64;
pub const DDFXCAPS_BLTMIRRORUPDOWN: u32 = 128;
pub const DDFXCAPS_BLTROTATION: u32 = 256;
pub const DDFXCAPS_BLTROTATION90: u32 = 512;
pub const DDFXCAPS_BLTSHRINKX: u32 = 1024;
pub const DDFXCAPS_BLTSHRINKXN: u32 = 2048;
pub const DDFXCAPS_BLTSHRINKY: u32 = 4096;
pub const DDFXCAPS_BLTSHRINKYN: u32 = 8192;
pub const DDFXCAPS_BLTSTRETCHX: u32 = 16384;
pub const DDFXCAPS_BLTSTRETCHXN: u32 = 32768;
pub const DDFXCAPS_BLTSTRETCHY: u32 = 65536;
pub const DDFXCAPS_BLTSTRETCHYN: u32 = 131072;
pub const DDFXCAPS_OVERLAYALPHA: u32 = 4;
pub const DDFXCAPS_OVERLAYARITHSTRETCHY: u32 = 262144;
pub const DDFXCAPS_OVERLAYARITHSTRETCHYN: u32 = 8;
pub const DDFXCAPS_OVERLAYDEINTERLACE: u32 = 536870912;
pub const DDFXCAPS_OVERLAYFILTER: u32 = 262144;
pub const DDFXCAPS_OVERLAYMIRRORLEFTRIGHT: u32 = 134217728;
pub const DDFXCAPS_OVERLAYMIRRORUPDOWN: u32 = 268435456;
pub const DDFXCAPS_OVERLAYSHRINKX: u32 = 524288;
pub const DDFXCAPS_OVERLAYSHRINKXN: u32 = 1048576;
pub const DDFXCAPS_OVERLAYSHRINKY: u32 = 2097152;
pub const DDFXCAPS_OVERLAYSHRINKYN: u32 = 4194304;
pub const DDFXCAPS_OVERLAYSTRETCHX: u32 = 8388608;
pub const DDFXCAPS_OVERLAYSTRETCHXN: u32 = 16777216;
pub const DDFXCAPS_OVERLAYSTRETCHY: u32 = 33554432;
pub const DDFXCAPS_OVERLAYSTRETCHYN: u32 = 67108864;
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
pub const DDGBS_CANBLT: u32 = 1;
pub const DDGBS_ISBLTDONE: u32 = 2;
pub const DDGDI_GETHOSTIDENTIFIER: u32 = 1;
pub const DDGFS_CANFLIP: u32 = 1;
pub const DDGFS_ISFLIPDONE: u32 = 2;
pub const DDLOCK_DISCARDCONTENTS: u32 = 8192;
pub const DDLOCK_DONOTWAIT: u32 = 16384;
pub const DDLOCK_EVENT: u32 = 2;
pub const DDLOCK_HASVOLUMETEXTUREBOXRECT: u32 = 32768;
pub const DDLOCK_NODIRTYUPDATE: u32 = 65536;
pub const DDLOCK_NOOVERWRITE: u32 = 4096;
pub const DDLOCK_NOSYSLOCK: u32 = 2048;
pub const DDLOCK_OKTOSWAP: u32 = 8192;
pub const DDLOCK_READONLY: u32 = 16;
pub const DDLOCK_SURFACEMEMORYPTR: u32 = 0;
pub const DDLOCK_WAIT: u32 = 1;
pub const DDLOCK_WRITEONLY: u32 = 32;
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
pub const DDOSDCAPS_MONOLITHICMIPMAP: u32 = 4;
pub const DDOSDCAPS_OPTCOMPRESSED: u32 = 1;
pub const DDOSDCAPS_OPTREORDERED: u32 = 2;
pub const DDOSDCAPS_VALIDOSCAPS: u32 = 7;
pub const DDOSDCAPS_VALIDSCAPS: u32 = 805324800;
pub const DDOSD_ALL: u32 = 15;
pub const DDOSD_COMPRESSION_RATIO: u32 = 2;
pub const DDOSD_GUID: u32 = 1;
pub const DDOSD_OSCAPS: u32 = 8;
pub const DDOSD_SCAPS: u32 = 4;
pub const DDOVERFX_ARITHSTRETCHY: u32 = 1;
pub const DDOVERFX_DEINTERLACE: u32 = 8;
pub const DDOVERFX_MIRRORLEFTRIGHT: u32 = 2;
pub const DDOVERFX_MIRRORUPDOWN: u32 = 4;
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
    pub lpDDSAlphaDest: *mut core::ffi::c_void,
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
    pub lpDDSAlphaSrc: *mut core::ffi::c_void,
}
impl Default for DDOVERLAYFX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDOVERZ_INSERTINBACKOF: u32 = 5;
pub const DDOVERZ_INSERTINFRONTOF: u32 = 4;
pub const DDOVERZ_MOVEBACKWARD: u32 = 3;
pub const DDOVERZ_MOVEFORWARD: u32 = 2;
pub const DDOVERZ_SENDTOBACK: u32 = 1;
pub const DDOVERZ_SENDTOFRONT: u32 = 0;
pub const DDOVER_ADDDIRTYRECT: u32 = 32768;
pub const DDOVER_ALPHADEST: u32 = 1;
pub const DDOVER_ALPHADESTCONSTOVERRIDE: u32 = 2;
pub const DDOVER_ALPHADESTNEG: u32 = 4;
pub const DDOVER_ALPHADESTSURFACEOVERRIDE: u32 = 8;
pub const DDOVER_ALPHAEDGEBLEND: u32 = 16;
pub const DDOVER_ALPHASRC: u32 = 32;
pub const DDOVER_ALPHASRCCONSTOVERRIDE: u32 = 64;
pub const DDOVER_ALPHASRCNEG: u32 = 128;
pub const DDOVER_ALPHASRCSURFACEOVERRIDE: u32 = 256;
pub const DDOVER_ARGBSCALEFACTORS: u32 = 33554432;
pub const DDOVER_AUTOFLIP: u32 = 1048576;
pub const DDOVER_BOB: u32 = 2097152;
pub const DDOVER_BOBHARDWARE: u32 = 16777216;
pub const DDOVER_DDFX: u32 = 524288;
pub const DDOVER_DEGRADEARGBSCALING: u32 = 67108864;
pub const DDOVER_HIDE: u32 = 512;
pub const DDOVER_INTERLEAVED: u32 = 8388608;
pub const DDOVER_KEYDEST: u32 = 1024;
pub const DDOVER_KEYDESTOVERRIDE: u32 = 2048;
pub const DDOVER_KEYSRC: u32 = 4096;
pub const DDOVER_KEYSRCOVERRIDE: u32 = 8192;
pub const DDOVER_OVERRIDEBOBWEAVE: u32 = 4194304;
pub const DDOVER_REFRESHALL: u32 = 131072;
pub const DDOVER_REFRESHDIRTYRECTS: u32 = 65536;
pub const DDOVER_SHOW: u32 = 16384;
pub const DDPCAPS_1BIT: u32 = 256;
pub const DDPCAPS_2BIT: u32 = 512;
pub const DDPCAPS_4BIT: u32 = 1;
pub const DDPCAPS_8BIT: u32 = 4;
pub const DDPCAPS_8BITENTRIES: u32 = 2;
pub const DDPCAPS_ALLOW256: u32 = 64;
pub const DDPCAPS_ALPHA: u32 = 1024;
pub const DDPCAPS_INITIALIZE: u32 = 0;
pub const DDPCAPS_PRIMARYSURFACE: u32 = 16;
pub const DDPCAPS_PRIMARYSURFACELEFT: u32 = 32;
pub const DDPCAPS_VSYNC: u32 = 128;
pub const DDPF_ALPHA: u32 = 2;
pub const DDPF_ALPHAPIXELS: u32 = 1;
pub const DDPF_ALPHAPREMULT: u32 = 32768;
pub const DDPF_BUMPDUDV: u32 = 524288;
pub const DDPF_BUMPLUMINANCE: u32 = 262144;
pub const DDPF_COMPRESSED: u32 = 128;
pub const DDPF_LUMINANCE: u32 = 131072;
pub const DDPF_PALETTEINDEXED1: u32 = 2048;
pub const DDPF_PALETTEINDEXED2: u32 = 4096;
pub const DDPF_PALETTEINDEXED4: u32 = 8;
pub const DDPF_PALETTEINDEXED8: u32 = 32;
pub const DDPF_PALETTEINDEXEDTO8: u32 = 16;
pub const DDPF_RGB: u32 = 64;
pub const DDPF_RGBTOYUV: u32 = 256;
pub const DDPF_STENCILBUFFER: u32 = 16384;
pub const DDPF_YUV: u32 = 512;
pub const DDPF_ZBUFFER: u32 = 1024;
pub const DDPF_ZPIXELS: u32 = 8192;
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
pub const DDSCAPS2_CUBEMAP: u32 = 512;
pub const DDSCAPS2_CUBEMAP_ALLFACES: u32 = 64512;
pub const DDSCAPS2_CUBEMAP_NEGATIVEX: u32 = 2048;
pub const DDSCAPS2_CUBEMAP_NEGATIVEY: u32 = 8192;
pub const DDSCAPS2_CUBEMAP_NEGATIVEZ: u32 = 32768;
pub const DDSCAPS2_CUBEMAP_POSITIVEX: u32 = 1024;
pub const DDSCAPS2_CUBEMAP_POSITIVEY: u32 = 4096;
pub const DDSCAPS2_CUBEMAP_POSITIVEZ: u32 = 16384;
pub const DDSCAPS2_D3DTEXTUREMANAGE: u32 = 131072;
pub const DDSCAPS2_DISCARDBACKBUFFER: u32 = 268435456;
pub const DDSCAPS2_DONOTPERSIST: u32 = 262144;
pub const DDSCAPS2_ENABLEALPHACHANNEL: u32 = 536870912;
pub const DDSCAPS2_EXTENDEDFORMATPRIMARY: u32 = 1073741824;
pub const DDSCAPS2_HARDWAREDEINTERLACE: u32 = 0;
pub const DDSCAPS2_HINTANTIALIASING: u32 = 256;
pub const DDSCAPS2_HINTDYNAMIC: u32 = 4;
pub const DDSCAPS2_HINTSTATIC: u32 = 8;
pub const DDSCAPS2_MIPMAPSUBLEVEL: u32 = 65536;
pub const DDSCAPS2_NOTUSERLOCKABLE: u32 = 4194304;
pub const DDSCAPS2_NPATCHES: u32 = 33554432;
pub const DDSCAPS2_OPAQUE: u32 = 128;
pub const DDSCAPS2_POINTS: u32 = 8388608;
pub const DDSCAPS2_RESERVED1: u32 = 32;
pub const DDSCAPS2_RESERVED2: u32 = 64;
pub const DDSCAPS2_RESERVED3: u32 = 67108864;
pub const DDSCAPS2_RESERVED4: u32 = 2;
pub const DDSCAPS2_RTPATCHES: u32 = 16777216;
pub const DDSCAPS2_STEREOSURFACELEFT: u32 = 524288;
pub const DDSCAPS2_TEXTUREMANAGE: u32 = 16;
pub const DDSCAPS2_VOLUME: u32 = 2097152;
pub const DDSCAPS3_AUTOGENMIPMAP: u32 = 2048;
pub const DDSCAPS3_CREATESHAREDRESOURCE: u32 = 8192;
pub const DDSCAPS3_DMAP: u32 = 4096;
pub const DDSCAPS3_LIGHTWEIGHTMIPMAP: u32 = 1024;
pub const DDSCAPS3_MULTISAMPLE_MASK: u32 = 31;
pub const DDSCAPS3_MULTISAMPLE_QUALITY_MASK: u32 = 224;
pub const DDSCAPS3_MULTISAMPLE_QUALITY_SHIFT: u32 = 5;
pub const DDSCAPS3_OPENSHAREDRESOURCE: u32 = 32768;
pub const DDSCAPS3_READONLYRESOURCE: u32 = 16384;
pub const DDSCAPS3_RESERVED1: u32 = 256;
pub const DDSCAPS3_RESERVED2: u32 = 512;
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
pub const DDSCAPS_3DDEVICE: u32 = 8192;
pub const DDSCAPS_ALLOCONLOAD: u32 = 67108864;
pub const DDSCAPS_ALPHA: u32 = 2;
pub const DDSCAPS_BACKBUFFER: u32 = 4;
pub const DDSCAPS_COMPLEX: u32 = 8;
pub const DDSCAPS_FLIP: u32 = 16;
pub const DDSCAPS_FRONTBUFFER: u32 = 32;
pub const DDSCAPS_HWCODEC: u32 = 1048576;
pub const DDSCAPS_LIVEVIDEO: u32 = 524288;
pub const DDSCAPS_LOCALVIDMEM: u32 = 268435456;
pub const DDSCAPS_MIPMAP: u32 = 4194304;
pub const DDSCAPS_MODEX: u32 = 2097152;
pub const DDSCAPS_NONLOCALVIDMEM: u32 = 536870912;
pub const DDSCAPS_OFFSCREENPLAIN: u32 = 64;
pub const DDSCAPS_OPTIMIZED: u32 = 2147483648;
pub const DDSCAPS_OVERLAY: u32 = 128;
pub const DDSCAPS_OWNDC: u32 = 262144;
pub const DDSCAPS_PALETTE: u32 = 256;
pub const DDSCAPS_PRIMARYSURFACE: u32 = 512;
pub const DDSCAPS_PRIMARYSURFACELEFT: u32 = 0;
pub const DDSCAPS_RESERVED1: u32 = 1;
pub const DDSCAPS_RESERVED2: u32 = 8388608;
pub const DDSCAPS_RESERVED3: u32 = 1024;
pub const DDSCAPS_STANDARDVGAMODE: u32 = 1073741824;
pub const DDSCAPS_SYSTEMMEMORY: u32 = 2048;
pub const DDSCAPS_TEXTURE: u32 = 4096;
pub const DDSCAPS_VIDEOMEMORY: u32 = 16384;
pub const DDSCAPS_VIDEOPORT: u32 = 134217728;
pub const DDSCAPS_VISIBLE: u32 = 32768;
pub const DDSCAPS_WRITEONLY: u32 = 65536;
pub const DDSCAPS_ZBUFFER: u32 = 131072;
pub const DDSCL_ALLOWMODEX: u32 = 64;
pub const DDSCL_ALLOWREBOOT: u32 = 2;
pub const DDSCL_CREATEDEVICEWINDOW: u32 = 512;
pub const DDSCL_EXCLUSIVE: u32 = 16;
pub const DDSCL_FPUPRESERVE: u32 = 4096;
pub const DDSCL_FPUSETUP: u32 = 2048;
pub const DDSCL_FULLSCREEN: u32 = 1;
pub const DDSCL_MULTITHREADED: u32 = 1024;
pub const DDSCL_NORMAL: u32 = 8;
pub const DDSCL_NOWINDOWCHANGES: u32 = 4;
pub const DDSCL_SETDEVICEWINDOW: u32 = 256;
pub const DDSCL_SETFOCUSWINDOW: u32 = 128;
pub const DDSDM_STANDARDVGAMODE: u32 = 1;
pub const DDSD_ALL: u32 = 16775662;
pub const DDSD_ALPHABITDEPTH: u32 = 128;
pub const DDSD_BACKBUFFERCOUNT: u32 = 32;
pub const DDSD_CAPS: u32 = 1;
pub const DDSD_CKDESTBLT: u32 = 16384;
pub const DDSD_CKDESTOVERLAY: u32 = 8192;
pub const DDSD_CKSRCBLT: u32 = 65536;
pub const DDSD_CKSRCOVERLAY: u32 = 32768;
pub const DDSD_DEPTH: u32 = 8388608;
pub const DDSD_FVF: u32 = 2097152;
pub const DDSD_HEIGHT: u32 = 2;
pub const DDSD_LINEARSIZE: u32 = 524288;
pub const DDSD_LPSURFACE: u32 = 2048;
pub const DDSD_MIPMAPCOUNT: u32 = 131072;
pub const DDSD_PITCH: u32 = 8;
pub const DDSD_PIXELFORMAT: u32 = 4096;
pub const DDSD_REFRESHRATE: u32 = 262144;
pub const DDSD_SRCVBHANDLE: u32 = 4194304;
pub const DDSD_TEXTURESTAGE: u32 = 1048576;
pub const DDSD_WIDTH: u32 = 4;
pub const DDSD_ZBUFFERBITDEPTH: u32 = 64;
pub const DDSGR_CALIBRATE: u32 = 1;
pub const DDSMT_ISTESTREQUIRED: u32 = 1;
pub const DDSPD_IUNKNOWNPOINTER: u32 = 1;
pub const DDSPD_VOLATILE: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
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
    pub ddpfPixelFormat: super::ksmedia::DDPIXELFORMAT,
    pub ddsCaps: DDSCAPS,
}
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC_0 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC_1 {
    pub dwMipMapCount: u32,
    pub dwZBufferBitDepth: u32,
    pub dwRefreshRate: u32,
}
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
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
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_0 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_1 {
    pub dwBackBufferCount: u32,
    pub dwDepth: u32,
}
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_2 {
    pub dwMipMapCount: u32,
    pub dwRefreshRate: u32,
    pub dwSrcVBHandle: u32,
}
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_3 {
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub dwEmptyFaceColor: u32,
}
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC2_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_ksmedia")]
#[derive(Clone, Copy)]
pub union DDSURFACEDESC2_4 {
    pub ddpfPixelFormat: super::ksmedia::DDPIXELFORMAT,
    pub dwFVF: u32,
}
#[cfg(feature = "Win32_ksmedia")]
impl Default for DDSURFACEDESC2_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DDSVCAPS_RESERVED1: u32 = 1;
pub const DDSVCAPS_RESERVED2: u32 = 2;
pub const DDSVCAPS_RESERVED3: u32 = 4;
pub const DDSVCAPS_RESERVED4: u32 = 8;
pub const DDSVCAPS_STEREOSEQUENTIAL: u32 = 16;
pub const DDWAITVB_BLOCKBEGIN: u32 = 1;
pub const DDWAITVB_BLOCKBEGINEVENT: u32 = 2;
pub const DDWAITVB_BLOCKEND: u32 = 4;
pub const DD_FALSE: u32 = 1;
pub const DD_OK: u32 = 0;
pub const DD_ROP_SPACE: u32 = 8;
pub const DIRECTDRAW_VERSION: u32 = 1792;
pub const FOURCC_DXT1: u32 = 827611204;
pub const FOURCC_DXT2: u32 = 844388420;
pub const FOURCC_DXT3: u32 = 861165636;
pub const FOURCC_DXT4: u32 = 877942852;
pub const FOURCC_DXT5: u32 = 894720068;
#[cfg(feature = "Win32_windef")]
pub type LPCLIPPERCALLBACK = Option<unsafe extern "system" fn(lpddclipper: *mut core::ffi::c_void, hwnd: super::windef::HWND, code: u32, lpcontext: *mut core::ffi::c_void) -> u32>;
pub type LPDDARGB = *mut DDARGB;
#[cfg(feature = "Win32_windef")]
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
pub type LPDDDEVICEIDENTIFIER = *mut DDDEVICEIDENTIFIER;
pub type LPDDDEVICEIDENTIFIER2 = *mut DDDEVICEIDENTIFIER2;
pub type LPDDENUMCALLBACK = LPDDENUMCALLBACKA;
pub type LPDDENUMCALLBACKA = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_windef")]
pub type LPDDENUMCALLBACKEX = LPDDENUMCALLBACKEXA;
#[cfg(feature = "Win32_windef")]
pub type LPDDENUMCALLBACKEXA = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: *mut core::ffi::c_void, param4: super::windef::HMONITOR) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_windef")]
pub type LPDDENUMCALLBACKEXW = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: *mut core::ffi::c_void, param4: super::windef::HMONITOR) -> windows_sys::core::BOOL>;
pub type LPDDENUMCALLBACKW = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_ksmedia")]
pub type LPDDENUMMODESCALLBACK = Option<unsafe extern "system" fn(param0: *mut DDSURFACEDESC, param1: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_ksmedia")]
pub type LPDDENUMMODESCALLBACK2 = Option<unsafe extern "system" fn(param0: *mut DDSURFACEDESC2, param1: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_ksmedia")]
pub type LPDDENUMSURFACESCALLBACK = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_ksmedia")]
pub type LPDDENUMSURFACESCALLBACK2 = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_ksmedia")]
pub type LPDDENUMSURFACESCALLBACK7 = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type LPDDFXROP = *mut _DDFXROP;
pub type LPDDGAMMARAMP = *mut DDGAMMARAMP;
pub type LPDDOSCAPS = *mut DDOSCAPS;
pub type LPDDOVERLAYFX = *mut DDOVERLAYFX;
pub type LPDDRGBA = *mut DDRGBA;
pub type LPDDSCAPS = *mut DDSCAPS;
pub type LPDDSCAPS2 = *mut DDSCAPS2;
pub type LPDDSCAPSEX = *mut DDSCAPSEX;
#[cfg(feature = "Win32_ksmedia")]
pub type LPDDSURFACEDESC = *mut DDSURFACEDESC;
#[cfg(feature = "Win32_ksmedia")]
pub type LPDDSURFACEDESC2 = *mut DDSURFACEDESC2;
#[cfg(feature = "Win32_windef")]
pub type LPDIRECTDRAWENUMERATEEX = LPDIRECTDRAWENUMERATEEXA;
#[cfg(feature = "Win32_windef")]
pub type LPDIRECTDRAWENUMERATEEXA = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: *mut core::ffi::c_void, dwflags: u32) -> windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_windef")]
pub type LPDIRECTDRAWENUMERATEEXW = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXW, lpcontext: *mut core::ffi::c_void, dwflags: u32) -> windows_sys::core::HRESULT>;
pub const MAX_DDDEVICEID_STRING: u32 = 512;
pub const REGSTR_KEY_DDHW_DESCRIPTION: windows_sys::core::PCSTR = windows_sys::core::s!("Description");
pub const REGSTR_KEY_DDHW_DRIVERNAME: windows_sys::core::PCSTR = windows_sys::core::s!("DriverName");
pub const REGSTR_PATH_DDHW: windows_sys::core::PCSTR = windows_sys::core::s!("Hardware\\DirectDrawDrivers");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _DDFXROP(pub u8);
