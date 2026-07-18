#[inline]
pub unsafe fn DirectDrawCreate<P2>(lpguid: *mut windows_core::GUID, lplpdd: *mut Option<IDirectDraw>, punkouter: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ddraw.dll" "system" fn DirectDrawCreate(lpguid : *mut windows_core::GUID, lplpdd : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectDrawCreate(lpguid as _, core::mem::transmute(lplpdd), punkouter.param().abi()) }
}
#[inline]
pub unsafe fn DirectDrawCreateClipper<P2>(dwflags: u32, lplpddclipper: *mut Option<IDirectDrawClipper>, punkouter: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ddraw.dll" "system" fn DirectDrawCreateClipper(dwflags : u32, lplpddclipper : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectDrawCreateClipper(dwflags, core::mem::transmute(lplpddclipper), punkouter.param().abi()) }
}
#[inline]
pub unsafe fn DirectDrawCreateEx<P3, T>(lpguid: *mut windows_core::GUID, punkouter: P3) -> windows_core::Result<T>
where
    P3: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ddraw.dll" "system" fn DirectDrawCreateEx(lpguid : *mut windows_core::GUID, lplpdd : *mut *mut core::ffi::c_void, iid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DirectDrawCreateEx(lpguid as _, &mut result__, &T::IID, punkouter.param().abi()).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn DirectDrawEnumerateA(lpcallback: LPDDENUMCALLBACKA, lpcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ddraw.dll" "system" fn DirectDrawEnumerateA(lpcallback : LPDDENUMCALLBACKA, lpcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectDrawEnumerateA(lpcallback, lpcontext as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DirectDrawEnumerateExA(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("ddraw.dll" "system" fn DirectDrawEnumerateExA(lpcallback : LPDDENUMCALLBACKEXA, lpcontext : *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { DirectDrawEnumerateExA(lpcallback, lpcontext as _, dwflags) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DirectDrawEnumerateExW(lpcallback: LPDDENUMCALLBACKEXW, lpcontext: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("ddraw.dll" "system" fn DirectDrawEnumerateExW(lpcallback : LPDDENUMCALLBACKEXW, lpcontext : *mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    unsafe { DirectDrawEnumerateExW(lpcallback, lpcontext as _, dwflags) }
}
#[inline]
pub unsafe fn DirectDrawEnumerateW(lpcallback: LPDDENUMCALLBACKW, lpcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ddraw.dll" "system" fn DirectDrawEnumerateW(lpcallback : LPDDENUMCALLBACKW, lpcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectDrawEnumerateW(lpcallback, lpcontext as _) }
}
pub const CLSID_DirectDraw: windows_core::GUID = windows_core::GUID::from_u128(0xd7b70ee0_4340_11cf_b063_0020afc2cd35);
pub const CLSID_DirectDraw7: windows_core::GUID = windows_core::GUID::from_u128(0x3c305196_50db_11d3_9cfe_00c04fd930c5);
pub const CLSID_DirectDrawClipper: windows_core::GUID = windows_core::GUID::from_u128(0x593817a0_7db3_11cf_a2de_00aa00b93356);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[cfg(feature = "windef")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DDBLTBATCH {
    pub lprDest: super::LPRECT,
    pub lpDDSSrc: core::mem::ManuallyDrop<Option<IDirectDrawSurface>>,
    pub lprSrc: super::LPRECT,
    pub dwFlags: u32,
    pub lpDDBltFx: LPDDBLTFX,
}
pub const DDBLTFAST_DESTCOLORKEY: u32 = 2;
pub const DDBLTFAST_DONOTWAIT: u32 = 32;
pub const DDBLTFAST_NOCOLORKEY: u32 = 0;
pub const DDBLTFAST_SRCCOLORKEY: u32 = 1;
pub const DDBLTFAST_WAIT: u32 = 16;
#[repr(C)]
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
impl Clone for DDBLTFX {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for DDBLTFX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DDBLTFX_0 {
    pub dwZDestConst: u32,
    pub lpDDSZBufferDest: core::mem::ManuallyDrop<Option<IDirectDrawSurface>>,
}
impl Clone for DDBLTFX_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for DDBLTFX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DDBLTFX_1 {
    pub dwZSrcConst: u32,
    pub lpDDSZBufferSrc: core::mem::ManuallyDrop<Option<IDirectDrawSurface>>,
}
impl Clone for DDBLTFX_1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for DDBLTFX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DDBLTFX_2 {
    pub dwAlphaDestConst: u32,
    pub lpDDSAlphaDest: core::mem::ManuallyDrop<Option<IDirectDrawSurface>>,
}
impl Clone for DDBLTFX_2 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for DDBLTFX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DDBLTFX_3 {
    pub dwAlphaSrcConst: u32,
    pub lpDDSAlphaSrc: core::mem::ManuallyDrop<Option<IDirectDrawSurface>>,
}
impl Clone for DDBLTFX_3 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for DDBLTFX_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DDBLTFX_4 {
    pub dwFillColor: u32,
    pub dwFillDepth: u32,
    pub dwFillPixel: u32,
    pub lpDDSPattern: core::mem::ManuallyDrop<Option<IDirectDrawSurface>>,
}
impl Clone for DDBLTFX_4 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDDEVICEIDENTIFIER {
    pub szDriver: [i8; 512],
    pub szDescription: [i8; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: windows_core::GUID,
}
impl Default for DDDEVICEIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DDDEVICEIDENTIFIER2 {
    pub szDriver: [i8; 512],
    pub szDescription: [i8; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub guid: windows_core::GUID,
    pub dwCompressionRatio: u32,
}
impl Default for DDOPTSURFACEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
impl Clone for DDOVERLAYFX {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for DDOVERLAYFX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DDOVERLAYFX_0 {
    pub dwAlphaDestConst: u32,
    pub lpDDSAlphaDest: core::mem::ManuallyDrop<Option<IDirectDrawSurface>>,
}
impl Clone for DDOVERLAYFX_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for DDOVERLAYFX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DDOVERLAYFX_1 {
    pub dwAlphaSrcConst: u32,
    pub lpDDSAlphaSrc: core::mem::ManuallyDrop<Option<IDirectDrawSurface>>,
}
impl Clone for DDOVERLAYFX_1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DDRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
windows_core::imp::define_interface!(IDirectDraw, IDirectDraw_Vtbl, 0x6c14db80_a733_11ce_a521_0020af0be560);
windows_core::imp::interface_hierarchy!(IDirectDraw, windows_core::IUnknown);
impl IDirectDraw {
    pub unsafe fn Compact(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateClipper<P2>(&self, param0: u32, param1: *mut Option<IDirectDrawClipper>, param2: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClipper)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2.param().abi()) }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn CreatePalette<P3>(&self, param0: u32, param1: *mut super::PALETTEENTRY, param2: *mut Option<IDirectDrawPalette>, param3: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePalette)(windows_core::Interface::as_raw(self), param0, param1 as _, core::mem::transmute(param2), param3.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn CreateSurface<P2>(&self, param0: *mut DDSURFACEDESC, param1: *mut Option<IDirectDrawSurface>, param2: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1), param2.param().abi()) }
    }
    pub unsafe fn DuplicateSurface<P0>(&self, param0: P0) -> windows_core::Result<IDirectDrawSurface>
    where
        P0: windows_core::Param<IDirectDrawSurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateSurface)(windows_core::Interface::as_raw(self), param0.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumDisplayModes)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumSurfaces)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3) }
    }
    pub unsafe fn FlipToGDISurface(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlipToGDISurface)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFourCCCodes)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGDISurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMonitorFrequency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMonitorFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScanLine(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScanLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVerticalBlankStatus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVerticalBlankStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Initialize(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreDisplayMode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreDisplayMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetCooperativeLevel(&self, param0: super::HWND, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayMode)(windows_core::Interface::as_raw(self), param0, param1, param2) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn WaitForVerticalBlank(&self, param0: u32, param1: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForVerticalBlank)(windows_core::Interface::as_raw(self), param0, param1) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClipper: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub CreatePalette: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::PALETTEENTRY, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    CreatePalette: usize,
    #[cfg(feature = "ksmedia")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    CreateSurface: usize,
    pub DuplicateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumDisplayModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDSURFACEDESC, *mut core::ffi::c_void, LPDDENUMMODESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumDisplayModes: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDSURFACEDESC, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumSurfaces: usize,
    pub FlipToGDISurface: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDCAPS, *mut DDCAPS) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetDisplayMode: usize,
    pub GetFourCCCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetGDISurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMonitorFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetScanLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetVerticalBlankStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RestoreDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetCooperativeLevel: usize,
    pub SetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub WaitForVerticalBlank: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    WaitForVerticalBlank: usize,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub trait IDirectDraw_Impl: windows_core::IUnknownImpl {
    fn Compact(&self) -> windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: windows_core::OutRef<IDirectDrawClipper>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::PALETTEENTRY, param2: windows_core::OutRef<IDirectDrawPalette>, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC, param1: windows_core::OutRef<IDirectDrawSurface>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: windows_core::Ref<IDirectDrawSurface>) -> windows_core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::Result<()>;
    fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(&self) -> windows_core::Result<u32>;
    fn GetScanLine(&self) -> windows_core::Result<u32>;
    fn GetVerticalBlankStatus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Initialize(&self) -> windows_core::Result<windows_core::GUID>;
    fn RestoreDisplayMode(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::HWND, param1: u32) -> windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl IDirectDraw_Vtbl {
    pub const fn new<Identity: IDirectDraw_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compact<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::Compact(this).into()
            }
        }
        unsafe extern "system" fn CreateClipper<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::CreateClipper(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn CreatePalette<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut super::PALETTEENTRY, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::CreatePalette(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::CreateSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn DuplicateSurface<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw_Impl::DuplicateSurface(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::EnumDisplayModes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumSurfaces<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::EnumSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::FlipToGDISurface(this).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::GetCaps(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::GetDisplayMode(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetGDISurface<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw_Impl::GetGDISurface(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw_Impl::GetMonitorFrequency(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScanLine<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw_Impl::GetScanLine(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw_Impl::GetVerticalBlankStatus(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw_Impl::Initialize(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::RestoreDisplayMode(this).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetDisplayMode<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::SetDisplayMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: IDirectDraw_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw_Impl::WaitForVerticalBlank(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, OFFSET>,
            CreateClipper: CreateClipper::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, OFFSET>,
            GetScanLine: GetScanLine::<Identity, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDraw as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDraw {}
windows_core::imp::define_interface!(IDirectDraw2, IDirectDraw2_Vtbl, 0xb3a6f3e0_2b43_11cf_a2de_00aa00b93356);
windows_core::imp::interface_hierarchy!(IDirectDraw2, windows_core::IUnknown);
impl IDirectDraw2 {
    pub unsafe fn Compact(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateClipper<P2>(&self, param0: u32, param1: *mut Option<IDirectDrawClipper>, param2: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClipper)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2.param().abi()) }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn CreatePalette<P3>(&self, param0: u32, param1: *mut super::PALETTEENTRY, param2: *mut Option<IDirectDrawPalette>, param3: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePalette)(windows_core::Interface::as_raw(self), param0, param1 as _, core::mem::transmute(param2), param3.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn CreateSurface<P2>(&self, param0: *mut DDSURFACEDESC, param1: *mut Option<IDirectDrawSurface>, param2: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1), param2.param().abi()) }
    }
    pub unsafe fn DuplicateSurface<P0>(&self, param0: P0) -> windows_core::Result<IDirectDrawSurface>
    where
        P0: windows_core::Param<IDirectDrawSurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateSurface)(windows_core::Interface::as_raw(self), param0.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumDisplayModes)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumSurfaces)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3) }
    }
    pub unsafe fn FlipToGDISurface(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlipToGDISurface)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFourCCCodes)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGDISurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMonitorFrequency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMonitorFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScanLine(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScanLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVerticalBlankStatus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVerticalBlankStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Initialize(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreDisplayMode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreDisplayMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetCooperativeLevel(&self, param0: super::HWND, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayMode)(windows_core::Interface::as_raw(self), param0, param1, param2, param3, param4) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn WaitForVerticalBlank(&self, param0: u32, param1: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForVerticalBlank)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableVidMem)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2 as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClipper: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub CreatePalette: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::PALETTEENTRY, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    CreatePalette: usize,
    #[cfg(feature = "ksmedia")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    CreateSurface: usize,
    pub DuplicateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumDisplayModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDSURFACEDESC, *mut core::ffi::c_void, LPDDENUMMODESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumDisplayModes: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDSURFACEDESC, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumSurfaces: usize,
    pub FlipToGDISurface: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDCAPS, *mut DDCAPS) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetDisplayMode: usize,
    pub GetFourCCCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetGDISurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMonitorFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetScanLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetVerticalBlankStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RestoreDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetCooperativeLevel: usize,
    pub SetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub WaitForVerticalBlank: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    WaitForVerticalBlank: usize,
    pub GetAvailableVidMem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS, *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub trait IDirectDraw2_Impl: windows_core::IUnknownImpl {
    fn Compact(&self) -> windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: windows_core::OutRef<IDirectDrawClipper>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::PALETTEENTRY, param2: windows_core::OutRef<IDirectDrawPalette>, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC, param1: windows_core::OutRef<IDirectDrawSurface>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: windows_core::Ref<IDirectDrawSurface>) -> windows_core::Result<IDirectDrawSurface>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::Result<()>;
    fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface>;
    fn GetMonitorFrequency(&self) -> windows_core::Result<u32>;
    fn GetScanLine(&self) -> windows_core::Result<u32>;
    fn GetVerticalBlankStatus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Initialize(&self) -> windows_core::Result<windows_core::GUID>;
    fn RestoreDisplayMode(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::HWND, param1: u32) -> windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::HANDLE) -> windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl IDirectDraw2_Vtbl {
    pub const fn new<Identity: IDirectDraw2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compact<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::Compact(this).into()
            }
        }
        unsafe extern "system" fn CreateClipper<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::CreateClipper(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn CreatePalette<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut super::PALETTEENTRY, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::CreatePalette(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::CreateSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn DuplicateSurface<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw2_Impl::DuplicateSurface(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::EnumDisplayModes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumSurfaces<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::EnumSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::FlipToGDISurface(this).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::GetCaps(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::GetDisplayMode(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetGDISurface<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw2_Impl::GetGDISurface(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw2_Impl::GetMonitorFrequency(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScanLine<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw2_Impl::GetScanLine(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw2_Impl::GetVerticalBlankStatus(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw2_Impl::Initialize(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::RestoreDisplayMode(this).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetDisplayMode<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::SetDisplayMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::WaitForVerticalBlank(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: IDirectDraw2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw2_Impl::GetAvailableVidMem(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, OFFSET>,
            CreateClipper: CreateClipper::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, OFFSET>,
            GetScanLine: GetScanLine::<Identity, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDraw2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDraw2 {}
windows_core::imp::define_interface!(IDirectDraw4, IDirectDraw4_Vtbl, 0x9c59509a_39bd_11d1_8c4a_00c04fd930c5);
windows_core::imp::interface_hierarchy!(IDirectDraw4, windows_core::IUnknown);
impl IDirectDraw4 {
    pub unsafe fn Compact(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateClipper<P2>(&self, param0: u32, param1: *mut Option<IDirectDrawClipper>, param2: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClipper)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2.param().abi()) }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn CreatePalette<P3>(&self, param0: u32, param1: *mut super::PALETTEENTRY, param2: *mut Option<IDirectDrawPalette>, param3: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePalette)(windows_core::Interface::as_raw(self), param0, param1 as _, core::mem::transmute(param2), param3.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn CreateSurface<P2>(&self, param0: *mut DDSURFACEDESC2, param1: *mut Option<IDirectDrawSurface4>, param2: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1), param2.param().abi()) }
    }
    pub unsafe fn DuplicateSurface<P0>(&self, param0: P0) -> windows_core::Result<IDirectDrawSurface4>
    where
        P0: windows_core::Param<IDirectDrawSurface4>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateSurface)(windows_core::Interface::as_raw(self), param0.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumDisplayModes)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumSurfaces)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3) }
    }
    pub unsafe fn FlipToGDISurface(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlipToGDISurface)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFourCCCodes)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGDISurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMonitorFrequency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMonitorFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScanLine(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScanLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVerticalBlankStatus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVerticalBlankStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Initialize(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreDisplayMode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreDisplayMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetCooperativeLevel(&self, param0: super::HWND, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayMode)(windows_core::Interface::as_raw(self), param0, param1, param2, param3, param4) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn WaitForVerticalBlank(&self, param0: u32, param1: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForVerticalBlank)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableVidMem)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2 as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetSurfaceFromDC(&self, param0: super::HDC) -> windows_core::Result<IDirectDrawSurface4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurfaceFromDC)(windows_core::Interface::as_raw(self), param0, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RestoreAllSurfaces(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreAllSurfaces)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TestCooperativeLevel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestCooperativeLevel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceIdentifier)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClipper: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub CreatePalette: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::PALETTEENTRY, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    CreatePalette: usize,
    #[cfg(feature = "ksmedia")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC2, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    CreateSurface: usize,
    pub DuplicateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumDisplayModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDSURFACEDESC2, *mut core::ffi::c_void, LPDDENUMMODESCALLBACK2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumDisplayModes: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDSURFACEDESC2, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumSurfaces: usize,
    pub FlipToGDISurface: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDCAPS, *mut DDCAPS) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetDisplayMode: usize,
    pub GetFourCCCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetGDISurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMonitorFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetScanLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetVerticalBlankStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RestoreDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetCooperativeLevel: usize,
    pub SetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub WaitForVerticalBlank: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    WaitForVerticalBlank: usize,
    pub GetAvailableVidMem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS2, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetSurfaceFromDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetSurfaceFromDC: usize,
    pub RestoreAllSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TestCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDDEVICEIDENTIFIER, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub trait IDirectDraw4_Impl: windows_core::IUnknownImpl {
    fn Compact(&self) -> windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: windows_core::OutRef<IDirectDrawClipper>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::PALETTEENTRY, param2: windows_core::OutRef<IDirectDrawPalette>, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC2, param1: windows_core::OutRef<IDirectDrawSurface4>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: windows_core::Ref<IDirectDrawSurface4>) -> windows_core::Result<IDirectDrawSurface4>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::Result<()>;
    fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface4>;
    fn GetMonitorFrequency(&self) -> windows_core::Result<u32>;
    fn GetScanLine(&self) -> windows_core::Result<u32>;
    fn GetVerticalBlankStatus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Initialize(&self) -> windows_core::Result<windows_core::GUID>;
    fn RestoreDisplayMode(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::HWND, param1: u32) -> windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::HANDLE) -> windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::Result<()>;
    fn GetSurfaceFromDC(&self, param0: super::HDC) -> windows_core::Result<IDirectDrawSurface4>;
    fn RestoreAllSurfaces(&self) -> windows_core::Result<()>;
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl IDirectDraw4_Vtbl {
    pub const fn new<Identity: IDirectDraw4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compact<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::Compact(this).into()
            }
        }
        unsafe extern "system" fn CreateClipper<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::CreateClipper(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn CreatePalette<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut super::PALETTEENTRY, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::CreatePalette(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::CreateSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn DuplicateSurface<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw4_Impl::DuplicateSurface(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::EnumDisplayModes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumSurfaces<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::EnumSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::FlipToGDISurface(this).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::GetCaps(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::GetDisplayMode(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetGDISurface<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw4_Impl::GetGDISurface(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw4_Impl::GetMonitorFrequency(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScanLine<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw4_Impl::GetScanLine(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw4_Impl::GetVerticalBlankStatus(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw4_Impl::Initialize(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::RestoreDisplayMode(this).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetDisplayMode<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::SetDisplayMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::WaitForVerticalBlank(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::GetAvailableVidMem(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HDC, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw4_Impl::GetSurfaceFromDC(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::RestoreAllSurfaces(this).into()
            }
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::TestCooperativeLevel(this).into()
            }
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: IDirectDraw4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw4_Impl::GetDeviceIdentifier(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, OFFSET>,
            CreateClipper: CreateClipper::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, OFFSET>,
            GetScanLine: GetScanLine::<Identity, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDraw4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDraw4 {}
windows_core::imp::define_interface!(IDirectDraw7, IDirectDraw7_Vtbl, 0x15e65ec0_3b9c_11d2_b92f_00609797ea5b);
windows_core::imp::interface_hierarchy!(IDirectDraw7, windows_core::IUnknown);
impl IDirectDraw7 {
    pub unsafe fn Compact(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateClipper<P2>(&self, param0: u32, param1: *mut Option<IDirectDrawClipper>, param2: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClipper)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2.param().abi()) }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn CreatePalette<P3>(&self, param0: u32, param1: *mut super::PALETTEENTRY, param2: *mut Option<IDirectDrawPalette>, param3: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePalette)(windows_core::Interface::as_raw(self), param0, param1 as _, core::mem::transmute(param2), param3.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn CreateSurface<P2>(&self, param0: *mut DDSURFACEDESC2, param1: *mut Option<IDirectDrawSurface7>, param2: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1), param2.param().abi()) }
    }
    pub unsafe fn DuplicateSurface<P0>(&self, param0: P0) -> windows_core::Result<IDirectDrawSurface7>
    where
        P0: windows_core::Param<IDirectDrawSurface7>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateSurface)(windows_core::Interface::as_raw(self), param0.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumDisplayModes)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumSurfaces)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3) }
    }
    pub unsafe fn FlipToGDISurface(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlipToGDISurface)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFourCCCodes)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface7> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGDISurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMonitorFrequency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMonitorFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScanLine(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScanLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVerticalBlankStatus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVerticalBlankStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Initialize(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RestoreDisplayMode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreDisplayMode)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetCooperativeLevel(&self, param0: super::HWND, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayMode)(windows_core::Interface::as_raw(self), param0, param1, param2, param3, param4) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn WaitForVerticalBlank(&self, param0: u32, param1: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForVerticalBlank)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableVidMem)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2 as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetSurfaceFromDC(&self, param0: super::HDC) -> windows_core::Result<IDirectDrawSurface7> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurfaceFromDC)(windows_core::Interface::as_raw(self), param0, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RestoreAllSurfaces(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreAllSurfaces)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TestCooperativeLevel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestCooperativeLevel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceIdentifier)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn StartModeTest(&self, param0: *mut super::SIZE, param1: u32, param2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartModeTest)(windows_core::Interface::as_raw(self), param0 as _, param1, param2) }
    }
    pub unsafe fn EvaluateMode(&self, param0: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EvaluateMode)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw7_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClipper: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub CreatePalette: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::PALETTEENTRY, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    CreatePalette: usize,
    #[cfg(feature = "ksmedia")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC2, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    CreateSurface: usize,
    pub DuplicateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumDisplayModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDSURFACEDESC2, *mut core::ffi::c_void, LPDDENUMMODESCALLBACK2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumDisplayModes: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDSURFACEDESC2, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumSurfaces: usize,
    pub FlipToGDISurface: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDCAPS, *mut DDCAPS) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetDisplayMode: usize,
    pub GetFourCCCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetGDISurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMonitorFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetScanLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetVerticalBlankStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RestoreDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetCooperativeLevel: usize,
    pub SetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub WaitForVerticalBlank: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    WaitForVerticalBlank: usize,
    pub GetAvailableVidMem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS2, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetSurfaceFromDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetSurfaceFromDC: usize,
    pub RestoreAllSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TestCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDDEVICEIDENTIFIER2, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub StartModeTest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SIZE, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    StartModeTest: usize,
    pub EvaluateMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub trait IDirectDraw7_Impl: windows_core::IUnknownImpl {
    fn Compact(&self) -> windows_core::Result<()>;
    fn CreateClipper(&self, param0: u32, param1: windows_core::OutRef<IDirectDrawClipper>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreatePalette(&self, param0: u32, param1: *mut super::PALETTEENTRY, param2: windows_core::OutRef<IDirectDrawPalette>, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateSurface(&self, param0: *mut DDSURFACEDESC2, param1: windows_core::OutRef<IDirectDrawSurface7>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DuplicateSurface(&self, param0: windows_core::Ref<IDirectDrawSurface7>) -> windows_core::Result<IDirectDrawSurface7>;
    fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::Result<()>;
    fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> windows_core::Result<()>;
    fn FlipToGDISurface(&self) -> windows_core::Result<()>;
    fn GetCaps(&self, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> windows_core::Result<()>;
    fn GetGDISurface(&self) -> windows_core::Result<IDirectDrawSurface7>;
    fn GetMonitorFrequency(&self) -> windows_core::Result<u32>;
    fn GetScanLine(&self) -> windows_core::Result<u32>;
    fn GetVerticalBlankStatus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Initialize(&self) -> windows_core::Result<windows_core::GUID>;
    fn RestoreDisplayMode(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::HWND, param1: u32) -> windows_core::Result<()>;
    fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::Result<()>;
    fn WaitForVerticalBlank(&self, param0: u32, param1: super::HANDLE) -> windows_core::Result<()>;
    fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::Result<()>;
    fn GetSurfaceFromDC(&self, param0: super::HDC) -> windows_core::Result<IDirectDrawSurface7>;
    fn RestoreAllSurfaces(&self) -> windows_core::Result<()>;
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> windows_core::Result<()>;
    fn StartModeTest(&self, param0: *mut super::SIZE, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn EvaluateMode(&self, param0: u32) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl IDirectDraw7_Vtbl {
    pub const fn new<Identity: IDirectDraw7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Compact<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::Compact(this).into()
            }
        }
        unsafe extern "system" fn CreateClipper<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::CreateClipper(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn CreatePalette<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut super::PALETTEENTRY, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::CreatePalette(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::CreateSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn DuplicateSurface<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw7_Impl::DuplicateSurface(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDisplayModes<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::EnumDisplayModes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumSurfaces<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::EnumSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn FlipToGDISurface<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::FlipToGDISurface(this).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCAPS, param1: *mut DDCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::GetCaps(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::GetDisplayMode(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetGDISurface<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw7_Impl::GetGDISurface(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMonitorFrequency<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw7_Impl::GetMonitorFrequency(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScanLine<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw7_Impl::GetScanLine(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVerticalBlankStatus<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw7_Impl::GetVerticalBlankStatus(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw7_Impl::Initialize(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreDisplayMode<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::RestoreDisplayMode(this).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetDisplayMode<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::SetDisplayMode(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn WaitForVerticalBlank<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::WaitForVerticalBlank(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetAvailableVidMem<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::GetAvailableVidMem(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceFromDC<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HDC, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw7_Impl::GetSurfaceFromDC(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RestoreAllSurfaces<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::RestoreAllSurfaces(this).into()
            }
        }
        unsafe extern "system" fn TestCooperativeLevel<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::TestCooperativeLevel(this).into()
            }
        }
        unsafe extern "system" fn GetDeviceIdentifier<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::GetDeviceIdentifier(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn StartModeTest<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::SIZE, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDraw7_Impl::StartModeTest(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EvaluateMode<Identity: IDirectDraw7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDraw7_Impl::EvaluateMode(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Compact: Compact::<Identity, OFFSET>,
            CreateClipper: CreateClipper::<Identity, OFFSET>,
            CreatePalette: CreatePalette::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            DuplicateSurface: DuplicateSurface::<Identity, OFFSET>,
            EnumDisplayModes: EnumDisplayModes::<Identity, OFFSET>,
            EnumSurfaces: EnumSurfaces::<Identity, OFFSET>,
            FlipToGDISurface: FlipToGDISurface::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            GetGDISurface: GetGDISurface::<Identity, OFFSET>,
            GetMonitorFrequency: GetMonitorFrequency::<Identity, OFFSET>,
            GetScanLine: GetScanLine::<Identity, OFFSET>,
            GetVerticalBlankStatus: GetVerticalBlankStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            RestoreDisplayMode: RestoreDisplayMode::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SetDisplayMode: SetDisplayMode::<Identity, OFFSET>,
            WaitForVerticalBlank: WaitForVerticalBlank::<Identity, OFFSET>,
            GetAvailableVidMem: GetAvailableVidMem::<Identity, OFFSET>,
            GetSurfaceFromDC: GetSurfaceFromDC::<Identity, OFFSET>,
            RestoreAllSurfaces: RestoreAllSurfaces::<Identity, OFFSET>,
            TestCooperativeLevel: TestCooperativeLevel::<Identity, OFFSET>,
            GetDeviceIdentifier: GetDeviceIdentifier::<Identity, OFFSET>,
            StartModeTest: StartModeTest::<Identity, OFFSET>,
            EvaluateMode: EvaluateMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDraw7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDraw7 {}
windows_core::imp::define_interface!(IDirectDrawClipper, IDirectDrawClipper_Vtbl, 0x6c14db85_a733_11ce_a521_0020af0be560);
windows_core::imp::interface_hierarchy!(IDirectDrawClipper, windows_core::IUnknown);
impl IDirectDrawClipper {
    #[cfg(all(feature = "windef", feature = "wingdi"))]
    pub unsafe fn GetClipList(&self, param0: *mut super::RECT, param1: *mut super::RGNDATA, param2: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClipList)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2 as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetHWnd(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHWnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDraw>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0.param().abi(), param1) }
    }
    pub unsafe fn IsClipListChanged(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsClipListChanged)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "windef", feature = "wingdi"))]
    pub unsafe fn SetClipList(&self, param0: *mut super::RGNDATA, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipList)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetHWnd(&self, param0: u32, param1: super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHWnd)(windows_core::Interface::as_raw(self), param0, param1) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawClipper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "windef", feature = "wingdi"))]
    pub GetClipList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut super::RGNDATA, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "windef", feature = "wingdi")))]
    GetClipList: usize,
    #[cfg(feature = "windef")]
    pub GetHWnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetHWnd: usize,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsClipListChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "windef", feature = "wingdi"))]
    pub SetClipList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RGNDATA, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "windef", feature = "wingdi")))]
    SetClipList: usize,
    #[cfg(feature = "windef")]
    pub SetHWnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetHWnd: usize,
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
pub trait IDirectDrawClipper_Impl: windows_core::IUnknownImpl {
    fn GetClipList(&self, param0: *mut super::RECT, param1: *mut super::RGNDATA, param2: *mut u32) -> windows_core::Result<()>;
    fn GetHWnd(&self) -> windows_core::Result<super::HWND>;
    fn Initialize(&self, param0: windows_core::Ref<IDirectDraw>, param1: u32) -> windows_core::Result<()>;
    fn IsClipListChanged(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetClipList(&self, param0: *mut super::RGNDATA, param1: u32) -> windows_core::Result<()>;
    fn SetHWnd(&self, param0: u32, param1: super::HWND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl IDirectDrawClipper_Vtbl {
    pub const fn new<Identity: IDirectDrawClipper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClipList<Identity: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut super::RGNDATA, param2: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawClipper_Impl::GetClipList(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetHWnd<Identity: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawClipper_Impl::GetHWnd(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawClipper_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn IsClipListChanged<Identity: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawClipper_Impl::IsClipListChanged(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipList<Identity: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RGNDATA, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawClipper_Impl::SetClipList(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetHWnd<Identity: IDirectDrawClipper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawClipper_Impl::SetHWnd(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClipList: GetClipList::<Identity, OFFSET>,
            GetHWnd: GetHWnd::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsClipListChanged: IsClipListChanged::<Identity, OFFSET>,
            SetClipList: SetClipList::<Identity, OFFSET>,
            SetHWnd: SetHWnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawClipper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IDirectDrawClipper {}
windows_core::imp::define_interface!(IDirectDrawColorControl, IDirectDrawColorControl_Vtbl, 0x4b9f0ee0_0d7e_11d0_9b06_00a0c903a3b8);
windows_core::imp::interface_hierarchy!(IDirectDrawColorControl, windows_core::IUnknown);
impl IDirectDrawColorControl {
    pub unsafe fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetColorControls)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorControls)(windows_core::Interface::as_raw(self), param0 as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawColorControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetColorControls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDCOLORCONTROL) -> windows_core::HRESULT,
    pub SetColorControls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDCOLORCONTROL) -> windows_core::HRESULT,
}
pub trait IDirectDrawColorControl_Impl: windows_core::IUnknownImpl {
    fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> windows_core::Result<()>;
    fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> windows_core::Result<()>;
}
impl IDirectDrawColorControl_Vtbl {
    pub const fn new<Identity: IDirectDrawColorControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColorControls<Identity: IDirectDrawColorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawColorControl_Impl::GetColorControls(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetColorControls<Identity: IDirectDrawColorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawColorControl_Impl::SetColorControls(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColorControls: GetColorControls::<Identity, OFFSET>,
            SetColorControls: SetColorControls::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawColorControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectDrawColorControl {}
windows_core::imp::define_interface!(IDirectDrawGammaControl, IDirectDrawGammaControl_Vtbl, 0x69c11c3e_b46b_11d1_ad7a_00c04fc29b4e);
windows_core::imp::interface_hierarchy!(IDirectDrawGammaControl, windows_core::IUnknown);
impl IDirectDrawGammaControl {
    pub unsafe fn GetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGammaRamp)(windows_core::Interface::as_raw(self), param0, param1 as _) }
    }
    pub unsafe fn SetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGammaRamp)(windows_core::Interface::as_raw(self), param0, param1 as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawGammaControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDGAMMARAMP) -> windows_core::HRESULT,
    pub SetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDGAMMARAMP) -> windows_core::HRESULT,
}
pub trait IDirectDrawGammaControl_Impl: windows_core::IUnknownImpl {
    fn GetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::Result<()>;
    fn SetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::Result<()>;
}
impl IDirectDrawGammaControl_Vtbl {
    pub const fn new<Identity: IDirectDrawGammaControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGammaRamp<Identity: IDirectDrawGammaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawGammaControl_Impl::GetGammaRamp(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetGammaRamp<Identity: IDirectDrawGammaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawGammaControl_Impl::SetGammaRamp(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGammaRamp: GetGammaRamp::<Identity, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawGammaControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectDrawGammaControl {}
windows_core::imp::define_interface!(IDirectDrawPalette, IDirectDrawPalette_Vtbl, 0x6c14db84_a733_11ce_a521_0020af0be560);
windows_core::imp::interface_hierarchy!(IDirectDrawPalette, windows_core::IUnknown);
impl IDirectDrawPalette {
    pub unsafe fn GetCaps(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn GetEntries(&self, param0: u32, param1: u32, param2: u32) -> windows_core::Result<super::PALETTEENTRY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntries)(windows_core::Interface::as_raw(self), param0, param1, param2, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> windows_core::Result<super::PALETTEENTRY>
    where
        P0: windows_core::Param<IDirectDraw>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn SetEntries(&self, param0: u32, param1: u32, param2: u32) -> windows_core::Result<super::PALETTEENTRY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetEntries)(windows_core::Interface::as_raw(self), param0, param1, param2, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawPalette_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub GetEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut super::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    GetEntries: usize,
    #[cfg(feature = "wingdi")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    Initialize: usize,
    #[cfg(feature = "wingdi")]
    pub SetEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut super::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    SetEntries: usize,
}
#[cfg(feature = "wingdi")]
pub trait IDirectDrawPalette_Impl: windows_core::IUnknownImpl {
    fn GetCaps(&self) -> windows_core::Result<u32>;
    fn GetEntries(&self, param0: u32, param1: u32, param2: u32) -> windows_core::Result<super::PALETTEENTRY>;
    fn Initialize(&self, param0: windows_core::Ref<IDirectDraw>, param1: u32) -> windows_core::Result<super::PALETTEENTRY>;
    fn SetEntries(&self, param0: u32, param1: u32, param2: u32) -> windows_core::Result<super::PALETTEENTRY>;
}
#[cfg(feature = "wingdi")]
impl IDirectDrawPalette_Vtbl {
    pub const fn new<Identity: IDirectDrawPalette_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaps<Identity: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawPalette_Impl::GetCaps(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntries<Identity: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawPalette_Impl::GetEntries(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)) {
                    Ok(ok__) => {
                        param3.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32, param2: *mut super::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawPalette_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)) {
                    Ok(ok__) => {
                        param2.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEntries<Identity: IDirectDrawPalette_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawPalette_Impl::SetEntries(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)) {
                    Ok(ok__) => {
                        param3.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetEntries: GetEntries::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            SetEntries: SetEntries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawPalette as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wingdi")]
impl windows_core::RuntimeName for IDirectDrawPalette {}
windows_core::imp::define_interface!(IDirectDrawSurface, IDirectDrawSurface_Vtbl, 0x6c14db81_a733_11ce_a521_0020af0be560);
windows_core::imp::interface_hierarchy!(IDirectDrawSurface, windows_core::IUnknown);
impl IDirectDrawSurface {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAttachedSurface)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddOverlayDirtyRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Blt<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Blt)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BltBatch)(windows_core::Interface::as_raw(self), param0, param1, param2) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltFast<P2>(&self, param0: u32, param1: u32, param2: P2, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).BltFast)(windows_core::Interface::as_raw(self), param0, param1, param2.param().abi(), param3 as _, param4) }
    }
    pub unsafe fn DeleteAttachedSurface<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttachedSurface)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAttachedSurfaces)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumOverlayZOrders)(windows_core::Interface::as_raw(self), param0, param1 as _, param2) }
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Flip)(windows_core::Interface::as_raw(self), param0.param().abi(), param1) }
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttachedSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1)) }
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBltStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DDSCAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDC(&self) -> windows_core::Result<super::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFlipStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOverlayPosition)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPalette)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSurfaceDesc)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDraw>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _) }
    }
    pub unsafe fn IsLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsLost)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub unsafe fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2, param3) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseDC(&self, param0: super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn Restore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawClipper>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClipper)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    pub unsafe fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOverlayPosition)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    pub unsafe fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn UpdateOverlay<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlay)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayDisplay)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn UpdateOverlayZOrder<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayZOrder)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "windef")]
    pub Blt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDBLTFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Blt: usize,
    #[cfg(feature = "windef")]
    pub BltBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDBLTBATCH, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltBatch: usize,
    #[cfg(feature = "windef")]
    pub BltFast: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut super::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumAttachedSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumAttachedSurfaces: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumOverlayZOrders: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumOverlayZOrders: usize,
    pub Flip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS) -> windows_core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DDPIXELFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetPixelFormat: usize,
    #[cfg(feature = "ksmedia")]
    pub GetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetSurfaceDesc: usize,
    #[cfg(feature = "ksmedia")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    Initialize: usize,
    pub IsLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut DDSURFACEDESC, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "windef", feature = "winnt")))]
    Lock: usize,
    #[cfg(feature = "windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub UpdateOverlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDOVERLAYFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
