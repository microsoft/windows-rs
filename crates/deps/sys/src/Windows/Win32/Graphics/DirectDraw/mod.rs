#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn DirectDrawCreate(lpguid: *mut ::windows_sys::core::GUID, lplpdd: *mut IDirectDraw, punkouter: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn DirectDrawCreateClipper(dwflags: u32, lplpddclipper: *mut IDirectDrawClipper, punkouter: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn DirectDrawCreateEx(lpguid: *mut ::windows_sys::core::GUID, lplpdd: *mut *mut ::core::ffi::c_void, iid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectDrawEnumerateA(lpcallback: LPDDENUMCALLBACKA, lpcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DirectDrawEnumerateExA(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DirectDrawEnumerateExW(lpcallback: LPDDENUMCALLBACKEXW, lpcontext: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectDrawEnumerateW(lpcallback: LPDDENUMCALLBACKW, lpcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct ACCESSRECTLIST(i32);
pub const ACCESSRECT_BROKEN: i32 = 4i32;
pub const ACCESSRECT_NOTHOLDINGWIN16LOCK: i32 = 2i32;
pub const ACCESSRECT_VRAMSTYLE: i32 = 1i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct ATTACHLIST(i32);
pub const CCHDEVICENAME: u32 = 32u32;
pub const CLSID_DirectDraw: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3619098336, data2: 17216, data3: 4559, data4: [176, 99, 0, 32, 175, 194, 205, 53] };
pub const CLSID_DirectDraw7: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1009799574, data2: 20699, data3: 4563, data4: [156, 254, 0, 192, 79, 217, 48, 197] };
pub const CLSID_DirectDrawClipper: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1496848288, data2: 32179, data3: 4559, data4: [162, 222, 0, 170, 0, 185, 51, 86] };
pub const D3DFMT_INTERNAL_D15S1: u32 = 73u32;
pub const D3DFMT_INTERNAL_D24S8: u32 = 75u32;
pub const D3DFMT_INTERNAL_D24X8: u32 = 77u32;
pub const D3DFMT_INTERNAL_D32: u32 = 71u32;
pub const D3DFMT_INTERNAL_S1D15: u32 = 72u32;
pub const D3DFMT_INTERNAL_S8D24: u32 = 74u32;
pub const D3DFMT_INTERNAL_X8D24: u32 = 76u32;
pub const D3DFORMAT_MEMBEROFGROUP_ARGB: i32 = 524288i32;
pub const D3DFORMAT_OP_3DACCELERATION: i32 = 2048i32;
pub const D3DFORMAT_OP_AUTOGENMIPMAP: i32 = 4194304i32;
pub const D3DFORMAT_OP_BUMPMAP: i32 = 65536i32;
pub const D3DFORMAT_OP_CONVERT_TO_ARGB: i32 = 8192i32;
pub const D3DFORMAT_OP_CUBETEXTURE: i32 = 4i32;
pub const D3DFORMAT_OP_DISPLAYMODE: i32 = 1024i32;
pub const D3DFORMAT_OP_DMAP: i32 = 131072i32;
pub const D3DFORMAT_OP_NOALPHABLEND: i32 = 2097152i32;
pub const D3DFORMAT_OP_NOFILTER: i32 = 262144i32;
pub const D3DFORMAT_OP_NOTEXCOORDWRAPNORMIP: i32 = 16777216i32;
pub const D3DFORMAT_OP_OFFSCREENPLAIN: i32 = 16384i32;
pub const D3DFORMAT_OP_OFFSCREEN_RENDERTARGET: i32 = 8i32;
pub const D3DFORMAT_OP_PIXELSIZE: i32 = 4096i32;
pub const D3DFORMAT_OP_SAME_FORMAT_RENDERTARGET: i32 = 16i32;
pub const D3DFORMAT_OP_SAME_FORMAT_UP_TO_ALPHA_RENDERTARGET: i32 = 256i32;
pub const D3DFORMAT_OP_SRGBREAD: i32 = 32768i32;
pub const D3DFORMAT_OP_SRGBWRITE: i32 = 1048576i32;
pub const D3DFORMAT_OP_TEXTURE: i32 = 1i32;
pub const D3DFORMAT_OP_VERTEXTEXTURE: i32 = 8388608i32;
pub const D3DFORMAT_OP_VOLUMETEXTURE: i32 = 2i32;
pub const D3DFORMAT_OP_ZSTENCIL: i32 = 64i32;
pub const D3DFORMAT_OP_ZSTENCIL_WITH_ARBITRARY_COLOR_DEPTH: i32 = 128i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DBLNODE(i32);
pub const DCICOMMAND: u32 = 3075u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD32BITDRIVERDATA(i32);
pub const DDABLT_SRCOVERDEST: i32 = 1i32;
pub const DDAL_IMPLICIT: i32 = 1i32;
#[repr(C)]
pub struct DDARGB(i32);
pub const DDBD_1: i32 = 16384i32;
pub const DDBD_16: i32 = 1024i32;
pub const DDBD_2: i32 = 8192i32;
pub const DDBD_24: i32 = 512i32;
pub const DDBD_32: i32 = 256i32;
pub const DDBD_4: i32 = 4096i32;
pub const DDBD_8: i32 = 2048i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DDBLTBATCH(i32);
pub const DDBLTFAST_DESTCOLORKEY: u32 = 2u32;
pub const DDBLTFAST_DONOTWAIT: u32 = 32u32;
pub const DDBLTFAST_NOCOLORKEY: u32 = 0u32;
pub const DDBLTFAST_SRCCOLORKEY: u32 = 1u32;
pub const DDBLTFAST_WAIT: u32 = 16u32;
#[repr(C)]
pub struct DDBLTFX(i32);
pub const DDBLTFX_ARITHSTRETCHY: i32 = 1i32;
pub const DDBLTFX_MIRRORLEFTRIGHT: i32 = 2i32;
pub const DDBLTFX_MIRRORUPDOWN: i32 = 4i32;
pub const DDBLTFX_NOTEARING: i32 = 8i32;
pub const DDBLTFX_ROTATE180: i32 = 16i32;
pub const DDBLTFX_ROTATE270: i32 = 32i32;
pub const DDBLTFX_ROTATE90: i32 = 64i32;
pub const DDBLTFX_ZBUFFERBASEDEST: i32 = 256i32;
pub const DDBLTFX_ZBUFFERRANGE: i32 = 128i32;
pub const DDBLT_AFLAGS: i32 = -2147483648i32;
pub const DDBLT_ALPHADEST: i32 = 1i32;
pub const DDBLT_ALPHADESTCONSTOVERRIDE: i32 = 2i32;
pub const DDBLT_ALPHADESTNEG: i32 = 4i32;
pub const DDBLT_ALPHADESTSURFACEOVERRIDE: i32 = 8i32;
pub const DDBLT_ALPHAEDGEBLEND: i32 = 16i32;
pub const DDBLT_ALPHASRC: i32 = 32i32;
pub const DDBLT_ALPHASRCCONSTOVERRIDE: i32 = 64i32;
pub const DDBLT_ALPHASRCNEG: i32 = 128i32;
pub const DDBLT_ALPHASRCSURFACEOVERRIDE: i32 = 256i32;
pub const DDBLT_ASYNC: i32 = 512i32;
pub const DDBLT_COLORFILL: i32 = 1024i32;
pub const DDBLT_DDFX: i32 = 2048i32;
pub const DDBLT_DDROPS: i32 = 4096i32;
pub const DDBLT_DEPTHFILL: i32 = 33554432i32;
pub const DDBLT_DONOTWAIT: i32 = 134217728i32;
pub const DDBLT_EXTENDED_FLAGS: i32 = 1073741824i32;
pub const DDBLT_EXTENDED_LINEAR_CONTENT: i32 = 4i32;
pub const DDBLT_KEYDEST: i32 = 8192i32;
pub const DDBLT_KEYDESTOVERRIDE: i32 = 16384i32;
pub const DDBLT_KEYSRC: i32 = 32768i32;
pub const DDBLT_KEYSRCOVERRIDE: i32 = 65536i32;
pub const DDBLT_LAST_PRESENTATION: i32 = 536870912i32;
pub const DDBLT_PRESENTATION: i32 = 268435456i32;
pub const DDBLT_ROP: i32 = 131072i32;
pub const DDBLT_ROTATIONANGLE: i32 = 262144i32;
pub const DDBLT_WAIT: i32 = 16777216i32;
pub const DDBLT_ZBUFFER: i32 = 524288i32;
pub const DDBLT_ZBUFFERDESTCONSTOVERRIDE: i32 = 1048576i32;
pub const DDBLT_ZBUFFERDESTOVERRIDE: i32 = 2097152i32;
pub const DDBLT_ZBUFFERSRCCONSTOVERRIDE: i32 = 4194304i32;
pub const DDBLT_ZBUFFERSRCOVERRIDE: i32 = 8388608i32;
#[repr(C)]
pub struct DDBOBNEXTFIELDINFO(i32);
pub const DDCAPS2_AUTOFLIPOVERLAY: i32 = 8i32;
pub const DDCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824i32;
pub const DDCAPS2_CANBOBHARDWARE: i32 = 16384i32;
pub const DDCAPS2_CANBOBINTERLEAVED: i32 = 16i32;
pub const DDCAPS2_CANBOBNONINTERLEAVED: i32 = 32i32;
pub const DDCAPS2_CANCALIBRATEGAMMA: i32 = 1048576i32;
pub const DDCAPS2_CANDROPZ16BIT: i32 = 256i32;
pub const DDCAPS2_CANFLIPODDEVEN: i32 = 8192i32;
pub const DDCAPS2_CANMANAGERESOURCE: i32 = 268435456i32;
pub const DDCAPS2_CANMANAGETEXTURE: i32 = 8388608i32;
pub const DDCAPS2_CANRENDERWINDOWED: i32 = 524288i32;
pub const DDCAPS2_CANSHARERESOURCE: i32 = -2147483648i32;
pub const DDCAPS2_CERTIFIED: i32 = 1i32;
pub const DDCAPS2_COLORCONTROLOVERLAY: i32 = 64i32;
pub const DDCAPS2_COLORCONTROLPRIMARY: i32 = 128i32;
pub const DDCAPS2_COPYFOURCC: i32 = 32768i32;
pub const DDCAPS2_DYNAMICTEXTURES: i32 = 536870912i32;
pub const DDCAPS2_FLIPINTERVAL: i32 = 2097152i32;
pub const DDCAPS2_FLIPNOVSYNC: i32 = 4194304i32;
pub const DDCAPS2_NO2DDURING3DSCENE: i32 = 2i32;
pub const DDCAPS2_NONLOCALVIDMEM: i32 = 512i32;
pub const DDCAPS2_NONLOCALVIDMEMCAPS: i32 = 1024i32;
pub const DDCAPS2_NOPAGELOCKREQUIRED: i32 = 2048i32;
pub const DDCAPS2_PRIMARYGAMMA: i32 = 131072i32;
pub const DDCAPS2_RESERVED1: i32 = 134217728i32;
pub const DDCAPS2_STEREO: i32 = 33554432i32;
pub const DDCAPS2_SYSTONONLOCAL_AS_SYSTOLOCAL: i32 = 67108864i32;
pub const DDCAPS2_TEXMANINNONLOCALVIDMEM: i32 = 16777216i32;
pub const DDCAPS2_VIDEOPORT: i32 = 4i32;
pub const DDCAPS2_WIDESURFACES: i32 = 4096i32;
pub const DDCAPS_3D: i32 = 1i32;
pub const DDCAPS_ALIGNBOUNDARYDEST: i32 = 2i32;
pub const DDCAPS_ALIGNBOUNDARYSRC: i32 = 8i32;
pub const DDCAPS_ALIGNSIZEDEST: i32 = 4i32;
pub const DDCAPS_ALIGNSIZESRC: i32 = 16i32;
pub const DDCAPS_ALIGNSTRIDE: i32 = 32i32;
pub const DDCAPS_ALPHA: i32 = 8388608i32;
pub const DDCAPS_BANKSWITCHED: i32 = 134217728i32;
pub const DDCAPS_BLT: i32 = 64i32;
pub const DDCAPS_BLTCOLORFILL: i32 = 67108864i32;
pub const DDCAPS_BLTDEPTHFILL: i32 = 268435456i32;
pub const DDCAPS_BLTFOURCC: i32 = 256i32;
pub const DDCAPS_BLTQUEUE: i32 = 128i32;
pub const DDCAPS_BLTSTRETCH: i32 = 512i32;
pub const DDCAPS_CANBLTSYSMEM: i32 = -2147483648i32;
pub const DDCAPS_CANCLIP: i32 = 536870912i32;
pub const DDCAPS_CANCLIPSTRETCHED: i32 = 1073741824i32;
pub const DDCAPS_COLORKEY: i32 = 4194304i32;
pub const DDCAPS_COLORKEYHWASSIST: i32 = 16777216i32;
#[repr(C)]
pub struct DDCAPS_DX1(i32);
#[repr(C)]
pub struct DDCAPS_DX3(i32);
#[repr(C)]
pub struct DDCAPS_DX5(i32);
#[repr(C)]
pub struct DDCAPS_DX6(i32);
#[repr(C)]
pub struct DDCAPS_DX7(i32);
pub const DDCAPS_GDI: i32 = 1024i32;
pub const DDCAPS_NOHARDWARE: i32 = 33554432i32;
pub const DDCAPS_OVERLAY: i32 = 2048i32;
pub const DDCAPS_OVERLAYCANTCLIP: i32 = 4096i32;
pub const DDCAPS_OVERLAYFOURCC: i32 = 8192i32;
pub const DDCAPS_OVERLAYSTRETCH: i32 = 16384i32;
pub const DDCAPS_PALETTE: i32 = 32768i32;
pub const DDCAPS_PALETTEVSYNC: i32 = 65536i32;
pub const DDCAPS_READSCANLINE: i32 = 131072i32;
pub const DDCAPS_RESERVED1: i32 = 262144i32;
pub const DDCAPS_VBI: i32 = 524288i32;
pub const DDCAPS_ZBLTS: i32 = 1048576i32;
pub const DDCAPS_ZOVERLAYS: i32 = 2097152i32;
pub const DDCKEYCAPS_DESTBLT: i32 = 1i32;
pub const DDCKEYCAPS_DESTBLTCLRSPACE: i32 = 2i32;
pub const DDCKEYCAPS_DESTBLTCLRSPACEYUV: i32 = 4i32;
pub const DDCKEYCAPS_DESTBLTYUV: i32 = 8i32;
pub const DDCKEYCAPS_DESTOVERLAY: i32 = 16i32;
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACE: i32 = 32i32;
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACEYUV: i32 = 64i32;
pub const DDCKEYCAPS_DESTOVERLAYONEACTIVE: i32 = 128i32;
pub const DDCKEYCAPS_DESTOVERLAYYUV: i32 = 256i32;
pub const DDCKEYCAPS_NOCOSTOVERLAY: i32 = 262144i32;
pub const DDCKEYCAPS_SRCBLT: i32 = 512i32;
pub const DDCKEYCAPS_SRCBLTCLRSPACE: i32 = 1024i32;
pub const DDCKEYCAPS_SRCBLTCLRSPACEYUV: i32 = 2048i32;
pub const DDCKEYCAPS_SRCBLTYUV: i32 = 4096i32;
pub const DDCKEYCAPS_SRCOVERLAY: i32 = 8192i32;
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACE: i32 = 16384i32;
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACEYUV: i32 = 32768i32;
pub const DDCKEYCAPS_SRCOVERLAYONEACTIVE: i32 = 65536i32;
pub const DDCKEYCAPS_SRCOVERLAYYUV: i32 = 131072i32;
pub const DDCKEY_COLORSPACE: i32 = 1i32;
pub const DDCKEY_DESTBLT: i32 = 2i32;
pub const DDCKEY_DESTOVERLAY: i32 = 4i32;
pub const DDCKEY_SRCBLT: i32 = 8i32;
pub const DDCKEY_SRCOVERLAY: i32 = 16i32;
#[repr(C)]
pub struct DDCOLORCONTROL(i32);
#[repr(C)]
pub struct DDCOLORKEY(i32);
pub const DDCOLOR_BRIGHTNESS: i32 = 1i32;
pub const DDCOLOR_COLORENABLE: i32 = 64i32;
pub const DDCOLOR_CONTRAST: i32 = 2i32;
pub const DDCOLOR_GAMMA: i32 = 32i32;
pub const DDCOLOR_HUE: i32 = 4i32;
pub const DDCOLOR_SATURATION: i32 = 8i32;
pub const DDCOLOR_SHARPNESS: i32 = 16i32;
#[repr(C)]
pub struct DDCOMPBUFFERINFO(i32);
#[repr(C)]
pub struct DDCORECAPS(i32);
pub const DDCREATEDRIVEROBJECT: u32 = 10u32;
pub const DDCREATE_EMULATIONONLY: i32 = 2i32;
pub const DDCREATE_HARDWAREONLY: i32 = 1i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DDDEVICEIDENTIFIER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DDDEVICEIDENTIFIER2(i32);
pub const DDEDM_REFRESHRATES: i32 = 1i32;
pub const DDEDM_STANDARDVGAMODES: i32 = 2i32;
pub const DDEM_MODEFAILED: i32 = 2i32;
pub const DDEM_MODEPASSED: i32 = 1i32;
#[repr(C)]
pub struct DDENABLEIRQINFO(i32);
pub const DDENUMOVERLAYZ_BACKTOFRONT: i32 = 0i32;
pub const DDENUMOVERLAYZ_FRONTTOBACK: i32 = 1i32;
pub const DDENUMRET_CANCEL: u32 = 0u32;
pub const DDENUMRET_OK: u32 = 1u32;
pub const DDENUMSURFACES_ALL: i32 = 1i32;
pub const DDENUMSURFACES_CANBECREATED: i32 = 8i32;
pub const DDENUMSURFACES_DOESEXIST: i32 = 16i32;
pub const DDENUMSURFACES_MATCH: i32 = 2i32;
pub const DDENUMSURFACES_NOMATCH: i32 = 4i32;
pub const DDENUM_ATTACHEDSECONDARYDEVICES: i32 = 1i32;
pub const DDENUM_DETACHEDSECONDARYDEVICES: i32 = 2i32;
pub const DDENUM_NONDISPLAYDEVICES: i32 = 4i32;
pub const DDERR_NOTINITIALIZED: i32 = -2147221008i32;
#[repr(C)]
pub struct DDFLIPOVERLAYINFO(i32);
#[repr(C)]
pub struct DDFLIPVIDEOPORTINFO(i32);
pub const DDFLIP_DONOTWAIT: i32 = 32i32;
pub const DDFLIP_EVEN: i32 = 2i32;
pub const DDFLIP_INTERVAL2: i32 = 33554432i32;
pub const DDFLIP_INTERVAL3: i32 = 50331648i32;
pub const DDFLIP_INTERVAL4: i32 = 67108864i32;
pub const DDFLIP_NOVSYNC: i32 = 8i32;
pub const DDFLIP_ODD: i32 = 4i32;
pub const DDFLIP_STEREO: i32 = 16i32;
pub const DDFLIP_WAIT: i32 = 1i32;
pub const DDFXALPHACAPS_BLTALPHAEDGEBLEND: i32 = 1i32;
pub const DDFXALPHACAPS_BLTALPHAPIXELS: i32 = 2i32;
pub const DDFXALPHACAPS_BLTALPHAPIXELSNEG: i32 = 4i32;
pub const DDFXALPHACAPS_BLTALPHASURFACES: i32 = 8i32;
pub const DDFXALPHACAPS_BLTALPHASURFACESNEG: i32 = 16i32;
pub const DDFXALPHACAPS_OVERLAYALPHAEDGEBLEND: i32 = 32i32;
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELS: i32 = 64i32;
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELSNEG: i32 = 128i32;
pub const DDFXALPHACAPS_OVERLAYALPHASURFACES: i32 = 256i32;
pub const DDFXALPHACAPS_OVERLAYALPHASURFACESNEG: i32 = 512i32;
pub const DDFXCAPS_BLTALPHA: i32 = 1i32;
pub const DDFXCAPS_BLTARITHSTRETCHY: i32 = 32i32;
pub const DDFXCAPS_BLTARITHSTRETCHYN: i32 = 16i32;
pub const DDFXCAPS_BLTFILTER: i32 = 32i32;
pub const DDFXCAPS_BLTMIRRORLEFTRIGHT: i32 = 64i32;
pub const DDFXCAPS_BLTMIRRORUPDOWN: i32 = 128i32;
pub const DDFXCAPS_BLTROTATION: i32 = 256i32;
pub const DDFXCAPS_BLTROTATION90: i32 = 512i32;
pub const DDFXCAPS_BLTSHRINKX: i32 = 1024i32;
pub const DDFXCAPS_BLTSHRINKXN: i32 = 2048i32;
pub const DDFXCAPS_BLTSHRINKY: i32 = 4096i32;
pub const DDFXCAPS_BLTSHRINKYN: i32 = 8192i32;
pub const DDFXCAPS_BLTSTRETCHX: i32 = 16384i32;
pub const DDFXCAPS_BLTSTRETCHXN: i32 = 32768i32;
pub const DDFXCAPS_BLTSTRETCHY: i32 = 65536i32;
pub const DDFXCAPS_BLTSTRETCHYN: i32 = 131072i32;
pub const DDFXCAPS_OVERLAYALPHA: i32 = 4i32;
pub const DDFXCAPS_OVERLAYARITHSTRETCHY: i32 = 262144i32;
pub const DDFXCAPS_OVERLAYARITHSTRETCHYN: i32 = 8i32;
pub const DDFXCAPS_OVERLAYDEINTERLACE: i32 = 536870912i32;
pub const DDFXCAPS_OVERLAYFILTER: i32 = 262144i32;
pub const DDFXCAPS_OVERLAYMIRRORLEFTRIGHT: i32 = 134217728i32;
pub const DDFXCAPS_OVERLAYMIRRORUPDOWN: i32 = 268435456i32;
pub const DDFXCAPS_OVERLAYSHRINKX: i32 = 524288i32;
pub const DDFXCAPS_OVERLAYSHRINKXN: i32 = 1048576i32;
pub const DDFXCAPS_OVERLAYSHRINKY: i32 = 2097152i32;
pub const DDFXCAPS_OVERLAYSHRINKYN: i32 = 4194304i32;
pub const DDFXCAPS_OVERLAYSTRETCHX: i32 = 8388608i32;
pub const DDFXCAPS_OVERLAYSTRETCHXN: i32 = 16777216i32;
pub const DDFXCAPS_OVERLAYSTRETCHY: i32 = 33554432i32;
pub const DDFXCAPS_OVERLAYSTRETCHYN: i32 = 67108864i32;
#[repr(C)]
pub struct DDGAMMARAMP(i32);
pub const DDGBS_CANBLT: i32 = 1i32;
pub const DDGBS_ISBLTDONE: i32 = 2i32;
pub const DDGDI_GETHOSTIDENTIFIER: i32 = 1i32;
pub const DDGET32BITDRIVERNAME: u32 = 11u32;
#[repr(C)]
pub struct DDGETCURRENTAUTOFLIPININFO(i32);
#[repr(C)]
pub struct DDGETCURRENTAUTOFLIPOUTINFO(i32);
#[repr(C)]
pub struct DDGETIRQINFO(i32);
#[repr(C)]
pub struct DDGETPOLARITYININFO(i32);
#[repr(C)]
pub struct DDGETPOLARITYOUTINFO(i32);
#[repr(C)]
pub struct DDGETPREVIOUSAUTOFLIPININFO(i32);
#[repr(C)]
pub struct DDGETPREVIOUSAUTOFLIPOUTINFO(i32);
#[repr(C)]
pub struct DDGETTRANSFERSTATUSOUTINFO(i32);
pub const DDGFS_CANFLIP: i32 = 1i32;
pub const DDGFS_ISFLIPDONE: i32 = 2i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHALDDRAWFNS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHALINFO(i32);
pub const DDHALINFO_GETDRIVERINFO2: i32 = 8i32;
pub const DDHALINFO_GETDRIVERINFOSET: i32 = 4i32;
pub const DDHALINFO_ISPRIMARYDISPLAY: i32 = 1i32;
pub const DDHALINFO_MODEXILLEGAL: i32 = 2i32;
#[repr(C)]
pub struct DDHALMODEINFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_ADDATTACHEDSURFACEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_BEGINMOCOMPFRAMEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_BLTDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_CALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_CANCREATESURFACEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_CANCREATEVPORTDATA(i32);
pub const DDHAL_CB32_CANCREATESURFACE: i32 = 32i32;
pub const DDHAL_CB32_CREATEPALETTE: i32 = 64i32;
pub const DDHAL_CB32_CREATESURFACE: i32 = 2i32;
pub const DDHAL_CB32_DESTROYDRIVER: i32 = 1i32;
pub const DDHAL_CB32_FLIPTOGDISURFACE: i32 = 512i32;
pub const DDHAL_CB32_GETSCANLINE: i32 = 128i32;
pub const DDHAL_CB32_MAPMEMORY: i32 = -2147483648i32;
pub const DDHAL_CB32_SETCOLORKEY: i32 = 4i32;
pub const DDHAL_CB32_SETEXCLUSIVEMODE: i32 = 256i32;
pub const DDHAL_CB32_SETMODE: i32 = 8i32;
pub const DDHAL_CB32_WAITFORVERTICALBLANK: i32 = 16i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_COLORCONTROLDATA(i32);
pub const DDHAL_COLOR_COLORCONTROL: i32 = 1i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_CREATEMOCOMPDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_CREATEPALETTEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_CREATESURFACEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_CREATESURFACEEXDATA(i32);
pub const DDHAL_CREATESURFACEEX_SWAPHANDLES: i32 = 1i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_CREATEVPORTDATA(i32);
pub const DDHAL_D3DBUFCB32_CANCREATED3DBUF: i32 = 1i32;
pub const DDHAL_D3DBUFCB32_CREATED3DBUF: i32 = 2i32;
pub const DDHAL_D3DBUFCB32_DESTROYD3DBUF: i32 = 4i32;
pub const DDHAL_D3DBUFCB32_LOCKD3DBUF: i32 = 8i32;
pub const DDHAL_D3DBUFCB32_UNLOCKD3DBUF: i32 = 16i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDCALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDCOLORCONTROLCALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDEXEBUFCALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDKERNELCALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDMISCELLANEOUS2CALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDMISCELLANEOUSCALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDMOTIONCOMPCALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDPALETTECALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDSURFACECALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DDVIDEOPORTCALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DESTROYDDLOCALDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DESTROYDRIVERDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DESTROYMOCOMPDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DESTROYPALETTEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DESTROYSURFACEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DESTROYVPORTDATA(i32);
pub const DDHAL_DRIVER_HANDLED: i32 = 1i32;
pub const DDHAL_DRIVER_NOCKEYHW: i32 = 2i32;
pub const DDHAL_DRIVER_NOTHANDLED: i32 = 0i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_DRVSETCOLORKEYDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_ENDMOCOMPFRAMEDATA(i32);
pub const DDHAL_EXEBUFCB32_CANCREATEEXEBUF: i32 = 1i32;
pub const DDHAL_EXEBUFCB32_CREATEEXEBUF: i32 = 2i32;
pub const DDHAL_EXEBUFCB32_DESTROYEXEBUF: i32 = 4i32;
pub const DDHAL_EXEBUFCB32_LOCKEXEBUF: i32 = 8i32;
pub const DDHAL_EXEBUFCB32_UNLOCKEXEBUF: i32 = 16i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_FLIPDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_FLIPTOGDISURFACEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_FLIPVPORTDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETAVAILDRIVERMEMORYDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETBLTSTATUSDATA(i32);
#[repr(C)]
pub struct DDHAL_GETDRIVERINFODATA(i32);
#[repr(C)]
pub struct DDHAL_GETDRIVERSTATEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETFLIPSTATUSDATA(i32);
#[repr(C)]
pub struct DDHAL_GETHEAPALIGNMENTDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETINTERNALMOCOMPDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETMOCOMPCOMPBUFFDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETMOCOMPFORMATSDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETMOCOMPGUIDSDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETSCANLINEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETVPORTBANDWIDTHDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETVPORTCONNECTDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETVPORTFIELDDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETVPORTFLIPSTATUSDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETVPORTINPUTFORMATDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETVPORTLINEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETVPORTOUTPUTFORMATDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_GETVPORTSIGNALDATA(i32);
pub const DDHAL_KERNEL_SYNCSURFACEDATA: i32 = 1i32;
pub const DDHAL_KERNEL_SYNCVIDEOPORTDATA: i32 = 2i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_LOCKDATA(i32);
pub const DDHAL_MISC2CB32_ALPHABLT: i32 = 1i32;
pub const DDHAL_MISC2CB32_CREATESURFACEEX: i32 = 2i32;
pub const DDHAL_MISC2CB32_DESTROYDDLOCAL: i32 = 8i32;
pub const DDHAL_MISC2CB32_GETDRIVERSTATE: i32 = 4i32;
pub const DDHAL_MISCCB32_GETAVAILDRIVERMEMORY: i32 = 1i32;
pub const DDHAL_MISCCB32_GETHEAPALIGNMENT: i32 = 4i32;
pub const DDHAL_MISCCB32_GETSYSMEMBLTSTATUS: i32 = 8i32;
pub const DDHAL_MISCCB32_UPDATENONLOCALHEAP: i32 = 2i32;
pub const DDHAL_MOCOMP32_BEGINFRAME: u32 = 32u32;
pub const DDHAL_MOCOMP32_CREATE: u32 = 4u32;
pub const DDHAL_MOCOMP32_DESTROY: u32 = 512u32;
pub const DDHAL_MOCOMP32_ENDFRAME: u32 = 64u32;
pub const DDHAL_MOCOMP32_GETCOMPBUFFINFO: u32 = 8u32;
pub const DDHAL_MOCOMP32_GETFORMATS: u32 = 2u32;
pub const DDHAL_MOCOMP32_GETGUIDS: u32 = 1u32;
pub const DDHAL_MOCOMP32_GETINTERNALINFO: u32 = 16u32;
pub const DDHAL_MOCOMP32_QUERYSTATUS: u32 = 256u32;
pub const DDHAL_MOCOMP32_RENDER: u32 = 128u32;
pub const DDHAL_NTCB32_FLIPTOGDISURFACE: i32 = 4i32;
pub const DDHAL_NTCB32_FREEDRIVERMEMORY: i32 = 1i32;
pub const DDHAL_NTCB32_SETEXCLUSIVEMODE: i32 = 2i32;
pub const DDHAL_PALCB32_DESTROYPALETTE: i32 = 1i32;
pub const DDHAL_PALCB32_SETENTRIES: i32 = 2i32;
pub const DDHAL_PLEASEALLOC_BLOCKSIZE: i32 = 2i32;
pub const DDHAL_PLEASEALLOC_LINEARSIZE: i32 = 3i32;
pub const DDHAL_PLEASEALLOC_USERMEM: i32 = 4i32;
pub const DDHAL_PRIVATECAP_ATOMICSURFACECREATION: i32 = 1i32;
pub const DDHAL_PRIVATECAP_NOTIFYPRIMARYCREATION: i32 = 2i32;
pub const DDHAL_PRIVATECAP_RESERVED1: i32 = 4i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_QUERYMOCOMPSTATUSDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_RENDERMOCOMPDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SETCLIPLISTDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SETCOLORKEYDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SETENTRIESDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SETEXCLUSIVEMODEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SETMODEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SETOVERLAYPOSITIONDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SETPALETTEDATA(i32);
pub const DDHAL_SURFCB32_ADDATTACHEDSURFACE: i32 = 128i32;
pub const DDHAL_SURFCB32_BLT: i32 = 32i32;
pub const DDHAL_SURFCB32_DESTROYSURFACE: i32 = 1i32;
pub const DDHAL_SURFCB32_FLIP: i32 = 2i32;
pub const DDHAL_SURFCB32_GETBLTSTATUS: i32 = 256i32;
pub const DDHAL_SURFCB32_GETFLIPSTATUS: i32 = 512i32;
pub const DDHAL_SURFCB32_LOCK: i32 = 8i32;
pub const DDHAL_SURFCB32_RESERVED4: i32 = 4096i32;
pub const DDHAL_SURFCB32_SETCLIPLIST: i32 = 4i32;
pub const DDHAL_SURFCB32_SETCOLORKEY: i32 = 64i32;
pub const DDHAL_SURFCB32_SETOVERLAYPOSITION: i32 = 2048i32;
pub const DDHAL_SURFCB32_SETPALETTE: i32 = 8192i32;
pub const DDHAL_SURFCB32_UNLOCK: i32 = 16i32;
pub const DDHAL_SURFCB32_UPDATEOVERLAY: i32 = 1024i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SYNCSURFACEDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_SYNCVIDEOPORTDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_UNLOCKDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_UPDATENONLOCALHEAPDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_UPDATEOVERLAYDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_UPDATEVPORTDATA(i32);
pub const DDHAL_VPORT32_CANCREATEVIDEOPORT: i32 = 1i32;
pub const DDHAL_VPORT32_COLORCONTROL: i32 = 32768i32;
pub const DDHAL_VPORT32_CREATEVIDEOPORT: i32 = 2i32;
pub const DDHAL_VPORT32_DESTROY: i32 = 1024i32;
pub const DDHAL_VPORT32_FLIP: i32 = 4i32;
pub const DDHAL_VPORT32_GETAUTOFLIPSURF: i32 = 64i32;
pub const DDHAL_VPORT32_GETBANDWIDTH: i32 = 8i32;
pub const DDHAL_VPORT32_GETCONNECT: i32 = 512i32;
pub const DDHAL_VPORT32_GETFIELD: i32 = 128i32;
pub const DDHAL_VPORT32_GETFLIPSTATUS: i32 = 2048i32;
pub const DDHAL_VPORT32_GETINPUTFORMATS: i32 = 16i32;
pub const DDHAL_VPORT32_GETLINE: i32 = 256i32;
pub const DDHAL_VPORT32_GETOUTPUTFORMATS: i32 = 32i32;
pub const DDHAL_VPORT32_GETSIGNALSTATUS: i32 = 16384i32;
pub const DDHAL_VPORT32_UPDATE: i32 = 4096i32;
pub const DDHAL_VPORT32_WAITFORSYNC: i32 = 8192i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_VPORTCOLORDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_WAITFORVERTICALBLANKDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDHAL_WAITFORVPORTSYNCDATA(i32);
pub const DDIRQ_BUSMASTER: i32 = 2i32;
pub const DDIRQ_DISPLAY_VSYNC: i32 = 1i32;
pub const DDIRQ_RESERVED1: i32 = 2i32;
pub const DDIRQ_VPORT0_LINE: i32 = 8i32;
pub const DDIRQ_VPORT0_VSYNC: i32 = 4i32;
pub const DDIRQ_VPORT1_LINE: i32 = 32i32;
pub const DDIRQ_VPORT1_VSYNC: i32 = 16i32;
pub const DDIRQ_VPORT2_LINE: i32 = 128i32;
pub const DDIRQ_VPORT2_VSYNC: i32 = 64i32;
pub const DDIRQ_VPORT3_LINE: i32 = 512i32;
pub const DDIRQ_VPORT3_VSYNC: i32 = 256i32;
pub const DDIRQ_VPORT4_LINE: i32 = 2048i32;
pub const DDIRQ_VPORT4_VSYNC: i32 = 1024i32;
pub const DDIRQ_VPORT5_LINE: i32 = 8192i32;
pub const DDIRQ_VPORT5_VSYNC: i32 = 4096i32;
pub const DDIRQ_VPORT6_LINE: i32 = 32768i32;
pub const DDIRQ_VPORT6_VSYNC: i32 = 16384i32;
pub const DDIRQ_VPORT7_LINE: i32 = 131072i32;
pub const DDIRQ_VPORT7_VSYNC: i32 = 65536i32;
pub const DDIRQ_VPORT8_LINE: i32 = 524288i32;
pub const DDIRQ_VPORT8_VSYNC: i32 = 262144i32;
pub const DDIRQ_VPORT9_LINE: i32 = 131072i32;
pub const DDIRQ_VPORT9_VSYNC: i32 = 65536i32;
#[repr(C)]
pub struct DDKERNELCAPS(i32);
pub const DDKERNELCAPS_AUTOFLIP: i32 = 2i32;
pub const DDKERNELCAPS_CAPTURE_INVERTED: i32 = 512i32;
pub const DDKERNELCAPS_CAPTURE_NONLOCALVIDMEM: i32 = 128i32;
pub const DDKERNELCAPS_CAPTURE_SYSMEM: i32 = 64i32;
pub const DDKERNELCAPS_FIELDPOLARITY: i32 = 256i32;
pub const DDKERNELCAPS_FLIPOVERLAY: i32 = 32i32;
pub const DDKERNELCAPS_FLIPVIDEOPORT: i32 = 16i32;
pub const DDKERNELCAPS_LOCK: i32 = 8i32;
pub const DDKERNELCAPS_SETSTATE: i32 = 4i32;
pub const DDKERNELCAPS_SKIPFIELDS: i32 = 1i32;
#[repr(C)]
pub struct DDLOCKININFO(i32);
#[repr(C)]
pub struct DDLOCKOUTINFO(i32);
pub const DDLOCK_DISCARDCONTENTS: i32 = 8192i32;
pub const DDLOCK_DONOTWAIT: i32 = 16384i32;
pub const DDLOCK_EVENT: i32 = 2i32;
pub const DDLOCK_HASVOLUMETEXTUREBOXRECT: i32 = 32768i32;
pub const DDLOCK_NODIRTYUPDATE: i32 = 65536i32;
pub const DDLOCK_NOOVERWRITE: i32 = 4096i32;
pub const DDLOCK_NOSYSLOCK: i32 = 2048i32;
pub const DDLOCK_OKTOSWAP: i32 = 8192i32;
pub const DDLOCK_READONLY: i32 = 16i32;
pub const DDLOCK_SURFACEMEMORYPTR: i32 = 0i32;
pub const DDLOCK_WAIT: i32 = 1i32;
pub const DDLOCK_WRITEONLY: i32 = 32i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDMCBUFFERINFO(i32);
#[repr(C)]
pub struct DDMCCOMPBUFFERINFO(i32);
pub const DDMCQUERY_READ: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DDMOCOMPBUFFERINFO(i32);
pub const DDMODEINFO_MAXREFRESH: u32 = 16u32;
pub const DDMODEINFO_MODEX: u32 = 2u32;
pub const DDMODEINFO_PALETTIZED: u32 = 1u32;
pub const DDMODEINFO_STANDARDVGA: u32 = 8u32;
pub const DDMODEINFO_STEREO: u32 = 32u32;
pub const DDMODEINFO_UNSUPPORTED: u32 = 4u32;
#[repr(C)]
pub struct DDMONITORINFO(i32);
#[repr(C)]
pub struct DDMORESURFACECAPS(i32);
pub const DDNEWCALLBACKFNS: u32 = 12u32;
#[repr(C)]
pub struct DDNONLOCALVIDMEMCAPS(i32);
#[repr(C)]
pub struct DDNTCORECAPS(i32);
#[repr(C)]
pub struct DDOPTSURFACEDESC(i32);
#[repr(C)]
pub struct DDOSCAPS(i32);
pub const DDOSDCAPS_MONOLITHICMIPMAP: i32 = 4i32;
pub const DDOSDCAPS_OPTCOMPRESSED: i32 = 1i32;
pub const DDOSDCAPS_OPTREORDERED: i32 = 2i32;
pub const DDOSDCAPS_VALIDOSCAPS: i32 = 7i32;
pub const DDOSDCAPS_VALIDSCAPS: i32 = 805324800i32;
pub const DDOSD_ALL: i32 = 15i32;
pub const DDOSD_COMPRESSION_RATIO: i32 = 2i32;
pub const DDOSD_GUID: i32 = 1i32;
pub const DDOSD_OSCAPS: i32 = 8i32;
pub const DDOSD_SCAPS: i32 = 4i32;
pub const DDOVERFX_ARITHSTRETCHY: i32 = 1i32;
pub const DDOVERFX_DEINTERLACE: i32 = 8i32;
pub const DDOVERFX_MIRRORLEFTRIGHT: i32 = 2i32;
pub const DDOVERFX_MIRRORUPDOWN: i32 = 4i32;
#[repr(C)]
pub struct DDOVERLAYFX(i32);
pub const DDOVERZ_INSERTINBACKOF: i32 = 5i32;
pub const DDOVERZ_INSERTINFRONTOF: i32 = 4i32;
pub const DDOVERZ_MOVEBACKWARD: i32 = 3i32;
pub const DDOVERZ_MOVEFORWARD: i32 = 2i32;
pub const DDOVERZ_SENDTOBACK: i32 = 1i32;
pub const DDOVERZ_SENDTOFRONT: i32 = 0i32;
pub const DDOVER_ADDDIRTYRECT: i32 = 32768i32;
pub const DDOVER_ALPHADEST: i32 = 1i32;
pub const DDOVER_ALPHADESTCONSTOVERRIDE: i32 = 2i32;
pub const DDOVER_ALPHADESTNEG: i32 = 4i32;
pub const DDOVER_ALPHADESTSURFACEOVERRIDE: i32 = 8i32;
pub const DDOVER_ALPHAEDGEBLEND: i32 = 16i32;
pub const DDOVER_ALPHASRC: i32 = 32i32;
pub const DDOVER_ALPHASRCCONSTOVERRIDE: i32 = 64i32;
pub const DDOVER_ALPHASRCNEG: i32 = 128i32;
pub const DDOVER_ALPHASRCSURFACEOVERRIDE: i32 = 256i32;
pub const DDOVER_ARGBSCALEFACTORS: i32 = 33554432i32;
pub const DDOVER_AUTOFLIP: i32 = 1048576i32;
pub const DDOVER_BOB: i32 = 2097152i32;
pub const DDOVER_BOBHARDWARE: i32 = 16777216i32;
pub const DDOVER_DDFX: i32 = 524288i32;
pub const DDOVER_DEGRADEARGBSCALING: i32 = 67108864i32;
pub const DDOVER_HIDE: i32 = 512i32;
pub const DDOVER_INTERLEAVED: i32 = 8388608i32;
pub const DDOVER_KEYDEST: i32 = 1024i32;
pub const DDOVER_KEYDESTOVERRIDE: i32 = 2048i32;
pub const DDOVER_KEYSRC: i32 = 4096i32;
pub const DDOVER_KEYSRCOVERRIDE: i32 = 8192i32;
pub const DDOVER_OVERRIDEBOBWEAVE: i32 = 4194304i32;
pub const DDOVER_REFRESHALL: i32 = 131072i32;
pub const DDOVER_REFRESHDIRTYRECTS: i32 = 65536i32;
pub const DDOVER_SHOW: i32 = 16384i32;
pub const DDPCAPS_1BIT: i32 = 256i32;
pub const DDPCAPS_2BIT: i32 = 512i32;
pub const DDPCAPS_4BIT: i32 = 1i32;
pub const DDPCAPS_8BIT: i32 = 4i32;
pub const DDPCAPS_8BITENTRIES: i32 = 2i32;
pub const DDPCAPS_ALLOW256: i32 = 64i32;
pub const DDPCAPS_ALPHA: i32 = 1024i32;
pub const DDPCAPS_INITIALIZE: i32 = 0i32;
pub const DDPCAPS_PRIMARYSURFACE: i32 = 16i32;
pub const DDPCAPS_PRIMARYSURFACELEFT: i32 = 32i32;
pub const DDPCAPS_VSYNC: i32 = 128i32;
pub const DDPF_ALPHA: i32 = 2i32;
pub const DDPF_ALPHAPIXELS: i32 = 1i32;
pub const DDPF_ALPHAPREMULT: i32 = 32768i32;
pub const DDPF_BUMPDUDV: i32 = 524288i32;
pub const DDPF_BUMPLUMINANCE: i32 = 262144i32;
pub const DDPF_COMPRESSED: i32 = 128i32;
pub const DDPF_D3DFORMAT: i32 = 2097152i32;
pub const DDPF_FOURCC: i32 = 4i32;
pub const DDPF_LUMINANCE: i32 = 131072i32;
pub const DDPF_NOVEL_TEXTURE_FORMAT: i32 = 1048576i32;
pub const DDPF_PALETTEINDEXED1: i32 = 2048i32;
pub const DDPF_PALETTEINDEXED2: i32 = 4096i32;
pub const DDPF_PALETTEINDEXED4: i32 = 8i32;
pub const DDPF_PALETTEINDEXED8: i32 = 32i32;
pub const DDPF_PALETTEINDEXEDTO8: i32 = 16i32;
pub const DDPF_RGB: i32 = 64i32;
pub const DDPF_RGBTOYUV: i32 = 256i32;
pub const DDPF_STENCILBUFFER: i32 = 16384i32;
pub const DDPF_YUV: i32 = 512i32;
pub const DDPF_ZBUFFER: i32 = 1024i32;
pub const DDPF_ZPIXELS: i32 = 8192i32;
#[repr(C)]
pub struct DDPIXELFORMAT(i32);
pub const DDRAWICLIP_INMASTERSPRITELIST: i32 = 4i32;
pub const DDRAWICLIP_ISINITIALIZED: i32 = 2i32;
pub const DDRAWICLIP_WATCHWINDOW: i32 = 1i32;
pub const DDRAWILCL_ACTIVENO: i32 = 16i32;
pub const DDRAWILCL_ACTIVEYES: i32 = 8i32;
pub const DDRAWILCL_ALLOWMODEX: i32 = 64i32;
pub const DDRAWILCL_ATTEMPTEDD3DCONTEXT: i32 = 262144i32;
pub const DDRAWILCL_CREATEDWINDOW: i32 = 512i32;
pub const DDRAWILCL_CURSORCLIPPED: i32 = 4096i32;
pub const DDRAWILCL_DIRECTDRAW7: i32 = 131072i32;
pub const DDRAWILCL_DIRECTDRAW8: i32 = 2097152i32;
pub const DDRAWILCL_DIRTYDC: i32 = 1024i32;
pub const DDRAWILCL_DISABLEINACTIVATE: i32 = 2048i32;
pub const DDRAWILCL_DX8DRIVER: i32 = 1048576i32;
pub const DDRAWILCL_EXPLICITMONITOR: i32 = 8192i32;
pub const DDRAWILCL_FPUPRESERVE: i32 = 524288i32;
pub const DDRAWILCL_FPUSETUP: i32 = 32768i32;
pub const DDRAWILCL_HASEXCLUSIVEMODE: i32 = 1i32;
pub const DDRAWILCL_HOOKEDHWND: i32 = 32i32;
pub const DDRAWILCL_ISFULLSCREEN: i32 = 2i32;
pub const DDRAWILCL_MODEHASBEENCHANGED: i32 = 256i32;
pub const DDRAWILCL_MULTITHREADED: i32 = 16384i32;
pub const DDRAWILCL_POWEREDDOWN: i32 = 65536i32;
pub const DDRAWILCL_SETCOOPCALLED: i32 = 4i32;
pub const DDRAWILCL_V1SCLBEHAVIOUR: i32 = 128i32;
pub const DDRAWIPAL_16: i32 = 2i32;
pub const DDRAWIPAL_2: i32 = 2048i32;
pub const DDRAWIPAL_256: i32 = 1i32;
pub const DDRAWIPAL_4: i32 = 1024i32;
pub const DDRAWIPAL_ALLOW256: i32 = 512i32;
pub const DDRAWIPAL_ALPHA: i32 = 8192i32;
pub const DDRAWIPAL_DIRTY: i32 = 256i32;
pub const DDRAWIPAL_EXCLUSIVE: i32 = 64i32;
pub const DDRAWIPAL_GDI: i32 = 4i32;
pub const DDRAWIPAL_INHEL: i32 = 128i32;
pub const DDRAWIPAL_STORED_16: i32 = 16i32;
pub const DDRAWIPAL_STORED_24: i32 = 32i32;
pub const DDRAWIPAL_STORED_8: i32 = 8i32;
pub const DDRAWIPAL_STORED_8INDEX: i32 = 4096i32;
pub const DDRAWISURFGBL_DDHELDONTFREE: i32 = 1048576i32;
pub const DDRAWISURFGBL_DX8SURFACE: i32 = 524288i32;
pub const DDRAWISURFGBL_FASTLOCKHELD: i32 = 32768i32;
pub const DDRAWISURFGBL_HARDWAREOPDEST: i32 = 1024i32;
pub const DDRAWISURFGBL_HARDWAREOPSOURCE: i32 = 512i32;
pub const DDRAWISURFGBL_IMPLICITHANDLE: i32 = 128i32;
pub const DDRAWISURFGBL_ISCLIENTMEM: i32 = 256i32;
pub const DDRAWISURFGBL_ISGDISURFACE: i32 = 4i32;
pub const DDRAWISURFGBL_LATEALLOCATELINEAR: i32 = 8192i32;
pub const DDRAWISURFGBL_LOCKBROKEN: i32 = 64i32;
pub const DDRAWISURFGBL_LOCKNOTHOLDINGWIN16LOCK: i32 = 16i32;
pub const DDRAWISURFGBL_LOCKVRAMSTYLE: i32 = 32i32;
pub const DDRAWISURFGBL_MEMFREE: i32 = 1i32;
pub const DDRAWISURFGBL_NOTIFYWHENUNLOCKED: i32 = 2097152i32;
pub const DDRAWISURFGBL_READONLYLOCKHELD: i32 = 65536i32;
pub const DDRAWISURFGBL_RESERVED0: i32 = -2147483648i32;
pub const DDRAWISURFGBL_SOFTWAREAUTOFLIP: i32 = 8i32;
pub const DDRAWISURFGBL_SYSMEMEXECUTEBUFFER: i32 = 16384i32;
pub const DDRAWISURFGBL_SYSMEMREQUESTED: i32 = 2i32;
pub const DDRAWISURFGBL_VPORTDATA: i32 = 4096i32;
pub const DDRAWISURFGBL_VPORTINTERLEAVED: i32 = 2048i32;
pub const DDRAWISURF_ATTACHED: i32 = 1i32;
pub const DDRAWISURF_ATTACHED_FROM: i32 = 8i32;
pub const DDRAWISURF_BACKBUFFER: i32 = 134217728i32;
pub const DDRAWISURF_DATAISALIASED: i32 = 64i32;
pub const DDRAWISURF_DCIBUSY: i32 = 536870912i32;
pub const DDRAWISURF_DCILOCK: i32 = -2147483648i32;
pub const DDRAWISURF_DRIVERMANAGED: i32 = 1073741824i32;
pub const DDRAWISURF_FRONTBUFFER: i32 = 67108864i32;
pub const DDRAWISURF_GETDCNULL: i32 = 1073741824i32;
pub const DDRAWISURF_HASCKEYDESTBLT: i32 = 512i32;
pub const DDRAWISURF_HASCKEYDESTOVERLAY: i32 = 256i32;
pub const DDRAWISURF_HASCKEYSRCBLT: i32 = 2048i32;
pub const DDRAWISURF_HASCKEYSRCOVERLAY: i32 = 1024i32;
pub const DDRAWISURF_HASDC: i32 = 128i32;
pub const DDRAWISURF_HASOVERLAYDATA: i32 = 16384i32;
pub const DDRAWISURF_HASPIXELFORMAT: i32 = 8192i32;
pub const DDRAWISURF_HELCB: i32 = 33554432i32;
pub const DDRAWISURF_HW_CKEYDESTBLT: i32 = 2097152i32;
pub const DDRAWISURF_HW_CKEYDESTOVERLAY: i32 = 1048576i32;
pub const DDRAWISURF_HW_CKEYSRCBLT: i32 = 8388608i32;
pub const DDRAWISURF_HW_CKEYSRCOVERLAY: i32 = 4194304i32;
pub const DDRAWISURF_IMPLICITCREATE: i32 = 2i32;
pub const DDRAWISURF_IMPLICITROOT: i32 = 16i32;
pub const DDRAWISURF_INMASTERSPRITELIST: i32 = 16777216i32;
pub const DDRAWISURF_INVALID: i32 = 268435456i32;
pub const DDRAWISURF_ISFREE: i32 = 4i32;
pub const DDRAWISURF_LOCKEXCLUDEDCURSOR: i32 = 4096i32;
pub const DDRAWISURF_PARTOFPRIMARYCHAIN: i32 = 32i32;
pub const DDRAWISURF_SETGAMMA: i32 = 32768i32;
pub const DDRAWISURF_STEREOSURFACELEFT: i32 = 536870912i32;
pub const DDRAWISURF_SW_CKEYDESTBLT: i32 = 131072i32;
pub const DDRAWISURF_SW_CKEYDESTOVERLAY: i32 = 65536i32;
pub const DDRAWISURF_SW_CKEYSRCBLT: i32 = 524288i32;
pub const DDRAWISURF_SW_CKEYSRCOVERLAY: i32 = 262144i32;
pub const DDRAWIVPORT_COLORKEYANDINTERP: u32 = 4u32;
pub const DDRAWIVPORT_NOKERNELHANDLES: u32 = 8u32;
pub const DDRAWIVPORT_ON: u32 = 1u32;
pub const DDRAWIVPORT_SOFTWARE_AUTOFLIP: u32 = 2u32;
pub const DDRAWIVPORT_SOFTWARE_BOB: u32 = 16u32;
pub const DDRAWIVPORT_VBION: u32 = 32u32;
pub const DDRAWIVPORT_VIDEOON: u32 = 64u32;
pub const DDRAWI_ATTACHEDTODESKTOP: i32 = 16777216i32;
pub const DDRAWI_BADPDEV: i32 = 1073741824i32;
pub const DDRAWI_CHANGINGMODE: i32 = 4194304i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDMOTIONCOMP_INT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDMOTIONCOMP_LCL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWCLIPPER_GBL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWCLIPPER_INT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWCLIPPER_LCL(i32);
pub const DDRAWI_DDRAWDATANOTFETCHED: i32 = 67108864i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWPALETTE_GBL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWPALETTE_INT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWPALETTE_LCL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWSURFACE_GBL(i32);
#[repr(C)]
pub struct DDRAWI_DDRAWSURFACE_GBL_MORE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWSURFACE_INT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWSURFACE_LCL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDRAWSURFACE_MORE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDVIDEOPORT_INT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DDVIDEOPORT_LCL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DIRECTDRAW_GBL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DIRECTDRAW_INT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DDRAWI_DIRECTDRAW_LCL(i32);
pub const DDRAWI_DISPLAYDRV: i32 = 32i32;
pub const DDRAWI_DRIVERINFO2: i32 = 536870912i32;
pub const DDRAWI_EMULATIONINITIALIZED: i32 = 16384i32;
pub const DDRAWI_EXTENDEDALIGNMENT: i32 = 2097152i32;
pub const DDRAWI_FLIPPEDTOGDI: i32 = 131072i32;
pub const DDRAWI_FULLSCREEN: i32 = 64i32;
pub const DDRAWI_GDIDRV: i32 = 8388608i32;
pub const DDRAWI_GETCOLOR: u32 = 1u32;
pub const DDRAWI_HASCKEYDESTOVERLAY: i32 = 2048i32;
pub const DDRAWI_HASCKEYSRCOVERLAY: i32 = 4096i32;
pub const DDRAWI_HASGDIPALETTE: i32 = 8192i32;
pub const DDRAWI_HASGDIPALETTE_EXCLUSIVE: i32 = 32768i32;
pub const DDRAWI_MODECHANGED: i32 = 128i32;
pub const DDRAWI_MODEX: i32 = 16i32;
pub const DDRAWI_MODEXILLEGAL: i32 = 65536i32;
pub const DDRAWI_NEEDSWIN16FORVRAMLOCK: i32 = 262144i32;
pub const DDRAWI_NOEMULATION: i32 = 1024i32;
pub const DDRAWI_NOHARDWARE: i32 = 256i32;
pub const DDRAWI_PALETTEINIT: i32 = 512i32;
pub const DDRAWI_PDEVICEVRAMBITCLEARED: i32 = 524288i32;
pub const DDRAWI_SECONDARYDRIVERLOADED: i32 = 134217728i32;
pub const DDRAWI_SETCOLOR: u32 = 2u32;
pub const DDRAWI_STANDARDVGA: i32 = 1048576i32;
pub const DDRAWI_TESTINGMODES: i32 = 268435456i32;
pub const DDRAWI_UMODELOADED: i32 = 33554432i32;
pub const DDRAWI_VIRTUALDESKTOP: i32 = 8i32;
pub const DDRAWI_VPORTGETCOLOR: u32 = 1u32;
pub const DDRAWI_VPORTSETCOLOR: u32 = 2u32;
pub const DDRAWI_VPORTSTART: u32 = 1u32;
pub const DDRAWI_VPORTSTOP: u32 = 2u32;
pub const DDRAWI_VPORTUPDATE: u32 = 3u32;
pub const DDRAWI_xxxxxxxxx1: i32 = 1i32;
pub const DDRAWI_xxxxxxxxx2: i32 = 2i32;
#[repr(C)]
pub struct DDRGBA(i32);
#[repr(C)]
pub struct DDSCAPS(i32);
#[repr(C)]
pub struct DDSCAPS2(i32);
pub const DDSCAPS2_ADDITIONALPRIMARY: i32 = -2147483648i32;
pub const DDSCAPS2_COMMANDBUFFER: i32 = 64i32;
pub const DDSCAPS2_CUBEMAP: i32 = 512i32;
pub const DDSCAPS2_CUBEMAP_NEGATIVEX: i32 = 2048i32;
pub const DDSCAPS2_CUBEMAP_NEGATIVEY: i32 = 8192i32;
pub const DDSCAPS2_CUBEMAP_NEGATIVEZ: i32 = 32768i32;
pub const DDSCAPS2_CUBEMAP_POSITIVEX: i32 = 1024i32;
pub const DDSCAPS2_CUBEMAP_POSITIVEY: i32 = 4096i32;
pub const DDSCAPS2_CUBEMAP_POSITIVEZ: i32 = 16384i32;
pub const DDSCAPS2_D3DTEXTUREMANAGE: i32 = 131072i32;
pub const DDSCAPS2_DISCARDBACKBUFFER: i32 = 268435456i32;
pub const DDSCAPS2_DONOTPERSIST: i32 = 262144i32;
pub const DDSCAPS2_ENABLEALPHACHANNEL: i32 = 536870912i32;
pub const DDSCAPS2_EXTENDEDFORMATPRIMARY: i32 = 1073741824i32;
pub const DDSCAPS2_HARDWAREDEINTERLACE: i32 = 0i32;
pub const DDSCAPS2_HINTANTIALIASING: i32 = 256i32;
pub const DDSCAPS2_HINTDYNAMIC: i32 = 4i32;
pub const DDSCAPS2_HINTSTATIC: i32 = 8i32;
pub const DDSCAPS2_INDEXBUFFER: i32 = 67108864i32;
pub const DDSCAPS2_MIPMAPSUBLEVEL: i32 = 65536i32;
pub const DDSCAPS2_NOTUSERLOCKABLE: i32 = 4194304i32;
pub const DDSCAPS2_NPATCHES: i32 = 33554432i32;
pub const DDSCAPS2_OPAQUE: i32 = 128i32;
pub const DDSCAPS2_POINTS: i32 = 8388608i32;
pub const DDSCAPS2_RESERVED1: i32 = 32i32;
pub const DDSCAPS2_RESERVED2: i32 = 64i32;
pub const DDSCAPS2_RESERVED3: i32 = 67108864i32;
pub const DDSCAPS2_RESERVED4: i32 = 2i32;
pub const DDSCAPS2_RTPATCHES: i32 = 16777216i32;
pub const DDSCAPS2_STEREOSURFACELEFT: i32 = 524288i32;
pub const DDSCAPS2_TEXTUREMANAGE: i32 = 16i32;
pub const DDSCAPS2_VERTEXBUFFER: i32 = 32i32;
pub const DDSCAPS2_VOLUME: i32 = 2097152i32;
pub const DDSCAPS3_AUTOGENMIPMAP: i32 = 2048i32;
pub const DDSCAPS3_CREATESHAREDRESOURCE: i32 = 8192i32;
pub const DDSCAPS3_DMAP: i32 = 4096i32;
pub const DDSCAPS3_LIGHTWEIGHTMIPMAP: i32 = 1024i32;
pub const DDSCAPS3_MULTISAMPLE_MASK: i32 = 31i32;
pub const DDSCAPS3_MULTISAMPLE_QUALITY_MASK: i32 = 224i32;
pub const DDSCAPS3_MULTISAMPLE_QUALITY_SHIFT: u32 = 5u32;
pub const DDSCAPS3_OPENSHAREDRESOURCE: i32 = 32768i32;
pub const DDSCAPS3_READONLYRESOURCE: i32 = 16384i32;
pub const DDSCAPS3_RESERVED1: i32 = 256i32;
pub const DDSCAPS3_RESERVED2: i32 = 512i32;
pub const DDSCAPS3_VIDEO: i32 = 512i32;
#[repr(C)]
pub struct DDSCAPSEX(i32);
pub const DDSCAPS_3DDEVICE: i32 = 8192i32;
pub const DDSCAPS_ALLOCONLOAD: i32 = 67108864i32;
pub const DDSCAPS_ALPHA: i32 = 2i32;
pub const DDSCAPS_BACKBUFFER: i32 = 4i32;
pub const DDSCAPS_COMMANDBUFFER: i32 = 1024i32;
pub const DDSCAPS_COMPLEX: i32 = 8i32;
pub const DDSCAPS_EXECUTEBUFFER: i32 = 8388608i32;
pub const DDSCAPS_FLIP: i32 = 16i32;
pub const DDSCAPS_FRONTBUFFER: i32 = 32i32;
pub const DDSCAPS_HWCODEC: i32 = 1048576i32;
pub const DDSCAPS_LIVEVIDEO: i32 = 524288i32;
pub const DDSCAPS_LOCALVIDMEM: i32 = 268435456i32;
pub const DDSCAPS_MIPMAP: i32 = 4194304i32;
pub const DDSCAPS_MODEX: i32 = 2097152i32;
pub const DDSCAPS_NONLOCALVIDMEM: i32 = 536870912i32;
pub const DDSCAPS_OFFSCREENPLAIN: i32 = 64i32;
pub const DDSCAPS_OPTIMIZED: i32 = -2147483648i32;
pub const DDSCAPS_OVERLAY: i32 = 128i32;
pub const DDSCAPS_OWNDC: i32 = 262144i32;
pub const DDSCAPS_PALETTE: i32 = 256i32;
pub const DDSCAPS_PRIMARYSURFACE: i32 = 512i32;
pub const DDSCAPS_PRIMARYSURFACELEFT: i32 = 0i32;
pub const DDSCAPS_RESERVED1: i32 = 1i32;
pub const DDSCAPS_RESERVED2: i32 = 8388608i32;
pub const DDSCAPS_RESERVED3: i32 = 1024i32;
pub const DDSCAPS_STANDARDVGAMODE: i32 = 1073741824i32;
pub const DDSCAPS_SYSTEMMEMORY: i32 = 2048i32;
pub const DDSCAPS_TEXTURE: i32 = 4096i32;
pub const DDSCAPS_VIDEOMEMORY: i32 = 16384i32;
pub const DDSCAPS_VIDEOPORT: i32 = 134217728i32;
pub const DDSCAPS_VISIBLE: i32 = 32768i32;
pub const DDSCAPS_WRITEONLY: i32 = 65536i32;
pub const DDSCAPS_ZBUFFER: i32 = 131072i32;
pub const DDSCL_ALLOWMODEX: i32 = 64i32;
pub const DDSCL_ALLOWREBOOT: i32 = 2i32;
pub const DDSCL_CREATEDEVICEWINDOW: i32 = 512i32;
pub const DDSCL_EXCLUSIVE: i32 = 16i32;
pub const DDSCL_FPUPRESERVE: i32 = 4096i32;
pub const DDSCL_FPUSETUP: i32 = 2048i32;
pub const DDSCL_FULLSCREEN: i32 = 1i32;
pub const DDSCL_MULTITHREADED: i32 = 1024i32;
pub const DDSCL_NORMAL: i32 = 8i32;
pub const DDSCL_NOWINDOWCHANGES: i32 = 4i32;
pub const DDSCL_SETDEVICEWINDOW: i32 = 256i32;
pub const DDSCL_SETFOCUSWINDOW: i32 = 128i32;
pub const DDSDM_STANDARDVGAMODE: i32 = 1i32;
pub const DDSD_ALL: i32 = 16775662i32;
pub const DDSD_ALPHABITDEPTH: i32 = 128i32;
pub const DDSD_BACKBUFFERCOUNT: i32 = 32i32;
pub const DDSD_CAPS: i32 = 1i32;
pub const DDSD_CKDESTBLT: i32 = 16384i32;
pub const DDSD_CKDESTOVERLAY: i32 = 8192i32;
pub const DDSD_CKSRCBLT: i32 = 65536i32;
pub const DDSD_CKSRCOVERLAY: i32 = 32768i32;
pub const DDSD_DEPTH: i32 = 8388608i32;
pub const DDSD_FVF: i32 = 2097152i32;
pub const DDSD_HEIGHT: i32 = 2i32;
pub const DDSD_LINEARSIZE: i32 = 524288i32;
pub const DDSD_LPSURFACE: i32 = 2048i32;
pub const DDSD_MIPMAPCOUNT: i32 = 131072i32;
pub const DDSD_PITCH: i32 = 8i32;
pub const DDSD_PIXELFORMAT: i32 = 4096i32;
pub const DDSD_REFRESHRATE: i32 = 262144i32;
pub const DDSD_SRCVBHANDLE: i32 = 4194304i32;
pub const DDSD_TEXTURESTAGE: i32 = 1048576i32;
pub const DDSD_WIDTH: i32 = 4i32;
pub const DDSD_ZBUFFERBITDEPTH: i32 = 64i32;
#[repr(C)]
pub struct DDSETSTATEININFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DDSETSTATEOUTINFO(i32);
pub const DDSETSURFACEDESC_PRESERVEDC: i32 = 1i32;
pub const DDSETSURFACEDESC_RECREATEDC: i32 = 0i32;
pub const DDSGR_CALIBRATE: i32 = 1i32;
#[repr(C)]
pub struct DDSKIPNEXTFIELDINFO(i32);
pub const DDSKIP_ENABLENEXT: u32 = 2u32;
pub const DDSKIP_SKIPNEXT: u32 = 1u32;
pub const DDSMT_ISTESTREQUIRED: i32 = 1i32;
pub const DDSPD_IUNKNOWNPOINTER: i32 = 1i32;
pub const DDSPD_VOLATILE: i32 = 2i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DDSTEREOMODE(i32);
#[repr(C)]
pub struct DDSURFACEDATA(i32);
#[repr(C)]
pub struct DDSURFACEDESC(i32);
#[repr(C)]
pub struct DDSURFACEDESC2(i32);
pub const DDSVCAPS_RESERVED1: i32 = 1i32;
pub const DDSVCAPS_RESERVED2: i32 = 2i32;
pub const DDSVCAPS_RESERVED3: i32 = 4i32;
pub const DDSVCAPS_RESERVED4: i32 = 8i32;
pub const DDSVCAPS_STEREOSEQUENTIAL: i32 = 16i32;
#[repr(C)]
pub struct DDTRANSFERININFO(i32);
#[repr(C)]
pub struct DDTRANSFEROUTINFO(i32);
pub const DDTRANSFER_CANCEL: u32 = 128u32;
pub const DDTRANSFER_HALFLINES: u32 = 256u32;
pub const DDTRANSFER_INVERT: u32 = 4u32;
pub const DDTRANSFER_NONLOCALVIDMEM: u32 = 2u32;
pub const DDTRANSFER_SYSTEMMEMORY: u32 = 1u32;
pub const DDUNSUPPORTEDMODE: u32 = 4294967295u32;
#[repr(C)]
pub struct DDVERSIONDATA(i32);
pub const DDVERSIONINFO: u32 = 13u32;
#[repr(C)]
pub struct DDVIDEOPORTBANDWIDTH(i32);
#[repr(C)]
pub struct DDVIDEOPORTCAPS(i32);
#[repr(C)]
pub struct DDVIDEOPORTCONNECT(i32);
#[repr(C)]
pub struct DDVIDEOPORTDATA(i32);
#[repr(C)]
pub struct DDVIDEOPORTDESC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DDVIDEOPORTINFO(i32);
#[repr(C)]
pub struct DDVIDEOPORTNOTIFY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DDVIDEOPORTSTATUS(i32);
pub const DDVPBCAPS_DESTINATION: i32 = 2i32;
pub const DDVPBCAPS_SOURCE: i32 = 1i32;
pub const DDVPB_OVERLAY: i32 = 2i32;
pub const DDVPB_TYPE: i32 = 4i32;
pub const DDVPB_VIDEOPORT: i32 = 1i32;
pub const DDVPCAPS_AUTOFLIP: i32 = 1i32;
pub const DDVPCAPS_COLORCONTROL: i32 = 1024i32;
pub const DDVPCAPS_HARDWAREDEINTERLACE: i32 = 16384i32;
pub const DDVPCAPS_INTERLACED: i32 = 2i32;
pub const DDVPCAPS_NONINTERLACED: i32 = 4i32;
pub const DDVPCAPS_OVERSAMPLEDVBI: i32 = 2048i32;
pub const DDVPCAPS_READBACKFIELD: i32 = 8i32;
pub const DDVPCAPS_READBACKLINE: i32 = 16i32;
pub const DDVPCAPS_SHAREABLE: i32 = 32i32;
pub const DDVPCAPS_SKIPEVENFIELDS: i32 = 64i32;
pub const DDVPCAPS_SKIPODDFIELDS: i32 = 128i32;
pub const DDVPCAPS_SYNCMASTER: i32 = 256i32;
pub const DDVPCAPS_SYSTEMMEMORY: i32 = 4096i32;
pub const DDVPCAPS_VBIANDVIDEOINDEPENDENT: i32 = 8192i32;
pub const DDVPCAPS_VBISURFACE: i32 = 512i32;
pub const DDVPCONNECT_DISCARDSVREFDATA: i32 = 8i32;
pub const DDVPCONNECT_DOUBLECLOCK: i32 = 1i32;
pub const DDVPCONNECT_HALFLINE: i32 = 16i32;
pub const DDVPCONNECT_INTERLACED: i32 = 32i32;
pub const DDVPCONNECT_INVERTPOLARITY: i32 = 4i32;
pub const DDVPCONNECT_SHAREEVEN: i32 = 64i32;
pub const DDVPCONNECT_SHAREODD: i32 = 128i32;
pub const DDVPCONNECT_VACT: i32 = 2i32;
pub const DDVPCREATE_VBIONLY: i32 = 1i32;
pub const DDVPCREATE_VIDEOONLY: i32 = 2i32;
pub const DDVPD_ALIGN: i32 = 64i32;
pub const DDVPD_AUTOFLIP: i32 = 32i32;
pub const DDVPD_CAPS: i32 = 8i32;
pub const DDVPD_FILTERQUALITY: i32 = 256i32;
pub const DDVPD_FX: i32 = 16i32;
pub const DDVPD_HEIGHT: i32 = 2i32;
pub const DDVPD_ID: i32 = 4i32;
pub const DDVPD_PREFERREDAUTOFLIP: i32 = 128i32;
pub const DDVPD_WIDTH: i32 = 1i32;
pub const DDVPFLIP_VBI: i32 = 2i32;
pub const DDVPFLIP_VIDEO: i32 = 1i32;
pub const DDVPFORMAT_VBI: i32 = 2i32;
pub const DDVPFORMAT_VIDEO: i32 = 1i32;
pub const DDVPFX_CROPTOPDATA: i32 = 1i32;
pub const DDVPFX_CROPX: i32 = 2i32;
pub const DDVPFX_CROPY: i32 = 4i32;
pub const DDVPFX_IGNOREVBIXCROP: i32 = 262144i32;
pub const DDVPFX_INTERLEAVE: i32 = 8i32;
pub const DDVPFX_MIRRORLEFTRIGHT: i32 = 16i32;
pub const DDVPFX_MIRRORUPDOWN: i32 = 32i32;
pub const DDVPFX_PRESHRINKX: i32 = 64i32;
pub const DDVPFX_PRESHRINKXB: i32 = 256i32;
pub const DDVPFX_PRESHRINKXS: i32 = 1024i32;
pub const DDVPFX_PRESHRINKY: i32 = 128i32;
pub const DDVPFX_PRESHRINKYB: i32 = 512i32;
pub const DDVPFX_PRESHRINKYS: i32 = 2048i32;
pub const DDVPFX_PRESTRETCHX: i32 = 4096i32;
pub const DDVPFX_PRESTRETCHXN: i32 = 16384i32;
pub const DDVPFX_PRESTRETCHY: i32 = 8192i32;
pub const DDVPFX_PRESTRETCHYN: i32 = 32768i32;
pub const DDVPFX_VBICONVERT: i32 = 65536i32;
pub const DDVPFX_VBINOINTERLEAVE: i32 = 524288i32;
pub const DDVPFX_VBINOSCALE: i32 = 131072i32;
pub const DDVPSQ_NOSIGNAL: i32 = 1i32;
pub const DDVPSQ_SIGNALOK: i32 = 2i32;
pub const DDVPSTATUS_VBIONLY: i32 = 1i32;
pub const DDVPSTATUS_VIDEOONLY: i32 = 2i32;
pub const DDVPTARGET_VBI: i32 = 2i32;
pub const DDVPTARGET_VIDEO: i32 = 1i32;
pub const DDVPTYPE_BROOKTREE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 324183392, data2: 55905, data3: 4559, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const DDVPTYPE_CCIR656: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4238550688, data2: 55904, data3: 4559, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const DDVPTYPE_E_HREFH_VREFH: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1425250688, data2: 55904, data3: 4559, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const DDVPTYPE_E_HREFH_VREFL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2457350688, data2: 55904, data3: 4559, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const DDVPTYPE_E_HREFL_VREFH: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2692350688, data2: 55904, data3: 4559, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const DDVPTYPE_E_HREFL_VREFL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3768350688, data2: 55904, data3: 4559, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const DDVPTYPE_PHILIPS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 858583392, data2: 55905, data3: 4559, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const DDVPWAIT_BEGIN: i32 = 1i32;
pub const DDVPWAIT_END: i32 = 2i32;
pub const DDVPWAIT_LINE: i32 = 3i32;
pub const DDVP_AUTOFLIP: i32 = 1i32;
pub const DDVP_CONVERT: i32 = 2i32;
pub const DDVP_CROP: i32 = 4i32;
pub const DDVP_HARDWAREDEINTERLACE: i32 = 32768i32;
pub const DDVP_IGNOREVBIXCROP: i32 = 8192i32;
pub const DDVP_INTERLEAVE: i32 = 8i32;
pub const DDVP_MIRRORLEFTRIGHT: i32 = 16i32;
pub const DDVP_MIRRORUPDOWN: i32 = 32i32;
pub const DDVP_OVERRIDEBOBWEAVE: i32 = 4096i32;
pub const DDVP_PRESCALE: i32 = 64i32;
pub const DDVP_SKIPEVENFIELDS: i32 = 128i32;
pub const DDVP_SKIPODDFIELDS: i32 = 256i32;
pub const DDVP_SYNCMASTER: i32 = 512i32;
pub const DDVP_VBICONVERT: i32 = 1024i32;
pub const DDVP_VBINOINTERLEAVE: i32 = 16384i32;
pub const DDVP_VBINOSCALE: i32 = 2048i32;
pub const DDWAITVB_BLOCKBEGIN: i32 = 1i32;
pub const DDWAITVB_BLOCKBEGINEVENT: i32 = 2i32;
pub const DDWAITVB_BLOCKEND: i32 = 4i32;
pub const DDWAITVB_I_TESTVB: i32 = -2147483642i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_ADDATTACHEDSURFACEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_ATTACHLIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_BEGINMOCOMPFRAMEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_BLTDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DD_CALLBACKS(i32);
#[repr(C)]
pub struct DD_CANCREATESURFACEDATA(i32);
#[repr(C)]
pub struct DD_CANCREATEVPORTDATA(i32);
#[repr(C)]
pub struct DD_CLIPPER_GLOBAL(i32);
#[repr(C)]
pub struct DD_CLIPPER_LOCAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_COLORCONTROLCALLBACKS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_COLORCONTROLDATA(i32);
#[repr(C)]
pub struct DD_CREATEMOCOMPDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DD_CREATEPALETTEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_CREATESURFACEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_CREATESURFACEEXDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_CREATEVPORTDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_D3DBUFCALLBACKS(i32);
#[repr(C)]
pub struct DD_DESTROYDDLOCALDATA(i32);
#[repr(C)]
pub struct DD_DESTROYMOCOMPDATA(i32);
#[repr(C)]
pub struct DD_DESTROYPALETTEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_DESTROYSURFACEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_DESTROYVPORTDATA(i32);
#[repr(C)]
pub struct DD_DIRECTDRAW_GLOBAL(i32);
#[repr(C)]
pub struct DD_DIRECTDRAW_LOCAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_DRVSETCOLORKEYDATA(i32);
#[repr(C)]
pub struct DD_ENDMOCOMPFRAMEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_FLIPDATA(i32);
#[repr(C)]
pub struct DD_FLIPTOGDISURFACEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_FLIPVPORTDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_FREEDRIVERMEMORYDATA(i32);
#[repr(C)]
pub struct DD_GETAVAILDRIVERMEMORYDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_GETBLTSTATUSDATA(i32);
#[repr(C)]
pub struct DD_GETDRIVERINFODATA(i32);
#[repr(C)]
pub struct DD_GETDRIVERSTATEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_GETFLIPSTATUSDATA(i32);
#[repr(C)]
pub struct DD_GETHEAPALIGNMENTDATA(i32);
#[repr(C)]
pub struct DD_GETINTERNALMOCOMPDATA(i32);
#[repr(C)]
pub struct DD_GETMOCOMPCOMPBUFFDATA(i32);
#[repr(C)]
pub struct DD_GETMOCOMPFORMATSDATA(i32);
#[repr(C)]
pub struct DD_GETMOCOMPGUIDSDATA(i32);
#[repr(C)]
pub struct DD_GETSCANLINEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_GETVPORTBANDWIDTHDATA(i32);
#[repr(C)]
pub struct DD_GETVPORTCONNECTDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_GETVPORTFIELDDATA(i32);
#[repr(C)]
pub struct DD_GETVPORTFLIPSTATUSDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_GETVPORTINPUTFORMATDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_GETVPORTLINEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_GETVPORTOUTPUTFORMATDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_GETVPORTSIGNALDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_HALINFO(i32);
#[repr(C)]
pub struct DD_HALINFO_V4(i32);
pub const DD_HAL_VERSION: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_KERNELCALLBACKS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_LOCKDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_MAPMEMORYDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_MISCELLANEOUS2CALLBACKS(i32);
#[repr(C)]
pub struct DD_MISCELLANEOUSCALLBACKS(i32);
#[repr(C)]
pub struct DD_MORECAPS(i32);
#[repr(C)]
pub struct DD_MORESURFACECAPS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_MOTIONCOMPCALLBACKS(i32);
#[repr(C)]
pub struct DD_MOTIONCOMP_LOCAL(i32);
#[repr(C)]
pub struct DD_NONLOCALVIDMEMCAPS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_NTCALLBACKS(i32);
#[repr(C)]
pub struct DD_NTPRIVATEDRIVERCAPS(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct DD_PALETTECALLBACKS(i32);
#[repr(C)]
pub struct DD_PALETTE_GLOBAL(i32);
#[repr(C)]
pub struct DD_PALETTE_LOCAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_QUERYMOCOMPSTATUSDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_RENDERMOCOMPDATA(i32);
pub const DD_RUNTIME_VERSION: i32 = 2306i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SETCLIPLISTDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SETCOLORKEYDATA(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct DD_SETENTRIESDATA(i32);
#[repr(C)]
pub struct DD_SETEXCLUSIVEMODEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SETOVERLAYPOSITIONDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SETPALETTEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_STEREOMODE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SURFACECALLBACKS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SURFACE_GLOBAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SURFACE_INT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SURFACE_LOCAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SURFACE_MORE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SYNCSURFACEDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_SYNCVIDEOPORTDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_UNLOCKDATA(i32);
#[repr(C)]
pub struct DD_UPDATENONLOCALHEAPDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_UPDATEOVERLAYDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_UPDATEVPORTDATA(i32);
pub const DD_VERSION: i32 = 512i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_VIDEOPORTCALLBACKS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_VIDEOPORT_LOCAL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_VPORTCOLORDATA(i32);
#[repr(C)]
pub struct DD_WAITFORVERTICALBLANKDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DD_WAITFORVPORTSYNCDATA(i32);
pub const DELETED_LASTONE: u32 = 1u32;
pub const DELETED_NOTFOUND: u32 = 2u32;
pub const DELETED_OK: u32 = 0u32;
pub const DIRECTDRAW_VERSION: u32 = 1792u32;
pub const DXAPI_HALVERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DXAPI_INTERFACE(i32);
pub const DXERR_GENERIC: u32 = 2147500037u32;
pub const DXERR_OUTOFCAPS: u32 = 2289434984u32;
pub const DXERR_UNSUPPORTED: u32 = 2147500033u32;
#[repr(C)]
pub struct DX_IRQDATA(i32);
pub const DX_OK: u32 = 0u32;
pub const GUID_ColorControlCallbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4023782594, data2: 18919, data3: 4560, data4: [136, 157, 0, 170, 0, 187, 183, 106] };
pub const GUID_D3DCallbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2079353232, data2: 34708, data3: 4560, data4: [145, 57, 8, 0, 54, 210, 239, 2] };
pub const GUID_D3DCallbacks2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 195396833, data2: 28854, data3: 4560, data4: [136, 157, 0, 170, 0, 187, 183, 106] };
pub const GUID_D3DCallbacks3: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3723760176, data2: 60426, data3: 4560, data4: [169, 182, 0, 170, 0, 192, 153, 62] };
pub const GUID_D3DCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2079353233, data2: 34708, data3: 4560, data4: [145, 57, 8, 0, 54, 210, 239, 2] };
pub const GUID_D3DExtendedCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2112102272, data2: 40339, data3: 4560, data4: [137, 171, 0, 160, 201, 5, 65, 41] };
pub const GUID_D3DParseUnknownCommandCallback: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 772079520, data2: 39140, data3: 4561, data4: [140, 225, 0, 160, 201, 6, 41, 168] };
pub const GUID_DDMoreCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2282467120, data2: 45104, data3: 4560, data4: [142, 167, 0, 96, 151, 151, 234, 91] };
pub const GUID_DDMoreSurfaceCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 998900838, data2: 62057, data3: 4561, data4: [136, 11, 0, 192, 79, 217, 48, 197] };
pub const GUID_DDStereoMode: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4163376796,
    data2: 43240,
    data3: 4562,
    data4: [161, 242, 0, 160, 201, 131, 234, 246],
};
pub const GUID_DxApi: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2323234544, data2: 47381, data3: 4560, data4: [145, 68, 8, 0, 54, 210, 239, 2] };
pub const GUID_GetHeapAlignment: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1121988374,
    data2: 31553,
    data3: 4562,
    data4: [139, 255, 0, 160, 201, 131, 234, 246],
};
pub const GUID_KernelCallbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2156279808, data2: 27398, data3: 4560, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const GUID_KernelCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4289361216, data2: 31400, data3: 4560, data4: [155, 6, 0, 160, 201, 3, 163, 184] };
pub const GUID_Miscellaneous2Callbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1080766208, data2: 15962, data3: 4561, data4: [182, 64, 0, 170, 0, 161, 249, 106] };
pub const GUID_MiscellaneousCallbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4023782592, data2: 18919, data3: 4560, data4: [136, 157, 0, 170, 0, 187, 183, 106] };
pub const GUID_MotionCompCallbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2970757952, data2: 23973, data3: 4561, data4: [143, 207, 0, 192, 79, 194, 155, 78] };
pub const GUID_NTCallbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1877601502, data2: 57225, data3: 4561, data4: [157, 176, 0, 96, 8, 39, 113, 186] };
pub const GUID_NTPrivateDriverCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4208028195, data2: 31590, data3: 4562, data4: [131, 215, 0, 192, 79, 124, 229, 140] };
pub const GUID_NonLocalVidMemCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2261056128, data2: 36228, data3: 4560, data4: [148, 232, 0, 192, 79, 195, 65, 55] };
pub const GUID_OptSurfaceKmodeInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3764159602, data2: 20948, data3: 4561, data4: [140, 206, 0, 160, 201, 6, 41, 168] };
pub const GUID_OptSurfaceUmodeInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2641963012, data2: 24488, data3: 4561, data4: [140, 208, 0, 160, 201, 6, 41, 168] };
pub const GUID_UpdateNonLocalHeap: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1121988375,
    data2: 31553,
    data3: 4562,
    data4: [139, 255, 0, 160, 201, 131, 234, 246],
};
pub const GUID_UserModeDriverInfo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4038125794, data2: 24471, data3: 4561, data4: [140, 208, 0, 160, 201, 6, 41, 168] };
pub const GUID_UserModeDriverPassword: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2549637558, data2: 24737, data3: 4561, data4: [140, 208, 0, 160, 201, 6, 41, 168] };
pub const GUID_VPE2Callbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1384653127, data2: 11591, data3: 18074, data4: [160, 209, 3, 69, 88, 144, 246, 200] };
pub const GUID_VideoPortCallbacks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4023782593, data2: 18919, data3: 4560, data4: [136, 157, 0, 170, 0, 187, 183, 106] };
pub const GUID_VideoPortCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4023782595, data2: 18919, data3: 4560, data4: [136, 157, 0, 170, 0, 187, 183, 106] };
pub const GUID_ZPixelFormats: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2475071616, data2: 14031, data3: 4561, data4: [155, 27, 0, 170, 0, 187, 184, 174] };
#[repr(C)]
pub struct HEAPALIAS(i32);
#[repr(C)]
pub struct HEAPALIASINFO(i32);
pub const HEAPALIASINFO_MAPPEDDUMMY: i32 = 2i32;
pub const HEAPALIASINFO_MAPPEDREAL: i32 = 1i32;
#[repr(C)]
pub struct HEAPALIGNMENT(i32);
#[repr(transparent)]
pub struct IDDVideoPortContainer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDDVideoPortContainerVtbl(i32);
#[repr(transparent)]
pub struct IDirectDraw(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectDraw2(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDraw2Vtbl(i32);
#[repr(transparent)]
pub struct IDirectDraw4(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDraw4Vtbl(i32);
#[repr(transparent)]
pub struct IDirectDraw7(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDraw7Vtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawClipper(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawClipperVtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawColorControl(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawColorControlVtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawGammaControl(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawGammaControlVtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawKernel(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawKernelVtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawPalette(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawPaletteVtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectDrawSurface2(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawSurface2Vtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawSurface3(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawSurface3Vtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawSurface4(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawSurface4Vtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawSurface7(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawSurface7Vtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawSurfaceKernel(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawSurfaceKernelVtbl(i32);
#[repr(C)]
pub struct IDirectDrawSurfaceVtbl(i32);
#[repr(transparent)]
pub struct IDirectDrawVideoPort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectDrawVideoPortNotify(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDirectDrawVideoPortNotifyVtbl(i32);
#[repr(C)]
pub struct IDirectDrawVideoPortVtbl(i32);
#[repr(C)]
pub struct IDirectDrawVtbl(i32);
pub const IRQINFO_HANDLED: u32 = 1u32;
pub const IRQINFO_NOTHANDLED: u32 = 2u32;
#[repr(C)]
pub struct IUNKNOWN_LIST(i32);
#[repr(C)]
pub struct LPCLIPPERCALLBACK(i32);
#[repr(C)]
pub struct LPDD32BITDRIVERINIT(i32);
#[repr(C)]
pub struct LPDDENUMCALLBACKA(i32);
#[repr(C)]
pub struct LPDDENUMCALLBACKEXA(i32);
#[repr(C)]
pub struct LPDDENUMCALLBACKEXW(i32);
#[repr(C)]
pub struct LPDDENUMCALLBACKW(i32);
#[repr(C)]
pub struct LPDDENUMMODESCALLBACK(i32);
#[repr(C)]
pub struct LPDDENUMMODESCALLBACK2(i32);
#[repr(C)]
pub struct LPDDENUMSURFACESCALLBACK(i32);
#[repr(C)]
pub struct LPDDENUMSURFACESCALLBACK2(i32);
#[repr(C)]
pub struct LPDDENUMSURFACESCALLBACK7(i32);
#[repr(C)]
pub struct LPDDENUMVIDEOCALLBACK(i32);
#[repr(C)]
pub struct LPDDGAMMACALIBRATORPROC(i32);
#[repr(C)]
pub struct LPDDHALCOLORCB_COLORCONTROL(i32);
#[repr(C)]
pub struct LPDDHALEXEBUFCB_CANCREATEEXEBUF(i32);
#[repr(C)]
pub struct LPDDHALEXEBUFCB_CREATEEXEBUF(i32);
#[repr(C)]
pub struct LPDDHALEXEBUFCB_DESTROYEXEBUF(i32);
#[repr(C)]
pub struct LPDDHALEXEBUFCB_LOCKEXEBUF(i32);
#[repr(C)]
pub struct LPDDHALEXEBUFCB_UNLOCKEXEBUF(i32);
#[repr(C)]
pub struct LPDDHALKERNELCB_SYNCSURFACE(i32);
#[repr(C)]
pub struct LPDDHALKERNELCB_SYNCVIDEOPORT(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_BEGINFRAME(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_CREATE(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_DESTROY(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_ENDFRAME(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_GETCOMPBUFFINFO(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_GETFORMATS(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_GETGUIDS(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_GETINTERNALINFO(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_QUERYSTATUS(i32);
#[repr(C)]
pub struct LPDDHALMOCOMPCB_RENDER(i32);
#[repr(C)]
pub struct LPDDHALPALCB_DESTROYPALETTE(i32);
#[repr(C)]
pub struct LPDDHALPALCB_SETENTRIES(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_ADDATTACHEDSURFACE(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_BLT(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_DESTROYSURFACE(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_FLIP(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_GETBLTSTATUS(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_GETFLIPSTATUS(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_LOCK(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_SETCLIPLIST(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_SETCOLORKEY(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_SETOVERLAYPOSITION(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_SETPALETTE(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_UNLOCK(i32);
#[repr(C)]
pub struct LPDDHALSURFCB_UPDATEOVERLAY(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_CANCREATEVIDEOPORT(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_COLORCONTROL(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_CREATEVIDEOPORT(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_DESTROYVPORT(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_FLIP(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_GETBANDWIDTH(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_GETFIELD(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_GETFLIPSTATUS(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_GETINPUTFORMATS(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_GETLINE(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_GETOUTPUTFORMATS(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_GETSIGNALSTATUS(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_GETVPORTCONNECT(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_UPDATE(i32);
#[repr(C)]
pub struct LPDDHALVPORTCB_WAITFORSYNC(i32);
#[repr(C)]
pub struct LPDDHAL_CANCREATESURFACE(i32);
#[repr(C)]
pub struct LPDDHAL_CREATEPALETTE(i32);
#[repr(C)]
pub struct LPDDHAL_CREATESURFACE(i32);
#[repr(C)]
pub struct LPDDHAL_CREATESURFACEEX(i32);
#[repr(C)]
pub struct LPDDHAL_DESTROYDDLOCAL(i32);
#[repr(C)]
pub struct LPDDHAL_DESTROYDRIVER(i32);
#[repr(C)]
pub struct LPDDHAL_FLIPTOGDISURFACE(i32);
#[repr(C)]
pub struct LPDDHAL_GETAVAILDRIVERMEMORY(i32);
#[repr(C)]
pub struct LPDDHAL_GETDRIVERINFO(i32);
#[repr(C)]
pub struct LPDDHAL_GETDRIVERSTATE(i32);
#[repr(C)]
pub struct LPDDHAL_GETHEAPALIGNMENT(i32);
#[repr(C)]
pub struct LPDDHAL_GETSCANLINE(i32);
#[repr(C)]
pub struct LPDDHAL_SETCOLORKEY(i32);
#[repr(C)]
pub struct LPDDHAL_SETEXCLUSIVEMODE(i32);
#[repr(C)]
pub struct LPDDHAL_SETINFO(i32);
#[repr(C)]
pub struct LPDDHAL_SETMODE(i32);
#[repr(C)]
pub struct LPDDHAL_UPDATENONLOCALHEAP(i32);
#[repr(C)]
pub struct LPDDHAL_VIDMEMALLOC(i32);
#[repr(C)]
pub struct LPDDHAL_VIDMEMFREE(i32);
#[repr(C)]
pub struct LPDDHAL_WAITFORVERTICALBLANK(i32);
#[repr(C)]
pub struct LPDDHEL_INIT(i32);
#[repr(C)]
pub struct LPDIRECTDRAWENUMERATEEXA(i32);
#[repr(C)]
pub struct LPDIRECTDRAWENUMERATEEXW(i32);
pub const MAX_AUTOFLIP_BUFFERS: u32 = 10u32;
pub const MAX_DDDEVICEID_STRING: u32 = 512u32;
pub const MAX_DRIVER_NAME: u32 = 32u32;
pub const MAX_PALETTE_SIZE: u32 = 256u32;
#[repr(C)]
pub struct MDL(i32);
pub const MDL_64_BIT_VA: u32 = 32768u32;
pub const MDL_ALLOCATED_FIXED_SIZE: u32 = 8u32;
pub const MDL_ALLOCATED_MUST_SUCCEED: u32 = 16384u32;
pub const MDL_IO_PAGE_READ: u32 = 64u32;
pub const MDL_IO_SPACE: u32 = 2048u32;
pub const MDL_LOCK_HELD: u32 = 512u32;
pub const MDL_MAPPED_TO_SYSTEM_VA: u32 = 1u32;
pub const MDL_MAPPING_CAN_FAIL: u32 = 8192u32;
pub const MDL_NETWORK_HEADER: u32 = 4096u32;
pub const MDL_PAGES_LOCKED: u32 = 2u32;
pub const MDL_PARENT_MAPPED_SYSTEM_VA: u32 = 256u32;
pub const MDL_PARTIAL: u32 = 16u32;
pub const MDL_PARTIAL_HAS_BEEN_MAPPED: u32 = 32u32;
pub const MDL_SCATTER_GATHER_VA: u32 = 1024u32;
pub const MDL_SOURCE_IS_NONPAGED_POOL: u32 = 4u32;
pub const MDL_WRITE_OPERATION: u32 = 128u32;
pub const OBJECT_ISROOT: i32 = -2147483648i32;
#[repr(C)]
pub struct PDD_ALPHABLT(i32);
#[repr(C)]
pub struct PDD_CANCREATESURFACE(i32);
#[repr(C)]
pub struct PDD_COLORCB_COLORCONTROL(i32);
#[repr(C)]
pub struct PDD_CREATEPALETTE(i32);
#[repr(C)]
pub struct PDD_CREATESURFACE(i32);
#[repr(C)]
pub struct PDD_CREATESURFACEEX(i32);
#[repr(C)]
pub struct PDD_DESTROYDDLOCAL(i32);
#[repr(C)]
pub struct PDD_DESTROYDRIVER(i32);
#[repr(C)]
pub struct PDD_FLIPTOGDISURFACE(i32);
#[repr(C)]
pub struct PDD_FREEDRIVERMEMORY(i32);
#[repr(C)]
pub struct PDD_GETAVAILDRIVERMEMORY(i32);
#[repr(C)]
pub struct PDD_GETDRIVERINFO(i32);
#[repr(C)]
pub struct PDD_GETDRIVERSTATE(i32);
#[repr(C)]
pub struct PDD_GETSCANLINE(i32);
#[repr(C)]
pub struct PDD_KERNELCB_SYNCSURFACE(i32);
#[repr(C)]
pub struct PDD_KERNELCB_SYNCVIDEOPORT(i32);
#[repr(C)]
pub struct PDD_MAPMEMORY(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_BEGINFRAME(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_CREATE(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_DESTROY(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_ENDFRAME(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_GETCOMPBUFFINFO(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_GETFORMATS(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_GETGUIDS(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_GETINTERNALINFO(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_QUERYSTATUS(i32);
#[repr(C)]
pub struct PDD_MOCOMPCB_RENDER(i32);
#[repr(C)]
pub struct PDD_PALCB_DESTROYPALETTE(i32);
#[repr(C)]
pub struct PDD_PALCB_SETENTRIES(i32);
#[repr(C)]
pub struct PDD_SETCOLORKEY(i32);
#[repr(C)]
pub struct PDD_SETEXCLUSIVEMODE(i32);
#[repr(C)]
pub struct PDD_SETMODE(i32);
#[repr(C)]
pub struct PDD_SURFCB_ADDATTACHEDSURFACE(i32);
#[repr(C)]
pub struct PDD_SURFCB_BLT(i32);
#[repr(C)]
pub struct PDD_SURFCB_DESTROYSURFACE(i32);
#[repr(C)]
pub struct PDD_SURFCB_FLIP(i32);
#[repr(C)]
pub struct PDD_SURFCB_GETBLTSTATUS(i32);
#[repr(C)]
pub struct PDD_SURFCB_GETFLIPSTATUS(i32);
#[repr(C)]
pub struct PDD_SURFCB_LOCK(i32);
#[repr(C)]
pub struct PDD_SURFCB_SETCLIPLIST(i32);
#[repr(C)]
pub struct PDD_SURFCB_SETCOLORKEY(i32);
#[repr(C)]
pub struct PDD_SURFCB_SETOVERLAYPOSITION(i32);
#[repr(C)]
pub struct PDD_SURFCB_SETPALETTE(i32);
#[repr(C)]
pub struct PDD_SURFCB_UNLOCK(i32);
#[repr(C)]
pub struct PDD_SURFCB_UPDATEOVERLAY(i32);
#[repr(C)]
pub struct PDD_VPORTCB_CANCREATEVIDEOPORT(i32);
#[repr(C)]
pub struct PDD_VPORTCB_COLORCONTROL(i32);
#[repr(C)]
pub struct PDD_VPORTCB_CREATEVIDEOPORT(i32);
#[repr(C)]
pub struct PDD_VPORTCB_DESTROYVPORT(i32);
#[repr(C)]
pub struct PDD_VPORTCB_FLIP(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETAUTOFLIPSURF(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETBANDWIDTH(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETFIELD(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETFLIPSTATUS(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETINPUTFORMATS(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETLINE(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETOUTPUTFORMATS(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETSIGNALSTATUS(i32);
#[repr(C)]
pub struct PDD_VPORTCB_GETVPORTCONNECT(i32);
#[repr(C)]
pub struct PDD_VPORTCB_UPDATE(i32);
#[repr(C)]
pub struct PDD_VPORTCB_WAITFORSYNC(i32);
#[repr(C)]
pub struct PDD_WAITFORVERTICALBLANK(i32);
#[repr(C)]
pub struct PDX_BOBNEXTFIELD(i32);
#[repr(C)]
pub struct PDX_ENABLEIRQ(i32);
#[repr(C)]
pub struct PDX_FLIPOVERLAY(i32);
#[repr(C)]
pub struct PDX_FLIPVIDEOPORT(i32);
#[repr(C)]
pub struct PDX_GETCURRENTAUTOFLIP(i32);
#[repr(C)]
pub struct PDX_GETIRQINFO(i32);
#[repr(C)]
pub struct PDX_GETPOLARITY(i32);
#[repr(C)]
pub struct PDX_GETPREVIOUSAUTOFLIP(i32);
#[repr(C)]
pub struct PDX_GETTRANSFERSTATUS(i32);
#[repr(C)]
pub struct PDX_IRQCALLBACK(i32);
#[repr(C)]
pub struct PDX_LOCK(i32);
#[repr(C)]
pub struct PDX_SETSTATE(i32);
#[repr(C)]
pub struct PDX_SKIPNEXTFIELD(i32);
#[repr(C)]
pub struct PDX_TRANSFER(i32);
pub const PFINDEX_UNINITIALIZED: u32 = 0u32;
#[repr(C)]
pub struct PROCESS_LIST(i32);
pub const ROP_HAS_PATTERN: i32 = 2i32;
pub const ROP_HAS_SOURCE: i32 = 1i32;
#[repr(C)]
pub struct SURFACEALIGNMENT(i32);
pub const SURFACEALIGN_DISCARDABLE: i32 = 1i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDEOMEMORY(i32);
#[repr(C)]
pub struct VIDEOMEMORYINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDMEM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDMEMINFO(i32);
pub const VIDMEM_HEAPDISABLED: i32 = 32i32;
pub const VIDMEM_ISHEAP: i32 = 4i32;
pub const VIDMEM_ISLINEAR: i32 = 1i32;
pub const VIDMEM_ISNONLOCAL: i32 = 8i32;
pub const VIDMEM_ISRECTANGULAR: i32 = 2i32;
pub const VIDMEM_ISWC: i32 = 16i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VMEMHEAP(i32);
pub const VMEMHEAP_ALIGNMENT: i32 = 4i32;
pub const VMEMHEAP_LINEAR: i32 = 1i32;
pub const VMEMHEAP_RECTANGULAR: i32 = 2i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VMEML(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VMEMR(i32);
#[repr(C)]
pub struct _DDFXROP(i32);
#[repr(C)]
pub struct _DD_DESTROYDRIVERDATA(i32);
#[repr(C)]
pub struct _DD_GETVPORTAUTOFLIPSURFACEDATA(i32);
#[repr(C)]
pub struct _DD_SETMODEDATA(i32);
pub const _FACDD: u32 = 2166u32;
