#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct ACCESSRECTLIST {
    pub lpLink: *mut ACCESSRECTLIST,
    pub rDest: super::super::Foundation::RECT,
    pub lpOwner: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpSurfaceData: *mut ::std::ffi::c_void,
    pub dwFlags: u32,
    pub lpHeapAliasInfo: *mut HEAPALIASINFO,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ACCESSRECTLIST {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for ACCESSRECTLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for ACCESSRECTLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACCESSRECTLIST").field("lpLink", &self.lpLink).field("rDest", &self.rDest).field("lpOwner", &self.lpOwner).field("lpSurfaceData", &self.lpSurfaceData).field("dwFlags", &self.dwFlags).field("lpHeapAliasInfo", &self.lpHeapAliasInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for ACCESSRECTLIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.rDest == other.rDest && self.lpOwner == other.lpOwner && self.lpSurfaceData == other.lpSurfaceData && self.dwFlags == other.dwFlags && self.lpHeapAliasInfo == other.lpHeapAliasInfo
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for ACCESSRECTLIST {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for ACCESSRECTLIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const ACCESSRECT_BROKEN: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const ACCESSRECT_NOTHOLDINGWIN16LOCK: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const ACCESSRECT_VRAMSTYLE: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct ATTACHLIST {
    pub dwFlags: u32,
    pub lpLink: *mut ATTACHLIST,
    pub lpAttached: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpIAttached: *mut DDRAWI_DDRAWSURFACE_INT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ATTACHLIST {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for ATTACHLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for ATTACHLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATTACHLIST").field("dwFlags", &self.dwFlags).field("lpLink", &self.lpLink).field("lpAttached", &self.lpAttached).field("lpIAttached", &self.lpIAttached).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for ATTACHLIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpLink == other.lpLink && self.lpAttached == other.lpAttached && self.lpIAttached == other.lpIAttached
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for ATTACHLIST {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for ATTACHLIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const CCHDEVICENAME: u32 = 32u32;
pub const CLSID_DirectDraw: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3619098336, 17216, 4559, [176, 99, 0, 32, 175, 194, 205, 53]);
pub const CLSID_DirectDraw7: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1009799574, 20699, 4563, [156, 254, 0, 192, 79, 217, 48, 197]);
pub const CLSID_DirectDrawClipper: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1496848288, 32179, 4559, [162, 222, 0, 170, 0, 185, 51, 86]);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const CO_E_NOTINITIALIZED: i32 = -2147221008i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFMT_INTERNAL_D15S1: u32 = 73u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFMT_INTERNAL_D24S8: u32 = 75u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFMT_INTERNAL_D24X8: u32 = 77u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFMT_INTERNAL_D32: u32 = 71u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFMT_INTERNAL_S1D15: u32 = 72u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFMT_INTERNAL_S8D24: u32 = 74u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFMT_INTERNAL_X8D24: u32 = 76u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_MEMBEROFGROUP_ARGB: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_3DACCELERATION: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_AUTOGENMIPMAP: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_BUMPMAP: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_CONVERT_TO_ARGB: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_CUBETEXTURE: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_DISPLAYMODE: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_DMAP: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_NOALPHABLEND: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_NOFILTER: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_NOTEXCOORDWRAPNORMIP: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_OFFSCREENPLAIN: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_OFFSCREEN_RENDERTARGET: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_PIXELSIZE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_SAME_FORMAT_RENDERTARGET: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_SAME_FORMAT_UP_TO_ALPHA_RENDERTARGET: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_SRGBREAD: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_SRGBWRITE: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_TEXTURE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_VERTEXTEXTURE: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_VOLUMETEXTURE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_ZSTENCIL: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const D3DFORMAT_OP_ZSTENCIL_WITH_ARBITRARY_COLOR_DEPTH: i32 = 128i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DBLNODE {
    pub next: *mut DBLNODE,
    pub prev: *mut DBLNODE,
    pub object: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub object_int: *mut DDRAWI_DDRAWSURFACE_INT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DBLNODE {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DBLNODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DBLNODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DBLNODE").field("next", &self.next).field("prev", &self.prev).field("object", &self.object).field("object_int", &self.object_int).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DBLNODE {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.prev == other.prev && self.object == other.object && self.object_int == other.object_int
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DBLNODE {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DBLNODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DCICOMMAND: u32 = 3075u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
pub struct DD32BITDRIVERDATA {
    pub szName: [super::super::Foundation::CHAR; 260],
    pub szEntryPoint: [super::super::Foundation::CHAR; 64],
    pub dwContext: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DD32BITDRIVERDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DD32BITDRIVERDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DD32BITDRIVERDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD32BITDRIVERDATA").field("szName", &self.szName).field("szEntryPoint", &self.szEntryPoint).field("dwContext", &self.dwContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DD32BITDRIVERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.szEntryPoint == other.szEntryPoint && self.dwContext == other.dwContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DD32BITDRIVERDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DD32BITDRIVERDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDABLT_SRCOVERDEST: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDAL_IMPLICIT: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDARGB {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}
impl DDARGB {}
impl ::std::default::Default for DDARGB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDARGB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDARGB").field("blue", &self.blue).field("green", &self.green).field("red", &self.red).field("alpha", &self.alpha).finish()
    }
}
impl ::std::cmp::PartialEq for DDARGB {
    fn eq(&self, other: &Self) -> bool {
        self.blue == other.blue && self.green == other.green && self.red == other.red && self.alpha == other.alpha
    }
}
impl ::std::cmp::Eq for DDARGB {}
unsafe impl ::windows::runtime::Abi for DDARGB {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBD_1: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBD_16: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBD_2: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBD_24: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBD_32: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBD_4: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBD_8: i32 = 2048i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
pub struct DDBLTBATCH {
    pub lprDest: *mut super::super::Foundation::RECT,
    pub lpDDSSrc: ::std::option::Option<IDirectDrawSurface>,
    pub lprSrc: *mut super::super::Foundation::RECT,
    pub dwFlags: u32,
    pub lpDDBltFx: *mut DDBLTFX,
}
#[cfg(feature = "Win32_Foundation")]
impl DDBLTBATCH {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DDBLTBATCH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DDBLTBATCH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDBLTBATCH").field("lprDest", &self.lprDest).field("lpDDSSrc", &self.lpDDSSrc).field("lprSrc", &self.lprSrc).field("dwFlags", &self.dwFlags).field("lpDDBltFx", &self.lpDDBltFx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DDBLTBATCH {
    fn eq(&self, other: &Self) -> bool {
        self.lprDest == other.lprDest && self.lpDDSSrc == other.lpDDSSrc && self.lprSrc == other.lprSrc && self.dwFlags == other.dwFlags && self.lpDDBltFx == other.lpDDBltFx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DDBLTBATCH {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DDBLTBATCH {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFAST_DESTCOLORKEY: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFAST_DONOTWAIT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFAST_NOCOLORKEY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFAST_SRCCOLORKEY: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFAST_WAIT: u32 = 16u32;
impl ::std::clone::Clone for DDBLTFX {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
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
    pub Anonymous1: DDBLTFX_0,
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
impl DDBLTFX {}
impl ::std::default::Default for DDBLTFX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDBLTFX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDBLTFX {}
unsafe impl ::windows::runtime::Abi for DDBLTFX {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
impl ::std::clone::Clone for DDBLTFX_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDBLTFX_0 {
    pub dwZDestConst: u32,
    pub lpDDSZBufferDest: ::windows::runtime::RawPtr,
}
impl DDBLTFX_0 {}
impl ::std::default::Default for DDBLTFX_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDBLTFX_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDBLTFX_0 {}
unsafe impl ::windows::runtime::Abi for DDBLTFX_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
impl ::std::clone::Clone for DDBLTFX_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDBLTFX_1 {
    pub dwZSrcConst: u32,
    pub lpDDSZBufferSrc: ::windows::runtime::RawPtr,
}
impl DDBLTFX_1 {}
impl ::std::default::Default for DDBLTFX_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDBLTFX_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDBLTFX_1 {}
unsafe impl ::windows::runtime::Abi for DDBLTFX_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
impl ::std::clone::Clone for DDBLTFX_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDBLTFX_2 {
    pub dwAlphaDestConst: u32,
    pub lpDDSAlphaDest: ::windows::runtime::RawPtr,
}
impl DDBLTFX_2 {}
impl ::std::default::Default for DDBLTFX_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDBLTFX_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDBLTFX_2 {}
unsafe impl ::windows::runtime::Abi for DDBLTFX_2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
impl ::std::clone::Clone for DDBLTFX_3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDBLTFX_3 {
    pub dwAlphaSrcConst: u32,
    pub lpDDSAlphaSrc: ::windows::runtime::RawPtr,
}
impl DDBLTFX_3 {}
impl ::std::default::Default for DDBLTFX_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDBLTFX_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDBLTFX_3 {}
unsafe impl ::windows::runtime::Abi for DDBLTFX_3 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
impl ::std::clone::Clone for DDBLTFX_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDBLTFX_4 {
    pub dwFillColor: u32,
    pub dwFillDepth: u32,
    pub dwFillPixel: u32,
    pub lpDDSPattern: ::windows::runtime::RawPtr,
}
impl DDBLTFX_4 {}
impl ::std::default::Default for DDBLTFX_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDBLTFX_4 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDBLTFX_4 {}
unsafe impl ::windows::runtime::Abi for DDBLTFX_4 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_ARITHSTRETCHY: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_MIRRORLEFTRIGHT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_MIRRORUPDOWN: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_NOTEARING: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_ROTATE180: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_ROTATE270: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_ROTATE90: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_ZBUFFERBASEDEST: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLTFX_ZBUFFERRANGE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_AFLAGS: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHADEST: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHADESTCONSTOVERRIDE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHADESTNEG: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHADESTSURFACEOVERRIDE: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHAEDGEBLEND: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHASRC: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHASRCCONSTOVERRIDE: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHASRCNEG: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ALPHASRCSURFACEOVERRIDE: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ASYNC: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_COLORFILL: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_DDFX: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_DDROPS: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_DEPTHFILL: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_DONOTWAIT: i32 = 134217728i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_EXTENDED_FLAGS: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_EXTENDED_LINEAR_CONTENT: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_KEYDEST: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_KEYDESTOVERRIDE: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_KEYSRC: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_KEYSRCOVERRIDE: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_LAST_PRESENTATION: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_PRESENTATION: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ROP: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ROTATIONANGLE: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_WAIT: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ZBUFFER: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ZBUFFERDESTCONSTOVERRIDE: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ZBUFFERDESTOVERRIDE: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ZBUFFERSRCCONSTOVERRIDE: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDBLT_ZBUFFERSRCOVERRIDE: i32 = 8388608i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDBOBNEXTFIELDINFO {
    pub lpSurface: *mut DDSURFACEDATA,
}
impl DDBOBNEXTFIELDINFO {}
impl ::std::default::Default for DDBOBNEXTFIELDINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDBOBNEXTFIELDINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDBOBNEXTFIELDINFO").field("lpSurface", &self.lpSurface).finish()
    }
}
impl ::std::cmp::PartialEq for DDBOBNEXTFIELDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurface == other.lpSurface
    }
}
impl ::std::cmp::Eq for DDBOBNEXTFIELDINFO {}
unsafe impl ::windows::runtime::Abi for DDBOBNEXTFIELDINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_AUTOFLIPOVERLAY: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANBOBHARDWARE: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANBOBINTERLEAVED: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANBOBNONINTERLEAVED: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANCALIBRATEGAMMA: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANDROPZ16BIT: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANFLIPODDEVEN: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANMANAGERESOURCE: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANMANAGETEXTURE: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANRENDERWINDOWED: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CANSHARERESOURCE: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_CERTIFIED: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_COLORCONTROLOVERLAY: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_COLORCONTROLPRIMARY: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_COPYFOURCC: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_DYNAMICTEXTURES: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_FLIPINTERVAL: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_FLIPNOVSYNC: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_NO2DDURING3DSCENE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_NONLOCALVIDMEM: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_NONLOCALVIDMEMCAPS: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_NOPAGELOCKREQUIRED: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_PRIMARYGAMMA: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_RESERVED1: i32 = 134217728i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_STEREO: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_SYSTONONLOCAL_AS_SYSTOLOCAL: i32 = 67108864i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_TEXMANINNONLOCALVIDMEM: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_VIDEOPORT: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS2_WIDESURFACES: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_3D: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_ALIGNBOUNDARYDEST: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_ALIGNBOUNDARYSRC: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_ALIGNSIZEDEST: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_ALIGNSIZESRC: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_ALIGNSTRIDE: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_ALPHA: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_BANKSWITCHED: i32 = 134217728i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_BLT: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_BLTCOLORFILL: i32 = 67108864i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_BLTDEPTHFILL: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_BLTFOURCC: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_BLTQUEUE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_BLTSTRETCH: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_CANBLTSYSMEM: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_CANCLIP: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_CANCLIPSTRETCHED: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_COLORKEY: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_COLORKEYHWASSIST: i32 = 16777216i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
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
impl DDCAPS_DX1 {}
impl ::std::default::Default for DDCAPS_DX1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDCAPS_DX1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDCAPS_DX1")
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
impl ::std::cmp::PartialEq for DDCAPS_DX1 {
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
impl ::std::cmp::Eq for DDCAPS_DX1 {}
unsafe impl ::windows::runtime::Abi for DDCAPS_DX1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
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
impl DDCAPS_DX3 {}
impl ::std::default::Default for DDCAPS_DX3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDCAPS_DX3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDCAPS_DX3")
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
impl ::std::cmp::PartialEq for DDCAPS_DX3 {
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
impl ::std::cmp::Eq for DDCAPS_DX3 {}
unsafe impl ::windows::runtime::Abi for DDCAPS_DX3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
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
impl DDCAPS_DX5 {}
impl ::std::default::Default for DDCAPS_DX5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDCAPS_DX5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDCAPS_DX5")
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
impl ::std::cmp::PartialEq for DDCAPS_DX5 {
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
impl ::std::cmp::Eq for DDCAPS_DX5 {}
unsafe impl ::windows::runtime::Abi for DDCAPS_DX5 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
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
impl DDCAPS_DX6 {}
impl ::std::default::Default for DDCAPS_DX6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDCAPS_DX6 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDCAPS_DX6 {}
unsafe impl ::windows::runtime::Abi for DDCAPS_DX6 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
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
impl DDCAPS_DX7 {}
impl ::std::default::Default for DDCAPS_DX7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDCAPS_DX7 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDCAPS_DX7 {}
unsafe impl ::windows::runtime::Abi for DDCAPS_DX7 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_GDI: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_NOHARDWARE: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_OVERLAY: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_OVERLAYCANTCLIP: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_OVERLAYFOURCC: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_OVERLAYSTRETCH: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_PALETTE: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_PALETTEVSYNC: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_READSCANLINE: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_RESERVED1: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_VBI: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_ZBLTS: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCAPS_ZOVERLAYS: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTBLT: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTBLTCLRSPACE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTBLTCLRSPACEYUV: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTBLTYUV: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTOVERLAY: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACE: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACEYUV: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTOVERLAYONEACTIVE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_DESTOVERLAYYUV: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_NOCOSTOVERLAY: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCBLT: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCBLTCLRSPACE: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCBLTCLRSPACEYUV: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCBLTYUV: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCOVERLAY: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACE: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACEYUV: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCOVERLAYONEACTIVE: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEYCAPS_SRCOVERLAYYUV: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEY_COLORSPACE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEY_DESTBLT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEY_DESTOVERLAY: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEY_SRCBLT: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCKEY_SRCOVERLAY: i32 = 16i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
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
impl DDCOLORCONTROL {}
impl ::std::default::Default for DDCOLORCONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDCOLORCONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDCOLORCONTROL")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("lBrightness", &self.lBrightness)
            .field("lContrast", &self.lContrast)
            .field("lHue", &self.lHue)
            .field("lSaturation", &self.lSaturation)
            .field("lSharpness", &self.lSharpness)
            .field("lGamma", &self.lGamma)
            .field("lColorEnable", &self.lColorEnable)
            .field("dwReserved1", &self.dwReserved1)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DDCOLORCONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.lBrightness == other.lBrightness && self.lContrast == other.lContrast && self.lHue == other.lHue && self.lSaturation == other.lSaturation && self.lSharpness == other.lSharpness && self.lGamma == other.lGamma && self.lColorEnable == other.lColorEnable && self.dwReserved1 == other.dwReserved1
    }
}
impl ::std::cmp::Eq for DDCOLORCONTROL {}
unsafe impl ::windows::runtime::Abi for DDCOLORCONTROL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDCOLORKEY {
    pub dwColorSpaceLowValue: u32,
    pub dwColorSpaceHighValue: u32,
}
impl DDCOLORKEY {}
impl ::std::default::Default for DDCOLORKEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDCOLORKEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDCOLORKEY").field("dwColorSpaceLowValue", &self.dwColorSpaceLowValue).field("dwColorSpaceHighValue", &self.dwColorSpaceHighValue).finish()
    }
}
impl ::std::cmp::PartialEq for DDCOLORKEY {
    fn eq(&self, other: &Self) -> bool {
        self.dwColorSpaceLowValue == other.dwColorSpaceLowValue && self.dwColorSpaceHighValue == other.dwColorSpaceHighValue
    }
}
impl ::std::cmp::Eq for DDCOLORKEY {}
unsafe impl ::windows::runtime::Abi for DDCOLORKEY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCOLOR_BRIGHTNESS: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCOLOR_COLORENABLE: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCOLOR_CONTRAST: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCOLOR_GAMMA: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCOLOR_HUE: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCOLOR_SATURATION: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCOLOR_SHARPNESS: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCREATEDRIVEROBJECT: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCREATE_EMULATIONONLY: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDCREATE_HARDWAREONLY: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
pub struct DDDEVICEIDENTIFIER {
    pub szDriver: [super::super::Foundation::CHAR; 512],
    pub szDescription: [super::super::Foundation::CHAR; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl DDDEVICEIDENTIFIER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DDDEVICEIDENTIFIER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DDDEVICEIDENTIFIER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDDEVICEIDENTIFIER")
            .field("szDriver", &self.szDriver)
            .field("szDescription", &self.szDescription)
            .field("liDriverVersion", &self.liDriverVersion)
            .field("dwVendorId", &self.dwVendorId)
            .field("dwDeviceId", &self.dwDeviceId)
            .field("dwSubSysId", &self.dwSubSysId)
            .field("dwRevision", &self.dwRevision)
            .field("guidDeviceIdentifier", &self.guidDeviceIdentifier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DDDEVICEIDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.szDriver == other.szDriver && self.szDescription == other.szDescription && self.liDriverVersion == other.liDriverVersion && self.dwVendorId == other.dwVendorId && self.dwDeviceId == other.dwDeviceId && self.dwSubSysId == other.dwSubSysId && self.dwRevision == other.dwRevision && self.guidDeviceIdentifier == other.guidDeviceIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DDDEVICEIDENTIFIER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DDDEVICEIDENTIFIER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
pub struct DDDEVICEIDENTIFIER2 {
    pub szDriver: [super::super::Foundation::CHAR; 512],
    pub szDescription: [super::super::Foundation::CHAR; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: ::windows::runtime::GUID,
    pub dwWHQLLevel: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DDDEVICEIDENTIFIER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DDDEVICEIDENTIFIER2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DDDEVICEIDENTIFIER2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDDEVICEIDENTIFIER2")
            .field("szDriver", &self.szDriver)
            .field("szDescription", &self.szDescription)
            .field("liDriverVersion", &self.liDriverVersion)
            .field("dwVendorId", &self.dwVendorId)
            .field("dwDeviceId", &self.dwDeviceId)
            .field("dwSubSysId", &self.dwSubSysId)
            .field("dwRevision", &self.dwRevision)
            .field("guidDeviceIdentifier", &self.guidDeviceIdentifier)
            .field("dwWHQLLevel", &self.dwWHQLLevel)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DDDEVICEIDENTIFIER2 {
    fn eq(&self, other: &Self) -> bool {
        self.szDriver == other.szDriver && self.szDescription == other.szDescription && self.liDriverVersion == other.liDriverVersion && self.dwVendorId == other.dwVendorId && self.dwDeviceId == other.dwDeviceId && self.dwSubSysId == other.dwSubSysId && self.dwRevision == other.dwRevision && self.guidDeviceIdentifier == other.guidDeviceIdentifier && self.dwWHQLLevel == other.dwWHQLLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DDDEVICEIDENTIFIER2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DDDEVICEIDENTIFIER2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDEDM_REFRESHRATES: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDEDM_STANDARDVGAMODES: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDEM_MODEFAILED: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDEM_MODEPASSED: i32 = 1i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDENABLEIRQINFO {
    pub dwIRQSources: u32,
    pub dwLine: u32,
    pub IRQCallback: ::std::option::Option<PDX_IRQCALLBACK>,
    pub lpIRQData: *mut DX_IRQDATA,
}
impl DDENABLEIRQINFO {}
impl ::std::default::Default for DDENABLEIRQINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDENABLEIRQINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDENABLEIRQINFO").field("dwIRQSources", &self.dwIRQSources).field("dwLine", &self.dwLine).field("lpIRQData", &self.lpIRQData).finish()
    }
}
impl ::std::cmp::PartialEq for DDENABLEIRQINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwIRQSources == other.dwIRQSources && self.dwLine == other.dwLine && self.IRQCallback.map(|f| f as usize) == other.IRQCallback.map(|f| f as usize) && self.lpIRQData == other.lpIRQData
    }
}
impl ::std::cmp::Eq for DDENABLEIRQINFO {}
unsafe impl ::windows::runtime::Abi for DDENABLEIRQINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMOVERLAYZ_BACKTOFRONT: i32 = 0i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMOVERLAYZ_FRONTTOBACK: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMRET_CANCEL: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMRET_OK: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMSURFACES_ALL: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMSURFACES_CANBECREATED: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMSURFACES_DOESEXIST: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMSURFACES_MATCH: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUMSURFACES_NOMATCH: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUM_ATTACHEDSECONDARYDEVICES: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUM_DETACHEDSECONDARYDEVICES: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDENUM_NONDISPLAYDEVICES: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDERR_NOTINITIALIZED: i32 = -2147221008i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDFLIPOVERLAYINFO {
    pub lpCurrentSurface: *mut DDSURFACEDATA,
    pub lpTargetSurface: *mut DDSURFACEDATA,
    pub dwFlags: u32,
}
impl DDFLIPOVERLAYINFO {}
impl ::std::default::Default for DDFLIPOVERLAYINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDFLIPOVERLAYINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDFLIPOVERLAYINFO").field("lpCurrentSurface", &self.lpCurrentSurface).field("lpTargetSurface", &self.lpTargetSurface).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::std::cmp::PartialEq for DDFLIPOVERLAYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpCurrentSurface == other.lpCurrentSurface && self.lpTargetSurface == other.lpTargetSurface && self.dwFlags == other.dwFlags
    }
}
impl ::std::cmp::Eq for DDFLIPOVERLAYINFO {}
unsafe impl ::windows::runtime::Abi for DDFLIPOVERLAYINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDFLIPVIDEOPORTINFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
    pub lpCurrentSurface: *mut DDSURFACEDATA,
    pub lpTargetSurface: *mut DDSURFACEDATA,
    pub dwFlipVPFlags: u32,
}
impl DDFLIPVIDEOPORTINFO {}
impl ::std::default::Default for DDFLIPVIDEOPORTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDFLIPVIDEOPORTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDFLIPVIDEOPORTINFO").field("lpVideoPortData", &self.lpVideoPortData).field("lpCurrentSurface", &self.lpCurrentSurface).field("lpTargetSurface", &self.lpTargetSurface).field("dwFlipVPFlags", &self.dwFlipVPFlags).finish()
    }
}
impl ::std::cmp::PartialEq for DDFLIPVIDEOPORTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData && self.lpCurrentSurface == other.lpCurrentSurface && self.lpTargetSurface == other.lpTargetSurface && self.dwFlipVPFlags == other.dwFlipVPFlags
    }
}
impl ::std::cmp::Eq for DDFLIPVIDEOPORTINFO {}
unsafe impl ::windows::runtime::Abi for DDFLIPVIDEOPORTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_DONOTWAIT: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_EVEN: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_INTERVAL2: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_INTERVAL3: i32 = 50331648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_INTERVAL4: i32 = 67108864i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_NOVSYNC: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_ODD: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_STEREO: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFLIP_WAIT: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_BLTALPHAEDGEBLEND: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_BLTALPHAPIXELS: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_BLTALPHAPIXELSNEG: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_BLTALPHASURFACES: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_BLTALPHASURFACESNEG: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_OVERLAYALPHAEDGEBLEND: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELS: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELSNEG: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_OVERLAYALPHASURFACES: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXALPHACAPS_OVERLAYALPHASURFACESNEG: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTALPHA: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTARITHSTRETCHY: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTARITHSTRETCHYN: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTFILTER: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTMIRRORLEFTRIGHT: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTMIRRORUPDOWN: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTROTATION: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTROTATION90: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTSHRINKX: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTSHRINKXN: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTSHRINKY: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTSHRINKYN: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTSTRETCHX: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTSTRETCHXN: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTSTRETCHY: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_BLTSTRETCHYN: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYALPHA: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYARITHSTRETCHY: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYARITHSTRETCHYN: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYDEINTERLACE: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYFILTER: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYMIRRORLEFTRIGHT: i32 = 134217728i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYMIRRORUPDOWN: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYSHRINKX: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYSHRINKXN: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYSHRINKY: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYSHRINKYN: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYSTRETCHX: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYSTRETCHXN: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYSTRETCHY: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDFXCAPS_OVERLAYSTRETCHYN: i32 = 67108864i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGAMMARAMP {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}
impl DDGAMMARAMP {}
impl ::std::default::Default for DDGAMMARAMP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGAMMARAMP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGAMMARAMP").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::std::cmp::PartialEq for DDGAMMARAMP {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::std::cmp::Eq for DDGAMMARAMP {}
unsafe impl ::windows::runtime::Abi for DDGAMMARAMP {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDGBS_CANBLT: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDGBS_ISBLTDONE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDGDI_GETHOSTIDENTIFIER: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDGET32BITDRIVERNAME: u32 = 11u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGETCURRENTAUTOFLIPININFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
}
impl DDGETCURRENTAUTOFLIPININFO {}
impl ::std::default::Default for DDGETCURRENTAUTOFLIPININFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGETCURRENTAUTOFLIPININFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGETCURRENTAUTOFLIPININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::std::cmp::PartialEq for DDGETCURRENTAUTOFLIPININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::std::cmp::Eq for DDGETCURRENTAUTOFLIPININFO {}
unsafe impl ::windows::runtime::Abi for DDGETCURRENTAUTOFLIPININFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGETCURRENTAUTOFLIPOUTINFO {
    pub dwSurfaceIndex: u32,
    pub dwVBISurfaceIndex: u32,
}
impl DDGETCURRENTAUTOFLIPOUTINFO {}
impl ::std::default::Default for DDGETCURRENTAUTOFLIPOUTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGETCURRENTAUTOFLIPOUTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGETCURRENTAUTOFLIPOUTINFO").field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
impl ::std::cmp::PartialEq for DDGETCURRENTAUTOFLIPOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
impl ::std::cmp::Eq for DDGETCURRENTAUTOFLIPOUTINFO {}
unsafe impl ::windows::runtime::Abi for DDGETCURRENTAUTOFLIPOUTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGETIRQINFO {
    pub dwFlags: u32,
}
impl DDGETIRQINFO {}
impl ::std::default::Default for DDGETIRQINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGETIRQINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGETIRQINFO").field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::std::cmp::PartialEq for DDGETIRQINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
    }
}
impl ::std::cmp::Eq for DDGETIRQINFO {}
unsafe impl ::windows::runtime::Abi for DDGETIRQINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGETPOLARITYININFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
}
impl DDGETPOLARITYININFO {}
impl ::std::default::Default for DDGETPOLARITYININFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGETPOLARITYININFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGETPOLARITYININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::std::cmp::PartialEq for DDGETPOLARITYININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::std::cmp::Eq for DDGETPOLARITYININFO {}
unsafe impl ::windows::runtime::Abi for DDGETPOLARITYININFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGETPOLARITYOUTINFO {
    pub bPolarity: u32,
}
impl DDGETPOLARITYOUTINFO {}
impl ::std::default::Default for DDGETPOLARITYOUTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGETPOLARITYOUTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGETPOLARITYOUTINFO").field("bPolarity", &self.bPolarity).finish()
    }
}
impl ::std::cmp::PartialEq for DDGETPOLARITYOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bPolarity == other.bPolarity
    }
}
impl ::std::cmp::Eq for DDGETPOLARITYOUTINFO {}
unsafe impl ::windows::runtime::Abi for DDGETPOLARITYOUTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGETPREVIOUSAUTOFLIPININFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
}
impl DDGETPREVIOUSAUTOFLIPININFO {}
impl ::std::default::Default for DDGETPREVIOUSAUTOFLIPININFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGETPREVIOUSAUTOFLIPININFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGETPREVIOUSAUTOFLIPININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::std::cmp::PartialEq for DDGETPREVIOUSAUTOFLIPININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::std::cmp::Eq for DDGETPREVIOUSAUTOFLIPININFO {}
unsafe impl ::windows::runtime::Abi for DDGETPREVIOUSAUTOFLIPININFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGETPREVIOUSAUTOFLIPOUTINFO {
    pub dwSurfaceIndex: u32,
    pub dwVBISurfaceIndex: u32,
}
impl DDGETPREVIOUSAUTOFLIPOUTINFO {}
impl ::std::default::Default for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGETPREVIOUSAUTOFLIPOUTINFO").field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
impl ::std::cmp::PartialEq for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
impl ::std::cmp::Eq for DDGETPREVIOUSAUTOFLIPOUTINFO {}
unsafe impl ::windows::runtime::Abi for DDGETPREVIOUSAUTOFLIPOUTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDGETTRANSFERSTATUSOUTINFO {
    pub dwTransferID: usize,
}
impl DDGETTRANSFERSTATUSOUTINFO {}
impl ::std::default::Default for DDGETTRANSFERSTATUSOUTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDGETTRANSFERSTATUSOUTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDGETTRANSFERSTATUSOUTINFO").field("dwTransferID", &self.dwTransferID).finish()
    }
}
impl ::std::cmp::PartialEq for DDGETTRANSFERSTATUSOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwTransferID == other.dwTransferID
    }
}
impl ::std::cmp::Eq for DDGETTRANSFERSTATUSOUTINFO {}
unsafe impl ::windows::runtime::Abi for DDGETTRANSFERSTATUSOUTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDGFS_CANFLIP: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDGFS_ISFLIPDONE: i32 = 2i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHALDDRAWFNS {
    pub dwSize: u32,
    pub lpSetInfo: ::std::option::Option<LPDDHAL_SETINFO>,
    pub lpVidMemAlloc: ::std::option::Option<LPDDHAL_VIDMEMALLOC>,
    pub lpVidMemFree: ::std::option::Option<LPDDHAL_VIDMEMFREE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHALDDRAWFNS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHALDDRAWFNS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHALDDRAWFNS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHALDDRAWFNS").field("dwSize", &self.dwSize).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHALDDRAWFNS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpSetInfo.map(|f| f as usize) == other.lpSetInfo.map(|f| f as usize) && self.lpVidMemAlloc.map(|f| f as usize) == other.lpVidMemAlloc.map(|f| f as usize) && self.lpVidMemFree.map(|f| f as usize) == other.lpVidMemFree.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHALDDRAWFNS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHALDDRAWFNS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::clone::Clone for DDHALINFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHALINFO {
    pub dwSize: u32,
    pub lpDDCallbacks: *mut DDHAL_DDCALLBACKS,
    pub lpDDSurfaceCallbacks: *mut DDHAL_DDSURFACECALLBACKS,
    pub lpDDPaletteCallbacks: *mut DDHAL_DDPALETTECALLBACKS,
    pub vmiData: VIDMEMINFO,
    pub ddCaps: super::super::UI::DisplayDevices::DDCORECAPS,
    pub dwMonitorFrequency: u32,
    pub GetDriverInfo: ::std::option::Option<LPDDHAL_GETDRIVERINFO>,
    pub dwModeIndex: u32,
    pub lpdwFourCC: *mut u32,
    pub dwNumModes: u32,
    pub lpModeInfo: *mut DDHALMODEINFO,
    pub dwFlags: u32,
    pub lpPDevice: *mut ::std::ffi::c_void,
    pub hInstance: u32,
    pub lpD3DGlobalDriverData: usize,
    pub lpD3DHALCallbacks: usize,
    pub lpDDExeBufCallbacks: *mut DDHAL_DDEXEBUFCALLBACKS,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHALINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHALINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHALINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHALINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHALINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHALINFO_GETDRIVERINFO2: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHALINFO_GETDRIVERINFOSET: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHALINFO_ISPRIMARYDISPLAY: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHALINFO_MODEXILLEGAL: i32 = 2i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDHALMODEINFO {
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lPitch: i32,
    pub dwBPP: u32,
    pub wFlags: u16,
    pub wRefreshRate: u16,
    pub dwRBitMask: u32,
    pub dwGBitMask: u32,
    pub dwBBitMask: u32,
    pub dwAlphaBitMask: u32,
}
impl DDHALMODEINFO {}
impl ::std::default::Default for DDHALMODEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDHALMODEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHALMODEINFO")
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("lPitch", &self.lPitch)
            .field("dwBPP", &self.dwBPP)
            .field("wFlags", &self.wFlags)
            .field("wRefreshRate", &self.wRefreshRate)
            .field("dwRBitMask", &self.dwRBitMask)
            .field("dwGBitMask", &self.dwGBitMask)
            .field("dwBBitMask", &self.dwBBitMask)
            .field("dwAlphaBitMask", &self.dwAlphaBitMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DDHALMODEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.lPitch == other.lPitch && self.dwBPP == other.dwBPP && self.wFlags == other.wFlags && self.wRefreshRate == other.wRefreshRate && self.dwRBitMask == other.dwRBitMask && self.dwGBitMask == other.dwGBitMask && self.dwBBitMask == other.dwBBitMask && self.dwAlphaBitMask == other.dwAlphaBitMask
    }
}
impl ::std::cmp::Eq for DDHALMODEINFO {}
unsafe impl ::windows::runtime::Abi for DDHALMODEINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_ADDATTACHEDSURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpSurfAttached: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub AddAttachedSurface: ::std::option::Option<LPDDHALSURFCB_ADDATTACHEDSURFACE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_ADDATTACHEDSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_ADDATTACHEDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_ADDATTACHEDSURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_ADDATTACHEDSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpSurfAttached", &self.lpSurfAttached).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_ADDATTACHEDSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpSurfAttached == other.lpSurfAttached && self.ddRVal == other.ddRVal && self.AddAttachedSurface.map(|f| f as usize) == other.AddAttachedSurface.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_ADDATTACHEDSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_ADDATTACHEDSURFACEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_BEGINMOCOMPFRAMEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpDestSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwInputDataSize: u32,
    pub lpInputData: *mut ::std::ffi::c_void,
    pub dwOutputDataSize: u32,
    pub lpOutputData: *mut ::std::ffi::c_void,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub BeginMoCompFrame: ::std::option::Option<LPDDHALMOCOMPCB_BEGINFRAME>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_BEGINMOCOMPFRAMEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_BEGINMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_BEGINMOCOMPFRAMEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_BEGINMOCOMPFRAMEDATA")
            .field("lpDD", &self.lpDD)
            .field("lpMoComp", &self.lpMoComp)
            .field("lpDestSurface", &self.lpDestSurface)
            .field("dwInputDataSize", &self.dwInputDataSize)
            .field("lpInputData", &self.lpInputData)
            .field("dwOutputDataSize", &self.dwOutputDataSize)
            .field("lpOutputData", &self.lpOutputData)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_BEGINMOCOMPFRAMEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpDestSurface == other.lpDestSurface && self.dwInputDataSize == other.dwInputDataSize && self.lpInputData == other.lpInputData && self.dwOutputDataSize == other.dwOutputDataSize && self.lpOutputData == other.lpOutputData && self.ddRVal == other.ddRVal && self.BeginMoCompFrame.map(|f| f as usize) == other.BeginMoCompFrame.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_BEGINMOCOMPFRAMEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_BEGINMOCOMPFRAMEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::clone::Clone for DDHAL_BLTDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_BLTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDDestSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub rDest: super::super::Foundation::RECTL,
    pub lpDDSrcSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub rSrc: super::super::Foundation::RECTL,
    pub dwFlags: u32,
    pub dwROPFlags: u32,
    pub bltFX: DDBLTFX,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub Blt: ::std::option::Option<LPDDHALSURFCB_BLT>,
    pub IsClipped: super::super::Foundation::BOOL,
    pub rOrigDest: super::super::Foundation::RECTL,
    pub rOrigSrc: super::super::Foundation::RECTL,
    pub dwRectCnt: u32,
    pub prDestRects: *mut super::super::Foundation::RECT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_BLTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_BLTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_BLTDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_BLTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_BLTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_CALLBACKS {
    pub cbDDCallbacks: DDHAL_DDCALLBACKS,
    pub cbDDSurfaceCallbacks: DDHAL_DDSURFACECALLBACKS,
    pub cbDDPaletteCallbacks: DDHAL_DDPALETTECALLBACKS,
    pub HALDD: DDHAL_DDCALLBACKS,
    pub HALDDSurface: DDHAL_DDSURFACECALLBACKS,
    pub HALDDPalette: DDHAL_DDPALETTECALLBACKS,
    pub HELDD: DDHAL_DDCALLBACKS,
    pub HELDDSurface: DDHAL_DDSURFACECALLBACKS,
    pub HELDDPalette: DDHAL_DDPALETTECALLBACKS,
    pub cbDDExeBufCallbacks: DDHAL_DDEXEBUFCALLBACKS,
    pub HALDDExeBuf: DDHAL_DDEXEBUFCALLBACKS,
    pub HELDDExeBuf: DDHAL_DDEXEBUFCALLBACKS,
    pub cbDDVideoPortCallbacks: DDHAL_DDVIDEOPORTCALLBACKS,
    pub HALDDVideoPort: DDHAL_DDVIDEOPORTCALLBACKS,
    pub cbDDColorControlCallbacks: DDHAL_DDCOLORCONTROLCALLBACKS,
    pub HALDDColorControl: DDHAL_DDCOLORCONTROLCALLBACKS,
    pub cbDDMiscellaneousCallbacks: DDHAL_DDMISCELLANEOUSCALLBACKS,
    pub HALDDMiscellaneous: DDHAL_DDMISCELLANEOUSCALLBACKS,
    pub cbDDKernelCallbacks: DDHAL_DDKERNELCALLBACKS,
    pub HALDDKernel: DDHAL_DDKERNELCALLBACKS,
    pub cbDDMotionCompCallbacks: DDHAL_DDMOTIONCOMPCALLBACKS,
    pub HALDDMotionComp: DDHAL_DDMOTIONCOMPCALLBACKS,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_CALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_CALLBACKS")
            .field("cbDDCallbacks", &self.cbDDCallbacks)
            .field("cbDDSurfaceCallbacks", &self.cbDDSurfaceCallbacks)
            .field("cbDDPaletteCallbacks", &self.cbDDPaletteCallbacks)
            .field("HALDD", &self.HALDD)
            .field("HALDDSurface", &self.HALDDSurface)
            .field("HALDDPalette", &self.HALDDPalette)
            .field("HELDD", &self.HELDD)
            .field("HELDDSurface", &self.HELDDSurface)
            .field("HELDDPalette", &self.HELDDPalette)
            .field("cbDDExeBufCallbacks", &self.cbDDExeBufCallbacks)
            .field("HALDDExeBuf", &self.HALDDExeBuf)
            .field("HELDDExeBuf", &self.HELDDExeBuf)
            .field("cbDDVideoPortCallbacks", &self.cbDDVideoPortCallbacks)
            .field("HALDDVideoPort", &self.HALDDVideoPort)
            .field("cbDDColorControlCallbacks", &self.cbDDColorControlCallbacks)
            .field("HALDDColorControl", &self.HALDDColorControl)
            .field("cbDDMiscellaneousCallbacks", &self.cbDDMiscellaneousCallbacks)
            .field("HALDDMiscellaneous", &self.HALDDMiscellaneous)
            .field("cbDDKernelCallbacks", &self.cbDDKernelCallbacks)
            .field("HALDDKernel", &self.HALDDKernel)
            .field("cbDDMotionCompCallbacks", &self.cbDDMotionCompCallbacks)
            .field("HALDDMotionComp", &self.HALDDMotionComp)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.cbDDCallbacks == other.cbDDCallbacks
            && self.cbDDSurfaceCallbacks == other.cbDDSurfaceCallbacks
            && self.cbDDPaletteCallbacks == other.cbDDPaletteCallbacks
            && self.HALDD == other.HALDD
            && self.HALDDSurface == other.HALDDSurface
            && self.HALDDPalette == other.HALDDPalette
            && self.HELDD == other.HELDD
            && self.HELDDSurface == other.HELDDSurface
            && self.HELDDPalette == other.HELDDPalette
            && self.cbDDExeBufCallbacks == other.cbDDExeBufCallbacks
            && self.HALDDExeBuf == other.HALDDExeBuf
            && self.HELDDExeBuf == other.HELDDExeBuf
            && self.cbDDVideoPortCallbacks == other.cbDDVideoPortCallbacks
            && self.HALDDVideoPort == other.HALDDVideoPort
            && self.cbDDColorControlCallbacks == other.cbDDColorControlCallbacks
            && self.HALDDColorControl == other.HALDDColorControl
            && self.cbDDMiscellaneousCallbacks == other.cbDDMiscellaneousCallbacks
            && self.HALDDMiscellaneous == other.HALDDMiscellaneous
            && self.cbDDKernelCallbacks == other.cbDDKernelCallbacks
            && self.HALDDKernel == other.HALDDKernel
            && self.cbDDMotionCompCallbacks == other.cbDDMotionCompCallbacks
            && self.HALDDMotionComp == other.HALDDMotionComp
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_CALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_CANCREATESURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurfaceDesc: *mut DDSURFACEDESC,
    pub bIsDifferentPixelFormat: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CanCreateSurface: ::std::option::Option<LPDDHAL_CANCREATESURFACE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_CANCREATESURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_CANCREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_CANCREATESURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_CANCREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("bIsDifferentPixelFormat", &self.bIsDifferentPixelFormat).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_CANCREATESURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurfaceDesc == other.lpDDSurfaceDesc && self.bIsDifferentPixelFormat == other.bIsDifferentPixelFormat && self.ddRVal == other.ddRVal && self.CanCreateSurface.map(|f| f as usize) == other.CanCreateSurface.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_CANCREATESURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_CANCREATESURFACEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_CANCREATEVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpDDVideoPortDesc: *mut super::super::UI::DisplayDevices::DDVIDEOPORTDESC,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CanCreateVideoPort: ::std::option::Option<LPDDHALVPORTCB_CANCREATEVIDEOPORT>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_CANCREATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_CANCREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_CANCREATEVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_CANCREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_CANCREATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDVideoPortDesc == other.lpDDVideoPortDesc && self.ddRVal == other.ddRVal && self.CanCreateVideoPort.map(|f| f as usize) == other.CanCreateVideoPort.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_CANCREATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_CANCREATEVPORTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_CANCREATESURFACE: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_CREATEPALETTE: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_CREATESURFACE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_DESTROYDRIVER: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_FLIPTOGDISURFACE: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_GETSCANLINE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_MAPMEMORY: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_SETCOLORKEY: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_SETEXCLUSIVEMODE: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_SETMODE: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CB32_WAITFORVERTICALBLANK: i32 = 16i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_COLORCONTROLDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpColorData: *mut DDCOLORCONTROL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub ColorControl: ::std::option::Option<LPDDHALCOLORCB_COLORCONTROL>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_COLORCONTROLDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_COLORCONTROLDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_COLORCONTROLDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_COLORCONTROLDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpColorData", &self.lpColorData).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_COLORCONTROLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpColorData == other.lpColorData && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.ColorControl.map(|f| f as usize) == other.ColorControl.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_COLORCONTROLDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_COLORCONTROLDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_COLOR_COLORCONTROL: i32 = 1i32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::clone::Clone for DDHAL_CREATEMOCOMPDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_CREATEMOCOMPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: DDPIXELFORMAT,
    pub lpData: *mut ::std::ffi::c_void,
    pub dwDataSize: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CreateMoComp: ::std::option::Option<LPDDHALMOCOMPCB_CREATE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_CREATEMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_CREATEMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_CREATEMOCOMPDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_CREATEMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_CREATEMOCOMPDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_CREATEPALETTEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub lpColorTable: *mut super::Gdi::PALETTEENTRY,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CreatePalette: ::std::option::Option<LPDDHAL_CREATEPALETTE>,
    pub is_excl: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_CREATEPALETTEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_CREATEPALETTEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_CREATEPALETTEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_CREATEPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("lpColorTable", &self.lpColorTable).field("ddRVal", &self.ddRVal).field("is_excl", &self.is_excl).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_CREATEPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.lpColorTable == other.lpColorTable && self.ddRVal == other.ddRVal && self.CreatePalette.map(|f| f as usize) == other.CreatePalette.map(|f| f as usize) && self.is_excl == other.is_excl
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_CREATEPALETTEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_CREATEPALETTEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_CREATESURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurfaceDesc: *mut DDSURFACEDESC,
    pub lplpSList: *mut *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwSCnt: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CreateSurface: ::std::option::Option<LPDDHAL_CREATESURFACE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_CREATESURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_CREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_CREATESURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_CREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("lplpSList", &self.lplpSList).field("dwSCnt", &self.dwSCnt).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_CREATESURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurfaceDesc == other.lpDDSurfaceDesc && self.lplpSList == other.lplpSList && self.dwSCnt == other.dwSCnt && self.ddRVal == other.ddRVal && self.CreateSurface.map(|f| f as usize) == other.CreateSurface.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_CREATESURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_CREATESURFACEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_CREATESURFACEEXDATA {
    pub dwFlags: u32,
    pub lpDDLcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpDDSLcl: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_CREATESURFACEEXDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_CREATESURFACEEXDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_CREATESURFACEEXDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_CREATESURFACEEXDATA").field("dwFlags", &self.dwFlags).field("lpDDLcl", &self.lpDDLcl).field("lpDDSLcl", &self.lpDDSLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_CREATESURFACEEXDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpDDLcl == other.lpDDLcl && self.lpDDSLcl == other.lpDDSLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_CREATESURFACEEXDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_CREATESURFACEEXDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_CREATESURFACEEX_SWAPHANDLES: i32 = 1i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_CREATEVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpDDVideoPortDesc: *mut super::super::UI::DisplayDevices::DDVIDEOPORTDESC,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CreateVideoPort: ::std::option::Option<LPDDHALVPORTCB_CREATEVIDEOPORT>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_CREATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_CREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_CREATEVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_CREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_CREATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDVideoPortDesc == other.lpDDVideoPortDesc && self.lpVideoPort == other.lpVideoPort && self.ddRVal == other.ddRVal && self.CreateVideoPort.map(|f| f as usize) == other.CreateVideoPort.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_CREATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_CREATEVPORTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_D3DBUFCB32_CANCREATED3DBUF: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_D3DBUFCB32_CREATED3DBUF: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_D3DBUFCB32_DESTROYD3DBUF: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_D3DBUFCB32_LOCKD3DBUF: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_D3DBUFCB32_UNLOCKD3DBUF: i32 = 16i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroyDriver: ::std::option::Option<LPDDHAL_DESTROYDRIVER>,
    pub CreateSurface: ::std::option::Option<LPDDHAL_CREATESURFACE>,
    pub SetColorKey: ::std::option::Option<LPDDHAL_SETCOLORKEY>,
    pub SetMode: ::std::option::Option<LPDDHAL_SETMODE>,
    pub WaitForVerticalBlank: ::std::option::Option<LPDDHAL_WAITFORVERTICALBLANK>,
    pub CanCreateSurface: ::std::option::Option<LPDDHAL_CANCREATESURFACE>,
    pub CreatePalette: ::std::option::Option<LPDDHAL_CREATEPALETTE>,
    pub GetScanLine: ::std::option::Option<LPDDHAL_GETSCANLINE>,
    pub SetExclusiveMode: ::std::option::Option<LPDDHAL_SETEXCLUSIVEMODE>,
    pub FlipToGDISurface: ::std::option::Option<LPDDHAL_FLIPTOGDISURFACE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.DestroyDriver.map(|f| f as usize) == other.DestroyDriver.map(|f| f as usize)
            && self.CreateSurface.map(|f| f as usize) == other.CreateSurface.map(|f| f as usize)
            && self.SetColorKey.map(|f| f as usize) == other.SetColorKey.map(|f| f as usize)
            && self.SetMode.map(|f| f as usize) == other.SetMode.map(|f| f as usize)
            && self.WaitForVerticalBlank.map(|f| f as usize) == other.WaitForVerticalBlank.map(|f| f as usize)
            && self.CanCreateSurface.map(|f| f as usize) == other.CanCreateSurface.map(|f| f as usize)
            && self.CreatePalette.map(|f| f as usize) == other.CreatePalette.map(|f| f as usize)
            && self.GetScanLine.map(|f| f as usize) == other.GetScanLine.map(|f| f as usize)
            && self.SetExclusiveMode.map(|f| f as usize) == other.SetExclusiveMode.map(|f| f as usize)
            && self.FlipToGDISurface.map(|f| f as usize) == other.FlipToGDISurface.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDCOLORCONTROLCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ColorControl: ::std::option::Option<LPDDHALCOLORCB_COLORCONTROL>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDCOLORCONTROLCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDCOLORCONTROLCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDCOLORCONTROLCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDCOLORCONTROLCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDCOLORCONTROLCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.ColorControl.map(|f| f as usize) == other.ColorControl.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDCOLORCONTROLCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDCOLORCONTROLCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDEXEBUFCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub CanCreateExecuteBuffer: ::std::option::Option<LPDDHALEXEBUFCB_CANCREATEEXEBUF>,
    pub CreateExecuteBuffer: ::std::option::Option<LPDDHALEXEBUFCB_CREATEEXEBUF>,
    pub DestroyExecuteBuffer: ::std::option::Option<LPDDHALEXEBUFCB_DESTROYEXEBUF>,
    pub LockExecuteBuffer: ::std::option::Option<LPDDHALEXEBUFCB_LOCKEXEBUF>,
    pub UnlockExecuteBuffer: ::std::option::Option<LPDDHALEXEBUFCB_UNLOCKEXEBUF>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDEXEBUFCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDEXEBUFCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDEXEBUFCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDEXEBUFCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDEXEBUFCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.CanCreateExecuteBuffer.map(|f| f as usize) == other.CanCreateExecuteBuffer.map(|f| f as usize)
            && self.CreateExecuteBuffer.map(|f| f as usize) == other.CreateExecuteBuffer.map(|f| f as usize)
            && self.DestroyExecuteBuffer.map(|f| f as usize) == other.DestroyExecuteBuffer.map(|f| f as usize)
            && self.LockExecuteBuffer.map(|f| f as usize) == other.LockExecuteBuffer.map(|f| f as usize)
            && self.UnlockExecuteBuffer.map(|f| f as usize) == other.UnlockExecuteBuffer.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDEXEBUFCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDEXEBUFCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDKERNELCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub SyncSurfaceData: ::std::option::Option<LPDDHALKERNELCB_SYNCSURFACE>,
    pub SyncVideoPortData: ::std::option::Option<LPDDHALKERNELCB_SYNCVIDEOPORT>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDKERNELCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDKERNELCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDKERNELCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDKERNELCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDKERNELCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.SyncSurfaceData.map(|f| f as usize) == other.SyncSurfaceData.map(|f| f as usize) && self.SyncVideoPortData.map(|f| f as usize) == other.SyncVideoPortData.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDKERNELCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDKERNELCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDMISCELLANEOUS2CALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub Reserved: *mut ::std::ffi::c_void,
    pub CreateSurfaceEx: ::std::option::Option<LPDDHAL_CREATESURFACEEX>,
    pub GetDriverState: ::std::option::Option<LPDDHAL_GETDRIVERSTATE>,
    pub DestroyDDLocal: ::std::option::Option<LPDDHAL_DESTROYDDLOCAL>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDMISCELLANEOUS2CALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDMISCELLANEOUS2CALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.Reserved == other.Reserved && self.CreateSurfaceEx.map(|f| f as usize) == other.CreateSurfaceEx.map(|f| f as usize) && self.GetDriverState.map(|f| f as usize) == other.GetDriverState.map(|f| f as usize) && self.DestroyDDLocal.map(|f| f as usize) == other.DestroyDDLocal.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDMISCELLANEOUS2CALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDMISCELLANEOUSCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub GetAvailDriverMemory: ::std::option::Option<LPDDHAL_GETAVAILDRIVERMEMORY>,
    pub UpdateNonLocalHeap: ::std::option::Option<LPDDHAL_UPDATENONLOCALHEAP>,
    pub GetHeapAlignment: ::std::option::Option<LPDDHAL_GETHEAPALIGNMENT>,
    pub GetSysmemBltStatus: ::std::option::Option<LPDDHALSURFCB_GETBLTSTATUS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDMISCELLANEOUSCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDMISCELLANEOUSCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDMISCELLANEOUSCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDMISCELLANEOUSCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDMISCELLANEOUSCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.GetAvailDriverMemory.map(|f| f as usize) == other.GetAvailDriverMemory.map(|f| f as usize) && self.UpdateNonLocalHeap.map(|f| f as usize) == other.UpdateNonLocalHeap.map(|f| f as usize) && self.GetHeapAlignment.map(|f| f as usize) == other.GetHeapAlignment.map(|f| f as usize) && self.GetSysmemBltStatus.map(|f| f as usize) == other.GetSysmemBltStatus.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDMISCELLANEOUSCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDMISCELLANEOUSCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDMOTIONCOMPCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub GetMoCompGuids: ::std::option::Option<LPDDHALMOCOMPCB_GETGUIDS>,
    pub GetMoCompFormats: ::std::option::Option<LPDDHALMOCOMPCB_GETFORMATS>,
    pub CreateMoComp: ::std::option::Option<LPDDHALMOCOMPCB_CREATE>,
    pub GetMoCompBuffInfo: ::std::option::Option<LPDDHALMOCOMPCB_GETCOMPBUFFINFO>,
    pub GetInternalMoCompInfo: ::std::option::Option<LPDDHALMOCOMPCB_GETINTERNALINFO>,
    pub BeginMoCompFrame: ::std::option::Option<LPDDHALMOCOMPCB_BEGINFRAME>,
    pub EndMoCompFrame: ::std::option::Option<LPDDHALMOCOMPCB_ENDFRAME>,
    pub RenderMoComp: ::std::option::Option<LPDDHALMOCOMPCB_RENDER>,
    pub QueryMoCompStatus: ::std::option::Option<LPDDHALMOCOMPCB_QUERYSTATUS>,
    pub DestroyMoComp: ::std::option::Option<LPDDHALMOCOMPCB_DESTROY>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDMOTIONCOMPCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDMOTIONCOMPCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDMOTIONCOMPCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDMOTIONCOMPCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDMOTIONCOMPCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.GetMoCompGuids.map(|f| f as usize) == other.GetMoCompGuids.map(|f| f as usize)
            && self.GetMoCompFormats.map(|f| f as usize) == other.GetMoCompFormats.map(|f| f as usize)
            && self.CreateMoComp.map(|f| f as usize) == other.CreateMoComp.map(|f| f as usize)
            && self.GetMoCompBuffInfo.map(|f| f as usize) == other.GetMoCompBuffInfo.map(|f| f as usize)
            && self.GetInternalMoCompInfo.map(|f| f as usize) == other.GetInternalMoCompInfo.map(|f| f as usize)
            && self.BeginMoCompFrame.map(|f| f as usize) == other.BeginMoCompFrame.map(|f| f as usize)
            && self.EndMoCompFrame.map(|f| f as usize) == other.EndMoCompFrame.map(|f| f as usize)
            && self.RenderMoComp.map(|f| f as usize) == other.RenderMoComp.map(|f| f as usize)
            && self.QueryMoCompStatus.map(|f| f as usize) == other.QueryMoCompStatus.map(|f| f as usize)
            && self.DestroyMoComp.map(|f| f as usize) == other.DestroyMoComp.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDMOTIONCOMPCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDMOTIONCOMPCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDPALETTECALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroyPalette: ::std::option::Option<LPDDHALPALCB_DESTROYPALETTE>,
    pub SetEntries: ::std::option::Option<LPDDHALPALCB_SETENTRIES>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDPALETTECALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDPALETTECALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDPALETTECALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDPALETTECALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDPALETTECALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.DestroyPalette.map(|f| f as usize) == other.DestroyPalette.map(|f| f as usize) && self.SetEntries.map(|f| f as usize) == other.SetEntries.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDPALETTECALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDPALETTECALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDSURFACECALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroySurface: ::std::option::Option<LPDDHALSURFCB_DESTROYSURFACE>,
    pub Flip: ::std::option::Option<LPDDHALSURFCB_FLIP>,
    pub SetClipList: ::std::option::Option<LPDDHALSURFCB_SETCLIPLIST>,
    pub Lock: ::std::option::Option<LPDDHALSURFCB_LOCK>,
    pub Unlock: ::std::option::Option<LPDDHALSURFCB_UNLOCK>,
    pub Blt: ::std::option::Option<LPDDHALSURFCB_BLT>,
    pub SetColorKey: ::std::option::Option<LPDDHALSURFCB_SETCOLORKEY>,
    pub AddAttachedSurface: ::std::option::Option<LPDDHALSURFCB_ADDATTACHEDSURFACE>,
    pub GetBltStatus: ::std::option::Option<LPDDHALSURFCB_GETBLTSTATUS>,
    pub GetFlipStatus: ::std::option::Option<LPDDHALSURFCB_GETFLIPSTATUS>,
    pub UpdateOverlay: ::std::option::Option<LPDDHALSURFCB_UPDATEOVERLAY>,
    pub SetOverlayPosition: ::std::option::Option<LPDDHALSURFCB_SETOVERLAYPOSITION>,
    pub reserved4: *mut ::std::ffi::c_void,
    pub SetPalette: ::std::option::Option<LPDDHALSURFCB_SETPALETTE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDSURFACECALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDSURFACECALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDSURFACECALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDSURFACECALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("reserved4", &self.reserved4).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDSURFACECALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.DestroySurface.map(|f| f as usize) == other.DestroySurface.map(|f| f as usize)
            && self.Flip.map(|f| f as usize) == other.Flip.map(|f| f as usize)
            && self.SetClipList.map(|f| f as usize) == other.SetClipList.map(|f| f as usize)
            && self.Lock.map(|f| f as usize) == other.Lock.map(|f| f as usize)
            && self.Unlock.map(|f| f as usize) == other.Unlock.map(|f| f as usize)
            && self.Blt.map(|f| f as usize) == other.Blt.map(|f| f as usize)
            && self.SetColorKey.map(|f| f as usize) == other.SetColorKey.map(|f| f as usize)
            && self.AddAttachedSurface.map(|f| f as usize) == other.AddAttachedSurface.map(|f| f as usize)
            && self.GetBltStatus.map(|f| f as usize) == other.GetBltStatus.map(|f| f as usize)
            && self.GetFlipStatus.map(|f| f as usize) == other.GetFlipStatus.map(|f| f as usize)
            && self.UpdateOverlay.map(|f| f as usize) == other.UpdateOverlay.map(|f| f as usize)
            && self.SetOverlayPosition.map(|f| f as usize) == other.SetOverlayPosition.map(|f| f as usize)
            && self.reserved4 == other.reserved4
            && self.SetPalette.map(|f| f as usize) == other.SetPalette.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDSURFACECALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDSURFACECALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DDVIDEOPORTCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub CanCreateVideoPort: ::std::option::Option<LPDDHALVPORTCB_CANCREATEVIDEOPORT>,
    pub CreateVideoPort: ::std::option::Option<LPDDHALVPORTCB_CREATEVIDEOPORT>,
    pub FlipVideoPort: ::std::option::Option<LPDDHALVPORTCB_FLIP>,
    pub GetVideoPortBandwidth: ::std::option::Option<LPDDHALVPORTCB_GETBANDWIDTH>,
    pub GetVideoPortInputFormats: ::std::option::Option<LPDDHALVPORTCB_GETINPUTFORMATS>,
    pub GetVideoPortOutputFormats: ::std::option::Option<LPDDHALVPORTCB_GETOUTPUTFORMATS>,
    pub lpReserved1: *mut ::std::ffi::c_void,
    pub GetVideoPortField: ::std::option::Option<LPDDHALVPORTCB_GETFIELD>,
    pub GetVideoPortLine: ::std::option::Option<LPDDHALVPORTCB_GETLINE>,
    pub GetVideoPortConnectInfo: ::std::option::Option<LPDDHALVPORTCB_GETVPORTCONNECT>,
    pub DestroyVideoPort: ::std::option::Option<LPDDHALVPORTCB_DESTROYVPORT>,
    pub GetVideoPortFlipStatus: ::std::option::Option<LPDDHALVPORTCB_GETFLIPSTATUS>,
    pub UpdateVideoPort: ::std::option::Option<LPDDHALVPORTCB_UPDATE>,
    pub WaitForVideoPortSync: ::std::option::Option<LPDDHALVPORTCB_WAITFORSYNC>,
    pub GetVideoSignalStatus: ::std::option::Option<LPDDHALVPORTCB_GETSIGNALSTATUS>,
    pub ColorControl: ::std::option::Option<LPDDHALVPORTCB_COLORCONTROL>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DDVIDEOPORTCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DDVIDEOPORTCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DDVIDEOPORTCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DDVIDEOPORTCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("lpReserved1", &self.lpReserved1).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DDVIDEOPORTCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.CanCreateVideoPort.map(|f| f as usize) == other.CanCreateVideoPort.map(|f| f as usize)
            && self.CreateVideoPort.map(|f| f as usize) == other.CreateVideoPort.map(|f| f as usize)
            && self.FlipVideoPort.map(|f| f as usize) == other.FlipVideoPort.map(|f| f as usize)
            && self.GetVideoPortBandwidth.map(|f| f as usize) == other.GetVideoPortBandwidth.map(|f| f as usize)
            && self.GetVideoPortInputFormats.map(|f| f as usize) == other.GetVideoPortInputFormats.map(|f| f as usize)
            && self.GetVideoPortOutputFormats.map(|f| f as usize) == other.GetVideoPortOutputFormats.map(|f| f as usize)
            && self.lpReserved1 == other.lpReserved1
            && self.GetVideoPortField.map(|f| f as usize) == other.GetVideoPortField.map(|f| f as usize)
            && self.GetVideoPortLine.map(|f| f as usize) == other.GetVideoPortLine.map(|f| f as usize)
            && self.GetVideoPortConnectInfo.map(|f| f as usize) == other.GetVideoPortConnectInfo.map(|f| f as usize)
            && self.DestroyVideoPort.map(|f| f as usize) == other.DestroyVideoPort.map(|f| f as usize)
            && self.GetVideoPortFlipStatus.map(|f| f as usize) == other.GetVideoPortFlipStatus.map(|f| f as usize)
            && self.UpdateVideoPort.map(|f| f as usize) == other.UpdateVideoPort.map(|f| f as usize)
            && self.WaitForVideoPortSync.map(|f| f as usize) == other.WaitForVideoPortSync.map(|f| f as usize)
            && self.GetVideoSignalStatus.map(|f| f as usize) == other.GetVideoSignalStatus.map(|f| f as usize)
            && self.ColorControl.map(|f| f as usize) == other.ColorControl.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DDVIDEOPORTCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DDVIDEOPORTCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DESTROYDRIVERDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub DestroyDriver: ::std::option::Option<LPDDHAL_DESTROYDRIVER>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DESTROYDRIVERDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DESTROYDRIVERDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DESTROYDRIVERDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DESTROYDRIVERDATA").field("lpDD", &self.lpDD).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DESTROYDRIVERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.ddRVal == other.ddRVal && self.DestroyDriver.map(|f| f as usize) == other.DestroyDriver.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DESTROYDRIVERDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DESTROYDRIVERDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DESTROYMOCOMPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub DestroyMoComp: ::std::option::Option<LPDDHALMOCOMPCB_DESTROY>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DESTROYMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DESTROYMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DESTROYMOCOMPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DESTROYMOCOMPDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DESTROYMOCOMPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.ddRVal == other.ddRVal && self.DestroyMoComp.map(|f| f as usize) == other.DestroyMoComp.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DESTROYMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DESTROYMOCOMPDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DESTROYPALETTEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub DestroyPalette: ::std::option::Option<LPDDHALPALCB_DESTROYPALETTE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DESTROYPALETTEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DESTROYPALETTEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DESTROYPALETTEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DESTROYPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DESTROYPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.ddRVal == other.ddRVal && self.DestroyPalette.map(|f| f as usize) == other.DestroyPalette.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DESTROYPALETTEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DESTROYPALETTEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DESTROYSURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub DestroySurface: ::std::option::Option<LPDDHALSURFCB_DESTROYSURFACE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DESTROYSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DESTROYSURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DESTROYSURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DESTROYSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DESTROYSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.DestroySurface.map(|f| f as usize) == other.DestroySurface.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DESTROYSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DESTROYSURFACEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DESTROYVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub DestroyVideoPort: ::std::option::Option<LPDDHALVPORTCB_DESTROYVPORT>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DESTROYVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DESTROYVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DESTROYVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DESTROYVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DESTROYVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.ddRVal == other.ddRVal && self.DestroyVideoPort.map(|f| f as usize) == other.DestroyVideoPort.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DESTROYVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DESTROYVPORTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_DRIVER_HANDLED: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_DRIVER_NOCKEYHW: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_DRIVER_NOTHANDLED: i32 = 0i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_DRVSETCOLORKEYDATA {
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ckNew: DDCOLORKEY,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetColorKey: ::std::option::Option<LPDDHAL_SETCOLORKEY>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_DRVSETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_DRVSETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_DRVSETCOLORKEYDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DRVSETCOLORKEYDATA").field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_DRVSETCOLORKEYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ckNew == other.ckNew && self.ddRVal == other.ddRVal && self.SetColorKey.map(|f| f as usize) == other.SetColorKey.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_DRVSETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DRVSETCOLORKEYDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_ENDMOCOMPFRAMEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpInputData: *mut ::std::ffi::c_void,
    pub dwInputDataSize: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub EndMoCompFrame: ::std::option::Option<LPDDHALMOCOMPCB_ENDFRAME>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_ENDMOCOMPFRAMEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_ENDMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_ENDMOCOMPFRAMEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_ENDMOCOMPFRAMEDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpInputData", &self.lpInputData).field("dwInputDataSize", &self.dwInputDataSize).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_ENDMOCOMPFRAMEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpInputData == other.lpInputData && self.dwInputDataSize == other.dwInputDataSize && self.ddRVal == other.ddRVal && self.EndMoCompFrame.map(|f| f as usize) == other.EndMoCompFrame.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_ENDMOCOMPFRAMEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_ENDMOCOMPFRAMEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_EXEBUFCB32_CANCREATEEXEBUF: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_EXEBUFCB32_CREATEEXEBUF: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_EXEBUFCB32_DESTROYEXEBUF: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_EXEBUFCB32_LOCKEXEBUF: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_EXEBUFCB32_UNLOCKEXEBUF: i32 = 16i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_FLIPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpSurfCurr: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpSurfTarg: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub Flip: ::std::option::Option<LPDDHALSURFCB_FLIP>,
    pub lpSurfCurrLeft: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpSurfTargLeft: *mut DDRAWI_DDRAWSURFACE_LCL,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_FLIPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_FLIPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_FLIPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_FLIPDATA").field("lpDD", &self.lpDD).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("lpSurfCurrLeft", &self.lpSurfCurrLeft).field("lpSurfTargLeft", &self.lpSurfTargLeft).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_FLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpSurfCurr == other.lpSurfCurr && self.lpSurfTarg == other.lpSurfTarg && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.Flip.map(|f| f as usize) == other.Flip.map(|f| f as usize) && self.lpSurfCurrLeft == other.lpSurfCurrLeft && self.lpSurfTargLeft == other.lpSurfTargLeft
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_FLIPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_FLIPDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_FLIPTOGDISURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwToGDI: u32,
    pub dwReserved: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub FlipToGDISurface: ::std::option::Option<LPDDHAL_FLIPTOGDISURFACE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_FLIPTOGDISURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_FLIPTOGDISURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_FLIPTOGDISURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_FLIPTOGDISURFACEDATA").field("lpDD", &self.lpDD).field("dwToGDI", &self.dwToGDI).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_FLIPTOGDISURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwToGDI == other.dwToGDI && self.dwReserved == other.dwReserved && self.ddRVal == other.ddRVal && self.FlipToGDISurface.map(|f| f as usize) == other.FlipToGDISurface.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_FLIPTOGDISURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_FLIPTOGDISURFACEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_FLIPVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lpSurfCurr: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpSurfTarg: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub FlipVideoPort: ::std::option::Option<LPDDHALVPORTCB_FLIP>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_FLIPVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_FLIPVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_FLIPVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_FLIPVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_FLIPVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lpSurfCurr == other.lpSurfCurr && self.lpSurfTarg == other.lpSurfTarg && self.ddRVal == other.ddRVal && self.FlipVideoPort.map(|f| f as usize) == other.FlipVideoPort.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_FLIPVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_FLIPVPORTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::clone::Clone for DDHAL_GETAVAILDRIVERMEMORYDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETAVAILDRIVERMEMORYDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub DDSCaps: DDSCAPS,
    pub dwTotal: u32,
    pub dwFree: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetAvailDriverMemory: ::std::option::Option<LPDDHAL_GETAVAILDRIVERMEMORY>,
    pub ddsCapsEx: DDSCAPSEX,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETAVAILDRIVERMEMORYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETAVAILDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETAVAILDRIVERMEMORYDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETAVAILDRIVERMEMORYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETAVAILDRIVERMEMORYDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETBLTSTATUSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetBltStatus: ::std::option::Option<LPDDHALSURFCB_GETBLTSTATUS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETBLTSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETBLTSTATUSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETBLTSTATUSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETBLTSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETBLTSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.GetBltStatus.map(|f| f as usize) == other.GetBltStatus.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETBLTSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETBLTSTATUSDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDHAL_GETDRIVERINFODATA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidInfo: ::windows::runtime::GUID,
    pub dwExpectedSize: u32,
    pub lpvData: *mut ::std::ffi::c_void,
    pub dwActualSize: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub dwContext: usize,
}
impl DDHAL_GETDRIVERINFODATA {}
impl ::std::default::Default for DDHAL_GETDRIVERINFODATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDHAL_GETDRIVERINFODATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETDRIVERINFODATA")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("guidInfo", &self.guidInfo)
            .field("dwExpectedSize", &self.dwExpectedSize)
            .field("lpvData", &self.lpvData)
            .field("dwActualSize", &self.dwActualSize)
            .field("ddRVal", &self.ddRVal)
            .field("dwContext", &self.dwContext)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DDHAL_GETDRIVERINFODATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidInfo == other.guidInfo && self.dwExpectedSize == other.dwExpectedSize && self.lpvData == other.lpvData && self.dwActualSize == other.dwActualSize && self.ddRVal == other.ddRVal && self.dwContext == other.dwContext
    }
}
impl ::std::cmp::Eq for DDHAL_GETDRIVERINFODATA {}
unsafe impl ::windows::runtime::Abi for DDHAL_GETDRIVERINFODATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDHAL_GETDRIVERSTATEDATA {
    pub dwFlags: u32,
    pub Anonymous: DDHAL_GETDRIVERSTATEDATA_0,
    pub lpdwStates: *mut u32,
    pub dwLength: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
impl DDHAL_GETDRIVERSTATEDATA {}
impl ::std::default::Default for DDHAL_GETDRIVERSTATEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDHAL_GETDRIVERSTATEDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDHAL_GETDRIVERSTATEDATA {}
unsafe impl ::windows::runtime::Abi for DDHAL_GETDRIVERSTATEDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDHAL_GETDRIVERSTATEDATA_0 {
    pub dwhContext: usize,
}
impl DDHAL_GETDRIVERSTATEDATA_0 {}
impl ::std::default::Default for DDHAL_GETDRIVERSTATEDATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDHAL_GETDRIVERSTATEDATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDHAL_GETDRIVERSTATEDATA_0 {}
unsafe impl ::windows::runtime::Abi for DDHAL_GETDRIVERSTATEDATA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETFLIPSTATUSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetFlipStatus: ::std::option::Option<LPDDHALSURFCB_GETFLIPSTATUS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETFLIPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETFLIPSTATUSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETFLIPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.GetFlipStatus.map(|f| f as usize) == other.GetFlipStatus.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETFLIPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETFLIPSTATUSDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Devices_Display")]
impl ::std::clone::Clone for DDHAL_GETHEAPALIGNMENTDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Devices_Display")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`*"]
pub struct DDHAL_GETHEAPALIGNMENTDATA {
    pub dwInstance: usize,
    pub dwHeap: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetHeapAlignment: ::std::option::Option<LPDDHAL_GETHEAPALIGNMENT>,
    pub Alignment: super::super::Devices::Display::HEAPALIGNMENT,
}
#[cfg(feature = "Win32_Devices_Display")]
impl DDHAL_GETHEAPALIGNMENTDATA {}
#[cfg(feature = "Win32_Devices_Display")]
impl ::std::default::Default for DDHAL_GETHEAPALIGNMENTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_Display")]
impl ::std::cmp::PartialEq for DDHAL_GETHEAPALIGNMENTDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Devices_Display")]
impl ::std::cmp::Eq for DDHAL_GETHEAPALIGNMENTDATA {}
#[cfg(feature = "Win32_Devices_Display")]
unsafe impl ::windows::runtime::Abi for DDHAL_GETHEAPALIGNMENTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::clone::Clone for DDHAL_GETINTERNALMOCOMPDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETINTERNALMOCOMPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub ddPixelFormat: DDPIXELFORMAT,
    pub dwScratchMemAlloc: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetInternalMoCompInfo: ::std::option::Option<LPDDHALMOCOMPCB_GETINTERNALINFO>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETINTERNALMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETINTERNALMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETINTERNALMOCOMPDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETINTERNALMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETINTERNALMOCOMPDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::clone::Clone for DDHAL_GETMOCOMPCOMPBUFFDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETMOCOMPCOMPBUFFDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub ddPixelFormat: DDPIXELFORMAT,
    pub dwNumTypesCompBuffs: u32,
    pub lpCompBuffInfo: *mut DDMCCOMPBUFFERINFO,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetMoCompBuffInfo: ::std::option::Option<LPDDHALMOCOMPCB_GETCOMPBUFFINFO>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETMOCOMPCOMPBUFFDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETMOCOMPCOMPBUFFDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETMOCOMPCOMPBUFFDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETMOCOMPCOMPBUFFDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETMOCOMPCOMPBUFFDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETMOCOMPFORMATSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub dwNumFormats: u32,
    pub lpFormats: *mut DDPIXELFORMAT,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetMoCompFormats: ::std::option::Option<LPDDHALMOCOMPCB_GETFORMATS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETMOCOMPFORMATSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETMOCOMPFORMATSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETMOCOMPFORMATSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETMOCOMPFORMATSDATA").field("lpDD", &self.lpDD).field("lpGuid", &self.lpGuid).field("dwNumFormats", &self.dwNumFormats).field("lpFormats", &self.lpFormats).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETMOCOMPFORMATSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpGuid == other.lpGuid && self.dwNumFormats == other.dwNumFormats && self.lpFormats == other.lpFormats && self.ddRVal == other.ddRVal && self.GetMoCompFormats.map(|f| f as usize) == other.GetMoCompFormats.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETMOCOMPFORMATSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETMOCOMPFORMATSDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETMOCOMPGUIDSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwNumGuids: u32,
    pub lpGuids: *mut ::windows::runtime::GUID,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetMoCompGuids: ::std::option::Option<LPDDHALMOCOMPCB_GETGUIDS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETMOCOMPGUIDSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETMOCOMPGUIDSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETMOCOMPGUIDSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETMOCOMPGUIDSDATA").field("lpDD", &self.lpDD).field("dwNumGuids", &self.dwNumGuids).field("lpGuids", &self.lpGuids).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETMOCOMPGUIDSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwNumGuids == other.dwNumGuids && self.lpGuids == other.lpGuids && self.ddRVal == other.ddRVal && self.GetMoCompGuids.map(|f| f as usize) == other.GetMoCompGuids.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETMOCOMPGUIDSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETMOCOMPGUIDSDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETSCANLINEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwScanLine: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetScanLine: ::std::option::Option<LPDDHAL_GETSCANLINE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETSCANLINEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETSCANLINEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETSCANLINEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETSCANLINEDATA").field("lpDD", &self.lpDD).field("dwScanLine", &self.dwScanLine).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETSCANLINEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwScanLine == other.dwScanLine && self.ddRVal == other.ddRVal && self.GetScanLine.map(|f| f as usize) == other.GetScanLine.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETSCANLINEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETSCANLINEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETVPORTBANDWIDTHDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lpddpfFormat: *mut DDPIXELFORMAT,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwFlags: u32,
    pub lpBandwidth: *mut super::super::UI::DisplayDevices::DDVIDEOPORTBANDWIDTH,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortBandwidth: ::std::option::Option<LPDDHALVPORTCB_GETBANDWIDTH>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETVPORTBANDWIDTHDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETVPORTBANDWIDTHDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETVPORTBANDWIDTHDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETVPORTBANDWIDTHDATA")
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("lpddpfFormat", &self.lpddpfFormat)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwFlags", &self.dwFlags)
            .field("lpBandwidth", &self.lpBandwidth)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETVPORTBANDWIDTHDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lpddpfFormat == other.lpddpfFormat && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwFlags == other.dwFlags && self.lpBandwidth == other.lpBandwidth && self.ddRVal == other.ddRVal && self.GetVideoPortBandwidth.map(|f| f as usize) == other.GetVideoPortBandwidth.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETVPORTBANDWIDTHDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETVPORTBANDWIDTHDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETVPORTCONNECTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwPortId: u32,
    pub lpConnect: *mut DDVIDEOPORTCONNECT,
    pub dwNumEntries: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortConnectInfo: ::std::option::Option<LPDDHALVPORTCB_GETVPORTCONNECT>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETVPORTCONNECTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETVPORTCONNECTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETVPORTCONNECTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETVPORTCONNECTDATA").field("lpDD", &self.lpDD).field("dwPortId", &self.dwPortId).field("lpConnect", &self.lpConnect).field("dwNumEntries", &self.dwNumEntries).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETVPORTCONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwPortId == other.dwPortId && self.lpConnect == other.lpConnect && self.dwNumEntries == other.dwNumEntries && self.ddRVal == other.ddRVal && self.GetVideoPortConnectInfo.map(|f| f as usize) == other.GetVideoPortConnectInfo.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETVPORTCONNECTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETVPORTCONNECTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETVPORTFIELDDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub bField: super::super::Foundation::BOOL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortField: ::std::option::Option<LPDDHALVPORTCB_GETFIELD>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETVPORTFIELDDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETVPORTFIELDDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETVPORTFIELDDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETVPORTFIELDDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("bField", &self.bField).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETVPORTFIELDDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.bField == other.bField && self.ddRVal == other.ddRVal && self.GetVideoPortField.map(|f| f as usize) == other.GetVideoPortField.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETVPORTFIELDDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETVPORTFIELDDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETVPORTFLIPSTATUSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub fpSurface: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortFlipStatus: ::std::option::Option<LPDDHALVPORTCB_GETFLIPSTATUS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETVPORTFLIPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETVPORTFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETVPORTFLIPSTATUSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETVPORTFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("fpSurface", &self.fpSurface).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETVPORTFLIPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.fpSurface == other.fpSurface && self.ddRVal == other.ddRVal && self.GetVideoPortFlipStatus.map(|f| f as usize) == other.GetVideoPortFlipStatus.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETVPORTFLIPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETVPORTFLIPSTATUSDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETVPORTINPUTFORMATDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwFlags: u32,
    pub lpddpfFormat: *mut DDPIXELFORMAT,
    pub dwNumFormats: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortInputFormats: ::std::option::Option<LPDDHALVPORTCB_GETINPUTFORMATS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETVPORTINPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETVPORTINPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETVPORTINPUTFORMATDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETVPORTINPUTFORMATDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpddpfFormat", &self.lpddpfFormat).field("dwNumFormats", &self.dwNumFormats).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETVPORTINPUTFORMATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpddpfFormat == other.lpddpfFormat && self.dwNumFormats == other.dwNumFormats && self.ddRVal == other.ddRVal && self.GetVideoPortInputFormats.map(|f| f as usize) == other.GetVideoPortInputFormats.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETVPORTINPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETVPORTINPUTFORMATDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETVPORTLINEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwLine: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortLine: ::std::option::Option<LPDDHALVPORTCB_GETLINE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETVPORTLINEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETVPORTLINEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETVPORTLINEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETVPORTLINEDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwLine", &self.dwLine).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETVPORTLINEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwLine == other.dwLine && self.ddRVal == other.ddRVal && self.GetVideoPortLine.map(|f| f as usize) == other.GetVideoPortLine.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETVPORTLINEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETVPORTLINEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETVPORTOUTPUTFORMATDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwFlags: u32,
    pub lpddpfInputFormat: *mut DDPIXELFORMAT,
    pub lpddpfOutputFormats: *mut DDPIXELFORMAT,
    pub dwNumFormats: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortOutputFormats: ::std::option::Option<LPDDHALVPORTCB_GETOUTPUTFORMATS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETVPORTOUTPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETVPORTOUTPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETVPORTOUTPUTFORMATDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETVPORTOUTPUTFORMATDATA")
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("dwFlags", &self.dwFlags)
            .field("lpddpfInputFormat", &self.lpddpfInputFormat)
            .field("lpddpfOutputFormats", &self.lpddpfOutputFormats)
            .field("dwNumFormats", &self.dwNumFormats)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETVPORTOUTPUTFORMATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpddpfInputFormat == other.lpddpfInputFormat && self.lpddpfOutputFormats == other.lpddpfOutputFormats && self.dwNumFormats == other.dwNumFormats && self.ddRVal == other.ddRVal && self.GetVideoPortOutputFormats.map(|f| f as usize) == other.GetVideoPortOutputFormats.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETVPORTOUTPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETVPORTOUTPUTFORMATDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_GETVPORTSIGNALDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwStatus: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoSignalStatus: ::std::option::Option<LPDDHALVPORTCB_GETSIGNALSTATUS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_GETVPORTSIGNALDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_GETVPORTSIGNALDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_GETVPORTSIGNALDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_GETVPORTSIGNALDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwStatus", &self.dwStatus).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_GETVPORTSIGNALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwStatus == other.dwStatus && self.ddRVal == other.ddRVal && self.GetVideoSignalStatus.map(|f| f as usize) == other.GetVideoSignalStatus.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_GETVPORTSIGNALDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_GETVPORTSIGNALDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_KERNEL_SYNCSURFACEDATA: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_KERNEL_SYNCVIDEOPORTDATA: i32 = 2i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_LOCKDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub bHasRect: u32,
    pub rArea: super::super::Foundation::RECTL,
    pub lpSurfData: *mut ::std::ffi::c_void,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub Lock: ::std::option::Option<LPDDHALSURFCB_LOCK>,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_LOCKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_LOCKDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_LOCKDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_LOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("bHasRect", &self.bHasRect).field("rArea", &self.rArea).field("lpSurfData", &self.lpSurfData).field("ddRVal", &self.ddRVal).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_LOCKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.bHasRect == other.bHasRect && self.rArea == other.rArea && self.lpSurfData == other.lpSurfData && self.ddRVal == other.ddRVal && self.Lock.map(|f| f as usize) == other.Lock.map(|f| f as usize) && self.dwFlags == other.dwFlags
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_LOCKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_LOCKDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MISC2CB32_ALPHABLT: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MISC2CB32_CREATESURFACEEX: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MISC2CB32_DESTROYDDLOCAL: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MISC2CB32_GETDRIVERSTATE: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MISCCB32_GETAVAILDRIVERMEMORY: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MISCCB32_GETHEAPALIGNMENT: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MISCCB32_GETSYSMEMBLTSTATUS: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MISCCB32_UPDATENONLOCALHEAP: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_BEGINFRAME: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_CREATE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_DESTROY: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_ENDFRAME: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_GETCOMPBUFFINFO: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_GETFORMATS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_GETGUIDS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_GETINTERNALINFO: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_QUERYSTATUS: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_MOCOMP32_RENDER: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_NTCB32_FLIPTOGDISURFACE: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_NTCB32_FREEDRIVERMEMORY: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_NTCB32_SETEXCLUSIVEMODE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_PALCB32_DESTROYPALETTE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_PALCB32_SETENTRIES: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_PLEASEALLOC_BLOCKSIZE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_PLEASEALLOC_LINEARSIZE: i32 = 3i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_PLEASEALLOC_USERMEM: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_PRIVATECAP_ATOMICSURFACECREATION: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_PRIVATECAP_NOTIFYPRIMARYCREATION: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_PRIVATECAP_RESERVED1: i32 = 4i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_QUERYMOCOMPSTATUSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub QueryMoCompStatus: ::std::option::Option<LPDDHALMOCOMPCB_QUERYSTATUS>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_QUERYMOCOMPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_QUERYMOCOMPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_QUERYMOCOMPSTATUSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_QUERYMOCOMPSTATUSDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpSurface", &self.lpSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_QUERYMOCOMPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpSurface == other.lpSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.QueryMoCompStatus.map(|f| f as usize) == other.QueryMoCompStatus.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_QUERYMOCOMPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_QUERYMOCOMPSTATUSDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_RENDERMOCOMPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub dwNumBuffers: u32,
    pub lpBufferInfo: *mut DDMCBUFFERINFO,
    pub dwFunction: u32,
    pub lpInputData: *mut ::std::ffi::c_void,
    pub dwInputDataSize: u32,
    pub lpOutputData: *mut ::std::ffi::c_void,
    pub dwOutputDataSize: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub RenderMoComp: ::std::option::Option<LPDDHALMOCOMPCB_RENDER>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_RENDERMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_RENDERMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_RENDERMOCOMPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_RENDERMOCOMPDATA")
            .field("lpDD", &self.lpDD)
            .field("lpMoComp", &self.lpMoComp)
            .field("dwNumBuffers", &self.dwNumBuffers)
            .field("lpBufferInfo", &self.lpBufferInfo)
            .field("dwFunction", &self.dwFunction)
            .field("lpInputData", &self.lpInputData)
            .field("dwInputDataSize", &self.dwInputDataSize)
            .field("lpOutputData", &self.lpOutputData)
            .field("dwOutputDataSize", &self.dwOutputDataSize)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_RENDERMOCOMPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.dwNumBuffers == other.dwNumBuffers && self.lpBufferInfo == other.lpBufferInfo && self.dwFunction == other.dwFunction && self.lpInputData == other.lpInputData && self.dwInputDataSize == other.dwInputDataSize && self.lpOutputData == other.lpOutputData && self.dwOutputDataSize == other.dwOutputDataSize && self.ddRVal == other.ddRVal && self.RenderMoComp.map(|f| f as usize) == other.RenderMoComp.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_RENDERMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_RENDERMOCOMPDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SETCLIPLISTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetClipList: ::std::option::Option<LPDDHALSURFCB_SETCLIPLIST>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SETCLIPLISTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SETCLIPLISTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SETCLIPLISTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SETCLIPLISTDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SETCLIPLISTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.SetClipList.map(|f| f as usize) == other.SetClipList.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SETCLIPLISTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SETCLIPLISTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SETCOLORKEYDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ckNew: DDCOLORKEY,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetColorKey: ::std::option::Option<LPDDHALSURFCB_SETCOLORKEY>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SETCOLORKEYDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SETCOLORKEYDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SETCOLORKEYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ckNew == other.ckNew && self.ddRVal == other.ddRVal && self.SetColorKey.map(|f| f as usize) == other.SetColorKey.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SETCOLORKEYDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SETENTRIESDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub dwBase: u32,
    pub dwNumEntries: u32,
    pub lpEntries: *mut super::Gdi::PALETTEENTRY,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetEntries: ::std::option::Option<LPDDHALPALCB_SETENTRIES>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SETENTRIESDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SETENTRIESDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SETENTRIESDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SETENTRIESDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("dwBase", &self.dwBase).field("dwNumEntries", &self.dwNumEntries).field("lpEntries", &self.lpEntries).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SETENTRIESDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.dwBase == other.dwBase && self.dwNumEntries == other.dwNumEntries && self.lpEntries == other.lpEntries && self.ddRVal == other.ddRVal && self.SetEntries.map(|f| f as usize) == other.SetEntries.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SETENTRIESDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SETENTRIESDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SETEXCLUSIVEMODEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwEnterExcl: u32,
    pub dwReserved: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetExclusiveMode: ::std::option::Option<LPDDHAL_SETEXCLUSIVEMODE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SETEXCLUSIVEMODEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SETEXCLUSIVEMODEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SETEXCLUSIVEMODEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SETEXCLUSIVEMODEDATA").field("lpDD", &self.lpDD).field("dwEnterExcl", &self.dwEnterExcl).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SETEXCLUSIVEMODEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwEnterExcl == other.dwEnterExcl && self.dwReserved == other.dwReserved && self.ddRVal == other.ddRVal && self.SetExclusiveMode.map(|f| f as usize) == other.SetExclusiveMode.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SETEXCLUSIVEMODEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SETEXCLUSIVEMODEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SETMODEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwModeIndex: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetMode: ::std::option::Option<LPDDHAL_SETMODE>,
    pub inexcl: super::super::Foundation::BOOL,
    pub useRefreshRate: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SETMODEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SETMODEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SETMODEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SETMODEDATA").field("lpDD", &self.lpDD).field("dwModeIndex", &self.dwModeIndex).field("ddRVal", &self.ddRVal).field("inexcl", &self.inexcl).field("useRefreshRate", &self.useRefreshRate).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SETMODEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwModeIndex == other.dwModeIndex && self.ddRVal == other.ddRVal && self.SetMode.map(|f| f as usize) == other.SetMode.map(|f| f as usize) && self.inexcl == other.inexcl && self.useRefreshRate == other.useRefreshRate
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SETMODEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SETMODEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SETOVERLAYPOSITIONDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSrcSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpDDDestSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lXPos: i32,
    pub lYPos: i32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetOverlayPosition: ::std::option::Option<LPDDHALSURFCB_SETOVERLAYPOSITION>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SETOVERLAYPOSITIONDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SETOVERLAYPOSITIONDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SETOVERLAYPOSITIONDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SETOVERLAYPOSITIONDATA").field("lpDD", &self.lpDD).field("lpDDSrcSurface", &self.lpDDSrcSurface).field("lpDDDestSurface", &self.lpDDDestSurface).field("lXPos", &self.lXPos).field("lYPos", &self.lYPos).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SETOVERLAYPOSITIONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSrcSurface == other.lpDDSrcSurface && self.lpDDDestSurface == other.lpDDDestSurface && self.lXPos == other.lXPos && self.lYPos == other.lYPos && self.ddRVal == other.ddRVal && self.SetOverlayPosition.map(|f| f as usize) == other.SetOverlayPosition.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SETOVERLAYPOSITIONDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SETOVERLAYPOSITIONDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SETPALETTEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetPalette: ::std::option::Option<LPDDHALSURFCB_SETPALETTE>,
    pub Attach: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SETPALETTEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SETPALETTEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SETPALETTEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SETPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).field("Attach", &self.Attach).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SETPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpDDPalette == other.lpDDPalette && self.ddRVal == other.ddRVal && self.SetPalette.map(|f| f as usize) == other.SetPalette.map(|f| f as usize) && self.Attach == other.Attach
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SETPALETTEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SETPALETTEDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_ADDATTACHEDSURFACE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_BLT: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_DESTROYSURFACE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_FLIP: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_GETBLTSTATUS: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_GETFLIPSTATUS: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_LOCK: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_RESERVED4: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_SETCLIPLIST: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_SETCOLORKEY: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_SETOVERLAYPOSITION: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_SETPALETTE: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_UNLOCK: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_SURFCB32_UPDATEOVERLAY: i32 = 1024i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SYNCSURFACEDATA {
    pub dwSize: u32,
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwSurfaceOffset: u32,
    pub fpLockPtr: usize,
    pub lPitch: i32,
    pub dwOverlayOffset: u32,
    pub dwOverlaySrcWidth: u32,
    pub dwOverlaySrcHeight: u32,
    pub dwOverlayDestWidth: u32,
    pub dwOverlayDestHeight: u32,
    pub dwDriverReserved1: usize,
    pub dwDriverReserved2: usize,
    pub dwDriverReserved3: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SYNCSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SYNCSURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SYNCSURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SYNCSURFACEDATA")
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
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SYNCSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.lpDD == other.lpDD
            && self.lpDDSurface == other.lpDDSurface
            && self.dwSurfaceOffset == other.dwSurfaceOffset
            && self.fpLockPtr == other.fpLockPtr
            && self.lPitch == other.lPitch
            && self.dwOverlayOffset == other.dwOverlayOffset
            && self.dwOverlaySrcWidth == other.dwOverlaySrcWidth
            && self.dwOverlaySrcHeight == other.dwOverlaySrcHeight
            && self.dwOverlayDestWidth == other.dwOverlayDestWidth
            && self.dwOverlayDestHeight == other.dwOverlayDestHeight
            && self.dwDriverReserved1 == other.dwDriverReserved1
            && self.dwDriverReserved2 == other.dwDriverReserved2
            && self.dwDriverReserved3 == other.dwDriverReserved3
            && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SYNCSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SYNCSURFACEDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_SYNCVIDEOPORTDATA {
    pub dwSize: u32,
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwOriginOffset: u32,
    pub dwHeight: u32,
    pub dwVBIHeight: u32,
    pub dwDriverReserved1: usize,
    pub dwDriverReserved2: usize,
    pub dwDriverReserved3: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_SYNCVIDEOPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_SYNCVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_SYNCVIDEOPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_SYNCVIDEOPORTDATA")
            .field("dwSize", &self.dwSize)
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("dwOriginOffset", &self.dwOriginOffset)
            .field("dwHeight", &self.dwHeight)
            .field("dwVBIHeight", &self.dwVBIHeight)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_SYNCVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_SYNCVIDEOPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_SYNCVIDEOPORTDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_UNLOCKDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub Unlock: ::std::option::Option<LPDDHALSURFCB_UNLOCK>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_UNLOCKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_UNLOCKDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_UNLOCKDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_UNLOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_UNLOCKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.Unlock.map(|f| f as usize) == other.Unlock.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_UNLOCKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_UNLOCKDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_UPDATENONLOCALHEAPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwHeap: u32,
    pub fpGARTLin: usize,
    pub fpGARTDev: usize,
    pub ulPolicyMaxBytes: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub UpdateNonLocalHeap: ::std::option::Option<LPDDHAL_UPDATENONLOCALHEAP>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_UPDATENONLOCALHEAPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_UPDATENONLOCALHEAPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_UPDATENONLOCALHEAPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_UPDATENONLOCALHEAPDATA").field("lpDD", &self.lpDD).field("dwHeap", &self.dwHeap).field("fpGARTLin", &self.fpGARTLin).field("fpGARTDev", &self.fpGARTDev).field("ulPolicyMaxBytes", &self.ulPolicyMaxBytes).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_UPDATENONLOCALHEAPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwHeap == other.dwHeap && self.fpGARTLin == other.fpGARTLin && self.fpGARTDev == other.fpGARTDev && self.ulPolicyMaxBytes == other.ulPolicyMaxBytes && self.ddRVal == other.ddRVal && self.UpdateNonLocalHeap.map(|f| f as usize) == other.UpdateNonLocalHeap.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_UPDATENONLOCALHEAPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_UPDATENONLOCALHEAPDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::clone::Clone for DDHAL_UPDATEOVERLAYDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_UPDATEOVERLAYDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDDestSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub rDest: super::super::Foundation::RECTL,
    pub lpDDSrcSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub rSrc: super::super::Foundation::RECTL,
    pub dwFlags: u32,
    pub overlayFX: DDOVERLAYFX,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub UpdateOverlay: ::std::option::Option<LPDDHALSURFCB_UPDATEOVERLAY>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_UPDATEOVERLAYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_UPDATEOVERLAYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_UPDATEOVERLAYDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_UPDATEOVERLAYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_UPDATEOVERLAYDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_UPDATEVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lplpDDSurface: *mut *mut DDRAWI_DDRAWSURFACE_INT,
    pub lplpDDVBISurface: *mut *mut DDRAWI_DDRAWSURFACE_INT,
    pub lpVideoInfo: *mut super::super::UI::DisplayDevices::DDVIDEOPORTINFO,
    pub dwFlags: u32,
    pub dwNumAutoflip: u32,
    pub dwNumVBIAutoflip: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub UpdateVideoPort: ::std::option::Option<LPDDHALVPORTCB_UPDATE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_UPDATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_UPDATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_UPDATEVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_UPDATEVPORTDATA")
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("lplpDDSurface", &self.lplpDDSurface)
            .field("lplpDDVBISurface", &self.lplpDDVBISurface)
            .field("lpVideoInfo", &self.lpVideoInfo)
            .field("dwFlags", &self.dwFlags)
            .field("dwNumAutoflip", &self.dwNumAutoflip)
            .field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_UPDATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lplpDDSurface == other.lplpDDSurface && self.lplpDDVBISurface == other.lplpDDVBISurface && self.lpVideoInfo == other.lpVideoInfo && self.dwFlags == other.dwFlags && self.dwNumAutoflip == other.dwNumAutoflip && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip && self.ddRVal == other.ddRVal && self.UpdateVideoPort.map(|f| f as usize) == other.UpdateVideoPort.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_UPDATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_UPDATEVPORTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_CANCREATEVIDEOPORT: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_COLORCONTROL: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_CREATEVIDEOPORT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_DESTROY: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_FLIP: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETAUTOFLIPSURF: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETBANDWIDTH: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETCONNECT: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETFIELD: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETFLIPSTATUS: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETINPUTFORMATS: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETLINE: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETOUTPUTFORMATS: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_GETSIGNALSTATUS: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_UPDATE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDHAL_VPORT32_WAITFORSYNC: i32 = 8192i32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_VPORTCOLORDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwFlags: u32,
    pub lpColorData: *mut DDCOLORCONTROL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub ColorControl: ::std::option::Option<LPDDHALVPORTCB_COLORCONTROL>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_VPORTCOLORDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_VPORTCOLORDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_VPORTCOLORDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_VPORTCOLORDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpColorData", &self.lpColorData).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_VPORTCOLORDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpColorData == other.lpColorData && self.ddRVal == other.ddRVal && self.ColorControl.map(|f| f as usize) == other.ColorControl.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_VPORTCOLORDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_VPORTCOLORDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDHAL_WAITFORVPORTSYNCDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwFlags: u32,
    pub dwLine: u32,
    pub dwTimeOut: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub WaitForVideoPortSync: ::std::option::Option<LPDDHALVPORTCB_WAITFORSYNC>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDHAL_WAITFORVPORTSYNCDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDHAL_WAITFORVPORTSYNCDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDHAL_WAITFORVPORTSYNCDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_WAITFORVPORTSYNCDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("dwLine", &self.dwLine).field("dwTimeOut", &self.dwTimeOut).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDHAL_WAITFORVPORTSYNCDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.dwLine == other.dwLine && self.dwTimeOut == other.dwTimeOut && self.ddRVal == other.ddRVal && self.WaitForVideoPortSync.map(|f| f as usize) == other.WaitForVideoPortSync.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDHAL_WAITFORVPORTSYNCDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDHAL_WAITFORVPORTSYNCDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_BUSMASTER: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_DISPLAY_VSYNC: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT0_LINE: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT0_VSYNC: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT1_LINE: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT1_VSYNC: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT2_LINE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT2_VSYNC: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT3_LINE: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT3_VSYNC: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT4_LINE: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT4_VSYNC: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT5_LINE: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT5_VSYNC: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT6_LINE: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT6_VSYNC: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT7_LINE: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT7_VSYNC: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT8_LINE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT8_VSYNC: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT9_LINE: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDIRQ_VPORT9_VSYNC: i32 = 65536i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDLOCKININFO {
    pub lpSurfaceData: *mut DDSURFACEDATA,
}
impl DDLOCKININFO {}
impl ::std::default::Default for DDLOCKININFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDLOCKININFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDLOCKININFO").field("lpSurfaceData", &self.lpSurfaceData).finish()
    }
}
impl ::std::cmp::PartialEq for DDLOCKININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData
    }
}
impl ::std::cmp::Eq for DDLOCKININFO {}
unsafe impl ::windows::runtime::Abi for DDLOCKININFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDLOCKOUTINFO {
    pub dwSurfacePtr: usize,
}
impl DDLOCKOUTINFO {}
impl ::std::default::Default for DDLOCKOUTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDLOCKOUTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDLOCKOUTINFO").field("dwSurfacePtr", &self.dwSurfacePtr).finish()
    }
}
impl ::std::cmp::PartialEq for DDLOCKOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfacePtr == other.dwSurfacePtr
    }
}
impl ::std::cmp::Eq for DDLOCKOUTINFO {}
unsafe impl ::windows::runtime::Abi for DDLOCKOUTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_DISCARDCONTENTS: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_DONOTWAIT: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_EVENT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_HASVOLUMETEXTUREBOXRECT: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_NODIRTYUPDATE: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_NOOVERWRITE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_NOSYSLOCK: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_OKTOSWAP: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_READONLY: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_SURFACEMEMORYPTR: i32 = 0i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_WAIT: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDLOCK_WRITEONLY: i32 = 32i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDMCBUFFERINFO {
    pub dwSize: u32,
    pub lpCompSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
    pub lpPrivate: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDMCBUFFERINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDMCBUFFERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDMCBUFFERINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDMCBUFFERINFO").field("dwSize", &self.dwSize).field("lpCompSurface", &self.lpCompSurface).field("dwDataOffset", &self.dwDataOffset).field("dwDataSize", &self.dwDataSize).field("lpPrivate", &self.lpPrivate).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDMCBUFFERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpCompSurface == other.lpCompSurface && self.dwDataOffset == other.dwDataOffset && self.dwDataSize == other.dwDataSize && self.lpPrivate == other.lpPrivate
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDMCBUFFERINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDMCBUFFERINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDMCCOMPBUFFERINFO {
    pub dwSize: u32,
    pub dwNumCompBuffers: u32,
    pub dwWidthToCreate: u32,
    pub dwHeightToCreate: u32,
    pub dwBytesToAllocate: u32,
    pub ddCompCaps: DDSCAPS2,
    pub ddPixelFormat: DDPIXELFORMAT,
}
impl DDMCCOMPBUFFERINFO {}
impl ::std::default::Default for DDMCCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDMCCOMPBUFFERINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDMCCOMPBUFFERINFO {}
unsafe impl ::windows::runtime::Abi for DDMCCOMPBUFFERINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDMCQUERY_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDMODEINFO_MAXREFRESH: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDMODEINFO_MODEX: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDMODEINFO_PALETTIZED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDMODEINFO_STANDARDVGA: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDMODEINFO_STEREO: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDMODEINFO_UNSUPPORTED: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDMONITORINFO {
    pub Manufacturer: u16,
    pub Product: u16,
    pub SerialNumber: u32,
    pub DeviceIdentifier: ::windows::runtime::GUID,
    pub Mode640x480: i32,
    pub Mode800x600: i32,
    pub Mode1024x768: i32,
    pub Mode1280x1024: i32,
    pub Mode1600x1200: i32,
    pub ModeReserved1: i32,
    pub ModeReserved2: i32,
    pub ModeReserved3: i32,
}
impl DDMONITORINFO {}
impl ::std::default::Default for DDMONITORINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDMONITORINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDMONITORINFO")
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
impl ::std::cmp::PartialEq for DDMONITORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer
            && self.Product == other.Product
            && self.SerialNumber == other.SerialNumber
            && self.DeviceIdentifier == other.DeviceIdentifier
            && self.Mode640x480 == other.Mode640x480
            && self.Mode800x600 == other.Mode800x600
            && self.Mode1024x768 == other.Mode1024x768
            && self.Mode1280x1024 == other.Mode1280x1024
            && self.Mode1600x1200 == other.Mode1600x1200
            && self.ModeReserved1 == other.ModeReserved1
            && self.ModeReserved2 == other.ModeReserved2
            && self.ModeReserved3 == other.ModeReserved3
    }
}
impl ::std::cmp::Eq for DDMONITORINFO {}
unsafe impl ::windows::runtime::Abi for DDMONITORINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDMORESURFACECAPS {
    pub dwSize: u32,
    pub ddsCapsMore: DDSCAPSEX,
    pub ddsExtendedHeapRestrictions: [DDMORESURFACECAPS_0; 1],
}
impl DDMORESURFACECAPS {}
impl ::std::default::Default for DDMORESURFACECAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDMORESURFACECAPS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDMORESURFACECAPS {}
unsafe impl ::windows::runtime::Abi for DDMORESURFACECAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDMORESURFACECAPS_0 {
    pub ddsCapsEx: DDSCAPSEX,
    pub ddsCapsExAlt: DDSCAPSEX,
}
impl DDMORESURFACECAPS_0 {}
impl ::std::default::Default for DDMORESURFACECAPS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDMORESURFACECAPS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDMORESURFACECAPS_0 {}
unsafe impl ::windows::runtime::Abi for DDMORESURFACECAPS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDNEWCALLBACKFNS: u32 = 12u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDNONLOCALVIDMEMCAPS {
    pub dwSize: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
}
impl DDNONLOCALVIDMEMCAPS {}
impl ::std::default::Default for DDNONLOCALVIDMEMCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDNONLOCALVIDMEMCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDNONLOCALVIDMEMCAPS").field("dwSize", &self.dwSize).field("dwNLVBCaps", &self.dwNLVBCaps).field("dwNLVBCaps2", &self.dwNLVBCaps2).field("dwNLVBCKeyCaps", &self.dwNLVBCKeyCaps).field("dwNLVBFXCaps", &self.dwNLVBFXCaps).field("dwNLVBRops", &self.dwNLVBRops).finish()
    }
}
impl ::std::cmp::PartialEq for DDNONLOCALVIDMEMCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwNLVBCaps == other.dwNLVBCaps && self.dwNLVBCaps2 == other.dwNLVBCaps2 && self.dwNLVBCKeyCaps == other.dwNLVBCKeyCaps && self.dwNLVBFXCaps == other.dwNLVBFXCaps && self.dwNLVBRops == other.dwNLVBRops
    }
}
impl ::std::cmp::Eq for DDNONLOCALVIDMEMCAPS {}
unsafe impl ::windows::runtime::Abi for DDNONLOCALVIDMEMCAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDNTCORECAPS {
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
}
impl DDNTCORECAPS {}
impl ::std::default::Default for DDNTCORECAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDNTCORECAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDNTCORECAPS")
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
impl ::std::cmp::PartialEq for DDNTCORECAPS {
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
impl ::std::cmp::Eq for DDNTCORECAPS {}
unsafe impl ::windows::runtime::Abi for DDNTCORECAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDOPTSURFACEDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ddSCaps: DDSCAPS2,
    pub ddOSCaps: DDOSCAPS,
    pub guid: ::windows::runtime::GUID,
    pub dwCompressionRatio: u32,
}
impl DDOPTSURFACEDESC {}
impl ::std::default::Default for DDOPTSURFACEDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDOPTSURFACEDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDOPTSURFACEDESC {}
unsafe impl ::windows::runtime::Abi for DDOPTSURFACEDESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDOSCAPS {
    pub dwCaps: u32,
}
impl DDOSCAPS {}
impl ::std::default::Default for DDOSCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDOSCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDOSCAPS").field("dwCaps", &self.dwCaps).finish()
    }
}
impl ::std::cmp::PartialEq for DDOSCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCaps == other.dwCaps
    }
}
impl ::std::cmp::Eq for DDOSCAPS {}
unsafe impl ::windows::runtime::Abi for DDOSCAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSDCAPS_MONOLITHICMIPMAP: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSDCAPS_OPTCOMPRESSED: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSDCAPS_OPTREORDERED: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSDCAPS_VALIDOSCAPS: i32 = 7i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSDCAPS_VALIDSCAPS: i32 = 805324800i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSD_ALL: i32 = 15i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSD_COMPRESSION_RATIO: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSD_GUID: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSD_OSCAPS: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOSD_SCAPS: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERFX_ARITHSTRETCHY: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERFX_DEINTERLACE: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERFX_MIRRORLEFTRIGHT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERFX_MIRRORUPDOWN: i32 = 4i32;
impl ::std::clone::Clone for DDOVERLAYFX {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDOVERLAYFX {
    pub dwSize: u32,
    pub dwAlphaEdgeBlendBitDepth: u32,
    pub dwAlphaEdgeBlend: u32,
    pub dwReserved: u32,
    pub dwAlphaDestConstBitDepth: u32,
    pub Anonymous1: DDOVERLAYFX_0,
    pub dwAlphaSrcConstBitDepth: u32,
    pub Anonymous2: DDOVERLAYFX_1,
    pub dckDestColorkey: DDCOLORKEY,
    pub dckSrcColorkey: DDCOLORKEY,
    pub dwDDFX: u32,
    pub dwFlags: u32,
}
impl DDOVERLAYFX {}
impl ::std::default::Default for DDOVERLAYFX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDOVERLAYFX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDOVERLAYFX {}
unsafe impl ::windows::runtime::Abi for DDOVERLAYFX {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
impl ::std::clone::Clone for DDOVERLAYFX_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDOVERLAYFX_0 {
    pub dwAlphaDestConst: u32,
    pub lpDDSAlphaDest: ::windows::runtime::RawPtr,
}
impl DDOVERLAYFX_0 {}
impl ::std::default::Default for DDOVERLAYFX_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDOVERLAYFX_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDOVERLAYFX_0 {}
unsafe impl ::windows::runtime::Abi for DDOVERLAYFX_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
impl ::std::clone::Clone for DDOVERLAYFX_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDOVERLAYFX_1 {
    pub dwAlphaSrcConst: u32,
    pub lpDDSAlphaSrc: ::windows::runtime::RawPtr,
}
impl DDOVERLAYFX_1 {}
impl ::std::default::Default for DDOVERLAYFX_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDOVERLAYFX_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDOVERLAYFX_1 {}
unsafe impl ::windows::runtime::Abi for DDOVERLAYFX_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERZ_INSERTINBACKOF: i32 = 5i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERZ_INSERTINFRONTOF: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERZ_MOVEBACKWARD: i32 = 3i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERZ_MOVEFORWARD: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERZ_SENDTOBACK: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVERZ_SENDTOFRONT: i32 = 0i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ADDDIRTYRECT: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHADEST: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHADESTCONSTOVERRIDE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHADESTNEG: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHADESTSURFACEOVERRIDE: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHAEDGEBLEND: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHASRC: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHASRCCONSTOVERRIDE: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHASRCNEG: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ALPHASRCSURFACEOVERRIDE: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_ARGBSCALEFACTORS: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_AUTOFLIP: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_BOB: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_BOBHARDWARE: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_DDFX: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_DEGRADEARGBSCALING: i32 = 67108864i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_HIDE: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_INTERLEAVED: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_KEYDEST: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_KEYDESTOVERRIDE: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_KEYSRC: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_KEYSRCOVERRIDE: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_OVERRIDEBOBWEAVE: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_REFRESHALL: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_REFRESHDIRTYRECTS: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDOVER_SHOW: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_1BIT: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_2BIT: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_4BIT: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_8BIT: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_8BITENTRIES: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_ALLOW256: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_ALPHA: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_INITIALIZE: i32 = 0i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_PRIMARYSURFACE: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_PRIMARYSURFACELEFT: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPCAPS_VSYNC: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_ALPHA: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_ALPHAPIXELS: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_ALPHAPREMULT: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_BUMPDUDV: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_BUMPLUMINANCE: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_COMPRESSED: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_D3DFORMAT: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_LUMINANCE: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_NOVEL_TEXTURE_FORMAT: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_PALETTEINDEXED1: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_PALETTEINDEXED2: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_PALETTEINDEXED4: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_PALETTEINDEXED8: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_PALETTEINDEXEDTO8: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_RGB: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_RGBTOYUV: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_STENCILBUFFER: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_YUV: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_ZBUFFER: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDPF_ZPIXELS: i32 = 8192i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDPIXELFORMAT {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFourCC: u32,
    pub Anonymous1: DDPIXELFORMAT_0,
    pub Anonymous2: DDPIXELFORMAT_1,
    pub Anonymous3: DDPIXELFORMAT_2,
    pub Anonymous4: DDPIXELFORMAT_3,
    pub Anonymous5: DDPIXELFORMAT_4,
}
impl DDPIXELFORMAT {}
impl ::std::default::Default for DDPIXELFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDPIXELFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDPIXELFORMAT {}
unsafe impl ::windows::runtime::Abi for DDPIXELFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDPIXELFORMAT_0 {
    pub dwRGBBitCount: u32,
    pub dwYUVBitCount: u32,
    pub dwZBufferBitDepth: u32,
    pub dwAlphaBitDepth: u32,
    pub dwLuminanceBitCount: u32,
    pub dwBumpBitCount: u32,
    pub dwPrivateFormatBitCount: u32,
}
impl DDPIXELFORMAT_0 {}
impl ::std::default::Default for DDPIXELFORMAT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDPIXELFORMAT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDPIXELFORMAT_0 {}
unsafe impl ::windows::runtime::Abi for DDPIXELFORMAT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDPIXELFORMAT_1 {
    pub dwRBitMask: u32,
    pub dwYBitMask: u32,
    pub dwStencilBitDepth: u32,
    pub dwLuminanceBitMask: u32,
    pub dwBumpDuBitMask: u32,
    pub dwOperations: u32,
}
impl DDPIXELFORMAT_1 {}
impl ::std::default::Default for DDPIXELFORMAT_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDPIXELFORMAT_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDPIXELFORMAT_1 {}
unsafe impl ::windows::runtime::Abi for DDPIXELFORMAT_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDPIXELFORMAT_2 {
    pub dwGBitMask: u32,
    pub dwUBitMask: u32,
    pub dwZBitMask: u32,
    pub dwBumpDvBitMask: u32,
    pub MultiSampleCaps: DDPIXELFORMAT_2_0,
}
impl DDPIXELFORMAT_2 {}
impl ::std::default::Default for DDPIXELFORMAT_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDPIXELFORMAT_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDPIXELFORMAT_2 {}
unsafe impl ::windows::runtime::Abi for DDPIXELFORMAT_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDPIXELFORMAT_2_0 {
    pub wFlipMSTypes: u16,
    pub wBltMSTypes: u16,
}
impl DDPIXELFORMAT_2_0 {}
impl ::std::default::Default for DDPIXELFORMAT_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDPIXELFORMAT_2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_MultiSampleCaps_e__Struct").field("wFlipMSTypes", &self.wFlipMSTypes).field("wBltMSTypes", &self.wBltMSTypes).finish()
    }
}
impl ::std::cmp::PartialEq for DDPIXELFORMAT_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wFlipMSTypes == other.wFlipMSTypes && self.wBltMSTypes == other.wBltMSTypes
    }
}
impl ::std::cmp::Eq for DDPIXELFORMAT_2_0 {}
unsafe impl ::windows::runtime::Abi for DDPIXELFORMAT_2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDPIXELFORMAT_3 {
    pub dwBBitMask: u32,
    pub dwVBitMask: u32,
    pub dwStencilBitMask: u32,
    pub dwBumpLuminanceBitMask: u32,
}
impl DDPIXELFORMAT_3 {}
impl ::std::default::Default for DDPIXELFORMAT_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDPIXELFORMAT_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDPIXELFORMAT_3 {}
unsafe impl ::windows::runtime::Abi for DDPIXELFORMAT_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDPIXELFORMAT_4 {
    pub dwRGBAlphaBitMask: u32,
    pub dwYUVAlphaBitMask: u32,
    pub dwLuminanceAlphaBitMask: u32,
    pub dwRGBZBitMask: u32,
    pub dwYUVZBitMask: u32,
}
impl DDPIXELFORMAT_4 {}
impl ::std::default::Default for DDPIXELFORMAT_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDPIXELFORMAT_4 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDPIXELFORMAT_4 {}
unsafe impl ::windows::runtime::Abi for DDPIXELFORMAT_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWICLIP_INMASTERSPRITELIST: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWICLIP_ISINITIALIZED: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWICLIP_WATCHWINDOW: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_ACTIVENO: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_ACTIVEYES: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_ALLOWMODEX: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_ATTEMPTEDD3DCONTEXT: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_CREATEDWINDOW: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_CURSORCLIPPED: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_DIRECTDRAW7: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_DIRECTDRAW8: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_DIRTYDC: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_DISABLEINACTIVATE: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_DX8DRIVER: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_EXPLICITMONITOR: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_FPUPRESERVE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_FPUSETUP: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_HASEXCLUSIVEMODE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_HOOKEDHWND: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_ISFULLSCREEN: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_MODEHASBEENCHANGED: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_MULTITHREADED: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_POWEREDDOWN: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_SETCOOPCALLED: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWILCL_V1SCLBEHAVIOUR: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_16: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_2: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_256: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_4: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_ALLOW256: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_ALPHA: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_DIRTY: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_EXCLUSIVE: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_GDI: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_INHEL: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_STORED_16: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_STORED_24: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_STORED_8: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIPAL_STORED_8INDEX: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_DDHELDONTFREE: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_DX8SURFACE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_FASTLOCKHELD: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_HARDWAREOPDEST: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_HARDWAREOPSOURCE: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_IMPLICITHANDLE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_ISCLIENTMEM: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_ISGDISURFACE: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_LATEALLOCATELINEAR: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_LOCKBROKEN: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_LOCKNOTHOLDINGWIN16LOCK: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_LOCKVRAMSTYLE: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_MEMFREE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_NOTIFYWHENUNLOCKED: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_READONLYLOCKHELD: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_RESERVED0: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_SOFTWAREAUTOFLIP: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_SYSMEMEXECUTEBUFFER: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_SYSMEMREQUESTED: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_VPORTDATA: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURFGBL_VPORTINTERLEAVED: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_ATTACHED: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_ATTACHED_FROM: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_BACKBUFFER: i32 = 134217728i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_DATAISALIASED: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_DCIBUSY: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_DCILOCK: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_DRIVERMANAGED: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_FRONTBUFFER: i32 = 67108864i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_GETDCNULL: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HASCKEYDESTBLT: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HASCKEYDESTOVERLAY: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HASCKEYSRCBLT: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HASCKEYSRCOVERLAY: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HASDC: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HASOVERLAYDATA: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HASPIXELFORMAT: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HELCB: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HW_CKEYDESTBLT: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HW_CKEYDESTOVERLAY: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HW_CKEYSRCBLT: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_HW_CKEYSRCOVERLAY: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_IMPLICITCREATE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_IMPLICITROOT: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_INMASTERSPRITELIST: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_INVALID: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_ISFREE: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_LOCKEXCLUDEDCURSOR: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_PARTOFPRIMARYCHAIN: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_SETGAMMA: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_STEREOSURFACELEFT: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_SW_CKEYDESTBLT: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_SW_CKEYDESTOVERLAY: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_SW_CKEYSRCBLT: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWISURF_SW_CKEYSRCOVERLAY: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIVPORT_COLORKEYANDINTERP: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIVPORT_NOKERNELHANDLES: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIVPORT_ON: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIVPORT_SOFTWARE_AUTOFLIP: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIVPORT_SOFTWARE_BOB: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIVPORT_VBION: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWIVPORT_VIDEOON: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_ATTACHEDTODESKTOP: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_BADPDEV: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_CHANGINGMODE: i32 = 4194304i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDMOTIONCOMP_INT {
    pub lpVtbl: *mut ::std::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpLink: *mut DDRAWI_DDMOTIONCOMP_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDMOTIONCOMP_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDMOTIONCOMP_INT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDMOTIONCOMP_INT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDMOTIONCOMP_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDMOTIONCOMP_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDMOTIONCOMP_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDMOTIONCOMP_INT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDMOTIONCOMP_LCL {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub guid: ::windows::runtime::GUID,
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: DDPIXELFORMAT,
    pub dwInternalFlags: u32,
    pub dwRefCnt: u32,
    pub dwProcessId: u32,
    pub hMoComp: super::super::Foundation::HANDLE,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub lpDriverReserved1: *mut ::std::ffi::c_void,
    pub lpDriverReserved2: *mut ::std::ffi::c_void,
    pub lpDriverReserved3: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDMOTIONCOMP_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDMOTIONCOMP_LCL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDMOTIONCOMP_LCL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDMOTIONCOMP_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDMOTIONCOMP_LCL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWCLIPPER_GBL {
    pub dwRefCnt: u32,
    pub dwFlags: u32,
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwProcessId: u32,
    pub dwReserved1: usize,
    pub hWnd: usize,
    pub lpStaticClipList: *mut super::Gdi::RGNDATA,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWCLIPPER_GBL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWCLIPPER_GBL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDRAWCLIPPER_GBL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDRAWCLIPPER_GBL").field("dwRefCnt", &self.dwRefCnt).field("dwFlags", &self.dwFlags).field("lpDD", &self.lpDD).field("dwProcessId", &self.dwProcessId).field("dwReserved1", &self.dwReserved1).field("hWnd", &self.hWnd).field("lpStaticClipList", &self.lpStaticClipList).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_GBL {
    fn eq(&self, other: &Self) -> bool {
        self.dwRefCnt == other.dwRefCnt && self.dwFlags == other.dwFlags && self.lpDD == other.lpDD && self.dwProcessId == other.dwProcessId && self.dwReserved1 == other.dwReserved1 && self.hWnd == other.hWnd && self.lpStaticClipList == other.lpStaticClipList
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWCLIPPER_GBL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWCLIPPER_GBL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWCLIPPER_INT {
    pub lpVtbl: *mut ::std::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDRAWCLIPPER_LCL,
    pub lpLink: *mut DDRAWI_DDRAWCLIPPER_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWCLIPPER_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWCLIPPER_INT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDRAWCLIPPER_INT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDRAWCLIPPER_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWCLIPPER_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWCLIPPER_INT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWCLIPPER_LCL {
    pub lpClipMore: u32,
    pub lpGbl: *mut DDRAWI_DDRAWCLIPPER_GBL,
    pub lpDD_lcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwLocalRefCnt: u32,
    pub pUnkOuter: ::std::option::Option<::windows::runtime::IUnknown>,
    pub lpDD_int: *mut DDRAWI_DIRECTDRAW_INT,
    pub dwReserved1: usize,
    pub pAddrefedThisOwner: ::std::option::Option<::windows::runtime::IUnknown>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWCLIPPER_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWCLIPPER_LCL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDRAWCLIPPER_LCL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDRAWCLIPPER_LCL")
            .field("lpClipMore", &self.lpClipMore)
            .field("lpGbl", &self.lpGbl)
            .field("lpDD_lcl", &self.lpDD_lcl)
            .field("dwLocalRefCnt", &self.dwLocalRefCnt)
            .field("pUnkOuter", &self.pUnkOuter)
            .field("lpDD_int", &self.lpDD_int)
            .field("dwReserved1", &self.dwReserved1)
            .field("pAddrefedThisOwner", &self.pAddrefedThisOwner)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpClipMore == other.lpClipMore && self.lpGbl == other.lpGbl && self.lpDD_lcl == other.lpDD_lcl && self.dwLocalRefCnt == other.dwLocalRefCnt && self.pUnkOuter == other.pUnkOuter && self.lpDD_int == other.lpDD_int && self.dwReserved1 == other.dwReserved1 && self.pAddrefedThisOwner == other.pAddrefedThisOwner
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWCLIPPER_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWCLIPPER_LCL {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_DDRAWDATANOTFETCHED: i32 = 67108864i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWPALETTE_GBL {
    pub dwRefCnt: u32,
    pub dwFlags: u32,
    pub lpDD_lcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwProcessId: u32,
    pub lpColorTable: *mut super::Gdi::PALETTEENTRY,
    pub Anonymous: DDRAWI_DDRAWPALETTE_GBL_0,
    pub dwDriverReserved: u32,
    pub dwContentsStamp: u32,
    pub dwSaveStamp: u32,
    pub dwHandle: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWPALETTE_GBL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWPALETTE_GBL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWPALETTE_GBL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWPALETTE_GBL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWPALETTE_GBL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
pub union DDRAWI_DDRAWPALETTE_GBL_0 {
    pub dwReserved1: usize,
    pub hHELGDIPalette: super::Gdi::HPALETTE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl DDRAWI_DDRAWPALETTE_GBL_0 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for DDRAWI_DDRAWPALETTE_GBL_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWPALETTE_GBL_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for DDRAWI_DDRAWPALETTE_GBL_0 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWPALETTE_GBL_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWPALETTE_INT {
    pub lpVtbl: *mut ::std::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDRAWPALETTE_LCL,
    pub lpLink: *mut DDRAWI_DDRAWPALETTE_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWPALETTE_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWPALETTE_INT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDRAWPALETTE_INT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDRAWPALETTE_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWPALETTE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWPALETTE_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWPALETTE_INT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWPALETTE_LCL {
    pub lpPalMore: u32,
    pub lpGbl: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub dwUnused0: usize,
    pub dwLocalRefCnt: u32,
    pub pUnkOuter: ::std::option::Option<::windows::runtime::IUnknown>,
    pub lpDD_lcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwReserved1: usize,
    pub dwDDRAWReserved1: usize,
    pub dwDDRAWReserved2: usize,
    pub dwDDRAWReserved3: usize,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWPALETTE_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWPALETTE_LCL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDRAWPALETTE_LCL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDRAWPALETTE_LCL")
            .field("lpPalMore", &self.lpPalMore)
            .field("lpGbl", &self.lpGbl)
            .field("dwUnused0", &self.dwUnused0)
            .field("dwLocalRefCnt", &self.dwLocalRefCnt)
            .field("pUnkOuter", &self.pUnkOuter)
            .field("lpDD_lcl", &self.lpDD_lcl)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwDDRAWReserved1", &self.dwDDRAWReserved1)
            .field("dwDDRAWReserved2", &self.dwDDRAWReserved2)
            .field("dwDDRAWReserved3", &self.dwDDRAWReserved3)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWPALETTE_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpPalMore == other.lpPalMore && self.lpGbl == other.lpGbl && self.dwUnused0 == other.dwUnused0 && self.dwLocalRefCnt == other.dwLocalRefCnt && self.pUnkOuter == other.pUnkOuter && self.lpDD_lcl == other.lpDD_lcl && self.dwReserved1 == other.dwReserved1 && self.dwDDRAWReserved1 == other.dwDDRAWReserved1 && self.dwDDRAWReserved2 == other.dwDDRAWReserved2 && self.dwDDRAWReserved3 == other.dwDDRAWReserved3
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWPALETTE_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWPALETTE_LCL {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWSURFACE_GBL {
    pub dwRefCnt: u32,
    pub dwGlobalFlags: u32,
    pub Anonymous1: DDRAWI_DDRAWSURFACE_GBL_0,
    pub Anonymous2: DDRAWI_DDRAWSURFACE_GBL_1,
    pub Anonymous3: DDRAWI_DDRAWSURFACE_GBL_2,
    pub fpVidMem: usize,
    pub Anonymous4: DDRAWI_DDRAWSURFACE_GBL_3,
    pub wHeight: u16,
    pub wWidth: u16,
    pub dwUsageCount: u32,
    pub dwReserved1: usize,
    pub ddpfSurface: DDPIXELFORMAT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWSURFACE_GBL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_GBL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_GBL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_GBL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_GBL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub union DDRAWI_DDRAWSURFACE_GBL_0 {
    pub lpRectList: *mut ACCESSRECTLIST,
    pub dwBlockSizeY: u32,
    pub lSlicePitch: i32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWSURFACE_GBL_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_GBL_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_GBL_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_GBL_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_GBL_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`*"]
pub union DDRAWI_DDRAWSURFACE_GBL_1 {
    pub lpVidMemHeap: *mut super::super::Devices::Display::VMEMHEAP,
    pub dwBlockSizeX: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl DDRAWI_DDRAWSURFACE_GBL_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_GBL_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_GBL_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_GBL_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_GBL_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub union DDRAWI_DDRAWSURFACE_GBL_2 {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDHandle: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWSURFACE_GBL_2 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_GBL_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_GBL_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_GBL_2 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_GBL_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDRAWI_DDRAWSURFACE_GBL_3 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
impl DDRAWI_DDRAWSURFACE_GBL_3 {}
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_GBL_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_GBL_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_GBL_3 {}
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_GBL_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDRAWI_DDRAWSURFACE_GBL_MORE {
    pub dwSize: u32,
    pub Anonymous: DDRAWI_DDRAWSURFACE_GBL_MORE_0,
    pub pPageTable: *mut u32,
    pub cPages: u32,
    pub dwSavedDCContext: usize,
    pub fpAliasedVidMem: usize,
    pub dwDriverReserved: usize,
    pub dwHELReserved: usize,
    pub cPageUnlocks: u32,
    pub hKernelSurface: usize,
    pub dwKernelRefCnt: u32,
    pub lpColorInfo: *mut DDCOLORCONTROL,
    pub fpNTAlias: usize,
    pub dwContentsStamp: u32,
    pub lpvUnswappedDriverReserved: *mut ::std::ffi::c_void,
    pub lpDDRAWReserved2: *mut ::std::ffi::c_void,
    pub dwDDRAWReserved1: u32,
    pub dwDDRAWReserved2: u32,
    pub fpAliasOfVidMem: usize,
}
impl DDRAWI_DDRAWSURFACE_GBL_MORE {}
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_GBL_MORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_GBL_MORE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_GBL_MORE {}
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_GBL_MORE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDRAWI_DDRAWSURFACE_GBL_MORE_0 {
    pub dwPhysicalPageTable: u32,
    pub fpPhysicalVidMem: usize,
}
impl DDRAWI_DDRAWSURFACE_GBL_MORE_0 {}
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_GBL_MORE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_GBL_MORE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_GBL_MORE_0 {}
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_GBL_MORE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWSURFACE_INT {
    pub lpVtbl: *mut ::std::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpLink: *mut DDRAWI_DDRAWSURFACE_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWSURFACE_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_INT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDRAWSURFACE_INT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDRAWSURFACE_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_INT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWSURFACE_LCL {
    pub lpSurfMore: *mut DDRAWI_DDRAWSURFACE_MORE,
    pub lpGbl: *mut DDRAWI_DDRAWSURFACE_GBL,
    pub hDDSurface: usize,
    pub lpAttachList: *mut ATTACHLIST,
    pub lpAttachListFrom: *mut ATTACHLIST,
    pub dwLocalRefCnt: u32,
    pub dwProcessId: u32,
    pub dwFlags: u32,
    pub ddsCaps: DDSCAPS,
    pub Anonymous1: DDRAWI_DDRAWSURFACE_LCL_0,
    pub Anonymous2: DDRAWI_DDRAWSURFACE_LCL_1,
    pub dwModeCreatedIn: u32,
    pub dwBackBufferCount: u32,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub hDC: usize,
    pub dwReserved1: usize,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub lpSurfaceOverlaying: *mut DDRAWI_DDRAWSURFACE_INT,
    pub dbnOverlayNode: DBLNODE,
    pub rcOverlaySrc: super::super::Foundation::RECT,
    pub rcOverlayDest: super::super::Foundation::RECT,
    pub dwClrXparent: u32,
    pub dwAlpha: u32,
    pub lOverlayX: i32,
    pub lOverlayY: i32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWSURFACE_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_LCL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_LCL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_LCL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub union DDRAWI_DDRAWSURFACE_LCL_0 {
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_INT,
    pub lp16DDPalette: *mut DDRAWI_DDRAWPALETTE_INT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWSURFACE_LCL_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_LCL_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_LCL_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_LCL_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_LCL_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub union DDRAWI_DDRAWSURFACE_LCL_1 {
    pub lpDDClipper: *mut ::std::mem::ManuallyDrop<DDRAWI_DDRAWCLIPPER_LCL>,
    pub lp16DDClipper: *mut DDRAWI_DDRAWCLIPPER_INT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWSURFACE_LCL_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_LCL_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_LCL_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_LCL_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_LCL_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDRAWSURFACE_MORE {
    pub dwSize: u32,
    pub lpIUnknowns: *mut IUNKNOWN_LIST,
    pub lpDD_lcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwPageLockCount: u32,
    pub dwBytesAllocated: u32,
    pub lpDD_int: *mut DDRAWI_DIRECTDRAW_INT,
    pub dwMipMapCount: u32,
    pub lpDDIClipper: *mut DDRAWI_DDRAWCLIPPER_INT,
    pub lpHeapAliasInfo: *mut HEAPALIASINFO,
    pub dwOverlayFlags: u32,
    pub rgjunc: *mut ::std::ffi::c_void,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lpddOverlayFX: *mut DDOVERLAYFX,
    pub ddsCapsEx: DDSCAPSEX,
    pub dwTextureStage: u32,
    pub lpDDRAWReserved: *mut ::std::ffi::c_void,
    pub lpDDRAWReserved2: *mut ::std::ffi::c_void,
    pub lpDDrawReserved3: *mut ::std::ffi::c_void,
    pub dwDDrawReserved4: u32,
    pub lpDDrawReserved5: *mut ::std::ffi::c_void,
    pub lpGammaRamp: *mut u32,
    pub lpOriginalGammaRamp: *mut u32,
    pub lpDDrawReserved6: *mut ::std::ffi::c_void,
    pub dwSurfaceHandle: u32,
    pub qwDDrawReserved8: [u32; 2],
    pub lpDDrawReserved9: *mut ::std::ffi::c_void,
    pub cSurfaces: u32,
    pub pCreatedDDSurfaceDesc2: *mut DDSURFACEDESC2,
    pub slist: *mut *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFVF: u32,
    pub lpVB: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDRAWSURFACE_MORE {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDRAWSURFACE_MORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDRAWSURFACE_MORE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDRAWSURFACE_MORE {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDRAWSURFACE_MORE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDVIDEOPORT_INT {
    pub lpVtbl: *mut ::std::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lpLink: *mut DDRAWI_DDVIDEOPORT_INT,
    pub dwIntRefCnt: u32,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDVIDEOPORT_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDVIDEOPORT_INT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDVIDEOPORT_INT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDVIDEOPORT_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDVIDEOPORT_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt && self.dwFlags == other.dwFlags
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDVIDEOPORT_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDVIDEOPORT_INT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DDVIDEOPORT_LCL {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub ddvpDesc: super::super::UI::DisplayDevices::DDVIDEOPORTDESC,
    pub ddvpInfo: super::super::UI::DisplayDevices::DDVIDEOPORTINFO,
    pub lpSurface: *mut DDRAWI_DDRAWSURFACE_INT,
    pub lpVBISurface: *mut DDRAWI_DDRAWSURFACE_INT,
    pub lpFlipInts: *mut *mut DDRAWI_DDRAWSURFACE_INT,
    pub dwNumAutoflip: u32,
    pub dwProcessID: u32,
    pub dwStateFlags: u32,
    pub dwFlags: u32,
    pub dwRefCnt: u32,
    pub fpLastFlip: usize,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub hDDVideoPort: super::super::Foundation::HANDLE,
    pub dwNumVBIAutoflip: u32,
    pub lpVBIDesc: *mut super::super::UI::DisplayDevices::DDVIDEOPORTDESC,
    pub lpVideoDesc: *mut super::super::UI::DisplayDevices::DDVIDEOPORTDESC,
    pub lpVBIInfo: *mut super::super::UI::DisplayDevices::DDVIDEOPORTINFO,
    pub lpVideoInfo: *mut super::super::UI::DisplayDevices::DDVIDEOPORTINFO,
    pub dwVBIProcessID: u32,
    pub lpVPNotify: *mut DDRAWI_DDVIDEOPORT_INT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DDVIDEOPORT_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DDVIDEOPORT_LCL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DDVIDEOPORT_LCL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DDVIDEOPORT_LCL")
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
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DDVIDEOPORT_LCL {
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
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DDVIDEOPORT_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DDVIDEOPORT_LCL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DIRECTDRAW_GBL {
    pub dwRefCnt: u32,
    pub dwFlags: u32,
    pub fpPrimaryOrig: usize,
    pub ddCaps: super::super::UI::DisplayDevices::DDCORECAPS,
    pub dwInternal1: u32,
    pub dwUnused1: [u32; 9],
    pub lpDDCBtmp: *mut DDHAL_CALLBACKS,
    pub dsList: *mut DDRAWI_DDRAWSURFACE_INT,
    pub palList: *mut DDRAWI_DDRAWPALETTE_INT,
    pub clipperList: *mut DDRAWI_DDRAWCLIPPER_INT,
    pub lp16DD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwMaxOverlays: u32,
    pub dwCurrOverlays: u32,
    pub dwMonitorFrequency: u32,
    pub ddHELCaps: super::super::UI::DisplayDevices::DDCORECAPS,
    pub dwUnused2: [u32; 50],
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub vmiData: VIDMEMINFO,
    pub lpDriverHandle: *mut ::std::ffi::c_void,
    pub lpExclusiveOwner: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwModeIndex: u32,
    pub dwModeIndexOrig: u32,
    pub dwNumFourCC: u32,
    pub lpdwFourCC: *mut u32,
    pub dwNumModes: u32,
    pub lpModeInfo: *mut DDHALMODEINFO,
    pub plProcessList: PROCESS_LIST,
    pub dwSurfaceLockCount: u32,
    pub dwAliasedLockCnt: u32,
    pub dwReserved3: usize,
    pub hDD: usize,
    pub cObsolete: [super::super::Foundation::CHAR; 12],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dbnOverlayRoot: DBLNODE,
    pub lpwPDeviceFlags: *mut u16,
    pub dwPDevice: u32,
    pub dwWin16LockCnt: u32,
    pub dwUnused3: u32,
    pub hInstance: u32,
    pub dwEvent16: u32,
    pub dwSaveNumModes: u32,
    pub lpD3DGlobalDriverData: usize,
    pub lpD3DHALCallbacks: usize,
    pub ddBothCaps: super::super::UI::DisplayDevices::DDCORECAPS,
    pub lpDDVideoPortCaps: *mut super::super::UI::DisplayDevices::DDVIDEOPORTCAPS,
    pub dvpList: *mut DDRAWI_DDVIDEOPORT_INT,
    pub lpD3DHALCallbacks2: usize,
    pub rectDevice: super::super::Foundation::RECT,
    pub cMonitors: u32,
    pub gpbmiSrc: *mut ::std::ffi::c_void,
    pub gpbmiDest: *mut ::std::ffi::c_void,
    pub phaiHeapAliases: *mut HEAPALIASINFO,
    pub hKernelHandle: usize,
    pub pfnNotifyProc: usize,
    pub lpDDKernelCaps: *mut super::super::UI::DisplayDevices::DDKERNELCAPS,
    pub lpddNLVCaps: *mut DDNONLOCALVIDMEMCAPS,
    pub lpddNLVHELCaps: *mut DDNONLOCALVIDMEMCAPS,
    pub lpddNLVBothCaps: *mut DDNONLOCALVIDMEMCAPS,
    pub lpD3DExtendedCaps: usize,
    pub dwDOSBoxEvent: u32,
    pub rectDesktop: super::super::Foundation::RECT,
    pub cDriverName: [super::super::Foundation::CHAR; 32],
    pub lpD3DHALCallbacks3: usize,
    pub dwNumZPixelFormats: u32,
    pub lpZPixelFormats: *mut DDPIXELFORMAT,
    pub mcList: *mut DDRAWI_DDMOTIONCOMP_INT,
    pub hDDVxd: u32,
    pub ddsCapsMore: DDSCAPSEX,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DIRECTDRAW_GBL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DIRECTDRAW_GBL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DIRECTDRAW_GBL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DIRECTDRAW_GBL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DIRECTDRAW_GBL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DIRECTDRAW_INT {
    pub lpVtbl: *mut ::std::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpLink: *mut DDRAWI_DIRECTDRAW_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DIRECTDRAW_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DIRECTDRAW_INT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DIRECTDRAW_INT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DIRECTDRAW_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DIRECTDRAW_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DIRECTDRAW_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DIRECTDRAW_INT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_DisplayDevices`*"]
pub struct DDRAWI_DIRECTDRAW_LCL {
    pub lpDDMore: u32,
    pub lpGbl: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwUnused0: u32,
    pub dwLocalFlags: u32,
    pub dwLocalRefCnt: u32,
    pub dwProcessId: u32,
    pub pUnkOuter: ::std::option::Option<::windows::runtime::IUnknown>,
    pub dwObsolete1: u32,
    pub hWnd: usize,
    pub hDC: usize,
    pub dwErrorMode: u32,
    pub lpPrimary: *mut DDRAWI_DDRAWSURFACE_INT,
    pub lpCB: *mut DDRAWI_DDRAWSURFACE_INT,
    pub dwPreferredMode: u32,
    pub hD3DInstance: super::super::Foundation::HINSTANCE,
    pub pD3DIUnknown: ::std::option::Option<::windows::runtime::IUnknown>,
    pub lpDDCB: *mut DDHAL_CALLBACKS,
    pub hDDVxd: usize,
    pub dwAppHackFlags: u32,
    pub hFocusWnd: usize,
    pub dwHotTracking: u32,
    pub dwIMEState: u32,
    pub hWndPopup: usize,
    pub hDD: usize,
    pub hGammaCalibrator: usize,
    pub lpGammaCalibrator: ::std::option::Option<LPDDGAMMACALIBRATORPROC>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl DDRAWI_DIRECTDRAW_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DDRAWI_DIRECTDRAW_LCL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DDRAWI_DIRECTDRAW_LCL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRAWI_DIRECTDRAW_LCL")
            .field("lpDDMore", &self.lpDDMore)
            .field("lpGbl", &self.lpGbl)
            .field("dwUnused0", &self.dwUnused0)
            .field("dwLocalFlags", &self.dwLocalFlags)
            .field("dwLocalRefCnt", &self.dwLocalRefCnt)
            .field("dwProcessId", &self.dwProcessId)
            .field("pUnkOuter", &self.pUnkOuter)
            .field("dwObsolete1", &self.dwObsolete1)
            .field("hWnd", &self.hWnd)
            .field("hDC", &self.hDC)
            .field("dwErrorMode", &self.dwErrorMode)
            .field("lpPrimary", &self.lpPrimary)
            .field("lpCB", &self.lpCB)
            .field("dwPreferredMode", &self.dwPreferredMode)
            .field("hD3DInstance", &self.hD3DInstance)
            .field("pD3DIUnknown", &self.pD3DIUnknown)
            .field("lpDDCB", &self.lpDDCB)
            .field("hDDVxd", &self.hDDVxd)
            .field("dwAppHackFlags", &self.dwAppHackFlags)
            .field("hFocusWnd", &self.hFocusWnd)
            .field("dwHotTracking", &self.dwHotTracking)
            .field("dwIMEState", &self.dwIMEState)
            .field("hWndPopup", &self.hWndPopup)
            .field("hDD", &self.hDD)
            .field("hGammaCalibrator", &self.hGammaCalibrator)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DDRAWI_DIRECTDRAW_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpDDMore == other.lpDDMore
            && self.lpGbl == other.lpGbl
            && self.dwUnused0 == other.dwUnused0
            && self.dwLocalFlags == other.dwLocalFlags
            && self.dwLocalRefCnt == other.dwLocalRefCnt
            && self.dwProcessId == other.dwProcessId
            && self.pUnkOuter == other.pUnkOuter
            && self.dwObsolete1 == other.dwObsolete1
            && self.hWnd == other.hWnd
            && self.hDC == other.hDC
            && self.dwErrorMode == other.dwErrorMode
            && self.lpPrimary == other.lpPrimary
            && self.lpCB == other.lpCB
            && self.dwPreferredMode == other.dwPreferredMode
            && self.hD3DInstance == other.hD3DInstance
            && self.pD3DIUnknown == other.pD3DIUnknown
            && self.lpDDCB == other.lpDDCB
            && self.hDDVxd == other.hDDVxd
            && self.dwAppHackFlags == other.dwAppHackFlags
            && self.hFocusWnd == other.hFocusWnd
            && self.dwHotTracking == other.dwHotTracking
            && self.dwIMEState == other.dwIMEState
            && self.hWndPopup == other.hWndPopup
            && self.hDD == other.hDD
            && self.hGammaCalibrator == other.hGammaCalibrator
            && self.lpGammaCalibrator.map(|f| f as usize) == other.lpGammaCalibrator.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DDRAWI_DIRECTDRAW_LCL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DDRAWI_DIRECTDRAW_LCL {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_DISPLAYDRV: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_DRIVERINFO2: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_EMULATIONINITIALIZED: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_EXTENDEDALIGNMENT: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_FLIPPEDTOGDI: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_FULLSCREEN: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_GDIDRV: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_GETCOLOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_HASCKEYDESTOVERLAY: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_HASCKEYSRCOVERLAY: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_HASGDIPALETTE: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_HASGDIPALETTE_EXCLUSIVE: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_MODECHANGED: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_MODEX: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_MODEXILLEGAL: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_NEEDSWIN16FORVRAMLOCK: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_NOEMULATION: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_NOHARDWARE: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_PALETTEINIT: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_PDEVICEVRAMBITCLEARED: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_SECONDARYDRIVERLOADED: i32 = 134217728i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_SETCOLOR: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_STANDARDVGA: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_TESTINGMODES: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_UMODELOADED: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_VIRTUALDESKTOP: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_VPORTGETCOLOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_VPORTSETCOLOR: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_VPORTSTART: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_VPORTSTOP: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_VPORTUPDATE: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_xxxxxxxxx1: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDRAWI_xxxxxxxxx2: i32 = 2i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
impl DDRGBA {}
impl ::std::default::Default for DDRGBA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDRGBA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDRGBA").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).field("alpha", &self.alpha).finish()
    }
}
impl ::std::cmp::PartialEq for DDRGBA {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue && self.alpha == other.alpha
    }
}
impl ::std::cmp::Eq for DDRGBA {}
unsafe impl ::windows::runtime::Abi for DDRGBA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDSCAPS {
    pub dwCaps: u32,
}
impl DDSCAPS {}
impl ::std::default::Default for DDSCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDSCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDSCAPS").field("dwCaps", &self.dwCaps).finish()
    }
}
impl ::std::cmp::PartialEq for DDSCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCaps == other.dwCaps
    }
}
impl ::std::cmp::Eq for DDSCAPS {}
unsafe impl ::windows::runtime::Abi for DDSCAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDSCAPS2 {
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCaps3: u32,
    pub Anonymous: DDSCAPS2_0,
}
impl DDSCAPS2 {}
impl ::std::default::Default for DDSCAPS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSCAPS2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSCAPS2 {}
unsafe impl ::windows::runtime::Abi for DDSCAPS2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSCAPS2_0 {
    pub dwCaps4: u32,
    pub dwVolumeDepth: u32,
}
impl DDSCAPS2_0 {}
impl ::std::default::Default for DDSCAPS2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSCAPS2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSCAPS2_0 {}
unsafe impl ::windows::runtime::Abi for DDSCAPS2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_ADDITIONALPRIMARY: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_COMMANDBUFFER: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_CUBEMAP: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_CUBEMAP_NEGATIVEX: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_CUBEMAP_NEGATIVEY: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_CUBEMAP_NEGATIVEZ: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_CUBEMAP_POSITIVEX: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_CUBEMAP_POSITIVEY: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_CUBEMAP_POSITIVEZ: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_D3DTEXTUREMANAGE: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_DISCARDBACKBUFFER: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_DONOTPERSIST: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_ENABLEALPHACHANNEL: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_EXTENDEDFORMATPRIMARY: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_HARDWAREDEINTERLACE: i32 = 0i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_HINTANTIALIASING: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_HINTDYNAMIC: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_HINTSTATIC: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_INDEXBUFFER: i32 = 67108864i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_MIPMAPSUBLEVEL: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_NOTUSERLOCKABLE: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_NPATCHES: i32 = 33554432i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_OPAQUE: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_POINTS: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_RESERVED1: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_RESERVED2: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_RESERVED3: i32 = 67108864i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_RESERVED4: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_RTPATCHES: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_STEREOSURFACELEFT: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_TEXTUREMANAGE: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_VERTEXBUFFER: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS2_VOLUME: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_AUTOGENMIPMAP: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_CREATESHAREDRESOURCE: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_DMAP: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_LIGHTWEIGHTMIPMAP: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_MULTISAMPLE_MASK: i32 = 31i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_MULTISAMPLE_QUALITY_MASK: i32 = 224i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_MULTISAMPLE_QUALITY_SHIFT: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_OPENSHAREDRESOURCE: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_READONLYRESOURCE: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_RESERVED1: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_RESERVED2: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS3_VIDEO: i32 = 512i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDSCAPSEX {
    pub dwCaps2: u32,
    pub dwCaps3: u32,
    pub Anonymous: DDSCAPSEX_0,
}
impl DDSCAPSEX {}
impl ::std::default::Default for DDSCAPSEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSCAPSEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSCAPSEX {}
unsafe impl ::windows::runtime::Abi for DDSCAPSEX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSCAPSEX_0 {
    pub dwCaps4: u32,
    pub dwVolumeDepth: u32,
}
impl DDSCAPSEX_0 {}
impl ::std::default::Default for DDSCAPSEX_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSCAPSEX_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSCAPSEX_0 {}
unsafe impl ::windows::runtime::Abi for DDSCAPSEX_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_3DDEVICE: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_ALLOCONLOAD: i32 = 67108864i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_ALPHA: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_BACKBUFFER: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_COMMANDBUFFER: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_COMPLEX: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_EXECUTEBUFFER: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_FLIP: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_FRONTBUFFER: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_HWCODEC: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_LIVEVIDEO: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_LOCALVIDMEM: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_MIPMAP: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_MODEX: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_NONLOCALVIDMEM: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_OFFSCREENPLAIN: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_OPTIMIZED: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_OVERLAY: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_OWNDC: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_PALETTE: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_PRIMARYSURFACE: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_PRIMARYSURFACELEFT: i32 = 0i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_RESERVED1: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_RESERVED2: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_RESERVED3: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_STANDARDVGAMODE: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_SYSTEMMEMORY: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_TEXTURE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_VIDEOMEMORY: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_VIDEOPORT: i32 = 134217728i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_VISIBLE: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_WRITEONLY: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCAPS_ZBUFFER: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_ALLOWMODEX: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_ALLOWREBOOT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_CREATEDEVICEWINDOW: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_EXCLUSIVE: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_FPUPRESERVE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_FPUSETUP: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_FULLSCREEN: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_MULTITHREADED: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_NORMAL: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_NOWINDOWCHANGES: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_SETDEVICEWINDOW: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSCL_SETFOCUSWINDOW: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSDM_STANDARDVGAMODE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_ALL: i32 = 16775662i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_ALPHABITDEPTH: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_BACKBUFFERCOUNT: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_CAPS: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_CKDESTBLT: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_CKDESTOVERLAY: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_CKSRCBLT: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_CKSRCOVERLAY: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_DEPTH: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_FVF: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_HEIGHT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_LINEARSIZE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_LPSURFACE: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_MIPMAPCOUNT: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_PITCH: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_PIXELFORMAT: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_REFRESHRATE: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_SRCVBHANDLE: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_TEXTURESTAGE: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_WIDTH: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSD_ZBUFFERBITDEPTH: i32 = 64i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDSETSTATEININFO {
    pub lpSurfaceData: *mut DDSURFACEDATA,
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
}
impl DDSETSTATEININFO {}
impl ::std::default::Default for DDSETSTATEININFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDSETSTATEININFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDSETSTATEININFO").field("lpSurfaceData", &self.lpSurfaceData).field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::std::cmp::PartialEq for DDSETSTATEININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData && self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::std::cmp::Eq for DDSETSTATEININFO {}
unsafe impl ::windows::runtime::Abi for DDSETSTATEININFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
pub struct DDSETSTATEOUTINFO {
    pub bSoftwareAutoflip: super::super::Foundation::BOOL,
    pub dwSurfaceIndex: u32,
    pub dwVBISurfaceIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DDSETSTATEOUTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DDSETSTATEOUTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DDSETSTATEOUTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDSETSTATEOUTINFO").field("bSoftwareAutoflip", &self.bSoftwareAutoflip).field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DDSETSTATEOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bSoftwareAutoflip == other.bSoftwareAutoflip && self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DDSETSTATEOUTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DDSETSTATEOUTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSETSURFACEDESC_PRESERVEDC: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSETSURFACEDESC_RECREATEDC: i32 = 0i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSGR_CALIBRATE: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDSKIPNEXTFIELDINFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
    pub dwSkipFlags: u32,
}
impl DDSKIPNEXTFIELDINFO {}
impl ::std::default::Default for DDSKIPNEXTFIELDINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDSKIPNEXTFIELDINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDSKIPNEXTFIELDINFO").field("lpVideoPortData", &self.lpVideoPortData).field("dwSkipFlags", &self.dwSkipFlags).finish()
    }
}
impl ::std::cmp::PartialEq for DDSKIPNEXTFIELDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData && self.dwSkipFlags == other.dwSkipFlags
    }
}
impl ::std::cmp::Eq for DDSKIPNEXTFIELDINFO {}
unsafe impl ::windows::runtime::Abi for DDSKIPNEXTFIELDINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSKIP_ENABLENEXT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSKIP_SKIPNEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSMT_ISTESTREQUIRED: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSPD_IUNKNOWNPOINTER: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSPD_VOLATILE: i32 = 2i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
pub struct DDSTEREOMODE {
    pub dwSize: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub dwBpp: u32,
    pub dwRefreshRate: u32,
    pub bSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DDSTEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DDSTEREOMODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DDSTEREOMODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDSTEREOMODE").field("dwSize", &self.dwSize).field("dwHeight", &self.dwHeight).field("dwWidth", &self.dwWidth).field("dwBpp", &self.dwBpp).field("dwRefreshRate", &self.dwRefreshRate).field("bSupported", &self.bSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DDSTEREOMODE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwHeight == other.dwHeight && self.dwWidth == other.dwWidth && self.dwBpp == other.dwBpp && self.dwRefreshRate == other.dwRefreshRate && self.bSupported == other.bSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DDSTEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DDSTEREOMODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDSURFACEDATA {
    pub ddsCaps: u32,
    pub dwSurfaceOffset: u32,
    pub fpLockPtr: usize,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lPitch: i32,
    pub dwOverlayFlags: u32,
    pub dwOverlayOffset: u32,
    pub dwOverlaySrcWidth: u32,
    pub dwOverlaySrcHeight: u32,
    pub dwOverlayDestWidth: u32,
    pub dwOverlayDestHeight: u32,
    pub dwVideoPortId: u32,
    pub dwFormatFlags: u32,
    pub dwFormatFourCC: u32,
    pub dwFormatBitCount: u32,
    pub dwRBitMask: u32,
    pub dwGBitMask: u32,
    pub dwBBitMask: u32,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub dwDriverReserved4: u32,
}
impl DDSURFACEDATA {}
impl ::std::default::Default for DDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDSURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDSURFACEDATA")
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
impl ::std::cmp::PartialEq for DDSURFACEDATA {
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
impl ::std::cmp::Eq for DDSURFACEDATA {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDSURFACEDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub Anonymous1: DDSURFACEDESC_0,
    pub dwBackBufferCount: u32,
    pub Anonymous2: DDSURFACEDESC_1,
    pub dwAlphaBitDepth: u32,
    pub dwReserved: u32,
    pub lpSurface: *mut ::std::ffi::c_void,
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub ddpfPixelFormat: DDPIXELFORMAT,
    pub ddsCaps: DDSCAPS,
}
impl DDSURFACEDESC {}
impl ::std::default::Default for DDSURFACEDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSURFACEDESC_0 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
impl DDSURFACEDESC_0 {}
impl ::std::default::Default for DDSURFACEDESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC_0 {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSURFACEDESC_1 {
    pub dwMipMapCount: u32,
    pub dwZBufferBitDepth: u32,
    pub dwRefreshRate: u32,
}
impl DDSURFACEDESC_1 {}
impl ::std::default::Default for DDSURFACEDESC_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC_1 {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDSURFACEDESC2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub Anonymous1: DDSURFACEDESC2_0,
    pub Anonymous2: DDSURFACEDESC2_1,
    pub Anonymous3: DDSURFACEDESC2_2,
    pub dwAlphaBitDepth: u32,
    pub dwReserved: u32,
    pub lpSurface: *mut ::std::ffi::c_void,
    pub Anonymous4: DDSURFACEDESC2_3,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub Anonymous5: DDSURFACEDESC2_4,
    pub ddsCaps: DDSCAPS2,
    pub dwTextureStage: u32,
}
impl DDSURFACEDESC2 {}
impl ::std::default::Default for DDSURFACEDESC2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC2 {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSURFACEDESC2_0 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
impl DDSURFACEDESC2_0 {}
impl ::std::default::Default for DDSURFACEDESC2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC2_0 {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSURFACEDESC2_1 {
    pub dwBackBufferCount: u32,
    pub dwDepth: u32,
}
impl DDSURFACEDESC2_1 {}
impl ::std::default::Default for DDSURFACEDESC2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC2_1 {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC2_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSURFACEDESC2_2 {
    pub dwMipMapCount: u32,
    pub dwRefreshRate: u32,
    pub dwSrcVBHandle: u32,
}
impl DDSURFACEDESC2_2 {}
impl ::std::default::Default for DDSURFACEDESC2_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC2_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC2_2 {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC2_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSURFACEDESC2_3 {
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub dwEmptyFaceColor: u32,
}
impl DDSURFACEDESC2_3 {}
impl ::std::default::Default for DDSURFACEDESC2_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC2_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC2_3 {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC2_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union DDSURFACEDESC2_4 {
    pub ddpfPixelFormat: DDPIXELFORMAT,
    pub dwFVF: u32,
}
impl DDSURFACEDESC2_4 {}
impl ::std::default::Default for DDSURFACEDESC2_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DDSURFACEDESC2_4 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DDSURFACEDESC2_4 {}
unsafe impl ::windows::runtime::Abi for DDSURFACEDESC2_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSVCAPS_RESERVED1: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSVCAPS_RESERVED2: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSVCAPS_RESERVED3: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSVCAPS_RESERVED4: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDSVCAPS_STEREOSEQUENTIAL: i32 = 16i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDTRANSFERININFO {
    pub lpSurfaceData: *mut DDSURFACEDATA,
    pub dwStartLine: u32,
    pub dwEndLine: u32,
    pub dwTransferID: usize,
    pub dwTransferFlags: u32,
    pub lpDestMDL: *mut MDL,
}
impl DDTRANSFERININFO {}
impl ::std::default::Default for DDTRANSFERININFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDTRANSFERININFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDTRANSFERININFO").field("lpSurfaceData", &self.lpSurfaceData).field("dwStartLine", &self.dwStartLine).field("dwEndLine", &self.dwEndLine).field("dwTransferID", &self.dwTransferID).field("dwTransferFlags", &self.dwTransferFlags).field("lpDestMDL", &self.lpDestMDL).finish()
    }
}
impl ::std::cmp::PartialEq for DDTRANSFERININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData && self.dwStartLine == other.dwStartLine && self.dwEndLine == other.dwEndLine && self.dwTransferID == other.dwTransferID && self.dwTransferFlags == other.dwTransferFlags && self.lpDestMDL == other.lpDestMDL
    }
}
impl ::std::cmp::Eq for DDTRANSFERININFO {}
unsafe impl ::windows::runtime::Abi for DDTRANSFERININFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDTRANSFEROUTINFO {
    pub dwBufferPolarity: u32,
}
impl DDTRANSFEROUTINFO {}
impl ::std::default::Default for DDTRANSFEROUTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDTRANSFEROUTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDTRANSFEROUTINFO").field("dwBufferPolarity", &self.dwBufferPolarity).finish()
    }
}
impl ::std::cmp::PartialEq for DDTRANSFEROUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwBufferPolarity == other.dwBufferPolarity
    }
}
impl ::std::cmp::Eq for DDTRANSFEROUTINFO {}
unsafe impl ::windows::runtime::Abi for DDTRANSFEROUTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDTRANSFER_CANCEL: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDTRANSFER_HALFLINES: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDTRANSFER_INVERT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDTRANSFER_NONLOCALVIDMEM: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDTRANSFER_SYSTEMMEMORY: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDUNSUPPORTEDMODE: u32 = 4294967295u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDVERSIONDATA {
    pub dwHALVersion: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl DDVERSIONDATA {}
impl ::std::default::Default for DDVERSIONDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDVERSIONDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVERSIONDATA").field("dwHALVersion", &self.dwHALVersion).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::std::cmp::PartialEq for DDVERSIONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwHALVersion == other.dwHALVersion && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::std::cmp::Eq for DDVERSIONDATA {}
unsafe impl ::windows::runtime::Abi for DDVERSIONDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVERSIONINFO: u32 = 13u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDVIDEOPORTCONNECT {
    pub dwSize: u32,
    pub dwPortWidth: u32,
    pub guidTypeID: ::windows::runtime::GUID,
    pub dwFlags: u32,
    pub dwReserved1: usize,
}
impl DDVIDEOPORTCONNECT {}
impl ::std::default::Default for DDVIDEOPORTCONNECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDVIDEOPORTCONNECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVIDEOPORTCONNECT").field("dwSize", &self.dwSize).field("dwPortWidth", &self.dwPortWidth).field("guidTypeID", &self.guidTypeID).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::std::cmp::PartialEq for DDVIDEOPORTCONNECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwPortWidth == other.dwPortWidth && self.guidTypeID == other.guidTypeID && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1
    }
}
impl ::std::cmp::Eq for DDVIDEOPORTCONNECT {}
unsafe impl ::windows::runtime::Abi for DDVIDEOPORTCONNECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDVIDEOPORTDATA {
    pub dwVideoPortId: u32,
    pub dwVPFlags: u32,
    pub dwOriginOffset: u32,
    pub dwHeight: u32,
    pub dwVBIHeight: u32,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
}
impl DDVIDEOPORTDATA {}
impl ::std::default::Default for DDVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDVIDEOPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVIDEOPORTDATA")
            .field("dwVideoPortId", &self.dwVideoPortId)
            .field("dwVPFlags", &self.dwVPFlags)
            .field("dwOriginOffset", &self.dwOriginOffset)
            .field("dwHeight", &self.dwHeight)
            .field("dwVBIHeight", &self.dwVBIHeight)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DDVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwVideoPortId == other.dwVideoPortId && self.dwVPFlags == other.dwVPFlags && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3
    }
}
impl ::std::cmp::Eq for DDVIDEOPORTDATA {}
unsafe impl ::windows::runtime::Abi for DDVIDEOPORTDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DDVIDEOPORTNOTIFY {
    pub ApproximateTimeStamp: i64,
    pub lField: i32,
    pub dwSurfaceIndex: u32,
    pub lDone: i32,
}
impl DDVIDEOPORTNOTIFY {}
impl ::std::default::Default for DDVIDEOPORTNOTIFY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDVIDEOPORTNOTIFY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVIDEOPORTNOTIFY").field("ApproximateTimeStamp", &self.ApproximateTimeStamp).field("lField", &self.lField).field("dwSurfaceIndex", &self.dwSurfaceIndex).field("lDone", &self.lDone).finish()
    }
}
impl ::std::cmp::PartialEq for DDVIDEOPORTNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.ApproximateTimeStamp == other.ApproximateTimeStamp && self.lField == other.lField && self.dwSurfaceIndex == other.dwSurfaceIndex && self.lDone == other.lDone
    }
}
impl ::std::cmp::Eq for DDVIDEOPORTNOTIFY {}
unsafe impl ::windows::runtime::Abi for DDVIDEOPORTNOTIFY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
pub struct DDVIDEOPORTSTATUS {
    pub dwSize: u32,
    pub bInUse: super::super::Foundation::BOOL,
    pub dwFlags: u32,
    pub dwReserved1: u32,
    pub VideoPortType: DDVIDEOPORTCONNECT,
    pub dwReserved2: usize,
    pub dwReserved3: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl DDVIDEOPORTSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DDVIDEOPORTSTATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DDVIDEOPORTSTATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVIDEOPORTSTATUS")
            .field("dwSize", &self.dwSize)
            .field("bInUse", &self.bInUse)
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved1", &self.dwReserved1)
            .field("VideoPortType", &self.VideoPortType)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DDVIDEOPORTSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bInUse == other.bInUse && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.VideoPortType == other.VideoPortType && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DDVIDEOPORTSTATUS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DDVIDEOPORTSTATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPBCAPS_DESTINATION: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPBCAPS_SOURCE: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPB_OVERLAY: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPB_TYPE: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPB_VIDEOPORT: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_AUTOFLIP: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_COLORCONTROL: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_HARDWAREDEINTERLACE: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_INTERLACED: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_NONINTERLACED: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_OVERSAMPLEDVBI: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_READBACKFIELD: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_READBACKLINE: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_SHAREABLE: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_SKIPEVENFIELDS: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_SKIPODDFIELDS: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_SYNCMASTER: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_SYSTEMMEMORY: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_VBIANDVIDEOINDEPENDENT: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCAPS_VBISURFACE: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCONNECT_DISCARDSVREFDATA: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCONNECT_DOUBLECLOCK: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCONNECT_HALFLINE: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCONNECT_INTERLACED: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCONNECT_INVERTPOLARITY: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCONNECT_SHAREEVEN: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCONNECT_SHAREODD: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCONNECT_VACT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCREATE_VBIONLY: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPCREATE_VIDEOONLY: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_ALIGN: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_AUTOFLIP: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_CAPS: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_FILTERQUALITY: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_FX: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_HEIGHT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_ID: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_PREFERREDAUTOFLIP: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPD_WIDTH: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFLIP_VBI: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFLIP_VIDEO: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFORMAT_VBI: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFORMAT_VIDEO: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_CROPTOPDATA: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_CROPX: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_CROPY: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_IGNOREVBIXCROP: i32 = 262144i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_INTERLEAVE: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_MIRRORLEFTRIGHT: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_MIRRORUPDOWN: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESHRINKX: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESHRINKXB: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESHRINKXS: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESHRINKY: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESHRINKYB: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESHRINKYS: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESTRETCHX: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESTRETCHXN: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESTRETCHY: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_PRESTRETCHYN: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_VBICONVERT: i32 = 65536i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_VBINOINTERLEAVE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPFX_VBINOSCALE: i32 = 131072i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPSQ_NOSIGNAL: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPSQ_SIGNALOK: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPSTATUS_VBIONLY: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPSTATUS_VIDEOONLY: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPTARGET_VBI: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPTARGET_VIDEO: i32 = 1i32;
pub const DDVPTYPE_BROOKTREE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(324183392, 55905, 4559, [155, 6, 0, 160, 201, 3, 163, 184]);
pub const DDVPTYPE_CCIR656: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4238550688, 55904, 4559, [155, 6, 0, 160, 201, 3, 163, 184]);
pub const DDVPTYPE_E_HREFH_VREFH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1425250688, 55904, 4559, [155, 6, 0, 160, 201, 3, 163, 184]);
pub const DDVPTYPE_E_HREFH_VREFL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2457350688, 55904, 4559, [155, 6, 0, 160, 201, 3, 163, 184]);
pub const DDVPTYPE_E_HREFL_VREFH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2692350688, 55904, 4559, [155, 6, 0, 160, 201, 3, 163, 184]);
pub const DDVPTYPE_E_HREFL_VREFL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3768350688, 55904, 4559, [155, 6, 0, 160, 201, 3, 163, 184]);
pub const DDVPTYPE_PHILIPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(858583392, 55905, 4559, [155, 6, 0, 160, 201, 3, 163, 184]);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPWAIT_BEGIN: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPWAIT_END: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVPWAIT_LINE: i32 = 3i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_AUTOFLIP: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_CONVERT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_CROP: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_HARDWAREDEINTERLACE: i32 = 32768i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_IGNOREVBIXCROP: i32 = 8192i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_INTERLEAVE: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_MIRRORLEFTRIGHT: i32 = 16i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_MIRRORUPDOWN: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_OVERRIDEBOBWEAVE: i32 = 4096i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_PRESCALE: i32 = 64i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_SKIPEVENFIELDS: i32 = 128i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_SKIPODDFIELDS: i32 = 256i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_SYNCMASTER: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_VBICONVERT: i32 = 1024i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_VBINOINTERLEAVE: i32 = 16384i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDVP_VBINOSCALE: i32 = 2048i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDWAITVB_BLOCKBEGIN: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDWAITVB_BLOCKBEGINEVENT: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDWAITVB_BLOCKEND: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DDWAITVB_I_TESTVB: i32 = -2147483642i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_UI_DisplayDevices")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_UI_DisplayDevices`*"]
pub struct DD_DESTROYDDLOCALDATA {
    pub dwFlags: u32,
    pub pDDLcl: *mut super::super::UI::DisplayDevices::DD_DIRECTDRAW_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl DD_DESTROYDDLOCALDATA {}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl ::std::default::Default for DD_DESTROYDDLOCALDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl ::std::fmt::Debug for DD_DESTROYDDLOCALDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_DESTROYDDLOCALDATA").field("dwFlags", &self.dwFlags).field("pDDLcl", &self.pDDLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl ::std::cmp::PartialEq for DD_DESTROYDDLOCALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pDDLcl == other.pDDLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl ::std::cmp::Eq for DD_DESTROYDDLOCALDATA {}
#[cfg(feature = "Win32_UI_DisplayDevices")]
unsafe impl ::windows::runtime::Abi for DD_DESTROYDDLOCALDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_UI_DisplayDevices`*"]
pub struct DD_DRVSETCOLORKEYDATA {
    pub lpDDSurface: *mut super::super::UI::DisplayDevices::DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ckNew: DDCOLORKEY,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetColorKey: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl DD_DRVSETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DD_DRVSETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DD_DRVSETCOLORKEYDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_DRVSETCOLORKEYDATA").field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).field("SetColorKey", &self.SetColorKey).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DD_DRVSETCOLORKEYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ckNew == other.ckNew && self.ddRVal == other.ddRVal && self.SetColorKey == other.SetColorKey
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DD_DRVSETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DD_DRVSETCOLORKEYDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl ::std::clone::Clone for DD_HALINFO_V4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_DisplayDevices")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_UI_DisplayDevices`*"]
pub struct DD_HALINFO_V4 {
    pub dwSize: u32,
    pub vmiData: super::super::UI::DisplayDevices::VIDEOMEMORYINFO,
    pub ddCaps: DDNTCORECAPS,
    pub GetDriverInfo: ::std::option::Option<super::super::UI::DisplayDevices::PDD_GETDRIVERINFO>,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl DD_HALINFO_V4 {}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl ::std::default::Default for DD_HALINFO_V4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl ::std::cmp::PartialEq for DD_HALINFO_V4 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_UI_DisplayDevices")]
impl ::std::cmp::Eq for DD_HALINFO_V4 {}
#[cfg(feature = "Win32_UI_DisplayDevices")]
unsafe impl ::windows::runtime::Abi for DD_HALINFO_V4 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DD_HAL_VERSION: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DD_MORECAPS {
    pub dwSize: u32,
    pub dwAlphaCaps: u32,
    pub dwSVBAlphaCaps: u32,
    pub dwVSBAlphaCaps: u32,
    pub dwSSBAlphaCaps: u32,
    pub dwFilterCaps: u32,
    pub dwSVBFilterCaps: u32,
    pub dwVSBFilterCaps: u32,
    pub dwSSBFilterCaps: u32,
}
impl DD_MORECAPS {}
impl ::std::default::Default for DD_MORECAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_MORECAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_MORECAPS")
            .field("dwSize", &self.dwSize)
            .field("dwAlphaCaps", &self.dwAlphaCaps)
            .field("dwSVBAlphaCaps", &self.dwSVBAlphaCaps)
            .field("dwVSBAlphaCaps", &self.dwVSBAlphaCaps)
            .field("dwSSBAlphaCaps", &self.dwSSBAlphaCaps)
            .field("dwFilterCaps", &self.dwFilterCaps)
            .field("dwSVBFilterCaps", &self.dwSVBFilterCaps)
            .field("dwVSBFilterCaps", &self.dwVSBFilterCaps)
            .field("dwSSBFilterCaps", &self.dwSSBFilterCaps)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DD_MORECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwAlphaCaps == other.dwAlphaCaps && self.dwSVBAlphaCaps == other.dwSVBAlphaCaps && self.dwVSBAlphaCaps == other.dwVSBAlphaCaps && self.dwSSBAlphaCaps == other.dwSSBAlphaCaps && self.dwFilterCaps == other.dwFilterCaps && self.dwSVBFilterCaps == other.dwSVBFilterCaps && self.dwVSBFilterCaps == other.dwVSBFilterCaps && self.dwSSBFilterCaps == other.dwSSBFilterCaps
    }
}
impl ::std::cmp::Eq for DD_MORECAPS {}
unsafe impl ::windows::runtime::Abi for DD_MORECAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DD_RUNTIME_VERSION: i32 = 2306i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_UI_DisplayDevices`*"]
pub struct DD_SETCLIPLISTDATA {
    pub lpDD: *mut super::super::UI::DisplayDevices::DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut super::super::UI::DisplayDevices::DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetClipList: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl DD_SETCLIPLISTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl ::std::default::Default for DD_SETCLIPLISTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl ::std::fmt::Debug for DD_SETCLIPLISTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SETCLIPLISTDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("SetClipList", &self.SetClipList).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::PartialEq for DD_SETCLIPLISTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.SetClipList == other.SetClipList
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
impl ::std::cmp::Eq for DD_SETCLIPLISTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
unsafe impl ::windows::runtime::Abi for DD_SETCLIPLISTDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DD_VERSION: i32 = 512i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DELETED_LASTONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DELETED_NOTFOUND: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DELETED_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DIRECTDRAW_VERSION: u32 = 1792u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DXAPI_HALVERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
pub struct DXAPI_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut ::std::ffi::c_void,
    pub InterfaceReference: *mut ::std::ffi::c_void,
    pub InterfaceDereference: *mut ::std::ffi::c_void,
    pub DxGetIrqInfo: ::std::option::Option<PDX_GETIRQINFO>,
    pub DxEnableIrq: ::std::option::Option<PDX_ENABLEIRQ>,
    pub DxSkipNextField: ::std::option::Option<PDX_SKIPNEXTFIELD>,
    pub DxBobNextField: ::std::option::Option<PDX_BOBNEXTFIELD>,
    pub DxSetState: ::std::option::Option<PDX_SETSTATE>,
    pub DxLock: ::std::option::Option<PDX_LOCK>,
    pub DxFlipOverlay: ::std::option::Option<PDX_FLIPOVERLAY>,
    pub DxFlipVideoPort: ::std::option::Option<PDX_FLIPVIDEOPORT>,
    pub DxGetPolarity: ::std::option::Option<PDX_GETPOLARITY>,
    pub DxGetCurrentAutoflip: ::std::option::Option<PDX_GETCURRENTAUTOFLIP>,
    pub DxGetPreviousAutoflip: ::std::option::Option<PDX_GETPREVIOUSAUTOFLIP>,
    pub DxTransfer: ::std::option::Option<PDX_TRANSFER>,
    pub DxGetTransferStatus: ::std::option::Option<PDX_GETTRANSFERSTATUS>,
}
#[cfg(feature = "Win32_Foundation")]
impl DXAPI_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DXAPI_INTERFACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DXAPI_INTERFACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXAPI_INTERFACE").field("Size", &self.Size).field("Version", &self.Version).field("Context", &self.Context).field("InterfaceReference", &self.InterfaceReference).field("InterfaceDereference", &self.InterfaceDereference).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DXAPI_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.Context == other.Context
            && self.InterfaceReference == other.InterfaceReference
            && self.InterfaceDereference == other.InterfaceDereference
            && self.DxGetIrqInfo.map(|f| f as usize) == other.DxGetIrqInfo.map(|f| f as usize)
            && self.DxEnableIrq.map(|f| f as usize) == other.DxEnableIrq.map(|f| f as usize)
            && self.DxSkipNextField.map(|f| f as usize) == other.DxSkipNextField.map(|f| f as usize)
            && self.DxBobNextField.map(|f| f as usize) == other.DxBobNextField.map(|f| f as usize)
            && self.DxSetState.map(|f| f as usize) == other.DxSetState.map(|f| f as usize)
            && self.DxLock.map(|f| f as usize) == other.DxLock.map(|f| f as usize)
            && self.DxFlipOverlay.map(|f| f as usize) == other.DxFlipOverlay.map(|f| f as usize)
            && self.DxFlipVideoPort.map(|f| f as usize) == other.DxFlipVideoPort.map(|f| f as usize)
            && self.DxGetPolarity.map(|f| f as usize) == other.DxGetPolarity.map(|f| f as usize)
            && self.DxGetCurrentAutoflip.map(|f| f as usize) == other.DxGetCurrentAutoflip.map(|f| f as usize)
            && self.DxGetPreviousAutoflip.map(|f| f as usize) == other.DxGetPreviousAutoflip.map(|f| f as usize)
            && self.DxTransfer.map(|f| f as usize) == other.DxTransfer.map(|f| f as usize)
            && self.DxGetTransferStatus.map(|f| f as usize) == other.DxGetTransferStatus.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DXAPI_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DXAPI_INTERFACE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DXERR_GENERIC: u32 = 2147500037u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DXERR_OUTOFCAPS: u32 = 2289434984u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DXERR_UNSUPPORTED: u32 = 2147500033u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct DX_IRQDATA {
    pub dwIrqFlags: u32,
}
impl DX_IRQDATA {}
impl ::std::default::Default for DX_IRQDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DX_IRQDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DX_IRQDATA").field("dwIrqFlags", &self.dwIrqFlags).finish()
    }
}
impl ::std::cmp::PartialEq for DX_IRQDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwIrqFlags == other.dwIrqFlags
    }
}
impl ::std::cmp::Eq for DX_IRQDATA {}
unsafe impl ::windows::runtime::Abi for DX_IRQDATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const DX_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[inline]
pub unsafe fn DirectDrawCreate<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(lpguid: *mut ::windows::runtime::GUID, lplpdd: *mut ::std::option::Option<IDirectDraw>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectDrawCreate(lpguid: *mut ::windows::runtime::GUID, lplpdd: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectDrawCreate(::std::mem::transmute(lpguid), ::std::mem::transmute(lplpdd), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[inline]
pub unsafe fn DirectDrawCreateClipper<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(dwflags: u32, lplpddclipper: *mut ::std::option::Option<IDirectDrawClipper>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectDrawCreateClipper(dwflags: u32, lplpddclipper: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectDrawCreateClipper(::std::mem::transmute(dwflags), ::std::mem::transmute(lplpddclipper), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[inline]
pub unsafe fn DirectDrawCreateEx<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(lpguid: *mut ::windows::runtime::GUID, lplpdd: *mut *mut ::std::ffi::c_void, iid: *const ::windows::runtime::GUID, punkouter: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectDrawCreateEx(lpguid: *mut ::windows::runtime::GUID, lplpdd: *mut *mut ::std::ffi::c_void, iid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectDrawCreateEx(::std::mem::transmute(lpguid), ::std::mem::transmute(lplpdd), ::std::mem::transmute(iid), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn DirectDrawEnumerateA(lpcallback: ::std::option::Option<LPDDENUMCALLBACKA>, lpcontext: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectDrawEnumerateA(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectDrawEnumerateA(::std::mem::transmute(lpcallback), ::std::mem::transmute(lpcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn DirectDrawEnumerateExA(lpcallback: ::std::option::Option<LPDDENUMCALLBACKEXA>, lpcontext: *mut ::std::ffi::c_void, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectDrawEnumerateExA(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::std::ffi::c_void, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        DirectDrawEnumerateExA(::std::mem::transmute(lpcallback), ::std::mem::transmute(lpcontext), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn DirectDrawEnumerateExW(lpcallback: ::std::option::Option<LPDDENUMCALLBACKEXW>, lpcontext: *mut ::std::ffi::c_void, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectDrawEnumerateExW(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::std::ffi::c_void, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        DirectDrawEnumerateExW(::std::mem::transmute(lpcallback), ::std::mem::transmute(lpcontext), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn DirectDrawEnumerateW(lpcallback: ::std::option::Option<LPDDENUMCALLBACKW>, lpcontext: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectDrawEnumerateW(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectDrawEnumerateW(::std::mem::transmute(lpcallback), ::std::mem::transmute(lpcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GUID_ColorControlCallbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023782594, 18919, 4560, [136, 157, 0, 170, 0, 187, 183, 106]);
pub const GUID_D3DCallbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2079353232, 34708, 4560, [145, 57, 8, 0, 54, 210, 239, 2]);
pub const GUID_D3DCallbacks2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(195396833, 28854, 4560, [136, 157, 0, 170, 0, 187, 183, 106]);
pub const GUID_D3DCallbacks3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3723760176, 60426, 4560, [169, 182, 0, 170, 0, 192, 153, 62]);
pub const GUID_D3DCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2079353233, 34708, 4560, [145, 57, 8, 0, 54, 210, 239, 2]);
pub const GUID_D3DExtendedCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2112102272, 40339, 4560, [137, 171, 0, 160, 201, 5, 65, 41]);
pub const GUID_D3DParseUnknownCommandCallback: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(772079520, 39140, 4561, [140, 225, 0, 160, 201, 6, 41, 168]);
pub const GUID_DDMoreCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2282467120, 45104, 4560, [142, 167, 0, 96, 151, 151, 234, 91]);
pub const GUID_DDMoreSurfaceCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(998900838, 62057, 4561, [136, 11, 0, 192, 79, 217, 48, 197]);
pub const GUID_DDStereoMode: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4163376796, 43240, 4562, [161, 242, 0, 160, 201, 131, 234, 246]);
pub const GUID_DxApi: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2323234544, 47381, 4560, [145, 68, 8, 0, 54, 210, 239, 2]);
pub const GUID_GetHeapAlignment: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1121988374, 31553, 4562, [139, 255, 0, 160, 201, 131, 234, 246]);
pub const GUID_KernelCallbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2156279808, 27398, 4560, [155, 6, 0, 160, 201, 3, 163, 184]);
pub const GUID_KernelCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4289361216, 31400, 4560, [155, 6, 0, 160, 201, 3, 163, 184]);
pub const GUID_Miscellaneous2Callbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1080766208, 15962, 4561, [182, 64, 0, 170, 0, 161, 249, 106]);
pub const GUID_MiscellaneousCallbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023782592, 18919, 4560, [136, 157, 0, 170, 0, 187, 183, 106]);
pub const GUID_MotionCompCallbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2970757952, 23973, 4561, [143, 207, 0, 192, 79, 194, 155, 78]);
pub const GUID_NTCallbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1877601502, 57225, 4561, [157, 176, 0, 96, 8, 39, 113, 186]);
pub const GUID_NTPrivateDriverCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4208028195, 31590, 4562, [131, 215, 0, 192, 79, 124, 229, 140]);
pub const GUID_NonLocalVidMemCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2261056128, 36228, 4560, [148, 232, 0, 192, 79, 195, 65, 55]);
pub const GUID_OptSurfaceKmodeInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3764159602, 20948, 4561, [140, 206, 0, 160, 201, 6, 41, 168]);
pub const GUID_OptSurfaceUmodeInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2641963012, 24488, 4561, [140, 208, 0, 160, 201, 6, 41, 168]);
pub const GUID_UpdateNonLocalHeap: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1121988375, 31553, 4562, [139, 255, 0, 160, 201, 131, 234, 246]);
pub const GUID_UserModeDriverInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4038125794, 24471, 4561, [140, 208, 0, 160, 201, 6, 41, 168]);
pub const GUID_UserModeDriverPassword: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2549637558, 24737, 4561, [140, 208, 0, 160, 201, 6, 41, 168]);
pub const GUID_VPE2Callbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1384653127, 11591, 18074, [160, 209, 3, 69, 88, 144, 246, 200]);
pub const GUID_VideoPortCallbacks: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023782593, 18919, 4560, [136, 157, 0, 170, 0, 187, 183, 106]);
pub const GUID_VideoPortCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023782595, 18919, 4560, [136, 157, 0, 170, 0, 187, 183, 106]);
pub const GUID_ZPixelFormats: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2475071616, 14031, 4561, [155, 27, 0, 170, 0, 187, 184, 174]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct HEAPALIAS {
    pub fpVidMem: usize,
    pub lpAlias: *mut ::std::ffi::c_void,
    pub dwAliasSize: u32,
}
impl HEAPALIAS {}
impl ::std::default::Default for HEAPALIAS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HEAPALIAS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HEAPALIAS").field("fpVidMem", &self.fpVidMem).field("lpAlias", &self.lpAlias).field("dwAliasSize", &self.dwAliasSize).finish()
    }
}
impl ::std::cmp::PartialEq for HEAPALIAS {
    fn eq(&self, other: &Self) -> bool {
        self.fpVidMem == other.fpVidMem && self.lpAlias == other.lpAlias && self.dwAliasSize == other.dwAliasSize
    }
}
impl ::std::cmp::Eq for HEAPALIAS {}
unsafe impl ::windows::runtime::Abi for HEAPALIAS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct HEAPALIASINFO {
    pub dwRefCnt: u32,
    pub dwFlags: u32,
    pub dwNumHeaps: u32,
    pub lpAliases: *mut HEAPALIAS,
}
impl HEAPALIASINFO {}
impl ::std::default::Default for HEAPALIASINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HEAPALIASINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HEAPALIASINFO").field("dwRefCnt", &self.dwRefCnt).field("dwFlags", &self.dwFlags).field("dwNumHeaps", &self.dwNumHeaps).field("lpAliases", &self.lpAliases).finish()
    }
}
impl ::std::cmp::PartialEq for HEAPALIASINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwRefCnt == other.dwRefCnt && self.dwFlags == other.dwFlags && self.dwNumHeaps == other.dwNumHeaps && self.lpAliases == other.lpAliases
    }
}
impl ::std::cmp::Eq for HEAPALIASINFO {}
unsafe impl ::windows::runtime::Abi for HEAPALIASINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const HEAPALIASINFO_MAPPEDDUMMY: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const HEAPALIASINFO_MAPPEDREAL: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDDVideoPortContainer(::windows::runtime::IUnknown);
impl IDDVideoPortContainer {
    #[cfg(feature = "Win32_UI_DisplayDevices")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_UI_DisplayDevices`*"]
    pub unsafe fn CreateVideoPort<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut super::super::UI::DisplayDevices::DDVIDEOPORTDESC, param2: *mut ::std::option::Option<IDirectDrawVideoPort>, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_DisplayDevices")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_UI_DisplayDevices`*"]
    pub unsafe fn EnumVideoPorts(&self, param0: u32, param1: *mut super::super::UI::DisplayDevices::DDVIDEOPORTCAPS, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMVIDEOCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetVideoPortConnectInfo(&self, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(pcinfo), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn QueryVideoPortStatus(&self, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDDVideoPortContainer {
    type Vtable = IDDVideoPortContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1813260128, 42803, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDVideoPortContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_DisplayDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut super::super::UI::DisplayDevices::DDVIDEOPORTDESC, param2: *mut ::windows::runtime::RawPtr, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_DisplayDevices"))] usize,
    #[cfg(feature = "Win32_UI_DisplayDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut super::super::UI::DisplayDevices::DDVIDEOPORTCAPS, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_DisplayDevices"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDDVideoPortContainerVtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDraw(::windows::runtime::IUnknown);
impl IDirectDraw {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn CreateClipper<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut ::std::option::Option<IDirectDrawClipper>, param2: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreatePalette<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::std::option::Option<IDirectDrawPalette>, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn CreateSurface<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: *mut DDSURFACEDESC, param1: *mut ::std::option::Option<IDirectDrawSurface>, param2: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DuplicateSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: Param0) -> ::windows::runtime::Result<IDirectDrawSurface> {
        let mut result__: <IDirectDrawSurface as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), param0.into_param().abi(), &mut result__).from_abi::<IDirectDrawSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMMODESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMSURFACESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn FlipToGDISurface(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetGDISurface(&self) -> ::windows::runtime::Result<IDirectDrawSurface> {
        let mut result__: <IDirectDrawSurface as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetScanLine(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetVerticalBlankStatus(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize(&self, param0: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn RestoreDisplayMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn WaitForVerticalBlank<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDraw {
    type Vtable = IDirectDraw_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1813306240, 42803, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::runtime::RawPtr, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC, param1: *mut ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32, param1: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDraw2(::windows::runtime::IUnknown);
impl IDirectDraw2 {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn CreateClipper<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut ::std::option::Option<IDirectDrawClipper>, param2: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreatePalette<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::std::option::Option<IDirectDrawPalette>, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn CreateSurface<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: *mut DDSURFACEDESC, param1: *mut ::std::option::Option<IDirectDrawSurface>, param2: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DuplicateSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: Param0) -> ::windows::runtime::Result<IDirectDrawSurface> {
        let mut result__: <IDirectDrawSurface as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), param0.into_param().abi(), &mut result__).from_abi::<IDirectDrawSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMMODESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMSURFACESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn FlipToGDISurface(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetGDISurface(&self) -> ::windows::runtime::Result<IDirectDrawSurface> {
        let mut result__: <IDirectDrawSurface as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetScanLine(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetVerticalBlankStatus(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize(&self, param0: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn RestoreDisplayMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn WaitForVerticalBlank<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDraw2 {
    type Vtable = IDirectDraw2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3014063072, 11075, 4559, [162, 222, 0, 170, 0, 185, 51, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::runtime::RawPtr, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC, param1: *mut ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32, param1: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDraw2Vtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDraw4(::windows::runtime::IUnknown);
impl IDirectDraw4 {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn CreateClipper<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut ::std::option::Option<IDirectDrawClipper>, param2: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreatePalette<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::std::option::Option<IDirectDrawPalette>, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn CreateSurface<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: *mut DDSURFACEDESC2, param1: *mut ::std::option::Option<IDirectDrawSurface4>, param2: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DuplicateSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface4>>(&self, param0: Param0) -> ::windows::runtime::Result<IDirectDrawSurface4> {
        let mut result__: <IDirectDrawSurface4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), param0.into_param().abi(), &mut result__).from_abi::<IDirectDrawSurface4>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMMODESCALLBACK2>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMSURFACESCALLBACK2>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn FlipToGDISurface(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetGDISurface(&self) -> ::windows::runtime::Result<IDirectDrawSurface4> {
        let mut result__: <IDirectDrawSurface4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawSurface4>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetScanLine(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetVerticalBlankStatus(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize(&self, param0: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn RestoreDisplayMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn WaitForVerticalBlank<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetSurfaceFromDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::Gdi::HDC>>(&self, param0: Param0) -> ::windows::runtime::Result<IDirectDrawSurface4> {
        let mut result__: <IDirectDrawSurface4 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), param0.into_param().abi(), &mut result__).from_abi::<IDirectDrawSurface4>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn RestoreAllSurfaces(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn TestCooperativeLevel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDraw4 {
    type Vtable = IDirectDraw4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2623099034, 14781, 4561, [140, 74, 0, 192, 79, 217, 48, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::runtime::RawPtr, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32, param1: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::Gdi::HDC, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDraw4Vtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDraw7(::windows::runtime::IUnknown);
impl IDirectDraw7 {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn CreateClipper<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut ::std::option::Option<IDirectDrawClipper>, param2: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreatePalette<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::std::option::Option<IDirectDrawPalette>, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn CreateSurface<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, param0: *mut DDSURFACEDESC2, param1: *mut ::std::option::Option<IDirectDrawSurface7>, param2: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DuplicateSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface7>>(&self, param0: Param0) -> ::windows::runtime::Result<IDirectDrawSurface7> {
        let mut result__: <IDirectDrawSurface7 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), param0.into_param().abi(), &mut result__).from_abi::<IDirectDrawSurface7>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMMODESCALLBACK2>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void, param3: ::std::option::Option<LPDDENUMSURFACESCALLBACK7>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn FlipToGDISurface(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetGDISurface(&self) -> ::windows::runtime::Result<IDirectDrawSurface7> {
        let mut result__: <IDirectDrawSurface7 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawSurface7>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetScanLine(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetVerticalBlankStatus(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize(&self, param0: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn RestoreDisplayMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn WaitForVerticalBlank<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetSurfaceFromDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::Gdi::HDC>>(&self, param0: Param0) -> ::windows::runtime::Result<IDirectDrawSurface7> {
        let mut result__: <IDirectDrawSurface7 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), param0.into_param().abi(), &mut result__).from_abi::<IDirectDrawSurface7>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn RestoreAllSurfaces(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn TestCooperativeLevel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn StartModeTest(&self, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EvaluateMode(&self, param0: u32, param1: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDraw7 {
    type Vtable = IDirectDraw7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(367419072, 15260, 4562, [185, 47, 0, 96, 151, 151, 234, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::windows::runtime::RawPtr, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC2, param1: *mut ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void, param3: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32, param1: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::Gdi::HDC, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDraw7Vtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawClipper(::windows::runtime::IUnknown);
impl IDirectDrawClipper {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetClipList(&self, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn GetHWnd(&self, param0: *mut super::super::Foundation::HWND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDraw>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn IsClipListChanged(&self, param0: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn SetClipList(&self, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn SetHWnd<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawClipper {
    type Vtable = IDirectDrawClipper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1813306245, 42803, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawClipper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawClipperVtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawColorControl(::windows::runtime::IUnknown);
impl IDirectDrawColorControl {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawColorControl {
    type Vtable = IDirectDrawColorControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1268715232, 3454, 4560, [155, 6, 0, 160, 201, 3, 163, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawColorControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDCOLORCONTROL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDCOLORCONTROL) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawColorControlVtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawGammaControl(::windows::runtime::IUnknown);
impl IDirectDrawGammaControl {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawGammaControl {
    type Vtable = IDirectDrawGammaControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1774263358, 46187, 4561, [173, 122, 0, 192, 79, 194, 155, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawGammaControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawGammaControlVtbl(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawKernelVtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawPalette(::windows::runtime::IUnknown);
impl IDirectDrawPalette {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetEntries(&self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDraw>>(&self, param0: Param0, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn SetEntries(&self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawPalette {
    type Vtable = IDirectDrawPalette_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1813306244, 42803, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawPalette_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawPaletteVtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawSurface(::windows::runtime::IUnknown);
impl IDirectDrawSurface {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn AddAttachedSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Blt<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltFast<'a, Param2: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: u32, param1: u32, param2: Param2, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi(), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DeleteAttachedSurface<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::std::ffi::c_void, param1: ::std::option::Option<LPDDENUMSURFACESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::std::option::Option<LPDDENUMSURFACESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Flip<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::std::option::Option<IDirectDrawSurface>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetClipper(&self) -> ::windows::runtime::Result<IDirectDrawClipper> {
        let mut result__: <IDirectDrawClipper as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawClipper>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPalette(&self) -> ::windows::runtime::Result<IDirectDrawPalette> {
        let mut result__: <IDirectDrawPalette as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawPalette>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDraw>>(&self, param0: Param0, param1: *mut DDSURFACEDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn IsLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Lock<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn ReleaseDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::Gdi::HDC>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetClipper<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawClipper>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetPalette<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawPalette>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Unlock(&self, param0: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn UpdateOverlay<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayZOrder<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawSurface {
    type Vtable = IDirectDrawSurface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1813306241, 42803, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDBLTFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::mem::ManuallyDrop<DDBLTBATCH>, param1: u32, param2: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: ::windows::runtime::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::ffi::c_void, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: i32, param1: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDOVERLAYFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawSurface2(::windows::runtime::IUnknown);
impl IDirectDrawSurface2 {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn AddAttachedSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface2>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Blt<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface2>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltFast<'a, Param2: ::windows::runtime::IntoParam<'a, IDirectDrawSurface2>>(&self, param0: u32, param1: u32, param2: Param2, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi(), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DeleteAttachedSurface<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface2>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::std::ffi::c_void, param1: ::std::option::Option<LPDDENUMSURFACESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::std::option::Option<LPDDENUMSURFACESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Flip<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface2>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::std::option::Option<IDirectDrawSurface2>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetClipper(&self) -> ::windows::runtime::Result<IDirectDrawClipper> {
        let mut result__: <IDirectDrawClipper as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawClipper>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPalette(&self) -> ::windows::runtime::Result<IDirectDrawPalette> {
        let mut result__: <IDirectDrawPalette as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawPalette>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDraw>>(&self, param0: Param0, param1: *mut DDSURFACEDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn IsLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Lock<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn ReleaseDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::Gdi::HDC>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetClipper<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawClipper>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetPalette<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawPalette>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Unlock(&self, param0: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn UpdateOverlay<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface2>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayZOrder<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface2>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn PageLock(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn PageUnlock(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawSurface2 {
    type Vtable = IDirectDrawSurface2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1468029061, 28396, 4559, [148, 65, 168, 35, 3, 193, 14, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDBLTFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::mem::ManuallyDrop<DDBLTBATCH>, param1: u32, param2: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: ::windows::runtime::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::ffi::c_void, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: i32, param1: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDOVERLAYFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawSurface2Vtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawSurface3(::windows::runtime::IUnknown);
impl IDirectDrawSurface3 {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn AddAttachedSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface3>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Blt<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface3>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltFast<'a, Param2: ::windows::runtime::IntoParam<'a, IDirectDrawSurface3>>(&self, param0: u32, param1: u32, param2: Param2, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi(), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DeleteAttachedSurface<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface3>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::std::ffi::c_void, param1: ::std::option::Option<LPDDENUMSURFACESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::std::option::Option<LPDDENUMSURFACESCALLBACK>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Flip<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface3>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::std::option::Option<IDirectDrawSurface3>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetClipper(&self) -> ::windows::runtime::Result<IDirectDrawClipper> {
        let mut result__: <IDirectDrawClipper as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawClipper>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPalette(&self) -> ::windows::runtime::Result<IDirectDrawPalette> {
        let mut result__: <IDirectDrawPalette as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawPalette>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDraw>>(&self, param0: Param0, param1: *mut DDSURFACEDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn IsLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Lock<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn ReleaseDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::Gdi::HDC>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetClipper<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawClipper>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetPalette<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawPalette>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Unlock(&self, param0: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn UpdateOverlay<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface3>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayZOrder<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface3>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn PageLock(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn PageUnlock(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawSurface3 {
    type Vtable = IDirectDrawSurface3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3657715200, 27058, 4560, [161, 213, 0, 170, 0, 184, 223, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDBLTFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::mem::ManuallyDrop<DDBLTBATCH>, param1: u32, param2: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: ::windows::runtime::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::ffi::c_void, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut DDSURFACEDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: i32, param1: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDOVERLAYFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawSurface3Vtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawSurface4(::windows::runtime::IUnknown);
impl IDirectDrawSurface4 {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn AddAttachedSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface4>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Blt<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface4>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltFast<'a, Param2: ::windows::runtime::IntoParam<'a, IDirectDrawSurface4>>(&self, param0: u32, param1: u32, param2: Param2, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi(), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DeleteAttachedSurface<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface4>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::std::ffi::c_void, param1: ::std::option::Option<LPDDENUMSURFACESCALLBACK2>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::std::option::Option<LPDDENUMSURFACESCALLBACK2>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Flip<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface4>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut ::std::option::Option<IDirectDrawSurface4>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetClipper(&self) -> ::windows::runtime::Result<IDirectDrawClipper> {
        let mut result__: <IDirectDrawClipper as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawClipper>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPalette(&self) -> ::windows::runtime::Result<IDirectDrawPalette> {
        let mut result__: <IDirectDrawPalette as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawPalette>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDraw>>(&self, param0: Param0, param1: *mut DDSURFACEDESC2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn IsLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Lock<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn ReleaseDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::Gdi::HDC>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetClipper<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawClipper>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetPalette<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawPalette>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Unlock(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn UpdateOverlay<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface4>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayZOrder<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface4>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn PageLock(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn PageUnlock(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetPrivateData(&self, param0: *const ::windows::runtime::GUID, param1: *mut ::std::ffi::c_void, param2: u32, param3: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPrivateData(&self, param0: *const ::windows::runtime::GUID, param1: *mut ::std::ffi::c_void, param2: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn FreePrivateData(&self, param0: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetUniquenessValue(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn ChangeUniquenessValue(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawSurface4 {
    type Vtable = IDirectDrawSurface4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(187401776, 44341, 4560, [142, 166, 0, 96, 151, 151, 234, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDBLTFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::mem::ManuallyDrop<DDBLTBATCH>, param1: u32, param2: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: ::windows::runtime::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::ffi::c_void, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS2, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: i32, param1: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDOVERLAYFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *const ::windows::runtime::GUID, param1: *mut ::std::ffi::c_void, param2: u32, param3: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *const ::windows::runtime::GUID, param1: *mut ::std::ffi::c_void, param2: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawSurface4Vtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawSurface7(::windows::runtime::IUnknown);
impl IDirectDrawSurface7 {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn AddAttachedSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface7>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Blt<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface7>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn BltFast<'a, Param2: ::windows::runtime::IntoParam<'a, IDirectDrawSurface7>>(&self, param0: u32, param1: u32, param2: Param2, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), param2.into_param().abi(), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn DeleteAttachedSurface<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface7>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::std::ffi::c_void, param1: ::std::option::Option<LPDDENUMSURFACESCALLBACK7>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::std::option::Option<LPDDENUMSURFACESCALLBACK7>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Flip<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface7>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut ::std::option::Option<IDirectDrawSurface7>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetClipper(&self) -> ::windows::runtime::Result<IDirectDrawClipper> {
        let mut result__: <IDirectDrawClipper as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawClipper>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPalette(&self) -> ::windows::runtime::Result<IDirectDrawPalette> {
        let mut result__: <IDirectDrawPalette as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IDirectDrawPalette>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDraw>>(&self, param0: Param0, param1: *mut DDSURFACEDESC2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn IsLost(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Lock<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn ReleaseDC<'a, Param0: ::windows::runtime::IntoParam<'a, super::Gdi::HDC>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetClipper<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawClipper>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetPalette<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawPalette>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn Unlock(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn UpdateOverlay<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface7>>(&self, param0: *mut super::super::Foundation::RECT, param1: Param1, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn UpdateOverlayZOrder<'a, Param1: ::windows::runtime::IntoParam<'a, IDirectDrawSurface7>>(&self, param0: u32, param1: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), param1.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn PageLock(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn PageUnlock(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetPrivateData(&self, param0: *const ::windows::runtime::GUID, param1: *mut ::std::ffi::c_void, param2: u32, param3: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPrivateData(&self, param0: *const ::windows::runtime::GUID, param1: *mut ::std::ffi::c_void, param2: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn FreePrivateData(&self, param0: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetUniquenessValue(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn ChangeUniquenessValue(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetPriority(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetPriority(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetLOD(&self, param0: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetLOD(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawSurface7 {
    type Vtable = IDirectDrawSurface7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(107436672, 15259, 4562, [185, 47, 0, 96, 151, 151, 234, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDBLTFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::mem::ManuallyDrop<DDBLTBATCH>, param1: u32, param2: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: ::windows::runtime::RawPtr, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::std::ffi::c_void, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut ::std::ffi::c_void, param2: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS2, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSCAPS2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32, param1: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDPIXELFORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: *mut DDSURFACEDESC2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::Gdi::HDC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: i32, param1: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::RECT, param1: ::windows::runtime::RawPtr, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut ::std::mem::ManuallyDrop<DDOVERLAYFX>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *const ::windows::runtime::GUID, param1: *mut ::std::ffi::c_void, param2: u32, param3: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *const ::windows::runtime::GUID, param1: *mut ::std::ffi::c_void, param2: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawSurface7Vtbl(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawSurfaceKernelVtbl(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawSurfaceVtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawVideoPort(::windows::runtime::IUnknown);
impl IDirectDrawVideoPort {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn Flip<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_UI_DisplayDevices")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_UI_DisplayDevices`*"]
    pub unsafe fn GetBandwidthInfo(&self, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut super::super::UI::DisplayDevices::DDVIDEOPORTBANDWIDTH) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetInputFormats(&self, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpnumformats), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetOutputFormats(&self, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(lpnumformats), ::std::mem::transmute(param2), ::std::mem::transmute(param3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetFieldPolarity(&self, param0: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetVideoLine(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn GetVideoSignalStatus(&self, param0: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn SetTargetSurface<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectDrawSurface>>(&self, param0: Param0, param1: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), param0.into_param().abi(), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_UI_DisplayDevices`*"]
    pub unsafe fn StartVideo(&self, param0: *mut super::super::UI::DisplayDevices::DDVIDEOPORTINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn StopVideo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_UI_DisplayDevices`*"]
    pub unsafe fn UpdateVideo(&self, param0: *mut super::super::UI::DisplayDevices::DDVIDEOPORTINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub unsafe fn WaitForSync(&self, param0: u32, param1: u32, param2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawVideoPort {
    type Vtable = IDirectDrawVideoPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3010302944, 11075, 4559, [162, 222, 0, 170, 0, 185, 51, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawVideoPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_DisplayDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut super::super::UI::DisplayDevices::DDVIDEOPORTBANDWIDTH) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_DisplayDevices"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDCOLORCONTROL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDCOLORCONTROL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: ::windows::runtime::RawPtr, param1: u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::UI::DisplayDevices::DDVIDEOPORTINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::UI::DisplayDevices::DDVIDEOPORTINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: u32, param1: u32, param2: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirectDrawVideoPortNotify(::windows::runtime::IUnknown);
impl IDirectDrawVideoPortNotify {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn AcquireNotification(&self, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0), ::std::mem::transmute(param1)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    pub unsafe fn ReleaseNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, param0: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), param0.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawVideoPortNotify {
    type Vtable = IDirectDrawVideoPortNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2790652820, 1417, 20055, [179, 51, 86, 122, 137, 70, 140, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawVideoPortNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawVideoPortNotifyVtbl(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawVideoPortVtbl(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IDirectDrawVtbl(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const IRQINFO_HANDLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const IRQINFO_NOTHANDLED: u32 = 2u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct IUNKNOWN_LIST {
    pub lpLink: *mut IUNKNOWN_LIST,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub lpIUnknown: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl IUNKNOWN_LIST {}
impl ::std::default::Default for IUNKNOWN_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IUNKNOWN_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IUNKNOWN_LIST").field("lpLink", &self.lpLink).field("lpGuid", &self.lpGuid).field("lpIUnknown", &self.lpIUnknown).finish()
    }
}
impl ::std::cmp::PartialEq for IUNKNOWN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.lpGuid == other.lpGuid && self.lpIUnknown == other.lpIUnknown
    }
}
impl ::std::cmp::Eq for IUNKNOWN_LIST {}
unsafe impl ::windows::runtime::Abi for IUNKNOWN_LIST {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type LPCLIPPERCALLBACK = unsafe extern "system" fn(lpddclipper: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, code: u32, lpcontext: *mut ::std::ffi::c_void) -> u32;
pub type LPDD32BITDRIVERINIT = unsafe extern "system" fn(dwcontext: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPDDENUMCALLBACKA = unsafe extern "system" fn(param0: *mut ::windows::runtime::GUID, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR, param3: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDENUMCALLBACKEXA = unsafe extern "system" fn(param0: *mut ::windows::runtime::GUID, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR, param3: *mut ::std::ffi::c_void, param4: super::Gdi::HMONITOR) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDENUMCALLBACKEXW = unsafe extern "system" fn(param0: *mut ::windows::runtime::GUID, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: *mut ::std::ffi::c_void, param4: super::Gdi::HMONITOR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPDDENUMCALLBACKW = unsafe extern "system" fn(param0: *mut ::windows::runtime::GUID, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
pub type LPDDENUMMODESCALLBACK = unsafe extern "system" fn(param0: *mut DDSURFACEDESC, param1: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
pub type LPDDENUMMODESCALLBACK2 = unsafe extern "system" fn(param0: *mut DDSURFACEDESC2, param1: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
pub type LPDDENUMSURFACESCALLBACK = unsafe extern "system" fn(param0: ::windows::runtime::RawPtr, param1: *mut DDSURFACEDESC, param2: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
pub type LPDDENUMSURFACESCALLBACK2 = unsafe extern "system" fn(param0: ::windows::runtime::RawPtr, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
pub type LPDDENUMSURFACESCALLBACK7 = unsafe extern "system" fn(param0: ::windows::runtime::RawPtr, param1: *mut DDSURFACEDESC2, param2: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_UI_DisplayDevices")]
pub type LPDDENUMVIDEOCALLBACK = unsafe extern "system" fn(param0: *mut super::super::UI::DisplayDevices::DDVIDEOPORTCAPS, param1: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
pub type LPDDGAMMACALIBRATORPROC = unsafe extern "system" fn(param0: *mut DDGAMMARAMP, param1: *mut u8) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALCOLORCB_COLORCONTROL = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_COLORCONTROLDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALEXEBUFCB_CANCREATEEXEBUF = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_CANCREATESURFACEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALEXEBUFCB_CREATEEXEBUF = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_CREATESURFACEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALEXEBUFCB_DESTROYEXEBUF = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_DESTROYSURFACEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALEXEBUFCB_LOCKEXEBUF = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_LOCKDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALEXEBUFCB_UNLOCKEXEBUF = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_UNLOCKDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALKERNELCB_SYNCSURFACE = unsafe extern "system" fn(param0: *mut DDHAL_SYNCSURFACEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALKERNELCB_SYNCVIDEOPORT = unsafe extern "system" fn(param0: *mut DDHAL_SYNCVIDEOPORTDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_BEGINFRAME = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_BEGINMOCOMPFRAMEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_CREATE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_CREATEMOCOMPDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_DESTROY = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_DESTROYMOCOMPDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_ENDFRAME = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_ENDMOCOMPFRAMEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_GETCOMPBUFFINFO = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETMOCOMPCOMPBUFFDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_GETFORMATS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETMOCOMPFORMATSDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_GETGUIDS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETMOCOMPGUIDSDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_GETINTERNALINFO = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETINTERNALMOCOMPDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_QUERYSTATUS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_QUERYMOCOMPSTATUSDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALMOCOMPCB_RENDER = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_RENDERMOCOMPDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALPALCB_DESTROYPALETTE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_DESTROYPALETTEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALPALCB_SETENTRIES = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_SETENTRIESDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_ADDATTACHEDSURFACE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_ADDATTACHEDSURFACEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_BLT = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_BLTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_DESTROYSURFACE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_DESTROYSURFACEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_FLIP = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_FLIPDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_GETBLTSTATUS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETBLTSTATUSDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_GETFLIPSTATUS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETFLIPSTATUSDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_LOCK = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_LOCKDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_SETCLIPLIST = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_SETCLIPLISTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_SETCOLORKEY = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_SETCOLORKEYDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_SETOVERLAYPOSITION = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_SETOVERLAYPOSITIONDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_SETPALETTE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_SETPALETTEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_UNLOCK = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_UNLOCKDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALSURFCB_UPDATEOVERLAY = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_UPDATEOVERLAYDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_CANCREATEVIDEOPORT = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_CANCREATEVPORTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_COLORCONTROL = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_VPORTCOLORDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_CREATEVIDEOPORT = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_CREATEVPORTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_DESTROYVPORT = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_DESTROYVPORTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_FLIP = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_FLIPVPORTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_GETBANDWIDTH = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETVPORTBANDWIDTHDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_GETFIELD = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETVPORTFIELDDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_GETFLIPSTATUS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETVPORTFLIPSTATUSDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_GETINPUTFORMATS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETVPORTINPUTFORMATDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_GETLINE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETVPORTLINEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_GETOUTPUTFORMATS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETVPORTOUTPUTFORMATDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_GETSIGNALSTATUS = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETVPORTSIGNALDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_GETVPORTCONNECT = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETVPORTCONNECTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_UPDATE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_UPDATEVPORTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHALVPORTCB_WAITFORSYNC = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_WAITFORVPORTSYNCDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_CANCREATESURFACE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_CANCREATESURFACEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_CREATEPALETTE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_CREATEPALETTEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_CREATESURFACE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_CREATESURFACEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_CREATESURFACEEX = unsafe extern "system" fn(param0: *mut DDHAL_CREATESURFACEEXDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_DESTROYDDLOCAL = unsafe extern "system" fn(param0: *mut super::super::UI::DisplayDevices::DDHAL_DESTROYDDLOCALDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_DESTROYDRIVER = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_DESTROYDRIVERDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_FLIPTOGDISURFACE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_FLIPTOGDISURFACEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_GETAVAILDRIVERMEMORY = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETAVAILDRIVERMEMORYDATA>) -> u32;
pub type LPDDHAL_GETDRIVERINFO = unsafe extern "system" fn(param0: *mut DDHAL_GETDRIVERINFODATA) -> u32;
pub type LPDDHAL_GETDRIVERSTATE = unsafe extern "system" fn(param0: *mut DDHAL_GETDRIVERSTATEDATA) -> u32;
#[cfg(feature = "Win32_Devices_Display")]
pub type LPDDHAL_GETHEAPALIGNMENT = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETHEAPALIGNMENTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_GETSCANLINE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_GETSCANLINEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_SETCOLORKEY = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_DRVSETCOLORKEYDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_SETEXCLUSIVEMODE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_SETEXCLUSIVEMODEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_SETINFO = unsafe extern "system" fn(lpddhalinfo: *mut ::std::mem::ManuallyDrop<DDHALINFO>, reset: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_SETMODE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_SETMODEDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_UPDATENONLOCALHEAP = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DDHAL_UPDATENONLOCALHEAPDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_VIDMEMALLOC = unsafe extern "system" fn(lpdd: *mut DDRAWI_DIRECTDRAW_GBL, heap: i32, dwwidth: u32, dwheight: u32) -> usize;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_VIDMEMFREE = unsafe extern "system" fn(lpdd: *mut DDRAWI_DIRECTDRAW_GBL, heap: i32, fpmem: usize);
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHAL_WAITFORVERTICALBLANK = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<super::super::UI::DisplayDevices::DDHAL_WAITFORVERTICALBLANKDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_DisplayDevices"))]
pub type LPDDHEL_INIT = unsafe extern "system" fn(param0: *mut DDRAWI_DIRECTDRAW_GBL, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDIRECTDRAWENUMERATEEXA = unsafe extern "system" fn(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::std::ffi::c_void, dwflags: u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDIRECTDRAWENUMERATEEXW = unsafe extern "system" fn(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::std::ffi::c_void, dwflags: u32) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MAX_AUTOFLIP_BUFFERS: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MAX_DDDEVICEID_STRING: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MAX_DRIVER_NAME: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MAX_PALETTE_SIZE: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct MDL {
    pub MdlNext: *mut MDL,
    pub MdlSize: i16,
    pub MdlFlags: i16,
    pub Process: *mut MDL_0,
    pub lpMappedSystemVa: *mut u32,
    pub lpStartVa: *mut u32,
    pub ByteCount: u32,
    pub ByteOffset: u32,
}
impl MDL {}
impl ::std::default::Default for MDL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MDL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MDL")
            .field("MdlNext", &self.MdlNext)
            .field("MdlSize", &self.MdlSize)
            .field("MdlFlags", &self.MdlFlags)
            .field("Process", &self.Process)
            .field("lpMappedSystemVa", &self.lpMappedSystemVa)
            .field("lpStartVa", &self.lpStartVa)
            .field("ByteCount", &self.ByteCount)
            .field("ByteOffset", &self.ByteOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MDL {
    fn eq(&self, other: &Self) -> bool {
        self.MdlNext == other.MdlNext && self.MdlSize == other.MdlSize && self.MdlFlags == other.MdlFlags && self.Process == other.Process && self.lpMappedSystemVa == other.lpMappedSystemVa && self.lpStartVa == other.lpStartVa && self.ByteCount == other.ByteCount && self.ByteOffset == other.ByteOffset
    }
}
impl ::std::cmp::Eq for MDL {}
unsafe impl ::windows::runtime::Abi for MDL {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct MDL_0(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_64_BIT_VA: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_ALLOCATED_FIXED_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_ALLOCATED_MUST_SUCCEED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_IO_PAGE_READ: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_IO_SPACE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_LOCK_HELD: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_MAPPED_TO_SYSTEM_VA: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_MAPPING_CAN_FAIL: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_NETWORK_HEADER: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_PAGES_LOCKED: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_PARENT_MAPPED_SYSTEM_VA: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_PARTIAL: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_PARTIAL_HAS_BEEN_MAPPED: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_SCATTER_GATHER_VA: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_SOURCE_IS_NONPAGED_POOL: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const MDL_WRITE_OPERATION: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const OBJECT_ISROOT: i32 = -2147483648i32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
pub type PDD_ALPHABLT = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<super::super::UI::DisplayDevices::DD_BLTDATA>) -> u32;
pub type PDD_DESTROYDRIVER = unsafe extern "system" fn(param0: *mut _DD_DESTROYDRIVERDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
pub type PDD_SETCOLORKEY = unsafe extern "system" fn(param0: *mut DD_DRVSETCOLORKEYDATA) -> u32;
pub type PDD_SETMODE = unsafe extern "system" fn(param0: *mut _DD_SETMODEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_UI_DisplayDevices"))]
pub type PDD_SURFCB_SETCLIPLIST = unsafe extern "system" fn(param0: *mut DD_SETCLIPLISTDATA) -> u32;
pub type PDD_VPORTCB_GETAUTOFLIPSURF = unsafe extern "system" fn(param0: *mut _DD_GETVPORTAUTOFLIPSURFACEDATA) -> u32;
pub type PDX_BOBNEXTFIELD = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDBOBNEXTFIELDINFO, param2: *mut ::std::ffi::c_void) -> u32;
pub type PDX_ENABLEIRQ = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut ::std::mem::ManuallyDrop<DDENABLEIRQINFO>, param2: *mut ::std::ffi::c_void) -> u32;
pub type PDX_FLIPOVERLAY = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDFLIPOVERLAYINFO, param2: *mut ::std::ffi::c_void) -> u32;
pub type PDX_FLIPVIDEOPORT = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDFLIPVIDEOPORTINFO, param2: *mut ::std::ffi::c_void) -> u32;
pub type PDX_GETCURRENTAUTOFLIP = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDGETCURRENTAUTOFLIPININFO, param2: *mut DDGETCURRENTAUTOFLIPOUTINFO) -> u32;
pub type PDX_GETIRQINFO = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut ::std::ffi::c_void, param2: *mut DDGETIRQINFO) -> u32;
pub type PDX_GETPOLARITY = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDGETPOLARITYININFO, param2: *mut DDGETPOLARITYOUTINFO) -> u32;
pub type PDX_GETPREVIOUSAUTOFLIP = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDGETPREVIOUSAUTOFLIPININFO, param2: *mut DDGETPREVIOUSAUTOFLIPOUTINFO) -> u32;
pub type PDX_GETTRANSFERSTATUS = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut ::std::ffi::c_void, param2: *mut DDGETTRANSFERSTATUSOUTINFO) -> u32;
pub type PDX_IRQCALLBACK = unsafe extern "system" fn(pirqdata: *mut DX_IRQDATA);
pub type PDX_LOCK = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDLOCKININFO, param2: *mut DDLOCKOUTINFO) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PDX_SETSTATE = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDSETSTATEININFO, param2: *mut DDSETSTATEOUTINFO) -> u32;
pub type PDX_SKIPNEXTFIELD = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDSKIPNEXTFIELDINFO, param2: *mut ::std::ffi::c_void) -> u32;
pub type PDX_TRANSFER = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut DDTRANSFERININFO, param2: *mut DDTRANSFEROUTINFO) -> u32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const PFINDEX_UNINITIALIZED: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub struct PROCESS_LIST {
    pub lpLink: *mut PROCESS_LIST,
    pub dwProcessId: u32,
    pub dwRefCnt: u32,
    pub dwAlphaDepth: u32,
    pub dwZDepth: u32,
}
impl PROCESS_LIST {}
impl ::std::default::Default for PROCESS_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESS_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_LIST").field("lpLink", &self.lpLink).field("dwProcessId", &self.dwProcessId).field("dwRefCnt", &self.dwRefCnt).field("dwAlphaDepth", &self.dwAlphaDepth).field("dwZDepth", &self.dwZDepth).finish()
    }
}
impl ::std::cmp::PartialEq for PROCESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.dwProcessId == other.dwProcessId && self.dwRefCnt == other.dwRefCnt && self.dwAlphaDepth == other.dwAlphaDepth && self.dwZDepth == other.dwZDepth
    }
}
impl ::std::cmp::Eq for PROCESS_LIST {}
unsafe impl ::windows::runtime::Abi for PROCESS_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const ROP_HAS_PATTERN: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const ROP_HAS_SOURCE: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDMEM {
    pub dwFlags: u32,
    pub fpStart: usize,
    pub Anonymous1: VIDMEM_0,
    pub ddsCaps: DDSCAPS,
    pub ddsCapsAlt: DDSCAPS,
    pub Anonymous2: VIDMEM_1,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl VIDMEM {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::default::Default for VIDMEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::cmp::PartialEq for VIDMEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::cmp::Eq for VIDMEM {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
unsafe impl ::windows::runtime::Abi for VIDMEM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub union VIDMEM_0 {
    pub fpEnd: usize,
    pub dwWidth: u32,
}
impl VIDMEM_0 {}
impl ::std::default::Default for VIDMEM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIDMEM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIDMEM_0 {}
unsafe impl ::windows::runtime::Abi for VIDMEM_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`*"]
pub union VIDMEM_1 {
    pub lpHeap: *mut super::super::Devices::Display::VMEMHEAP,
    pub dwHeight: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl VIDMEM_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::default::Default for VIDMEM_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::cmp::PartialEq for VIDMEM_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::cmp::Eq for VIDMEM_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
unsafe impl ::windows::runtime::Abi for VIDMEM_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
#[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDMEMINFO {
    pub fpPrimary: usize,
    pub dwFlags: u32,
    pub dwDisplayWidth: u32,
    pub dwDisplayHeight: u32,
    pub lDisplayPitch: i32,
    pub ddpfDisplay: DDPIXELFORMAT,
    pub dwOffscreenAlign: u32,
    pub dwOverlayAlign: u32,
    pub dwTextureAlign: u32,
    pub dwZBufferAlign: u32,
    pub dwAlphaAlign: u32,
    pub dwNumHeaps: u32,
    pub pvmList: *mut VIDMEM,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl VIDMEMINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::default::Default for VIDMEMINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::cmp::PartialEq for VIDMEMINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
impl ::std::cmp::Eq for VIDMEMINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation"))]
unsafe impl ::windows::runtime::Abi for VIDMEMINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const VIDMEM_HEAPDISABLED: i32 = 32i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const VIDMEM_ISHEAP: i32 = 4i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const VIDMEM_ISLINEAR: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const VIDMEM_ISNONLOCAL: i32 = 8i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const VIDMEM_ISRECTANGULAR: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const VIDMEM_ISWC: i32 = 16i32;
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _DDFXROP(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _DD_DESTROYDRIVERDATA(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _DD_GETVPORTAUTOFLIPSURFACEDATA(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _DD_SETMODEDATA(pub u8);
#[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
pub const _FACDD: u32 = 2166u32;