pub trait IDirectDrawSurface_Impl: windows_core::IUnknownImpl {
    fn AddAttachedSurface(&self, param0: windows_core::Ref<IDirectDrawSurface>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT>;
    fn Blt(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface>, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: windows_core::Ref<IDirectDrawSurface>, param3: *mut super::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn Flip(&self, param0: windows_core::Ref<IDirectDrawSurface>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: windows_core::OutRef<IDirectDrawSurface>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self) -> windows_core::Result<DDSCAPS>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn GetDC(&self) -> windows_core::Result<super::HDC>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn Initialize(&self, param0: windows_core::Ref<IDirectDraw>, param1: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: windows_core::Ref<IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: windows_core::Ref<IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface>, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl IDirectDrawSurface_Vtbl {
    pub const fn new<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAttachedSurface<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::AddAttachedSurface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface_Impl::AddOverlayDirtyRect(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Blt<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::Blt(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn BltBatch<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn BltFast<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Flip<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::Flip(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetBltStatus<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface_Impl::GetCaps(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClipper<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface_Impl::GetClipper(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface_Impl::GetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDC<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface_Impl::GetDC(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlipStatus<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetPalette<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface_Impl::GetPalette(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn IsLost<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::IsLost(this).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::Restore(this).into()
            }
        }
        unsafe extern "system" fn SetClipper<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::SetClipper(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetColorKey<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface_Impl::SetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetPalette<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::SetPalette(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::Unlock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlay<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: IDirectDrawSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDrawSurface {}
windows_core::imp::define_interface!(IDirectDrawSurface2, IDirectDrawSurface2_Vtbl, 0x57805885_6eec_11cf_9441_a82303c10e27);
windows_core::imp::interface_hierarchy!(IDirectDrawSurface2, windows_core::IUnknown);
impl IDirectDrawSurface2 {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAttachedSurface)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddOverlayDirtyRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Blt<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Blt)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BltBatch)(windows_core::Interface::as_raw(self), param0, param1, param2) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltFast<P2>(&self, param0: u32, param1: u32, param2: P2, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).BltFast)(windows_core::Interface::as_raw(self), param0, param1, param2.param().abi(), param3 as _, param4) }
    }
    pub unsafe fn DeleteAttachedSurface<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttachedSurface)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAttachedSurfaces)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumOverlayZOrders)(windows_core::Interface::as_raw(self), param0, param1 as _, param2) }
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Flip)(windows_core::Interface::as_raw(self), param0.param().abi(), param1) }
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttachedSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1)) }
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBltStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DDSCAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDC(&self) -> windows_core::Result<super::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFlipStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOverlayPosition)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPalette)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSurfaceDesc)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDraw>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _) }
    }
    pub unsafe fn IsLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsLost)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub unsafe fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2, param3) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseDC(&self, param0: super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn Restore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawClipper>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClipper)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    pub unsafe fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOverlayPosition)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    pub unsafe fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn UpdateOverlay<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlay)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayDisplay)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn UpdateOverlayZOrder<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayZOrder)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDDInterface)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn PageLock(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PageLock)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn PageUnlock(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PageUnlock)(windows_core::Interface::as_raw(self), param0) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "windef")]
    pub Blt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDBLTFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Blt: usize,
    #[cfg(feature = "windef")]
    pub BltBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDBLTBATCH, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltBatch: usize,
    #[cfg(feature = "windef")]
    pub BltFast: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut super::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumAttachedSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumAttachedSurfaces: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumOverlayZOrders: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumOverlayZOrders: usize,
    pub Flip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS) -> windows_core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DDPIXELFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetPixelFormat: usize,
    #[cfg(feature = "ksmedia")]
    pub GetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetSurfaceDesc: usize,
    #[cfg(feature = "ksmedia")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    Initialize: usize,
    pub IsLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut DDSURFACEDESC, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "windef", feature = "winnt")))]
    Lock: usize,
    #[cfg(feature = "windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub UpdateOverlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDOVERLAYFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDDInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PageUnlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
pub trait IDirectDrawSurface2_Impl: windows_core::IUnknownImpl {
    fn AddAttachedSurface(&self, param0: windows_core::Ref<IDirectDrawSurface2>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT>;
    fn Blt(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface2>, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: windows_core::Ref<IDirectDrawSurface2>, param3: *mut super::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface2>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn Flip(&self, param0: windows_core::Ref<IDirectDrawSurface2>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: windows_core::OutRef<IDirectDrawSurface2>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self) -> windows_core::Result<DDSCAPS>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn GetDC(&self) -> windows_core::Result<super::HDC>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn Initialize(&self, param0: windows_core::Ref<IDirectDraw>, param1: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: windows_core::Ref<IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: windows_core::Ref<IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface2>, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface2>) -> windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl IDirectDrawSurface2_Vtbl {
    pub const fn new<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAttachedSurface<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::AddAttachedSurface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface2_Impl::AddOverlayDirtyRect(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Blt<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::Blt(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn BltBatch<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn BltFast<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Flip<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::Flip(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetBltStatus<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface2_Impl::GetCaps(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClipper<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface2_Impl::GetClipper(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface2_Impl::GetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDC<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface2_Impl::GetDC(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlipStatus<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetPalette<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface2_Impl::GetPalette(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn IsLost<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::IsLost(this).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::Restore(this).into()
            }
        }
        unsafe extern "system" fn SetClipper<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::SetClipper(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetColorKey<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface2_Impl::SetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetPalette<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::SetPalette(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::Unlock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlay<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDDInterface<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::GetDDInterface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn PageLock<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::PageLock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn PageUnlock<Identity: IDirectDrawSurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface2_Impl::PageUnlock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, OFFSET>,
            PageLock: PageLock::<Identity, OFFSET>,
            PageUnlock: PageUnlock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDrawSurface2 {}
windows_core::imp::define_interface!(IDirectDrawSurface3, IDirectDrawSurface3_Vtbl, 0xda044e00_69b2_11d0_a1d5_00aa00b8dfbb);
windows_core::imp::interface_hierarchy!(IDirectDrawSurface3, windows_core::IUnknown);
impl IDirectDrawSurface3 {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAttachedSurface)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddOverlayDirtyRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Blt<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Blt)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BltBatch)(windows_core::Interface::as_raw(self), param0, param1, param2) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltFast<P2>(&self, param0: u32, param1: u32, param2: P2, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).BltFast)(windows_core::Interface::as_raw(self), param0, param1, param2.param().abi(), param3 as _, param4) }
    }
    pub unsafe fn DeleteAttachedSurface<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttachedSurface)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAttachedSurfaces)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumOverlayZOrders)(windows_core::Interface::as_raw(self), param0, param1 as _, param2) }
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Flip)(windows_core::Interface::as_raw(self), param0.param().abi(), param1) }
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttachedSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1)) }
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBltStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DDSCAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDC(&self) -> windows_core::Result<super::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFlipStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOverlayPosition)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPalette)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSurfaceDesc)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDraw>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _) }
    }
    pub unsafe fn IsLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsLost)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub unsafe fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2, param3) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseDC(&self, param0: super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn Restore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawClipper>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClipper)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    pub unsafe fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOverlayPosition)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    pub unsafe fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn UpdateOverlay<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlay)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayDisplay)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn UpdateOverlayZOrder<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayZOrder)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDDInterface)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn PageLock(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PageLock)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn PageUnlock(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PageUnlock)(windows_core::Interface::as_raw(self), param0) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSurfaceDesc)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "windef")]
    pub Blt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDBLTFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Blt: usize,
    #[cfg(feature = "windef")]
    pub BltBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDBLTBATCH, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltBatch: usize,
    #[cfg(feature = "windef")]
    pub BltFast: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut super::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumAttachedSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumAttachedSurfaces: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumOverlayZOrders: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumOverlayZOrders: usize,
    pub Flip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS) -> windows_core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DDPIXELFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetPixelFormat: usize,
    #[cfg(feature = "ksmedia")]
    pub GetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetSurfaceDesc: usize,
    #[cfg(feature = "ksmedia")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    Initialize: usize,
    pub IsLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut DDSURFACEDESC, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "windef", feature = "winnt")))]
    Lock: usize,
    #[cfg(feature = "windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub UpdateOverlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDOVERLAYFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDDInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PageUnlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub SetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    SetSurfaceDesc: usize,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
pub trait IDirectDrawSurface3_Impl: windows_core::IUnknownImpl {
    fn AddAttachedSurface(&self, param0: windows_core::Ref<IDirectDrawSurface3>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT>;
    fn Blt(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface3>, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: windows_core::Ref<IDirectDrawSurface3>, param3: *mut super::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface3>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::Result<()>;
    fn Flip(&self, param0: windows_core::Ref<IDirectDrawSurface3>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: windows_core::OutRef<IDirectDrawSurface3>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self) -> windows_core::Result<DDSCAPS>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn GetDC(&self) -> windows_core::Result<super::HDC>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn Initialize(&self, param0: windows_core::Ref<IDirectDraw>, param1: *mut DDSURFACEDESC) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: windows_core::Ref<IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: windows_core::Ref<IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self, param0: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateOverlay(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface3>, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface3>) -> windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC, param1: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl IDirectDrawSurface3_Vtbl {
    pub const fn new<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAttachedSurface<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::AddAttachedSurface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface3_Impl::AddOverlayDirtyRect(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Blt<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::Blt(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn BltBatch<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn BltFast<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Flip<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::Flip(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetBltStatus<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface3_Impl::GetCaps(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClipper<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface3_Impl::GetClipper(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface3_Impl::GetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDC<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface3_Impl::GetDC(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlipStatus<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetPalette<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface3_Impl::GetPalette(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn IsLost<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::IsLost(this).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::Restore(this).into()
            }
        }
        unsafe extern "system" fn SetClipper<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::SetClipper(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetColorKey<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface3_Impl::SetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetPalette<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::SetPalette(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::Unlock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlay<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDDInterface<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::GetDDInterface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn PageLock<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::PageLock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn PageUnlock<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::PageUnlock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: IDirectDrawSurface3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface3_Impl::SetSurfaceDesc(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, OFFSET>,
            PageLock: PageLock::<Identity, OFFSET>,
            PageUnlock: PageUnlock::<Identity, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDrawSurface3 {}
windows_core::imp::define_interface!(IDirectDrawSurface4, IDirectDrawSurface4_Vtbl, 0x0b2b8630_ad35_11d0_8ea6_00609797ea5b);
windows_core::imp::interface_hierarchy!(IDirectDrawSurface4, windows_core::IUnknown);
impl IDirectDrawSurface4 {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAttachedSurface)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddOverlayDirtyRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Blt<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Blt)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BltBatch)(windows_core::Interface::as_raw(self), param0, param1, param2) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltFast<P2>(&self, param0: u32, param1: u32, param2: P2, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).BltFast)(windows_core::Interface::as_raw(self), param0, param1, param2.param().abi(), param3 as _, param4) }
    }
    pub unsafe fn DeleteAttachedSurface<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttachedSurface)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAttachedSurfaces)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumOverlayZOrders)(windows_core::Interface::as_raw(self), param0, param1 as _, param2) }
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Flip)(windows_core::Interface::as_raw(self), param0.param().abi(), param1) }
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttachedSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1)) }
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBltStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DDSCAPS2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDC(&self) -> windows_core::Result<super::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFlipStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOverlayPosition)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPalette)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSurfaceDesc)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDraw>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _) }
    }
    pub unsafe fn IsLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsLost)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub unsafe fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2, param3) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseDC(&self, param0: super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn Restore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawClipper>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClipper)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    pub unsafe fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOverlayPosition)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Unlock(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn UpdateOverlay<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlay)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayDisplay)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn UpdateOverlayZOrder<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayZOrder)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDDInterface)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn PageLock(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PageLock)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn PageUnlock(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PageUnlock)(windows_core::Interface::as_raw(self), param0) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSurfaceDesc)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    pub unsafe fn SetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2, param3) }
    }
    pub unsafe fn GetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _) }
    }
    pub unsafe fn FreePrivateData(&self, param0: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetUniquenessValue(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUniquenessValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ChangeUniquenessValue(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeUniquenessValue)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "windef")]
    pub Blt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDBLTFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Blt: usize,
    #[cfg(feature = "windef")]
    pub BltBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDBLTBATCH, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltBatch: usize,
    #[cfg(feature = "windef")]
    pub BltFast: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut super::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumAttachedSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumAttachedSurfaces: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumOverlayZOrders: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumOverlayZOrders: usize,
    pub Flip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS2) -> windows_core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DDPIXELFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetPixelFormat: usize,
    #[cfg(feature = "ksmedia")]
    pub GetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetSurfaceDesc: usize,
    #[cfg(feature = "ksmedia")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut DDSURFACEDESC2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    Initialize: usize,
    pub IsLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut DDSURFACEDESC2, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "windef", feature = "winnt")))]
    Lock: usize,
    #[cfg(feature = "windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Unlock: usize,
    #[cfg(feature = "windef")]
    pub UpdateOverlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDOVERLAYFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDDInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PageUnlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub SetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC2, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    SetSurfaceDesc: usize,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetUniquenessValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ChangeUniquenessValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
pub trait IDirectDrawSurface4_Impl: windows_core::IUnknownImpl {
    fn AddAttachedSurface(&self, param0: windows_core::Ref<IDirectDrawSurface4>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT>;
    fn Blt(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface4>, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: windows_core::Ref<IDirectDrawSurface4>, param3: *mut super::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface4>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> windows_core::Result<()>;
    fn Flip(&self, param0: windows_core::Ref<IDirectDrawSurface4>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: windows_core::OutRef<IDirectDrawSurface4>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self) -> windows_core::Result<DDSCAPS2>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn GetDC(&self) -> windows_core::Result<super::HDC>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn Initialize(&self, param0: windows_core::Ref<IDirectDraw>, param1: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: windows_core::Ref<IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: windows_core::Ref<IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<super::RECT>;
    fn UpdateOverlay(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface4>, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface4>) -> windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::Result<()>;
    fn SetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetUniquenessValue(&self) -> windows_core::Result<u32>;
    fn ChangeUniquenessValue(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl IDirectDrawSurface4_Vtbl {
    pub const fn new<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAttachedSurface<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::AddAttachedSurface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::AddOverlayDirtyRect(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Blt<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::Blt(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn BltBatch<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn BltFast<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Flip<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::Flip(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetBltStatus<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::GetCaps(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClipper<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::GetClipper(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::GetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDC<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::GetDC(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlipStatus<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetPalette<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::GetPalette(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn IsLost<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::IsLost(this).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::Restore(this).into()
            }
        }
        unsafe extern "system" fn SetClipper<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::SetClipper(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetColorKey<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::SetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetPalette<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::SetPalette(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::Unlock(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateOverlay<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDDInterface<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::GetDDInterface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn PageLock<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::PageLock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn PageUnlock<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::PageUnlock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::SetSurfaceDesc(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::SetPrivateData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::GetPrivateData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::FreePrivateData(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface4_Impl::GetUniquenessValue(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: IDirectDrawSurface4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface4_Impl::ChangeUniquenessValue(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, OFFSET>,
            PageLock: PageLock::<Identity, OFFSET>,
            PageUnlock: PageUnlock::<Identity, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDrawSurface4 {}
windows_core::imp::define_interface!(IDirectDrawSurface7, IDirectDrawSurface7_Vtbl, 0x06675a80_3b9b_11d2_b92f_00609797ea5b);
windows_core::imp::interface_hierarchy!(IDirectDrawSurface7, windows_core::IUnknown);
impl IDirectDrawSurface7 {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAttachedSurface)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddOverlayDirtyRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Blt<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Blt)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BltBatch)(windows_core::Interface::as_raw(self), param0, param1, param2) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn BltFast<P2>(&self, param0: u32, param1: u32, param2: P2, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).BltFast)(windows_core::Interface::as_raw(self), param0, param1, param2.param().abi(), param3 as _, param4) }
    }
    pub unsafe fn DeleteAttachedSurface<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAttachedSurface)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumAttachedSurfaces)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumOverlayZOrders)(windows_core::Interface::as_raw(self), param0, param1 as _, param2) }
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Flip)(windows_core::Interface::as_raw(self), param0.param().abi(), param1) }
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttachedSurface)(windows_core::Interface::as_raw(self), param0 as _, core::mem::transmute(param1)) }
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBltStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DDSCAPS2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipper)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDC(&self) -> windows_core::Result<super::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFlipStatus)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOverlayPosition)(windows_core::Interface::as_raw(self), param0 as _, param1 as _) }
    }
    pub unsafe fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPalette)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSurfaceDesc)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDraw>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _) }
    }
    pub unsafe fn IsLost(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsLost)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub unsafe fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), param0 as _, param1 as _, param2, param3) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReleaseDC(&self, param0: super::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn Restore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawClipper>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClipper)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    pub unsafe fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetColorKey)(windows_core::Interface::as_raw(self), param0, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOverlayPosition)(windows_core::Interface::as_raw(self), param0, param1) }
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectDrawPalette>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPalette)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Unlock(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn UpdateOverlay<P1>(&self, param0: *mut super::RECT, param1: P1, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlay)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2 as _, param3, param4) }
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayDisplay)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn UpdateOverlayZOrder<P1>(&self, param0: u32, param1: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateOverlayZOrder)(windows_core::Interface::as_raw(self), param0, param1.param().abi()) }
    }
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDDInterface)(windows_core::Interface::as_raw(self), param0 as _) }
    }
    pub unsafe fn PageLock(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PageLock)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn PageUnlock(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PageUnlock)(windows_core::Interface::as_raw(self), param0) }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSurfaceDesc)(windows_core::Interface::as_raw(self), param0 as _, param1) }
    }
    pub unsafe fn SetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2, param3) }
    }
    pub unsafe fn GetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _) }
    }
    pub unsafe fn FreePrivateData(&self, param0: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetUniquenessValue(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUniquenessValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ChangeUniquenessValue(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeUniquenessValue)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPriority(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLOD(&self, param0: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLOD)(windows_core::Interface::as_raw(self), param0) }
    }
    pub unsafe fn GetLOD(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLOD)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface7_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "windef")]
    pub Blt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDBLTFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Blt: usize,
    #[cfg(feature = "windef")]
    pub BltBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDBLTBATCH, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltBatch: usize,
    #[cfg(feature = "windef")]
    pub BltFast: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut super::RECT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub EnumAttachedSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumAttachedSurfaces: usize,
    #[cfg(feature = "ksmedia")]
    pub EnumOverlayZOrders: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    EnumOverlayZOrders: usize,
    pub Flip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSCAPS2) -> windows_core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DDPIXELFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetPixelFormat: usize,
    #[cfg(feature = "ksmedia")]
    pub GetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetSurfaceDesc: usize,
    #[cfg(feature = "ksmedia")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut DDSURFACEDESC2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    Initialize: usize,
    pub IsLost: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut DDSURFACEDESC2, u32, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "windef", feature = "winnt")))]
    Lock: usize,
    #[cfg(feature = "windef")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DDCOLORKEY) -> windows_core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Unlock: usize,
    #[cfg(feature = "windef")]
    pub UpdateOverlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT, *mut core::ffi::c_void, *mut super::RECT, u32, *mut DDOVERLAYFX) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDDInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PageLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PageUnlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub SetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DDSURFACEDESC2, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    SetSurfaceDesc: usize,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetUniquenessValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ChangeUniquenessValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLOD: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetLOD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
pub trait IDirectDrawSurface7_Impl: windows_core::IUnknownImpl {
    fn AddAttachedSurface(&self, param0: windows_core::Ref<IDirectDrawSurface7>) -> windows_core::Result<()>;
    fn AddOverlayDirtyRect(&self) -> windows_core::Result<super::RECT>;
    fn Blt(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface7>, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::Result<()>;
    fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn BltFast(&self, param0: u32, param1: u32, param2: windows_core::Ref<IDirectDrawSurface7>, param3: *mut super::RECT, param4: u32) -> windows_core::Result<()>;
    fn DeleteAttachedSurface(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface7>) -> windows_core::Result<()>;
    fn EnumAttachedSurfaces(&self, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> windows_core::Result<()>;
    fn EnumOverlayZOrders(&self, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> windows_core::Result<()>;
    fn Flip(&self, param0: windows_core::Ref<IDirectDrawSurface7>, param1: u32) -> windows_core::Result<()>;
    fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: windows_core::OutRef<IDirectDrawSurface7>) -> windows_core::Result<()>;
    fn GetBltStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetCaps(&self) -> windows_core::Result<DDSCAPS2>;
    fn GetClipper(&self) -> windows_core::Result<IDirectDrawClipper>;
    fn GetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn GetDC(&self) -> windows_core::Result<super::HDC>;
    fn GetFlipStatus(&self, param0: u32) -> windows_core::Result<()>;
    fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> windows_core::Result<()>;
    fn GetPalette(&self) -> windows_core::Result<IDirectDrawPalette>;
    fn GetPixelFormat(&self, param0: *mut super::DDPIXELFORMAT) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn Initialize(&self, param0: windows_core::Ref<IDirectDraw>, param1: *mut DDSURFACEDESC2) -> windows_core::Result<()>;
    fn IsLost(&self) -> windows_core::Result<()>;
    fn Lock(&self, param0: *mut super::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::HANDLE) -> windows_core::Result<()>;
    fn ReleaseDC(&self, param0: super::HDC) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn SetClipper(&self, param0: windows_core::Ref<IDirectDrawClipper>) -> windows_core::Result<()>;
    fn SetColorKey(&self, param0: u32) -> windows_core::Result<DDCOLORKEY>;
    fn SetOverlayPosition(&self, param0: i32, param1: i32) -> windows_core::Result<()>;
    fn SetPalette(&self, param0: windows_core::Ref<IDirectDrawPalette>) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<super::RECT>;
    fn UpdateOverlay(&self, param0: *mut super::RECT, param1: windows_core::Ref<IDirectDrawSurface7>, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::Result<()>;
    fn UpdateOverlayDisplay(&self, param0: u32) -> windows_core::Result<()>;
    fn UpdateOverlayZOrder(&self, param0: u32, param1: windows_core::Ref<IDirectDrawSurface7>) -> windows_core::Result<()>;
    fn GetDDInterface(&self, param0: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn PageLock(&self, param0: u32) -> windows_core::Result<()>;
    fn PageUnlock(&self, param0: u32) -> windows_core::Result<()>;
    fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::Result<()>;
    fn SetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetUniquenessValue(&self) -> windows_core::Result<u32>;
    fn ChangeUniquenessValue(&self) -> windows_core::Result<()>;
    fn SetPriority(&self, param0: u32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<u32>;
    fn SetLOD(&self, param0: u32) -> windows_core::Result<()>;
    fn GetLOD(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl IDirectDrawSurface7_Vtbl {
    pub const fn new<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAttachedSurface<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::AddAttachedSurface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn AddOverlayDirtyRect<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::AddOverlayDirtyRect(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Blt<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDBLTFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::Blt(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn BltBatch<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::BltBatch(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn BltFast<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut core::ffi::c_void, param3: *mut super::RECT, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::BltFast(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn DeleteAttachedSurface<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::DeleteAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumAttachedSurfaces<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::EnumAttachedSurfaces(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn EnumOverlayZOrders<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::EnumOverlayZOrders(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Flip<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::Flip(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetAttachedSurface<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::GetAttachedSurface(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetBltStatus<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::GetBltStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSCAPS2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::GetCaps(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClipper<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::GetClipper(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColorKey<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::GetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDC<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::GetDC(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlipStatus<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::GetFlipStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetOverlayPosition<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::GetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetPalette<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::GetPalette(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::DDPIXELFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::GetPixelFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn IsLost<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::IsLost(this).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::Lock(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::ReleaseDC(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::Restore(this).into()
            }
        }
        unsafe extern "system" fn SetClipper<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::SetClipper(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetColorKey<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::SetColorKey(this, core::mem::transmute_copy(&param0)) {
                    Ok(ok__) => {
                        param1.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlayPosition<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: i32, param1: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::SetOverlayPosition(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetPalette<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::SetPalette(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::Unlock(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateOverlay<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::RECT, param1: *mut core::ffi::c_void, param2: *mut super::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::UpdateOverlay(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayDisplay<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::UpdateOverlayDisplay(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn UpdateOverlayZOrder<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::UpdateOverlayZOrder(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDDInterface<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::GetDDInterface(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn PageLock<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::PageLock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn PageUnlock<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::PageUnlock(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetSurfaceDesc<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::SetSurfaceDesc(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::SetPrivateData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut core::ffi::c_void, param2: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::GetPrivateData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::FreePrivateData(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetUniquenessValue<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::GetUniquenessValue(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeUniquenessValue<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::ChangeUniquenessValue(this).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::SetPriority(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLOD<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawSurface7_Impl::SetLOD(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetLOD<Identity: IDirectDrawSurface7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawSurface7_Impl::GetLOD(this) {
                    Ok(ok__) => {
                        param0.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAttachedSurface: AddAttachedSurface::<Identity, OFFSET>,
            AddOverlayDirtyRect: AddOverlayDirtyRect::<Identity, OFFSET>,
            Blt: Blt::<Identity, OFFSET>,
            BltBatch: BltBatch::<Identity, OFFSET>,
            BltFast: BltFast::<Identity, OFFSET>,
            DeleteAttachedSurface: DeleteAttachedSurface::<Identity, OFFSET>,
            EnumAttachedSurfaces: EnumAttachedSurfaces::<Identity, OFFSET>,
            EnumOverlayZOrders: EnumOverlayZOrders::<Identity, OFFSET>,
            Flip: Flip::<Identity, OFFSET>,
            GetAttachedSurface: GetAttachedSurface::<Identity, OFFSET>,
            GetBltStatus: GetBltStatus::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetClipper: GetClipper::<Identity, OFFSET>,
            GetColorKey: GetColorKey::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            GetFlipStatus: GetFlipStatus::<Identity, OFFSET>,
            GetOverlayPosition: GetOverlayPosition::<Identity, OFFSET>,
            GetPalette: GetPalette::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            IsLost: IsLost::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            SetClipper: SetClipper::<Identity, OFFSET>,
            SetColorKey: SetColorKey::<Identity, OFFSET>,
            SetOverlayPosition: SetOverlayPosition::<Identity, OFFSET>,
            SetPalette: SetPalette::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            UpdateOverlay: UpdateOverlay::<Identity, OFFSET>,
            UpdateOverlayDisplay: UpdateOverlayDisplay::<Identity, OFFSET>,
            UpdateOverlayZOrder: UpdateOverlayZOrder::<Identity, OFFSET>,
            GetDDInterface: GetDDInterface::<Identity, OFFSET>,
            PageLock: PageLock::<Identity, OFFSET>,
            PageUnlock: PageUnlock::<Identity, OFFSET>,
            SetSurfaceDesc: SetSurfaceDesc::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            GetUniquenessValue: GetUniquenessValue::<Identity, OFFSET>,
            ChangeUniquenessValue: ChangeUniquenessValue::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetLOD: SetLOD::<Identity, OFFSET>,
            GetLOD: GetLOD::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawSurface7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectDrawSurface7 {}
#[cfg(feature = "windef")]
pub type LPCLIPPERCALLBACK = Option<unsafe extern "system" fn(lpddclipper: windows_core::Ref<IDirectDrawClipper>, hwnd: super::HWND, code: u32, lpcontext: *mut core::ffi::c_void) -> u32>;
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
pub type LPDDDEVICEIDENTIFIER = *mut DDDEVICEIDENTIFIER;
pub type LPDDDEVICEIDENTIFIER2 = *mut DDDEVICEIDENTIFIER2;
pub type LPDDENUMCALLBACK = LPDDENUMCALLBACKA;
pub type LPDDENUMCALLBACKA = Option<unsafe extern "system" fn(param0: *mut windows_core::GUID, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "windef")]
pub type LPDDENUMCALLBACKEX = LPDDENUMCALLBACKEXA;
#[cfg(feature = "windef")]
pub type LPDDENUMCALLBACKEXA = Option<unsafe extern "system" fn(param0: *mut windows_core::GUID, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: *mut core::ffi::c_void, param4: super::HMONITOR) -> windows_core::BOOL>;
#[cfg(feature = "windef")]
pub type LPDDENUMCALLBACKEXW = Option<unsafe extern "system" fn(param0: *mut windows_core::GUID, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: *mut core::ffi::c_void, param4: super::HMONITOR) -> windows_core::BOOL>;
pub type LPDDENUMCALLBACKW = Option<unsafe extern "system" fn(param0: *mut windows_core::GUID, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMMODESCALLBACK = Option<unsafe extern "system" fn(param0: *mut DDSURFACEDESC, param1: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMMODESCALLBACK2 = Option<unsafe extern "system" fn(param0: *mut DDSURFACEDESC2, param1: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMSURFACESCALLBACK = Option<unsafe extern "system" fn(param0: windows_core::Ref<IDirectDrawSurface>, param1: *mut DDSURFACEDESC, param2: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMSURFACESCALLBACK2 = Option<unsafe extern "system" fn(param0: windows_core::Ref<IDirectDrawSurface4>, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "ksmedia")]
pub type LPDDENUMSURFACESCALLBACK7 = Option<unsafe extern "system" fn(param0: windows_core::Ref<IDirectDrawSurface7>, param1: *mut DDSURFACEDESC2, param2: *mut core::ffi::c_void) -> windows_core::HRESULT>;
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
#[cfg(feature = "windef")]
pub type LPDIRECTDRAWENUMERATEEX = LPDIRECTDRAWENUMERATEEXA;
#[cfg(feature = "windef")]
pub type LPDIRECTDRAWENUMERATEEXA = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT>;
#[cfg(feature = "windef")]
pub type LPDIRECTDRAWENUMERATEEXW = Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXW, lpcontext: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT>;
pub const MAX_DDDEVICEID_STRING: u32 = 512;
pub const REGSTR_KEY_DDHW_DESCRIPTION: windows_core::PCSTR = windows_core::s!("Description");
pub const REGSTR_KEY_DDHW_DRIVERNAME: windows_core::PCSTR = windows_core::s!("DriverName");
pub const REGSTR_PATH_DDHW: windows_core::PCSTR = windows_core::s!("Hardware\\DirectDrawDrivers");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _DDFXROP(pub u8);
