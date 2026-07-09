pub type DWRITE_AUTOMATIC_FONT_AXES = u32;
pub const DWRITE_AUTOMATIC_FONT_AXES_NONE: DWRITE_AUTOMATIC_FONT_AXES = 0;
pub const DWRITE_AUTOMATIC_FONT_AXES_OPTICAL_SIZE: DWRITE_AUTOMATIC_FONT_AXES = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_BITMAP_DATA_BGRA32 {
    pub width: u32,
    pub height: u32,
    pub pixels: *mut u32,
}
impl Default for DWRITE_BITMAP_DATA_BGRA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DWRITE_COLOR_COMPOSITE_CLEAR: DWRITE_COLOR_COMPOSITE_MODE = 0;
pub const DWRITE_COLOR_COMPOSITE_COLOR_BURN: DWRITE_COLOR_COMPOSITE_MODE = 18;
pub const DWRITE_COLOR_COMPOSITE_COLOR_DODGE: DWRITE_COLOR_COMPOSITE_MODE = 17;
pub const DWRITE_COLOR_COMPOSITE_DARKEN: DWRITE_COLOR_COMPOSITE_MODE = 15;
pub const DWRITE_COLOR_COMPOSITE_DEST: DWRITE_COLOR_COMPOSITE_MODE = 2;
pub const DWRITE_COLOR_COMPOSITE_DEST_ATOP: DWRITE_COLOR_COMPOSITE_MODE = 10;
pub const DWRITE_COLOR_COMPOSITE_DEST_IN: DWRITE_COLOR_COMPOSITE_MODE = 6;
pub const DWRITE_COLOR_COMPOSITE_DEST_OUT: DWRITE_COLOR_COMPOSITE_MODE = 8;
pub const DWRITE_COLOR_COMPOSITE_DEST_OVER: DWRITE_COLOR_COMPOSITE_MODE = 4;
pub const DWRITE_COLOR_COMPOSITE_DIFFERENCE: DWRITE_COLOR_COMPOSITE_MODE = 21;
pub const DWRITE_COLOR_COMPOSITE_EXCLUSION: DWRITE_COLOR_COMPOSITE_MODE = 22;
pub const DWRITE_COLOR_COMPOSITE_HARD_LIGHT: DWRITE_COLOR_COMPOSITE_MODE = 19;
pub const DWRITE_COLOR_COMPOSITE_HSL_COLOR: DWRITE_COLOR_COMPOSITE_MODE = 26;
pub const DWRITE_COLOR_COMPOSITE_HSL_HUE: DWRITE_COLOR_COMPOSITE_MODE = 24;
pub const DWRITE_COLOR_COMPOSITE_HSL_LUMINOSITY: DWRITE_COLOR_COMPOSITE_MODE = 27;
pub const DWRITE_COLOR_COMPOSITE_HSL_SATURATION: DWRITE_COLOR_COMPOSITE_MODE = 25;
pub const DWRITE_COLOR_COMPOSITE_LIGHTEN: DWRITE_COLOR_COMPOSITE_MODE = 16;
pub type DWRITE_COLOR_COMPOSITE_MODE = i32;
pub const DWRITE_COLOR_COMPOSITE_MULTIPLY: DWRITE_COLOR_COMPOSITE_MODE = 23;
pub const DWRITE_COLOR_COMPOSITE_OVERLAY: DWRITE_COLOR_COMPOSITE_MODE = 14;
pub const DWRITE_COLOR_COMPOSITE_PLUS: DWRITE_COLOR_COMPOSITE_MODE = 12;
pub const DWRITE_COLOR_COMPOSITE_SCREEN: DWRITE_COLOR_COMPOSITE_MODE = 13;
pub const DWRITE_COLOR_COMPOSITE_SOFT_LIGHT: DWRITE_COLOR_COMPOSITE_MODE = 20;
pub const DWRITE_COLOR_COMPOSITE_SRC: DWRITE_COLOR_COMPOSITE_MODE = 1;
pub const DWRITE_COLOR_COMPOSITE_SRC_ATOP: DWRITE_COLOR_COMPOSITE_MODE = 9;
pub const DWRITE_COLOR_COMPOSITE_SRC_IN: DWRITE_COLOR_COMPOSITE_MODE = 5;
pub const DWRITE_COLOR_COMPOSITE_SRC_OUT: DWRITE_COLOR_COMPOSITE_MODE = 7;
pub const DWRITE_COLOR_COMPOSITE_SRC_OVER: DWRITE_COLOR_COMPOSITE_MODE = 3;
pub const DWRITE_COLOR_COMPOSITE_XOR: DWRITE_COLOR_COMPOSITE_MODE = 11;
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DWRITE_COLOR_GLYPH_RUN1 {
    pub Base: super::dwrite_2::DWRITE_COLOR_GLYPH_RUN,
    pub glyphImageFormat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS,
    pub measuringMode: super::dcommon::DWRITE_MEASURING_MODE,
}
pub type DWRITE_CONTAINER_TYPE = i32;
pub const DWRITE_CONTAINER_TYPE_UNKNOWN: DWRITE_CONTAINER_TYPE = 0;
pub const DWRITE_CONTAINER_TYPE_WOFF: DWRITE_CONTAINER_TYPE = 1;
pub const DWRITE_CONTAINER_TYPE_WOFF2: DWRITE_CONTAINER_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_FILE_FRAGMENT {
    pub fileOffset: u64,
    pub fragmentSize: u64,
}
pub type DWRITE_FONT_AXIS_ATTRIBUTES = u32;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN: DWRITE_FONT_AXIS_ATTRIBUTES = 2;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_NONE: DWRITE_FONT_AXIS_ATTRIBUTES = 0;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE: DWRITE_FONT_AXIS_ATTRIBUTES = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_FONT_AXIS_RANGE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub minValue: f32,
    pub maxValue: f32,
}
pub type DWRITE_FONT_AXIS_TAG = i32;
pub const DWRITE_FONT_AXIS_TAG_ITALIC: DWRITE_FONT_AXIS_TAG = 1818326121;
pub const DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE: DWRITE_FONT_AXIS_TAG = 2054385775;
pub const DWRITE_FONT_AXIS_TAG_SLANT: DWRITE_FONT_AXIS_TAG = 1953393779;
pub const DWRITE_FONT_AXIS_TAG_WEIGHT: DWRITE_FONT_AXIS_TAG = 1952999287;
pub const DWRITE_FONT_AXIS_TAG_WIDTH: DWRITE_FONT_AXIS_TAG = 1752458359;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_FONT_AXIS_VALUE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub value: f32,
}
pub type DWRITE_FONT_FAMILY_MODEL = i32;
pub const DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC: DWRITE_FONT_FAMILY_MODEL = 0;
pub const DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE: DWRITE_FONT_FAMILY_MODEL = 1;
pub type DWRITE_FONT_LINE_GAP_USAGE = i32;
pub const DWRITE_FONT_LINE_GAP_USAGE_DEFAULT: DWRITE_FONT_LINE_GAP_USAGE = 0;
pub const DWRITE_FONT_LINE_GAP_USAGE_DISABLED: DWRITE_FONT_LINE_GAP_USAGE = 1;
pub const DWRITE_FONT_LINE_GAP_USAGE_ENABLED: DWRITE_FONT_LINE_GAP_USAGE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_FONT_PROPERTY {
    pub propertyId: DWRITE_FONT_PROPERTY_ID,
    pub propertyValue: *const u16,
    pub localeName: *const u16,
}
impl Default for DWRITE_FONT_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DWRITE_FONT_PROPERTY_ID = i32;
pub const DWRITE_FONT_PROPERTY_ID_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = 7;
pub const DWRITE_FONT_PROPERTY_ID_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 3;
pub const DWRITE_FONT_PROPERTY_ID_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 1;
pub const DWRITE_FONT_PROPERTY_ID_FULL_NAME: DWRITE_FONT_PROPERTY_ID = 4;
pub const DWRITE_FONT_PROPERTY_ID_NONE: DWRITE_FONT_PROPERTY_ID = 0;
pub const DWRITE_FONT_PROPERTY_ID_POSTSCRIPT_NAME: DWRITE_FONT_PROPERTY_ID = 6;
pub const DWRITE_FONT_PROPERTY_ID_PREFERRED_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 2;
pub const DWRITE_FONT_PROPERTY_ID_SEMANTIC_TAG: DWRITE_FONT_PROPERTY_ID = 9;
pub const DWRITE_FONT_PROPERTY_ID_STRETCH: DWRITE_FONT_PROPERTY_ID = 11;
pub const DWRITE_FONT_PROPERTY_ID_STYLE: DWRITE_FONT_PROPERTY_ID = 12;
pub const DWRITE_FONT_PROPERTY_ID_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = 8;
pub const DWRITE_FONT_PROPERTY_ID_TOTAL: DWRITE_FONT_PROPERTY_ID = 13;
pub const DWRITE_FONT_PROPERTY_ID_TOTAL_RS3: DWRITE_FONT_PROPERTY_ID = 14;
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 13;
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 2;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT: DWRITE_FONT_PROPERTY_ID = 10;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FACE_NAME: DWRITE_FONT_PROPERTY_ID = 3;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 1;
pub const DWRITE_FONT_PROPERTY_ID_WIN32_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = 5;
pub type DWRITE_FONT_SOURCE_TYPE = i32;
pub const DWRITE_FONT_SOURCE_TYPE_APPX_PACKAGE: DWRITE_FONT_SOURCE_TYPE = 3;
pub const DWRITE_FONT_SOURCE_TYPE_PER_MACHINE: DWRITE_FONT_SOURCE_TYPE = 1;
pub const DWRITE_FONT_SOURCE_TYPE_PER_USER: DWRITE_FONT_SOURCE_TYPE = 2;
pub const DWRITE_FONT_SOURCE_TYPE_REMOTE_FONT_PROVIDER: DWRITE_FONT_SOURCE_TYPE = 4;
pub const DWRITE_FONT_SOURCE_TYPE_UNKNOWN: DWRITE_FONT_SOURCE_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_GLYPH_IMAGE_DATA {
    pub imageData: *const core::ffi::c_void,
    pub imageDataSize: u32,
    pub uniqueDataId: u32,
    pub pixelsPerEm: u32,
    pub pixelSize: super::dcommon::D2D_SIZE_U,
    pub horizontalLeftOrigin: super::dcommon::D2D_POINT_2L,
    pub horizontalRightOrigin: super::dcommon::D2D_POINT_2L,
    pub verticalTopOrigin: super::dcommon::D2D_POINT_2L,
    pub verticalBottomOrigin: super::dcommon::D2D_POINT_2L,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
impl Default for DWRITE_GLYPH_IMAGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dwrite")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_METRICS1 {
    pub Base: super::dwrite::DWRITE_LINE_METRICS,
    pub leadingBefore: f32,
    pub leadingAfter: f32,
}
#[repr(C)]
#[cfg(feature = "Win32_dwrite")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_SPACING {
    pub method: super::dwrite::DWRITE_LINE_SPACING_METHOD,
    pub height: f32,
    pub baseline: f32,
    pub leadingBefore: f32,
    pub fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE,
}
pub type DWRITE_LOCALITY = i32;
pub const DWRITE_LOCALITY_LOCAL: DWRITE_LOCALITY = 2;
pub const DWRITE_LOCALITY_PARTIAL: DWRITE_LOCALITY = 1;
pub const DWRITE_LOCALITY_REMOTE: DWRITE_LOCALITY = 0;
pub type DWRITE_PAINT_ATTRIBUTES = u32;
pub const DWRITE_PAINT_ATTRIBUTES_NONE: DWRITE_PAINT_ATTRIBUTES = 0;
pub const DWRITE_PAINT_ATTRIBUTES_USES_PALETTE: DWRITE_PAINT_ATTRIBUTES = 1;
pub const DWRITE_PAINT_ATTRIBUTES_USES_TEXT_COLOR: DWRITE_PAINT_ATTRIBUTES = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_COLOR {
    pub value: super::dwrite_2::DWRITE_COLOR_F,
    pub paletteEntryIndex: u16,
    pub alphaMultiplier: f32,
    pub colorAttributes: DWRITE_PAINT_ATTRIBUTES,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy)]
pub struct DWRITE_PAINT_ELEMENT {
    pub paintType: DWRITE_PAINT_TYPE,
    pub paint: DWRITE_PAINT_ELEMENT_0,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl Default for DWRITE_PAINT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy)]
pub union DWRITE_PAINT_ELEMENT_0 {
    pub layers: DWRITE_PAINT_ELEMENT_0_0,
    pub solidGlyph: DWRITE_PAINT_ELEMENT_0_1,
    pub solid: DWRITE_PAINT_COLOR,
    pub linearGradient: DWRITE_PAINT_ELEMENT_0_2,
    pub radialGradient: DWRITE_PAINT_ELEMENT_0_3,
    pub sweepGradient: DWRITE_PAINT_ELEMENT_0_4,
    pub glyph: DWRITE_PAINT_ELEMENT_0_5,
    pub colorGlyph: DWRITE_PAINT_ELEMENT_0_6,
    pub transform: super::dwrite::DWRITE_MATRIX,
    pub composite: DWRITE_PAINT_ELEMENT_0_7,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl Default for DWRITE_PAINT_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_0 {
    pub childCount: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_1 {
    pub glyphIndex: u32,
    pub color: DWRITE_PAINT_COLOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_2 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_3 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub x0: f32,
    pub y0: f32,
    pub radius0: f32,
    pub x1: f32,
    pub y1: f32,
    pub radius1: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_4 {
    pub extendMode: u32,
    pub gradientStopCount: u32,
    pub centerX: f32,
    pub centerY: f32,
    pub startAngle: f32,
    pub endAngle: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_5 {
    pub glyphIndex: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_6 {
    pub glyphIndex: u32,
    pub clipBox: super::dcommon::D2D_RECT_F,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PAINT_ELEMENT_0_7 {
    pub mode: DWRITE_COLOR_COMPOSITE_MODE,
}
pub type DWRITE_PAINT_FEATURE_LEVEL = i32;
pub const DWRITE_PAINT_FEATURE_LEVEL_COLR_V0: DWRITE_PAINT_FEATURE_LEVEL = 1;
pub const DWRITE_PAINT_FEATURE_LEVEL_COLR_V1: DWRITE_PAINT_FEATURE_LEVEL = 2;
pub const DWRITE_PAINT_FEATURE_LEVEL_NONE: DWRITE_PAINT_FEATURE_LEVEL = 0;
pub type DWRITE_PAINT_TYPE = i32;
pub const DWRITE_PAINT_TYPE_COLOR_GLYPH: DWRITE_PAINT_TYPE = 8;
pub const DWRITE_PAINT_TYPE_COMPOSITE: DWRITE_PAINT_TYPE = 10;
pub const DWRITE_PAINT_TYPE_GLYPH: DWRITE_PAINT_TYPE = 7;
pub const DWRITE_PAINT_TYPE_LAYERS: DWRITE_PAINT_TYPE = 1;
pub const DWRITE_PAINT_TYPE_LINEAR_GRADIENT: DWRITE_PAINT_TYPE = 4;
pub const DWRITE_PAINT_TYPE_NONE: DWRITE_PAINT_TYPE = 0;
pub const DWRITE_PAINT_TYPE_RADIAL_GRADIENT: DWRITE_PAINT_TYPE = 5;
pub const DWRITE_PAINT_TYPE_SOLID: DWRITE_PAINT_TYPE = 3;
pub const DWRITE_PAINT_TYPE_SOLID_GLYPH: DWRITE_PAINT_TYPE = 2;
pub const DWRITE_PAINT_TYPE_SWEEP_GRADIENT: DWRITE_PAINT_TYPE = 6;
pub const DWRITE_PAINT_TYPE_TRANSFORM: DWRITE_PAINT_TYPE = 9;
pub type DWRITE_RENDERING_MODE1 = i32;
pub const DWRITE_RENDERING_MODE1_ALIASED: DWRITE_RENDERING_MODE1 = 1;
pub const DWRITE_RENDERING_MODE1_DEFAULT: DWRITE_RENDERING_MODE1 = 0;
pub const DWRITE_RENDERING_MODE1_GDI_CLASSIC: DWRITE_RENDERING_MODE1 = 2;
pub const DWRITE_RENDERING_MODE1_GDI_NATURAL: DWRITE_RENDERING_MODE1 = 3;
pub const DWRITE_RENDERING_MODE1_NATURAL: DWRITE_RENDERING_MODE1 = 4;
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE1 = 5;
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC_DOWNSAMPLED: DWRITE_RENDERING_MODE1 = 7;
pub const DWRITE_RENDERING_MODE1_OUTLINE: DWRITE_RENDERING_MODE1 = 6;
pub const DWRITE_STANDARD_FONT_AXIS_COUNT: u32 = 5;
windows_core::imp::define_interface!(IDWriteAsyncResult, IDWriteAsyncResult_Vtbl, 0xce25f8fd_863b_4d13_9651_c1f88dc73fe2);
windows_core::imp::interface_hierarchy!(IDWriteAsyncResult, windows_core::IUnknown);
impl IDWriteAsyncResult {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetWaitHandle(&self) -> super::winnt::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).GetWaitHandle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetResult(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetWaitHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::winnt::HANDLE,
    #[cfg(not(feature = "Win32_winnt"))]
    GetWaitHandle: usize,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IDWriteAsyncResult_Impl: windows_core::IUnknownImpl {
    fn GetWaitHandle(&self) -> super::winnt::HANDLE;
    fn GetResult(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_winnt")]
impl IDWriteAsyncResult_Vtbl {
    pub const fn new<Identity: IDWriteAsyncResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWaitHandle<Identity: IDWriteAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::winnt::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteAsyncResult_Impl::GetWaitHandle(this)
            }
        }
        unsafe extern "system" fn GetResult<Identity: IDWriteAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteAsyncResult_Impl::GetResult(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWaitHandle: GetWaitHandle::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteAsyncResult as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_winnt")]
impl windows_core::RuntimeName for IDWriteAsyncResult {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::define_interface!(IDWriteBitmapRenderTarget2, IDWriteBitmapRenderTarget2_Vtbl, 0xc553a742_fc01_44da_a66e_b8b9ed6c3995);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl core::ops::Deref for IDWriteBitmapRenderTarget2 {
    type Target = super::dwrite_1::IDWriteBitmapRenderTarget1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget2, windows_core::IUnknown, super::dwrite::IDWriteBitmapRenderTarget, super::dwrite_1::IDWriteBitmapRenderTarget1);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteBitmapRenderTarget2 {
    pub unsafe fn GetBitmapData(&self) -> windows_core::Result<DWRITE_BITMAP_DATA_BGRA32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmapData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteBitmapRenderTarget2_Vtbl {
    pub base__: super::dwrite_1::IDWriteBitmapRenderTarget1_Vtbl,
    pub GetBitmapData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_BITMAP_DATA_BGRA32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_windef"))]
pub trait IDWriteBitmapRenderTarget2_Impl: super::dwrite_1::IDWriteBitmapRenderTarget1_Impl {
    fn GetBitmapData(&self) -> windows_core::Result<DWRITE_BITMAP_DATA_BGRA32>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_windef"))]
impl IDWriteBitmapRenderTarget2_Vtbl {
    pub const fn new<Identity: IDWriteBitmapRenderTarget2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBitmapData<Identity: IDWriteBitmapRenderTarget2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmapdata: *mut DWRITE_BITMAP_DATA_BGRA32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteBitmapRenderTarget2_Impl::GetBitmapData(this) {
                    Ok(ok__) => {
                        bitmapdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::dwrite_1::IDWriteBitmapRenderTarget1_Vtbl::new::<Identity, OFFSET>(), GetBitmapData: GetBitmapData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteBitmapRenderTarget as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteBitmapRenderTarget1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteBitmapRenderTarget2 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::define_interface!(IDWriteBitmapRenderTarget3, IDWriteBitmapRenderTarget3_Vtbl, 0xaeec37db_c337_40f1_8e2a_9a41b167b238);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl core::ops::Deref for IDWriteBitmapRenderTarget3 {
    type Target = IDWriteBitmapRenderTarget2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget3, windows_core::IUnknown, super::dwrite::IDWriteBitmapRenderTarget, super::dwrite_1::IDWriteBitmapRenderTarget1, IDWriteBitmapRenderTarget2);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
impl IDWriteBitmapRenderTarget3 {
    pub unsafe fn GetPaintFeatureLevel(&self) -> DWRITE_PAINT_FEATURE_LEVEL {
        unsafe { (windows_core::Interface::vtable(self).GetPaintFeatureLevel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
    pub unsafe fn DrawPaintGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, textcolor: super::windef::COLORREF, colorpaletteindex: u32, blackboxrect: Option<*mut super::windef::RECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DrawPaintGlyphRun)(windows_core::Interface::as_raw(self), baselineoriginx, baselineoriginy, measuringmode, core::mem::transmute(glyphrun), glyphimageformat, textcolor, colorpaletteindex, blackboxrect.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
    pub unsafe fn DrawGlyphRunWithColorSupport<P4>(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, renderingparams: P4, textcolor: super::windef::COLORREF, colorpaletteindex: u32, blackboxrect: Option<*mut super::windef::RECT>) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::dwrite::IDWriteRenderingParams>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGlyphRunWithColorSupport)(windows_core::Interface::as_raw(self), baselineoriginx, baselineoriginy, measuringmode, core::mem::transmute(glyphrun), renderingparams.param().abi(), textcolor, colorpaletteindex, blackboxrect.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteBitmapRenderTarget3_Vtbl {
    pub base__: IDWriteBitmapRenderTarget2_Vtbl,
    pub GetPaintFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PAINT_FEATURE_LEVEL,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
    pub DrawPaintGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, super::dcommon::DWRITE_MEASURING_MODE, *const super::dwrite::DWRITE_GLYPH_RUN, super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, super::windef::COLORREF, u32, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_windef")))]
    DrawPaintGlyphRun: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
    pub DrawGlyphRunWithColorSupport: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, super::dcommon::DWRITE_MEASURING_MODE, *const super::dwrite::DWRITE_GLYPH_RUN, *mut core::ffi::c_void, super::windef::COLORREF, u32, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_windef")))]
    DrawGlyphRunWithColorSupport: usize,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_windef"))]
pub trait IDWriteBitmapRenderTarget3_Impl: IDWriteBitmapRenderTarget2_Impl {
    fn GetPaintFeatureLevel(&self) -> DWRITE_PAINT_FEATURE_LEVEL;
    fn DrawPaintGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, textcolor: super::windef::COLORREF, colorpaletteindex: u32, blackboxrect: *mut super::windef::RECT) -> windows_core::Result<()>;
    fn DrawGlyphRunWithColorSupport(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, renderingparams: windows_core::Ref<super::dwrite::IDWriteRenderingParams>, textcolor: super::windef::COLORREF, colorpaletteindex: u32, blackboxrect: *mut super::windef::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_windef"))]
impl IDWriteBitmapRenderTarget3_Vtbl {
    pub const fn new<Identity: IDWriteBitmapRenderTarget3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPaintFeatureLevel<Identity: IDWriteBitmapRenderTarget3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_PAINT_FEATURE_LEVEL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget3_Impl::GetPaintFeatureLevel(this)
            }
        }
        unsafe extern "system" fn DrawPaintGlyphRun<Identity: IDWriteBitmapRenderTarget3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, textcolor: super::windef::COLORREF, colorpaletteindex: u32, blackboxrect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget3_Impl::DrawPaintGlyphRun(this, core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphimageformat), core::mem::transmute_copy(&textcolor), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&blackboxrect)).into()
            }
        }
        unsafe extern "system" fn DrawGlyphRunWithColorSupport<Identity: IDWriteBitmapRenderTarget3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, renderingparams: *mut core::ffi::c_void, textcolor: super::windef::COLORREF, colorpaletteindex: u32, blackboxrect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget3_Impl::DrawGlyphRunWithColorSupport(this, core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&renderingparams), core::mem::transmute_copy(&textcolor), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&blackboxrect)).into()
            }
        }
        Self {
            base__: IDWriteBitmapRenderTarget2_Vtbl::new::<Identity, OFFSET>(),
            GetPaintFeatureLevel: GetPaintFeatureLevel::<Identity, OFFSET>,
            DrawPaintGlyphRun: DrawPaintGlyphRun::<Identity, OFFSET>,
            DrawGlyphRunWithColorSupport: DrawGlyphRunWithColorSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget3 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteBitmapRenderTarget as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteBitmapRenderTarget1 as windows_core::Interface>::IID || iid == &<IDWriteBitmapRenderTarget2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteBitmapRenderTarget3 {}
#[cfg(feature = "Win32_dwrite_2")]
windows_core::imp::define_interface!(IDWriteColorGlyphRunEnumerator1, IDWriteColorGlyphRunEnumerator1_Vtbl, 0x7c5f86da_c7a1_4f05_b8e1_55a179fe5a35);
#[cfg(feature = "Win32_dwrite_2")]
impl core::ops::Deref for IDWriteColorGlyphRunEnumerator1 {
    type Target = super::dwrite_2::IDWriteColorGlyphRunEnumerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite_2")]
windows_core::imp::interface_hierarchy!(IDWriteColorGlyphRunEnumerator1, windows_core::IUnknown, super::dwrite_2::IDWriteColorGlyphRunEnumerator);
#[cfg(feature = "Win32_dwrite_2")]
impl IDWriteColorGlyphRunEnumerator1 {
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
    pub unsafe fn GetCurrentRun(&self) -> windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_dwrite_2")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteColorGlyphRunEnumerator1_Vtbl {
    pub base__: super::dwrite_2::IDWriteColorGlyphRunEnumerator_Vtbl,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
    pub GetCurrentRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype")))]
    GetCurrentRun: usize,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
pub trait IDWriteColorGlyphRunEnumerator1_Impl: super::dwrite_2::IDWriteColorGlyphRunEnumerator_Impl {
    fn GetCurrentRun(&self) -> windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN1>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl IDWriteColorGlyphRunEnumerator1_Vtbl {
    pub const fn new<Identity: IDWriteColorGlyphRunEnumerator1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentRun<Identity: IDWriteColorGlyphRunEnumerator1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteColorGlyphRunEnumerator1_Impl::GetCurrentRun(this) {
                    Ok(ok__) => {
                        colorglyphrun.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::dwrite_2::IDWriteColorGlyphRunEnumerator_Vtbl::new::<Identity, OFFSET>(), GetCurrentRun: GetCurrentRun::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteColorGlyphRunEnumerator1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteColorGlyphRunEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for IDWriteColorGlyphRunEnumerator1 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFactory3, IDWriteFactory3_Vtbl, 0x9a1b41c3_d3bb_466a_87fc_fe67556a3b65);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFactory3 {
    type Target = super::dwrite_2::IDWriteFactory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFactory3, windows_core::IUnknown, super::dwrite::IDWriteFactory, super::dwrite_1::IDWriteFactory1, super::dwrite_2::IDWriteFactory2);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFactory3 {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, transform: Option<*const super::dwrite::DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, gridfitmode: super::dwrite_2::DWRITE_GRID_FIT_MODE, antialiasmode: super::dwrite_1::DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<super::dwrite::IDWriteGlyphRunAnalysis> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphrun), transform.unwrap_or(core::mem::zeroed()) as _, renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: super::dwrite_2::DWRITE_GRID_FIT_MODE) -> windows_core::Result<IDWriteRenderingParams3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(windows_core::Interface::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn CreateFontFaceReference<P0>(&self, filepath: P0, lastwritetime: Option<*const super::minwindef::FILETIME>, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFaceReference>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFaceReference)(windows_core::Interface::as_raw(self), filepath.param().abi(), lastwritetime.unwrap_or(core::mem::zeroed()) as _, faceindex, fontsimulations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontFaceReference2<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFaceReference>
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFaceReference2)(windows_core::Interface::as_raw(self), fontfile.param().abi(), faceindex, fontsimulations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSystemFontSet(&self) -> windows_core::Result<IDWriteFontSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontSetBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> windows_core::Result<IDWriteFontCollection1>
    where
        P0: windows_core::Param<IDWriteFontSet>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontCollectionFromFontSet)(windows_core::Interface::as_raw(self), fontset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSystemFontCollection(&self, includedownloadablefonts: bool, fontcollection: *mut Option<IDWriteFontCollection1>, checkforupdates: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), includedownloadablefonts.into(), core::mem::transmute(fontcollection), checkforupdates.into()) }
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> windows_core::Result<IDWriteFontDownloadQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontDownloadQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory3_Vtbl {
    pub base__: super::dwrite_2::IDWriteFactory2_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_MATRIX, DWRITE_RENDERING_MODE1, super::dcommon::DWRITE_MEASURING_MODE, super::dwrite_2::DWRITE_GRID_FIT_MODE, super::dwrite_1::DWRITE_TEXT_ANTIALIAS_MODE, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CreateGlyphRunAnalysis: usize,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, super::dwrite::DWRITE_PIXEL_GEOMETRY, DWRITE_RENDERING_MODE1, super::dwrite_2::DWRITE_GRID_FIT_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub CreateFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::minwindef::FILETIME, u32, super::dwrite::DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    CreateFontFaceReference: usize,
    pub CreateFontFaceReference2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::dwrite::DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontSetBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontCollectionFromFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFontDownloadQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory3_Impl: super::dwrite_2::IDWriteFactory2_Impl {
    fn CreateGlyphRunAnalysis(&self, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, transform: *const super::dwrite::DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, gridfitmode: super::dwrite_2::DWRITE_GRID_FIT_MODE, antialiasmode: super::dwrite_1::DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<super::dwrite::IDWriteGlyphRunAnalysis>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: super::dwrite_2::DWRITE_GRID_FIT_MODE) -> windows_core::Result<IDWriteRenderingParams3>;
    fn CreateFontFaceReference(&self, filepath: &windows_core::PCWSTR, lastwritetime: *const super::minwindef::FILETIME, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFaceReference>;
    fn CreateFontFaceReference2(&self, fontfile: windows_core::Ref<super::dwrite::IDWriteFontFile>, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFaceReference>;
    fn GetSystemFontSet(&self) -> windows_core::Result<IDWriteFontSet>;
    fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder>;
    fn CreateFontCollectionFromFontSet(&self, fontset: windows_core::Ref<IDWriteFontSet>) -> windows_core::Result<IDWriteFontCollection1>;
    fn GetSystemFontCollection(&self, includedownloadablefonts: windows_core::BOOL, fontcollection: windows_core::OutRef<IDWriteFontCollection1>, checkforupdates: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFontDownloadQueue(&self) -> windows_core::Result<IDWriteFontDownloadQueue>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory3_Vtbl {
    pub const fn new<Identity: IDWriteFactory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateGlyphRunAnalysis<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, transform: *const super::dwrite::DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, gridfitmode: super::dwrite_2::DWRITE_GRID_FIT_MODE, antialiasmode: super::dwrite_1::DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory3_Impl::CreateGlyphRunAnalysis(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&gridfitmode), core::mem::transmute_copy(&antialiasmode), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy)) {
                    Ok(ok__) => {
                        glyphrunanalysis.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: super::dwrite_2::DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory3_Impl::CreateCustomRenderingParams(this, core::mem::transmute_copy(&gamma), core::mem::transmute_copy(&enhancedcontrast), core::mem::transmute_copy(&grayscaleenhancedcontrast), core::mem::transmute_copy(&cleartypelevel), core::mem::transmute_copy(&pixelgeometry), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&gridfitmode)) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: windows_core::PCWSTR, lastwritetime: *const super::minwindef::FILETIME, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory3_Impl::CreateFontFaceReference(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&lastwritetime), core::mem::transmute_copy(&faceindex), core::mem::transmute_copy(&fontsimulations)) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFaceReference2<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory3_Impl::CreateFontFaceReference2(this, core::mem::transmute_copy(&fontfile), core::mem::transmute_copy(&faceindex), core::mem::transmute_copy(&fontsimulations)) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSystemFontSet<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory3_Impl::GetSystemFontSet(this) {
                    Ok(ok__) => {
                        fontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontSetBuilder<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsetbuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory3_Impl::CreateFontSetBuilder(this) {
                    Ok(ok__) => {
                        fontsetbuilder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory3_Impl::CreateFontCollectionFromFontSet(this, core::mem::transmute_copy(&fontset)) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: windows_core::BOOL, fontcollection: *mut *mut core::ffi::c_void, checkforupdates: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory3_Impl::GetSystemFontCollection(this, core::mem::transmute_copy(&includedownloadablefonts), core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&checkforupdates)).into()
            }
        }
        unsafe extern "system" fn GetFontDownloadQueue<Identity: IDWriteFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontdownloadqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory3_Impl::GetFontDownloadQueue(this) {
                    Ok(ok__) => {
                        fontdownloadqueue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite_2::IDWriteFactory2_Vtbl::new::<Identity, OFFSET>(),
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
            CreateFontFaceReference: CreateFontFaceReference::<Identity, OFFSET>,
            CreateFontFaceReference2: CreateFontFaceReference2::<Identity, OFFSET>,
            GetSystemFontSet: GetSystemFontSet::<Identity, OFFSET>,
            CreateFontSetBuilder: CreateFontSetBuilder::<Identity, OFFSET>,
            CreateFontCollectionFromFontSet: CreateFontCollectionFromFontSet::<Identity, OFFSET>,
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
            GetFontDownloadQueue: GetFontDownloadQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFactory as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFactory1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory3 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFactory4, IDWriteFactory4_Vtbl, 0x4b0b5bd3_0797_4549_8ac5_fe915cc53856);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFactory4 {
    type Target = IDWriteFactory3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFactory4, windows_core::IUnknown, super::dwrite::IDWriteFactory, super::dwrite_1::IDWriteFactory1, super::dwrite_2::IDWriteFactory2, IDWriteFactory3);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFactory4 {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldanddpitransform: Option<*const super::dwrite::DWRITE_MATRIX>, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TranslateColorGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), glyphrundescription.unwrap_or(core::mem::zeroed()) as _, desiredglyphimageformats, measuringmode, worldanddpitransform.unwrap_or(core::mem::zeroed()) as _, colorpaletteindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, baselineorigin: windows_numerics::Vector2, worldanddpitransform: Option<*const super::dwrite::DWRITE_MATRIX>) -> windows_core::Result<windows_numerics::Vector2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeGlyphOrigins)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphrun), measuringmode, core::mem::transmute(baselineorigin), worldanddpitransform.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, baselineorigin: windows_numerics::Vector2) -> windows_core::Result<windows_numerics::Vector2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeGlyphOrigins2)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphrun), core::mem::transmute(baselineorigin), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory4_Vtbl {
    pub base__: IDWriteFactory3_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub TranslateColorGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, super::dcommon::DWRITE_MEASURING_MODE, *const super::dwrite::DWRITE_MATRIX, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    TranslateColorGlyphRun: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub ComputeGlyphOrigins: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dwrite::DWRITE_GLYPH_RUN, super::dcommon::DWRITE_MEASURING_MODE, windows_numerics::Vector2, *const super::dwrite::DWRITE_MATRIX, *mut windows_numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    ComputeGlyphOrigins: usize,
    pub ComputeGlyphOrigins2: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dwrite::DWRITE_GLYPH_RUN, windows_numerics::Vector2, *mut windows_numerics::Vector2) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory4_Impl: IDWriteFactory3_Impl {
    fn TranslateColorGlyphRun(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldanddpitransform: *const super::dwrite::DWRITE_MATRIX, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator1>;
    fn ComputeGlyphOrigins(&self, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, baselineorigin: &windows_numerics::Vector2, worldanddpitransform: *const super::dwrite::DWRITE_MATRIX) -> windows_core::Result<windows_numerics::Vector2>;
    fn ComputeGlyphOrigins2(&self, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, baselineorigin: &windows_numerics::Vector2) -> windows_core::Result<windows_numerics::Vector2>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory4_Vtbl {
    pub const fn new<Identity: IDWriteFactory4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TranslateColorGlyphRun<Identity: IDWriteFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldanddpitransform: *const super::dwrite::DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory4_Impl::TranslateColorGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&desiredglyphimageformats), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&worldanddpitransform), core::mem::transmute_copy(&colorpaletteindex)) {
                    Ok(ok__) => {
                        colorlayers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputeGlyphOrigins<Identity: IDWriteFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, baselineorigin: windows_numerics::Vector2, worldanddpitransform: *const super::dwrite::DWRITE_MATRIX, glyphorigins: *mut windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory4_Impl::ComputeGlyphOrigins(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&measuringmode), core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&worldanddpitransform)) {
                    Ok(ok__) => {
                        glyphorigins.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComputeGlyphOrigins2<Identity: IDWriteFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, baselineorigin: windows_numerics::Vector2, glyphorigins: *mut windows_numerics::Vector2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory4_Impl::ComputeGlyphOrigins2(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute(&baselineorigin)) {
                    Ok(ok__) => {
                        glyphorigins.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFactory3_Vtbl::new::<Identity, OFFSET>(),
            TranslateColorGlyphRun: TranslateColorGlyphRun::<Identity, OFFSET>,
            ComputeGlyphOrigins: ComputeGlyphOrigins::<Identity, OFFSET>,
            ComputeGlyphOrigins2: ComputeGlyphOrigins2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory4 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFactory as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFactory1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory4 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFactory5, IDWriteFactory5_Vtbl, 0x958db99a_be2a_4f09_af7d_65189803d1d3);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFactory5 {
    type Target = IDWriteFactory4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFactory5, windows_core::IUnknown, super::dwrite::IDWriteFactory, super::dwrite_1::IDWriteFactory1, super::dwrite_2::IDWriteFactory2, IDWriteFactory3, IDWriteFactory4);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFactory5 {
    pub unsafe fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontSetBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateInMemoryFontFileLoader(&self) -> windows_core::Result<IDWriteInMemoryFontFileLoader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInMemoryFontFileLoader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateHttpFontFileLoader<P0, P1>(&self, referrerurl: P0, extraheaders: P1) -> windows_core::Result<IDWriteRemoteFontFileLoader>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateHttpFontFileLoader)(windows_core::Interface::as_raw(self), referrerurl.param().abi(), extraheaders.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AnalyzeContainerType(&self, filedata: *const core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeContainerType)(windows_core::Interface::as_raw(self), filedata, filedatasize) }
    }
    pub unsafe fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const core::ffi::c_void, filedatasize: u32) -> windows_core::Result<super::dwrite::IDWriteFontFileStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnpackFontFile)(windows_core::Interface::as_raw(self), containertype, filedata, filedatasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory5_Vtbl {
    pub base__: IDWriteFactory4_Vtbl,
    pub CreateFontSetBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInMemoryFontFileLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateHttpFontFileLoader: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AnalyzeContainerType: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32) -> DWRITE_CONTAINER_TYPE,
    pub UnpackFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_CONTAINER_TYPE, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory5_Impl: IDWriteFactory4_Impl {
    fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder1>;
    fn CreateInMemoryFontFileLoader(&self) -> windows_core::Result<IDWriteInMemoryFontFileLoader>;
    fn CreateHttpFontFileLoader(&self, referrerurl: &windows_core::PCWSTR, extraheaders: &windows_core::PCWSTR) -> windows_core::Result<IDWriteRemoteFontFileLoader>;
    fn AnalyzeContainerType(&self, filedata: *const core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE;
    fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const core::ffi::c_void, filedatasize: u32) -> windows_core::Result<super::dwrite::IDWriteFontFileStream>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory5_Vtbl {
    pub const fn new<Identity: IDWriteFactory5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFontSetBuilder<Identity: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsetbuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory5_Impl::CreateFontSetBuilder(this) {
                    Ok(ok__) => {
                        fontsetbuilder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInMemoryFontFileLoader<Identity: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newloader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory5_Impl::CreateInMemoryFontFileLoader(this) {
                    Ok(ok__) => {
                        newloader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateHttpFontFileLoader<Identity: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referrerurl: windows_core::PCWSTR, extraheaders: windows_core::PCWSTR, newloader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory5_Impl::CreateHttpFontFileLoader(this, core::mem::transmute(&referrerurl), core::mem::transmute(&extraheaders)) {
                    Ok(ok__) => {
                        newloader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AnalyzeContainerType<Identity: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filedata: *const core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory5_Impl::AnalyzeContainerType(this, core::mem::transmute_copy(&filedata), core::mem::transmute_copy(&filedatasize))
            }
        }
        unsafe extern "system" fn UnpackFontFile<Identity: IDWriteFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, containertype: DWRITE_CONTAINER_TYPE, filedata: *const core::ffi::c_void, filedatasize: u32, unpackedfontstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory5_Impl::UnpackFontFile(this, core::mem::transmute_copy(&containertype), core::mem::transmute_copy(&filedata), core::mem::transmute_copy(&filedatasize)) {
                    Ok(ok__) => {
                        unpackedfontstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFactory4_Vtbl::new::<Identity, OFFSET>(),
            CreateFontSetBuilder: CreateFontSetBuilder::<Identity, OFFSET>,
            CreateInMemoryFontFileLoader: CreateInMemoryFontFileLoader::<Identity, OFFSET>,
            CreateHttpFontFileLoader: CreateHttpFontFileLoader::<Identity, OFFSET>,
            AnalyzeContainerType: AnalyzeContainerType::<Identity, OFFSET>,
            UnpackFontFile: UnpackFontFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory5 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFactory as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFactory1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory5 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFactory6, IDWriteFactory6_Vtbl, 0xf3744d80_21f7_42eb_b35d_995bc72fc223);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFactory6 {
    type Target = IDWriteFactory5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFactory6, windows_core::IUnknown, super::dwrite::IDWriteFactory, super::dwrite_1::IDWriteFactory1, super::dwrite_2::IDWriteFactory2, IDWriteFactory3, IDWriteFactory4, IDWriteFactory5);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFactory6 {
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontFaceReference1>
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFaceReference)(windows_core::Interface::as_raw(self), fontfile.param().abi(), faceindex, fontsimulations, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontResource<P0>(&self, fontfile: P0, faceindex: u32) -> windows_core::Result<IDWriteFontResource>
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontResource)(windows_core::Interface::as_raw(self), fontfile.param().abi(), faceindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSystemFontSet(&self, includedownloadablefonts: bool) -> windows_core::Result<IDWriteFontSet1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemFontSet)(windows_core::Interface::as_raw(self), includedownloadablefonts.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSystemFontCollection(&self, includedownloadablefonts: bool, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), includedownloadablefonts.into(), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection2>
    where
        P0: windows_core::Param<IDWriteFontSet>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontCollectionFromFontSet)(windows_core::Interface::as_raw(self), fontset.param().abi(), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontSetBuilder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTextFormat<P0, P1, P5>(&self, fontfamilyname: P0, fontcollection: P1, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], fontsize: f32, localename: P5) -> windows_core::Result<IDWriteTextFormat3>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::dwrite::IDWriteFontCollection>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextFormat)(windows_core::Interface::as_raw(self), fontfamilyname.param().abi(), fontcollection.param().abi(), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), fontsize, localename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory6_Vtbl {
    pub base__: IDWriteFactory5_Vtbl,
    pub CreateFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::dwrite::DWRITE_FONT_SIMULATIONS, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontCollectionFromFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontSetBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32, f32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory6_Impl: IDWriteFactory5_Impl {
    fn CreateFontFaceReference(&self, fontfile: windows_core::Ref<super::dwrite::IDWriteFontFile>, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontFaceReference1>;
    fn CreateFontResource(&self, fontfile: windows_core::Ref<super::dwrite::IDWriteFontFile>, faceindex: u32) -> windows_core::Result<IDWriteFontResource>;
    fn GetSystemFontSet(&self, includedownloadablefonts: windows_core::BOOL) -> windows_core::Result<IDWriteFontSet1>;
    fn GetSystemFontCollection(&self, includedownloadablefonts: windows_core::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection2>;
    fn CreateFontCollectionFromFontSet(&self, fontset: windows_core::Ref<IDWriteFontSet>, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection2>;
    fn CreateFontSetBuilder(&self) -> windows_core::Result<IDWriteFontSetBuilder2>;
    fn CreateTextFormat(&self, fontfamilyname: &windows_core::PCWSTR, fontcollection: windows_core::Ref<super::dwrite::IDWriteFontCollection>, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: &windows_core::PCWSTR) -> windows_core::Result<IDWriteTextFormat3>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory6_Vtbl {
    pub const fn new<Identity: IDWriteFactory6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFontFaceReference<Identity: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void, faceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory6_Impl::CreateFontFaceReference(this, core::mem::transmute_copy(&fontfile), core::mem::transmute_copy(&faceindex), core::mem::transmute_copy(&fontsimulations), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void, faceindex: u32, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory6_Impl::CreateFontResource(this, core::mem::transmute_copy(&fontfile), core::mem::transmute_copy(&faceindex)) {
                    Ok(ok__) => {
                        fontresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSystemFontSet<Identity: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: windows_core::BOOL, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory6_Impl::GetSystemFontSet(this, core::mem::transmute_copy(&includedownloadablefonts)) {
                    Ok(ok__) => {
                        fontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Identity: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: windows_core::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory6_Impl::GetSystemFontCollection(this, core::mem::transmute_copy(&includedownloadablefonts), core::mem::transmute_copy(&fontfamilymodel)) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Identity: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory6_Impl::CreateFontCollectionFromFontSet(this, core::mem::transmute_copy(&fontset), core::mem::transmute_copy(&fontfamilymodel)) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontSetBuilder<Identity: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsetbuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory6_Impl::CreateFontSetBuilder(this) {
                    Ok(ok__) => {
                        fontsetbuilder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTextFormat<Identity: IDWriteFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilyname: windows_core::PCWSTR, fontcollection: *mut core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: windows_core::PCWSTR, textformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory6_Impl::CreateTextFormat(this, core::mem::transmute(&fontfamilyname), core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&fontsize), core::mem::transmute(&localename)) {
                    Ok(ok__) => {
                        textformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFactory5_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFaceReference: CreateFontFaceReference::<Identity, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, OFFSET>,
            GetSystemFontSet: GetSystemFontSet::<Identity, OFFSET>,
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
            CreateFontCollectionFromFontSet: CreateFontCollectionFromFontSet::<Identity, OFFSET>,
            CreateFontSetBuilder: CreateFontSetBuilder::<Identity, OFFSET>,
            CreateTextFormat: CreateTextFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory6 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFactory as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFactory1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory4 as windows_core::Interface>::IID || iid == &<IDWriteFactory5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory6 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFactory7, IDWriteFactory7_Vtbl, 0x35d0e0b3_9076_4d2e_a016_a91b568a06b4);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFactory7 {
    type Target = IDWriteFactory6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFactory7, windows_core::IUnknown, super::dwrite::IDWriteFactory, super::dwrite_1::IDWriteFactory1, super::dwrite_2::IDWriteFactory2, IDWriteFactory3, IDWriteFactory4, IDWriteFactory5, IDWriteFactory6);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFactory7 {
    pub unsafe fn GetSystemFontSet(&self, includedownloadablefonts: bool) -> windows_core::Result<IDWriteFontSet2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemFontSet)(windows_core::Interface::as_raw(self), includedownloadablefonts.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSystemFontCollection(&self, includedownloadablefonts: bool, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), includedownloadablefonts.into(), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory7_Vtbl {
    pub base__: IDWriteFactory6_Vtbl,
    pub GetSystemFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory7_Impl: IDWriteFactory6_Impl {
    fn GetSystemFontSet(&self, includedownloadablefonts: windows_core::BOOL) -> windows_core::Result<IDWriteFontSet2>;
    fn GetSystemFontCollection(&self, includedownloadablefonts: windows_core::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<IDWriteFontCollection3>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory7_Vtbl {
    pub const fn new<Identity: IDWriteFactory7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSystemFontSet<Identity: IDWriteFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: windows_core::BOOL, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory7_Impl::GetSystemFontSet(this, core::mem::transmute_copy(&includedownloadablefonts)) {
                    Ok(ok__) => {
                        fontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSystemFontCollection<Identity: IDWriteFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includedownloadablefonts: windows_core::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory7_Impl::GetSystemFontCollection(this, core::mem::transmute_copy(&includedownloadablefonts), core::mem::transmute_copy(&fontfamilymodel)) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFactory6_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontSet: GetSystemFontSet::<Identity, OFFSET>,
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory7 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFactory as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFactory1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory4 as windows_core::Interface>::IID || iid == &<IDWriteFactory5 as windows_core::Interface>::IID || iid == &<IDWriteFactory6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory7 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFactory8, IDWriteFactory8_Vtbl, 0xee0a7fb5_def4_4c23_a454_c9c7dc878398);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFactory8 {
    type Target = IDWriteFactory7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFactory8, windows_core::IUnknown, super::dwrite::IDWriteFactory, super::dwrite_1::IDWriteFactory1, super::dwrite_2::IDWriteFactory2, IDWriteFactory3, IDWriteFactory4, IDWriteFactory5, IDWriteFactory6, IDWriteFactory7);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFactory8 {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldanddpitransform: Option<*const super::dwrite::DWRITE_MATRIX>, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TranslateColorGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), glyphrundescription.unwrap_or(core::mem::zeroed()) as _, desiredglyphimageformats, paintfeaturelevel, measuringmode, worldanddpitransform.unwrap_or(core::mem::zeroed()) as _, colorpaletteindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory8_Vtbl {
    pub base__: IDWriteFactory7_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub TranslateColorGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, DWRITE_PAINT_FEATURE_LEVEL, super::dcommon::DWRITE_MEASURING_MODE, *const super::dwrite::DWRITE_MATRIX, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    TranslateColorGlyphRun: usize,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory8_Impl: IDWriteFactory7_Impl {
    fn TranslateColorGlyphRun(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldanddpitransform: *const super::dwrite::DWRITE_MATRIX, colorpaletteindex: u32) -> windows_core::Result<IDWriteColorGlyphRunEnumerator1>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory8_Vtbl {
    pub const fn new<Identity: IDWriteFactory8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TranslateColorGlyphRun<Identity: IDWriteFactory8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, worldanddpitransform: *const super::dwrite::DWRITE_MATRIX, colorpaletteindex: u32, colorenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory8_Impl::TranslateColorGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&desiredglyphimageformats), core::mem::transmute_copy(&paintfeaturelevel), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&worldanddpitransform), core::mem::transmute_copy(&colorpaletteindex)) {
                    Ok(ok__) => {
                        colorenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDWriteFactory7_Vtbl::new::<Identity, OFFSET>(), TranslateColorGlyphRun: TranslateColorGlyphRun::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory8 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFactory as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFactory1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFactory2 as windows_core::Interface>::IID || iid == &<IDWriteFactory3 as windows_core::Interface>::IID || iid == &<IDWriteFactory4 as windows_core::Interface>::IID || iid == &<IDWriteFactory5 as windows_core::Interface>::IID || iid == &<IDWriteFactory6 as windows_core::Interface>::IID || iid == &<IDWriteFactory7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory8 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFont3, IDWriteFont3_Vtbl, 0x29748ed6_8c9c_4a6a_be0b_d912e8538944);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFont3 {
    type Target = super::dwrite_2::IDWriteFont2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFont3, windows_core::IUnknown, super::dwrite::IDWriteFont, super::dwrite_1::IDWriteFont1, super::dwrite_2::IDWriteFont2);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFont3 {
    pub unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Equals<P0>(&self, font: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<super::dwrite::IDWriteFont>,
    {
        unsafe { (windows_core::Interface::vtable(self).Equals)(windows_core::Interface::as_raw(self), font.param().abi()) }
    }
    pub unsafe fn GetFontFaceReference(&self) -> windows_core::Result<IDWriteFontFaceReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasCharacter)(windows_core::Interface::as_raw(self), unicodevalue) }
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        unsafe { (windows_core::Interface::vtable(self).GetLocality)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFont3_Vtbl {
    pub base__: super::dwrite_2::IDWriteFont2_Vtbl,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasCharacter: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::BOOL,
    pub GetLocality: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_LOCALITY,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteFont3_Impl: super::dwrite_2::IDWriteFont2_Impl {
    fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace3>;
    fn Equals(&self, font: windows_core::Ref<super::dwrite::IDWriteFont>) -> windows_core::BOOL;
    fn GetFontFaceReference(&self) -> windows_core::Result<IDWriteFontFaceReference>;
    fn HasCharacter(&self, unicodevalue: u32) -> windows_core::BOOL;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFont3_Vtbl {
    pub const fn new<Identity: IDWriteFont3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFontFace<Identity: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFont3_Impl::CreateFontFace(this) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Equals<Identity: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, font: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont3_Impl::Equals(this, core::mem::transmute_copy(&font))
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFont3_Impl::GetFontFaceReference(this) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasCharacter<Identity: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodevalue: u32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont3_Impl::HasCharacter(this, core::mem::transmute_copy(&unicodevalue))
            }
        }
        unsafe extern "system" fn GetLocality<Identity: IDWriteFont3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_LOCALITY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont3_Impl::GetLocality(this)
            }
        }
        Self {
            base__: super::dwrite_2::IDWriteFont2_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            Equals: Equals::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
            HasCharacter: HasCharacter::<Identity, OFFSET>,
            GetLocality: GetLocality::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFont3 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFont as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFont1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFont2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteFont3 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFontCollection1, IDWriteFontCollection1_Vtbl, 0x53585141_d9f8_4095_8321_d73cf6bd116c);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFontCollection1 {
    type Target = super::dwrite::IDWriteFontCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFontCollection1, windows_core::IUnknown, super::dwrite::IDWriteFontCollection);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontCollection1 {
    pub unsafe fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection1_Vtbl {
    pub base__: super::dwrite::IDWriteFontCollection_Vtbl,
    pub GetFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteFontCollection1_Impl: super::dwrite::IDWriteFontCollection_Impl {
    fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet>;
    fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily1>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontCollection1_Vtbl {
    pub const fn new<Identity: IDWriteFontCollection1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontSet<Identity: IDWriteFontCollection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontCollection1_Impl::GetFontSet(this) {
                    Ok(ok__) => {
                        fontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontFamily<Identity: IDWriteFontCollection1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, fontfamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontCollection1_Impl::GetFontFamily(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        fontfamily.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite::IDWriteFontCollection_Vtbl::new::<Identity, OFFSET>(),
            GetFontSet: GetFontSet::<Identity, OFFSET>,
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollection1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteFontCollection1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFontCollection2, IDWriteFontCollection2_Vtbl, 0x514039c6_4617_4064_bf8b_92ea83e506e0);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFontCollection2 {
    type Target = IDWriteFontCollection1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFontCollection2, windows_core::IUnknown, super::dwrite::IDWriteFontCollection, IDWriteFontCollection1);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontCollection2 {
    pub unsafe fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontList2>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), familyname.param().abi(), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontFamilyModel(&self) -> DWRITE_FONT_FAMILY_MODEL {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyModel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection2_Vtbl {
    pub base__: IDWriteFontCollection1_Vtbl,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFamilyModel: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL,
    pub GetFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteFontCollection2_Impl: IDWriteFontCollection1_Impl {
    fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily2>;
    fn GetMatchingFonts(&self, familyname: &windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontList2>;
    fn GetFontFamilyModel(&self) -> DWRITE_FONT_FAMILY_MODEL;
    fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontCollection2_Vtbl {
    pub const fn new<Identity: IDWriteFontCollection2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontFamily<Identity: IDWriteFontCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, fontfamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontCollection2_Impl::GetFontFamily(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        fontfamily.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: IDWriteFontCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontCollection2_Impl::GetMatchingFonts(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                    Ok(ok__) => {
                        fontlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontFamilyModel<Identity: IDWriteFontCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontCollection2_Impl::GetFontFamilyModel(this)
            }
        }
        unsafe extern "system" fn GetFontSet<Identity: IDWriteFontCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontCollection2_Impl::GetFontSet(this) {
                    Ok(ok__) => {
                        fontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFontCollection1_Vtbl::new::<Identity, OFFSET>(),
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
            GetFontFamilyModel: GetFontFamilyModel::<Identity, OFFSET>,
            GetFontSet: GetFontSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollection2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontCollection as windows_core::Interface>::IID || iid == &<IDWriteFontCollection1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteFontCollection2 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFontCollection3, IDWriteFontCollection3_Vtbl, 0xa4d055a6_f9e3_4e25_93b7_9e309f3af8e9);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFontCollection3 {
    type Target = IDWriteFontCollection2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFontCollection3, windows_core::IUnknown, super::dwrite::IDWriteFontCollection, IDWriteFontCollection1, IDWriteFontCollection2);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontCollection3 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetExpirationEvent(&self) -> super::winnt::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).GetExpirationEvent)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection3_Vtbl {
    pub base__: IDWriteFontCollection2_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetExpirationEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::winnt::HANDLE,
    #[cfg(not(feature = "Win32_winnt"))]
    GetExpirationEvent: usize,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_winnt"))]
pub trait IDWriteFontCollection3_Impl: IDWriteFontCollection2_Impl {
    fn GetExpirationEvent(&self) -> super::winnt::HANDLE;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_winnt"))]
impl IDWriteFontCollection3_Vtbl {
    pub const fn new<Identity: IDWriteFontCollection3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExpirationEvent<Identity: IDWriteFontCollection3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::winnt::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontCollection3_Impl::GetExpirationEvent(this)
            }
        }
        Self { base__: IDWriteFontCollection2_Vtbl::new::<Identity, OFFSET>(), GetExpirationEvent: GetExpirationEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollection3 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontCollection as windows_core::Interface>::IID || iid == &<IDWriteFontCollection1 as windows_core::Interface>::IID || iid == &<IDWriteFontCollection2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDWriteFontCollection3 {}
windows_core::imp::define_interface!(IDWriteFontDownloadListener, IDWriteFontDownloadListener_Vtbl, 0xb06fe5b9_43ec_4393_881b_dbe4dc72fda7);
windows_core::imp::interface_hierarchy!(IDWriteFontDownloadListener, windows_core::IUnknown);
impl IDWriteFontDownloadListener {
    pub unsafe fn DownloadCompleted<P0, P1>(&self, downloadqueue: P0, context: P1, downloadresult: windows_core::HRESULT)
    where
        P0: windows_core::Param<IDWriteFontDownloadQueue>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DownloadCompleted)(windows_core::Interface::as_raw(self), downloadqueue.param().abi(), context.param().abi(), downloadresult);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontDownloadListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DownloadCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT),
}
pub trait IDWriteFontDownloadListener_Impl: windows_core::IUnknownImpl {
    fn DownloadCompleted(&self, downloadqueue: windows_core::Ref<IDWriteFontDownloadQueue>, context: windows_core::Ref<windows_core::IUnknown>, downloadresult: windows_core::HRESULT);
}
impl IDWriteFontDownloadListener_Vtbl {
    pub const fn new<Identity: IDWriteFontDownloadListener_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DownloadCompleted<Identity: IDWriteFontDownloadListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadqueue: *mut core::ffi::c_void, context: *mut core::ffi::c_void, downloadresult: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontDownloadListener_Impl::DownloadCompleted(this, core::mem::transmute_copy(&downloadqueue), core::mem::transmute_copy(&context), core::mem::transmute_copy(&downloadresult));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DownloadCompleted: DownloadCompleted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontDownloadListener as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontDownloadListener {}
windows_core::imp::define_interface!(IDWriteFontDownloadQueue, IDWriteFontDownloadQueue_Vtbl, 0xb71e6052_5aea_4fa3_832e_f60d431f7e91);
windows_core::imp::interface_hierarchy!(IDWriteFontDownloadQueue, windows_core::IUnknown);
impl IDWriteFontDownloadQueue {
    pub unsafe fn AddListener<P0>(&self, listener: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDWriteFontDownloadListener>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddListener)(windows_core::Interface::as_raw(self), listener.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveListener(&self, token: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveListener)(windows_core::Interface::as_raw(self), token) }
    }
    pub unsafe fn IsEmpty(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsEmpty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BeginDownload<P0>(&self, context: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginDownload)(windows_core::Interface::as_raw(self), context.param().abi()) }
    }
    pub unsafe fn CancelDownload(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelDownload)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetGenerationCount(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetGenerationCount)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontDownloadQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveListener: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsEmpty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub BeginDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelDownload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGenerationCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
}
pub trait IDWriteFontDownloadQueue_Impl: windows_core::IUnknownImpl {
    fn AddListener(&self, listener: windows_core::Ref<IDWriteFontDownloadListener>) -> windows_core::Result<u32>;
    fn RemoveListener(&self, token: u32) -> windows_core::Result<()>;
    fn IsEmpty(&self) -> windows_core::BOOL;
    fn BeginDownload(&self, context: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CancelDownload(&self) -> windows_core::Result<()>;
    fn GetGenerationCount(&self) -> u64;
}
impl IDWriteFontDownloadQueue_Vtbl {
    pub const fn new<Identity: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddListener<Identity: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listener: *mut core::ffi::c_void, token: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontDownloadQueue_Impl::AddListener(this, core::mem::transmute_copy(&listener)) {
                    Ok(ok__) => {
                        token.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveListener<Identity: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontDownloadQueue_Impl::RemoveListener(this, core::mem::transmute_copy(&token)).into()
            }
        }
        unsafe extern "system" fn IsEmpty<Identity: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontDownloadQueue_Impl::IsEmpty(this)
            }
        }
        unsafe extern "system" fn BeginDownload<Identity: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontDownloadQueue_Impl::BeginDownload(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn CancelDownload<Identity: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontDownloadQueue_Impl::CancelDownload(this).into()
            }
        }
        unsafe extern "system" fn GetGenerationCount<Identity: IDWriteFontDownloadQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontDownloadQueue_Impl::GetGenerationCount(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddListener: AddListener::<Identity, OFFSET>,
            RemoveListener: RemoveListener::<Identity, OFFSET>,
            IsEmpty: IsEmpty::<Identity, OFFSET>,
            BeginDownload: BeginDownload::<Identity, OFFSET>,
            CancelDownload: CancelDownload::<Identity, OFFSET>,
            GetGenerationCount: GetGenerationCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontDownloadQueue as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontDownloadQueue {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFontFace3, IDWriteFontFace3_Vtbl, 0xd37d7598_09be_4222_a236_2081341cc1f2);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFontFace3 {
    type Target = super::dwrite_2::IDWriteFontFace2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFontFace3, windows_core::IUnknown, super::dwrite::IDWriteFontFace, super::dwrite_1::IDWriteFontFace1, super::dwrite_2::IDWriteFontFace2);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontFace3 {
    pub unsafe fn GetFontFaceReference(&self) -> windows_core::Result<IDWriteFontFaceReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPanose(&self) -> super::dwrite_1::DWRITE_PANOSE {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPanose)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetWeight(&self) -> super::dwrite::DWRITE_FONT_WEIGHT {
        unsafe { (windows_core::Interface::vtable(self).GetWeight)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStretch(&self) -> super::dwrite::DWRITE_FONT_STRETCH {
        unsafe { (windows_core::Interface::vtable(self).GetStretch)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStyle(&self) -> super::dwrite::DWRITE_FONT_STYLE {
        unsafe { (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFamilyNames(&self) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFaceNames(&self) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFaceNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: super::dwrite::DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut Option<super::dwrite::IDWriteLocalizedStrings>, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInformationalStrings)(windows_core::Interface::as_raw(self), informationalstringid, core::mem::transmute(informationalstrings), exists as _) }
    }
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasCharacter)(windows_core::Interface::as_raw(self), unicodevalue) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetRecommendedRenderingMode<P7>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: Option<*const super::dwrite::DWRITE_MATRIX>, issideways: bool, outlinethreshold: super::dwrite_1::DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: P7, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut super::dwrite_2::DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT
    where
        P7: windows_core::Param<super::dwrite::IDWriteRenderingParams>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(windows_core::Interface::as_raw(self), fontemsize, dpix, dpiy, transform.unwrap_or(core::mem::zeroed()) as _, issideways.into(), outlinethreshold, measuringmode, renderingparams.param().abi(), renderingmode as _, gridfitmode as _) }
    }
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsCharacterLocal)(windows_core::Interface::as_raw(self), unicodevalue) }
    }
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsGlyphLocal)(windows_core::Interface::as_raw(self), glyphid) }
    }
    pub unsafe fn AreCharactersLocal(&self, characters: &[u16], enqueueifnotlocal: bool) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreCharactersLocal)(windows_core::Interface::as_raw(self), core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap(), enqueueifnotlocal.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AreGlyphsLocal(&self, glyphindices: &[u16], enqueueifnotlocal: bool) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreGlyphsLocal)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap(), enqueueifnotlocal.into(), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace3_Vtbl {
    pub base__: super::dwrite_2::IDWriteFontFace2_Vtbl,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPanose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dwrite_1::DWRITE_PANOSE),
    pub GetWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dwrite::DWRITE_FONT_WEIGHT,
    pub GetStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dwrite::DWRITE_FONT_STRETCH,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dwrite::DWRITE_FONT_STYLE,
    pub GetFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFaceNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInformationalStrings: unsafe extern "system" fn(*mut core::ffi::c_void, super::dwrite::DWRITE_INFORMATIONAL_STRING_ID, *mut *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub HasCharacter: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::BOOL,
    #[cfg(feature = "Win32_dcommon")]
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, *const super::dwrite::DWRITE_MATRIX, windows_core::BOOL, super::dwrite_1::DWRITE_OUTLINE_THRESHOLD, super::dcommon::DWRITE_MEASURING_MODE, *mut core::ffi::c_void, *mut DWRITE_RENDERING_MODE1, *mut super::dwrite_2::DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetRecommendedRenderingMode: usize,
    pub IsCharacterLocal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::BOOL,
    pub IsGlyphLocal: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::BOOL,
    pub AreCharactersLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub AreGlyphsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
pub trait IDWriteFontFace3_Impl: super::dwrite_2::IDWriteFontFace2_Impl {
    fn GetFontFaceReference(&self) -> windows_core::Result<IDWriteFontFaceReference>;
    fn GetPanose(&self, panose: *mut super::dwrite_1::DWRITE_PANOSE);
    fn GetWeight(&self) -> super::dwrite::DWRITE_FONT_WEIGHT;
    fn GetStretch(&self) -> super::dwrite::DWRITE_FONT_STRETCH;
    fn GetStyle(&self) -> super::dwrite::DWRITE_FONT_STYLE;
    fn GetFamilyNames(&self) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings>;
    fn GetFaceNames(&self) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings>;
    fn GetInformationalStrings(&self, informationalstringid: super::dwrite::DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: windows_core::OutRef<super::dwrite::IDWriteLocalizedStrings>, exists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn HasCharacter(&self, unicodevalue: u32) -> windows_core::BOOL;
    fn GetRecommendedRenderingMode(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const super::dwrite::DWRITE_MATRIX, issideways: windows_core::BOOL, outlinethreshold: super::dwrite_1::DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: windows_core::Ref<super::dwrite::IDWriteRenderingParams>, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut super::dwrite_2::DWRITE_GRID_FIT_MODE) -> windows_core::Result<()>;
    fn IsCharacterLocal(&self, unicodevalue: u32) -> windows_core::BOOL;
    fn IsGlyphLocal(&self, glyphid: u16) -> windows_core::BOOL;
    fn AreCharactersLocal(&self, characters: *const u16, charactercount: u32, enqueueifnotlocal: windows_core::BOOL) -> windows_core::Result<windows_core::BOOL>;
    fn AreGlyphsLocal(&self, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: windows_core::BOOL) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl IDWriteFontFace3_Vtbl {
    pub const fn new<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontFaceReference<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace3_Impl::GetFontFaceReference(this) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPanose<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, panose: *mut super::dwrite_1::DWRITE_PANOSE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::GetPanose(this, core::mem::transmute_copy(&panose));
            }
        }
        unsafe extern "system" fn GetWeight<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dwrite::DWRITE_FONT_WEIGHT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::GetWeight(this)
            }
        }
        unsafe extern "system" fn GetStretch<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dwrite::DWRITE_FONT_STRETCH {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::GetStretch(this)
            }
        }
        unsafe extern "system" fn GetStyle<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dwrite::DWRITE_FONT_STYLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::GetStyle(this)
            }
        }
        unsafe extern "system" fn GetFamilyNames<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace3_Impl::GetFamilyNames(this) {
                    Ok(ok__) => {
                        names.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFaceNames<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace3_Impl::GetFaceNames(this) {
                    Ok(ok__) => {
                        names.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInformationalStrings<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, informationalstringid: super::dwrite::DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut core::ffi::c_void, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::GetInformationalStrings(this, core::mem::transmute_copy(&informationalstringid), core::mem::transmute_copy(&informationalstrings), core::mem::transmute_copy(&exists)).into()
            }
        }
        unsafe extern "system" fn HasCharacter<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodevalue: u32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::HasCharacter(this, core::mem::transmute_copy(&unicodevalue))
            }
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const super::dwrite::DWRITE_MATRIX, issideways: windows_core::BOOL, outlinethreshold: super::dwrite_1::DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: *mut core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut super::dwrite_2::DWRITE_GRID_FIT_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::GetRecommendedRenderingMode(this, core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&outlinethreshold), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&renderingparams), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&gridfitmode)).into()
            }
        }
        unsafe extern "system" fn IsCharacterLocal<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodevalue: u32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::IsCharacterLocal(this, core::mem::transmute_copy(&unicodevalue))
            }
        }
        unsafe extern "system" fn IsGlyphLocal<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphid: u16) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace3_Impl::IsGlyphLocal(this, core::mem::transmute_copy(&glyphid))
            }
        }
        unsafe extern "system" fn AreCharactersLocal<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, characters: *const u16, charactercount: u32, enqueueifnotlocal: windows_core::BOOL, islocal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace3_Impl::AreCharactersLocal(this, core::mem::transmute_copy(&characters), core::mem::transmute_copy(&charactercount), core::mem::transmute_copy(&enqueueifnotlocal)) {
                    Ok(ok__) => {
                        islocal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AreGlyphsLocal<Identity: IDWriteFontFace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: windows_core::BOOL, islocal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace3_Impl::AreGlyphsLocal(this, core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&enqueueifnotlocal)) {
                    Ok(ok__) => {
                        islocal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite_2::IDWriteFontFace2_Vtbl::new::<Identity, OFFSET>(),
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
            GetPanose: GetPanose::<Identity, OFFSET>,
            GetWeight: GetWeight::<Identity, OFFSET>,
            GetStretch: GetStretch::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            GetFamilyNames: GetFamilyNames::<Identity, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, OFFSET>,
            GetInformationalStrings: GetInformationalStrings::<Identity, OFFSET>,
            HasCharacter: HasCharacter::<Identity, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, OFFSET>,
            IsCharacterLocal: IsCharacterLocal::<Identity, OFFSET>,
            IsGlyphLocal: IsGlyphLocal::<Identity, OFFSET>,
            AreCharactersLocal: AreCharactersLocal::<Identity, OFFSET>,
            AreGlyphsLocal: AreGlyphsLocal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace3 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFace as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFontFace2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for IDWriteFontFace3 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFontFace4, IDWriteFontFace4_Vtbl, 0x27f2a904_4eb8_441d_9678_0563f53e3e2f);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFontFace4 {
    type Target = IDWriteFontFace3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFontFace4, windows_core::IUnknown, super::dwrite::IDWriteFontFace, super::dwrite_1::IDWriteFontFace1, super::dwrite_2::IDWriteFontFace2, IDWriteFontFace3);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontFace4 {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetGlyphImageFormats(&self) -> super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphImageFormats)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetGlyphImageFormats2(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> windows_core::Result<super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlyphImageFormats2)(windows_core::Interface::as_raw(self), glyphid, pixelsperemfirst, pixelsperemlast, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
    pub unsafe fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphImageData)(windows_core::Interface::as_raw(self), glyphid, pixelsperem, glyphimageformat, glyphdata as _, glyphdatacontext as _) }
    }
    pub unsafe fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseGlyphImageData)(windows_core::Interface::as_raw(self), glyphdatacontext as _);
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace4_Vtbl {
    pub base__: IDWriteFontFace3_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub GetGlyphImageFormats: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetGlyphImageFormats: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetGlyphImageFormats2: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, u32, *mut super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetGlyphImageFormats2: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
    pub GetGlyphImageData: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32, super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, *mut DWRITE_GLYPH_IMAGE_DATA, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_windef")))]
    GetGlyphImageData: usize,
    pub ReleaseGlyphImageData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDWriteFontFace4_Impl: IDWriteFontFace3_Impl {
    fn GetGlyphImageFormats(&self) -> super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS;
    fn GetGlyphImageFormats2(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> windows_core::Result<super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS>;
    fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut core::ffi::c_void);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDWriteFontFace4_Vtbl {
    pub const fn new<Identity: IDWriteFontFace4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGlyphImageFormats<Identity: IDWriteFontFace4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace4_Impl::GetGlyphImageFormats(this)
            }
        }
        unsafe extern "system" fn GetGlyphImageFormats2<Identity: IDWriteFontFace4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32, glyphimageformats: *mut super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace4_Impl::GetGlyphImageFormats2(this, core::mem::transmute_copy(&glyphid), core::mem::transmute_copy(&pixelsperemfirst), core::mem::transmute_copy(&pixelsperemlast)) {
                    Ok(ok__) => {
                        glyphimageformats.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlyphImageData<Identity: IDWriteFontFace4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphid: u16, pixelsperem: u32, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace4_Impl::GetGlyphImageData(this, core::mem::transmute_copy(&glyphid), core::mem::transmute_copy(&pixelsperem), core::mem::transmute_copy(&glyphimageformat), core::mem::transmute_copy(&glyphdata), core::mem::transmute_copy(&glyphdatacontext)).into()
            }
        }
        unsafe extern "system" fn ReleaseGlyphImageData<Identity: IDWriteFontFace4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphdatacontext: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace4_Impl::ReleaseGlyphImageData(this, core::mem::transmute_copy(&glyphdatacontext));
            }
        }
        Self {
            base__: IDWriteFontFace3_Vtbl::new::<Identity, OFFSET>(),
            GetGlyphImageFormats: GetGlyphImageFormats::<Identity, OFFSET>,
            GetGlyphImageFormats2: GetGlyphImageFormats2::<Identity, OFFSET>,
            GetGlyphImageData: GetGlyphImageData::<Identity, OFFSET>,
            ReleaseGlyphImageData: ReleaseGlyphImageData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace4 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFace as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFontFace4 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFontFace5, IDWriteFontFace5_Vtbl, 0x98eff3a5_b667_479a_b145_e2fa5b9fdc29);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFontFace5 {
    type Target = IDWriteFontFace4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFontFace5, windows_core::IUnknown, super::dwrite::IDWriteFontFace, super::dwrite_1::IDWriteFontFace1, super::dwrite_2::IDWriteFontFace2, IDWriteFontFace3, IDWriteFontFace4);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontFace5 {
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisValueCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()) }
    }
    pub unsafe fn HasVariations(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasVariations)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontResource(&self) -> windows_core::Result<IDWriteFontResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Equals<P0>(&self, fontface: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFace>,
    {
        unsafe { (windows_core::Interface::vtable(self).Equals)(windows_core::Interface::as_raw(self), fontface.param().abi()) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace5_Vtbl {
    pub base__: IDWriteFontFace4_Vtbl,
    pub GetFontAxisValueCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
    pub HasVariations: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDWriteFontFace5_Impl: IDWriteFontFace4_Impl {
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
    fn HasVariations(&self) -> windows_core::BOOL;
    fn GetFontResource(&self) -> windows_core::Result<IDWriteFontResource>;
    fn Equals(&self, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDWriteFontFace5_Vtbl {
    pub const fn new<Identity: IDWriteFontFace5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontAxisValueCount<Identity: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace5_Impl::GetFontAxisValueCount(this)
            }
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace5_Impl::GetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
            }
        }
        unsafe extern "system" fn HasVariations<Identity: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace5_Impl::HasVariations(this)
            }
        }
        unsafe extern "system" fn GetFontResource<Identity: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace5_Impl::GetFontResource(this) {
                    Ok(ok__) => {
                        fontresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Equals<Identity: IDWriteFontFace5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace5_Impl::Equals(this, core::mem::transmute_copy(&fontface))
            }
        }
        Self {
            base__: IDWriteFontFace4_Vtbl::new::<Identity, OFFSET>(),
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, OFFSET>,
            HasVariations: HasVariations::<Identity, OFFSET>,
            GetFontResource: GetFontResource::<Identity, OFFSET>,
            Equals: Equals::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace5 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFace as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace3 as windows_core::Interface>::IID || iid == &<IDWriteFontFace4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFontFace5 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFontFace6, IDWriteFontFace6_Vtbl, 0xc4b1fe1b_6e84_47d5_b54c_a597981b06ad);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFontFace6 {
    type Target = IDWriteFontFace5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFontFace6, windows_core::IUnknown, super::dwrite::IDWriteFontFace, super::dwrite_1::IDWriteFontFace1, super::dwrite_2::IDWriteFontFace2, IDWriteFontFace3, IDWriteFontFace4, IDWriteFontFace5);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontFace6 {
    pub unsafe fn GetFamilyNames(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyNames)(windows_core::Interface::as_raw(self), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFaceNames(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFaceNames)(windows_core::Interface::as_raw(self), fontfamilymodel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace6_Vtbl {
    pub base__: IDWriteFontFace5_Vtbl,
    pub GetFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFaceNames: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FAMILY_MODEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDWriteFontFace6_Impl: IDWriteFontFace5_Impl {
    fn GetFamilyNames(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings>;
    fn GetFaceNames(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDWriteFontFace6_Vtbl {
    pub const fn new<Identity: IDWriteFontFace6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFamilyNames<Identity: IDWriteFontFace6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace6_Impl::GetFamilyNames(this, core::mem::transmute_copy(&fontfamilymodel)) {
                    Ok(ok__) => {
                        names.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFaceNames<Identity: IDWriteFontFace6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace6_Impl::GetFaceNames(this, core::mem::transmute_copy(&fontfamilymodel)) {
                    Ok(ok__) => {
                        names.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFontFace5_Vtbl::new::<Identity, OFFSET>(),
            GetFamilyNames: GetFamilyNames::<Identity, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace6 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFace as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace3 as windows_core::Interface>::IID || iid == &<IDWriteFontFace4 as windows_core::Interface>::IID || iid == &<IDWriteFontFace5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFontFace6 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteFontFace7, IDWriteFontFace7_Vtbl, 0x3945b85b_bc95_40f7_b72c_8b73bfc7e13b);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteFontFace7 {
    type Target = IDWriteFontFace6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteFontFace7, windows_core::IUnknown, super::dwrite::IDWriteFontFace, super::dwrite_1::IDWriteFontFace1, super::dwrite_2::IDWriteFontFace2, IDWriteFontFace3, IDWriteFontFace4, IDWriteFontFace5, IDWriteFontFace6);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontFace7 {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetPaintFeatureLevel(&self, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS) -> DWRITE_PAINT_FEATURE_LEVEL {
        unsafe { (windows_core::Interface::vtable(self).GetPaintFeatureLevel)(windows_core::Interface::as_raw(self), glyphimageformat) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CreatePaintReader(&self, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL) -> windows_core::Result<IDWritePaintReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePaintReader)(windows_core::Interface::as_raw(self), glyphimageformat, paintfeaturelevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace7_Vtbl {
    pub base__: IDWriteFontFace6_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub GetPaintFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS) -> DWRITE_PAINT_FEATURE_LEVEL,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetPaintFeatureLevel: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub CreatePaintReader: unsafe extern "system" fn(*mut core::ffi::c_void, super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, DWRITE_PAINT_FEATURE_LEVEL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CreatePaintReader: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDWriteFontFace7_Impl: IDWriteFontFace6_Impl {
    fn GetPaintFeatureLevel(&self, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS) -> DWRITE_PAINT_FEATURE_LEVEL;
    fn CreatePaintReader(&self, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL) -> windows_core::Result<IDWritePaintReader>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDWriteFontFace7_Vtbl {
    pub const fn new<Identity: IDWriteFontFace7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPaintFeatureLevel<Identity: IDWriteFontFace7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS) -> DWRITE_PAINT_FEATURE_LEVEL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace7_Impl::GetPaintFeatureLevel(this, core::mem::transmute_copy(&glyphimageformat))
            }
        }
        unsafe extern "system" fn CreatePaintReader<Identity: IDWriteFontFace7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, paintfeaturelevel: DWRITE_PAINT_FEATURE_LEVEL, paintreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace7_Impl::CreatePaintReader(this, core::mem::transmute_copy(&glyphimageformat), core::mem::transmute_copy(&paintfeaturelevel)) {
                    Ok(ok__) => {
                        paintreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFontFace6_Vtbl::new::<Identity, OFFSET>(),
            GetPaintFeatureLevel: GetPaintFeatureLevel::<Identity, OFFSET>,
            CreatePaintReader: CreatePaintReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace7 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFace as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFontFace2 as windows_core::Interface>::IID || iid == &<IDWriteFontFace3 as windows_core::Interface>::IID || iid == &<IDWriteFontFace4 as windows_core::Interface>::IID || iid == &<IDWriteFontFace5 as windows_core::Interface>::IID || iid == &<IDWriteFontFace6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFontFace7 {}
windows_core::imp::define_interface!(IDWriteFontFaceReference, IDWriteFontFaceReference_Vtbl, 0x5e7fa7ca_dde3_424c_89f0_9fcd6fed58cd);
windows_core::imp::interface_hierarchy!(IDWriteFontFaceReference, windows_core::IUnknown);
impl IDWriteFontFaceReference {
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub unsafe fn CreateFontFaceWithSimulations(&self, fontfacesimulationflags: super::dwrite::DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFace3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFaceWithSimulations)(windows_core::Interface::as_raw(self), fontfacesimulationflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Equals<P0>(&self, fontfacereference: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Equals)(windows_core::Interface::as_raw(self), fontfacereference.param().abi()) }
    }
    pub unsafe fn GetFontFaceIndex(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontFaceIndex)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetSimulations(&self) -> super::dwrite::DWRITE_FONT_SIMULATIONS {
        unsafe { (windows_core::Interface::vtable(self).GetSimulations)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetFontFile(&self) -> windows_core::Result<super::dwrite::IDWriteFontFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLocalFileSize(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetLocalFileSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFileSize(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetFileTime(&self) -> windows_core::Result<super::minwindef::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        unsafe { (windows_core::Interface::vtable(self).GetLocality)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnqueueFontDownloadRequest(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnqueueFontDownloadRequest)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnqueueCharacterDownloadRequest(&self, characters: &[u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnqueueCharacterDownloadRequest)(windows_core::Interface::as_raw(self), core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap()) }
    }
    pub unsafe fn EnqueueGlyphDownloadRequest(&self, glyphindices: &[u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnqueueGlyphDownloadRequest)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap()) }
    }
    pub unsafe fn EnqueueFileFragmentDownloadRequest(&self, fileoffset: u64, fragmentsize: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnqueueFileFragmentDownloadRequest)(windows_core::Interface::as_raw(self), fileoffset, fragmentsize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFaceReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2")))]
    CreateFontFace: usize,
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub CreateFontFaceWithSimulations: unsafe extern "system" fn(*mut core::ffi::c_void, super::dwrite::DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2")))]
    CreateFontFaceWithSimulations: usize,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetFontFaceIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_dwrite")]
    pub GetSimulations: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dwrite::DWRITE_FONT_SIMULATIONS,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetSimulations: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub GetFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetFontFile: usize,
    pub GetLocalFileSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    #[cfg(feature = "Win32_minwindef")]
    pub GetFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetFileTime: usize,
    pub GetLocality: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_LOCALITY,
    pub EnqueueFontDownloadRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnqueueCharacterDownloadRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32) -> windows_core::HRESULT,
    pub EnqueueGlyphDownloadRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32) -> windows_core::HRESULT,
    pub EnqueueFileFragmentDownloadRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef"))]
pub trait IDWriteFontFaceReference_Impl: windows_core::IUnknownImpl {
    fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace3>;
    fn CreateFontFaceWithSimulations(&self, fontfacesimulationflags: super::dwrite::DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFace3>;
    fn Equals(&self, fontfacereference: windows_core::Ref<IDWriteFontFaceReference>) -> windows_core::BOOL;
    fn GetFontFaceIndex(&self) -> u32;
    fn GetSimulations(&self) -> super::dwrite::DWRITE_FONT_SIMULATIONS;
    fn GetFontFile(&self) -> windows_core::Result<super::dwrite::IDWriteFontFile>;
    fn GetLocalFileSize(&self) -> u64;
    fn GetFileSize(&self) -> u64;
    fn GetFileTime(&self) -> windows_core::Result<super::minwindef::FILETIME>;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
    fn EnqueueFontDownloadRequest(&self) -> windows_core::Result<()>;
    fn EnqueueCharacterDownloadRequest(&self, characters: *const u16, charactercount: u32) -> windows_core::Result<()>;
    fn EnqueueGlyphDownloadRequest(&self, glyphindices: *const u16, glyphcount: u32) -> windows_core::Result<()>;
    fn EnqueueFileFragmentDownloadRequest(&self, fileoffset: u64, fragmentsize: u64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef"))]
impl IDWriteFontFaceReference_Vtbl {
    pub const fn new<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFontFace<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFaceReference_Impl::CreateFontFace(this) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFaceWithSimulations<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacesimulationflags: super::dwrite::DWRITE_FONT_SIMULATIONS, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFaceReference_Impl::CreateFontFaceWithSimulations(this, core::mem::transmute_copy(&fontfacesimulationflags)) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Equals<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::Equals(this, core::mem::transmute_copy(&fontfacereference))
            }
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::GetFontFaceIndex(this)
            }
        }
        unsafe extern "system" fn GetSimulations<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dwrite::DWRITE_FONT_SIMULATIONS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::GetSimulations(this)
            }
        }
        unsafe extern "system" fn GetFontFile<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFaceReference_Impl::GetFontFile(this) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalFileSize<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::GetLocalFileSize(this)
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::GetFileSize(this)
            }
        }
        unsafe extern "system" fn GetFileTime<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastwritetime: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFaceReference_Impl::GetFileTime(this) {
                    Ok(ok__) => {
                        lastwritetime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocality<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_LOCALITY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::GetLocality(this)
            }
        }
        unsafe extern "system" fn EnqueueFontDownloadRequest<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::EnqueueFontDownloadRequest(this).into()
            }
        }
        unsafe extern "system" fn EnqueueCharacterDownloadRequest<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, characters: *const u16, charactercount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::EnqueueCharacterDownloadRequest(this, core::mem::transmute_copy(&characters), core::mem::transmute_copy(&charactercount)).into()
            }
        }
        unsafe extern "system" fn EnqueueGlyphDownloadRequest<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphindices: *const u16, glyphcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::EnqueueGlyphDownloadRequest(this, core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphcount)).into()
            }
        }
        unsafe extern "system" fn EnqueueFileFragmentDownloadRequest<Identity: IDWriteFontFaceReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference_Impl::EnqueueFileFragmentDownloadRequest(this, core::mem::transmute_copy(&fileoffset), core::mem::transmute_copy(&fragmentsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            CreateFontFaceWithSimulations: CreateFontFaceWithSimulations::<Identity, OFFSET>,
            Equals: Equals::<Identity, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, OFFSET>,
            GetSimulations: GetSimulations::<Identity, OFFSET>,
            GetFontFile: GetFontFile::<Identity, OFFSET>,
            GetLocalFileSize: GetLocalFileSize::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            GetFileTime: GetFileTime::<Identity, OFFSET>,
            GetLocality: GetLocality::<Identity, OFFSET>,
            EnqueueFontDownloadRequest: EnqueueFontDownloadRequest::<Identity, OFFSET>,
            EnqueueCharacterDownloadRequest: EnqueueCharacterDownloadRequest::<Identity, OFFSET>,
            EnqueueGlyphDownloadRequest: EnqueueGlyphDownloadRequest::<Identity, OFFSET>,
            EnqueueFileFragmentDownloadRequest: EnqueueFileFragmentDownloadRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFaceReference as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef"))]
impl windows_core::RuntimeName for IDWriteFontFaceReference {}
windows_core::imp::define_interface!(IDWriteFontFaceReference1, IDWriteFontFaceReference1_Vtbl, 0xc081fe77_2fd1_41ac_a5a3_34983c4ba61a);
impl core::ops::Deref for IDWriteFontFaceReference1 {
    type Target = IDWriteFontFaceReference;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFaceReference1, windows_core::IUnknown, IDWriteFontFaceReference);
impl IDWriteFontFaceReference1 {
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace5> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisValueCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFaceReference1_Vtbl {
    pub base__: IDWriteFontFaceReference_Vtbl,
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2")))]
    CreateFontFace: usize,
    pub GetFontAxisValueCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef"))]
pub trait IDWriteFontFaceReference1_Impl: IDWriteFontFaceReference_Impl {
    fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace5>;
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef"))]
impl IDWriteFontFaceReference1_Vtbl {
    pub const fn new<Identity: IDWriteFontFaceReference1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFontFace<Identity: IDWriteFontFaceReference1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFaceReference1_Impl::CreateFontFace(this) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: IDWriteFontFaceReference1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference1_Impl::GetFontAxisValueCount(this)
            }
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: IDWriteFontFaceReference1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFaceReference1_Impl::GetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
            }
        }
        Self {
            base__: IDWriteFontFaceReference_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFaceReference1 as windows_core::Interface>::IID || iid == &<IDWriteFontFaceReference as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_minwindef"))]
impl windows_core::RuntimeName for IDWriteFontFaceReference1 {}
#[cfg(feature = "Win32_dwrite_2")]
windows_core::imp::define_interface!(IDWriteFontFallback1, IDWriteFontFallback1_Vtbl, 0x2397599d_dd0d_4681_bd6a_f4f31eaade77);
#[cfg(feature = "Win32_dwrite_2")]
impl core::ops::Deref for IDWriteFontFallback1 {
    type Target = super::dwrite_2::IDWriteFontFallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite_2")]
windows_core::imp::interface_hierarchy!(IDWriteFontFallback1, windows_core::IUnknown, super::dwrite_2::IDWriteFontFallback);
#[cfg(feature = "Win32_dwrite_2")]
impl IDWriteFontFallback1 {
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
    pub unsafe fn MapCharacters<P0, P3, P4>(&self, analysissource: P0, textposition: u32, textlength: u32, basefontcollection: P3, basefamilyname: P4, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut Option<IDWriteFontFace5>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteTextAnalysisSource>,
        P3: windows_core::Param<super::dwrite::IDWriteFontCollection>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MapCharacters)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, basefontcollection.param().abi(), basefamilyname.param().abi(), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), mappedlength as _, scale as _, core::mem::transmute(mappedfontface)) }
    }
}
#[cfg(feature = "Win32_dwrite_2")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFallback1_Vtbl {
    pub base__: super::dwrite_2::IDWriteFontFallback_Vtbl,
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1"))]
    pub MapCharacters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, windows_core::PCWSTR, *const DWRITE_FONT_AXIS_VALUE, u32, *mut u32, *mut f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1")))]
    MapCharacters: usize,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteFontFallback1_Impl: super::dwrite_2::IDWriteFontFallback_Impl {
    fn MapCharacters(&self, analysissource: windows_core::Ref<super::dwrite::IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, basefontcollection: windows_core::Ref<super::dwrite::IDWriteFontCollection>, basefamilyname: &windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: windows_core::OutRef<IDWriteFontFace5>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontFallback1_Vtbl {
    pub const fn new<Identity: IDWriteFontFallback1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MapCharacters<Identity: IDWriteFontFallback1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut core::ffi::c_void, basefamilyname: windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFallback1_Impl::MapCharacters(this, core::mem::transmute_copy(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&basefontcollection), core::mem::transmute(&basefamilyname), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&mappedlength), core::mem::transmute_copy(&scale), core::mem::transmute_copy(&mappedfontface)).into()
            }
        }
        Self { base__: super::dwrite_2::IDWriteFontFallback_Vtbl::new::<Identity, OFFSET>(), MapCharacters: MapCharacters::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFallback1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteFontFallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteFontFallback1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFontFamily1, IDWriteFontFamily1_Vtbl, 0xda20d8ef_812a_4c43_9802_62ec4abd7adf);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFontFamily1 {
    type Target = super::dwrite::IDWriteFontFamily;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFontFamily1, windows_core::IUnknown, super::dwrite::IDWriteFontList, super::dwrite::IDWriteFontFamily);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontFamily1 {
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        unsafe { (windows_core::Interface::vtable(self).GetFontLocality)(windows_core::Interface::as_raw(self), listindex) }
    }
    #[cfg(all(feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub unsafe fn GetFont(&self, listindex: u32) -> windows_core::Result<IDWriteFont3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFamily1_Vtbl {
    pub base__: super::dwrite::IDWriteFontFamily_Vtbl,
    pub GetFontLocality: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_LOCALITY,
    #[cfg(all(feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite_1", feature = "Win32_dwrite_2")))]
    GetFont: usize,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteFontFamily1_Impl: super::dwrite::IDWriteFontFamily_Impl {
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
    fn GetFont(&self, listindex: u32) -> windows_core::Result<IDWriteFont3>;
    fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontFamily1_Vtbl {
    pub const fn new<Identity: IDWriteFontFamily1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontLocality<Identity: IDWriteFontFamily1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFamily1_Impl::GetFontLocality(this, core::mem::transmute_copy(&listindex))
            }
        }
        unsafe extern "system" fn GetFont<Identity: IDWriteFontFamily1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFamily1_Impl::GetFont(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        font.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: IDWriteFontFamily1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFamily1_Impl::GetFontFaceReference(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite::IDWriteFontFamily_Vtbl::new::<Identity, OFFSET>(),
            GetFontLocality: GetFontLocality::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFamily1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontList as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFamily as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteFontFamily1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFontFamily2, IDWriteFontFamily2_Vtbl, 0x3ed49e77_a398_4261_b9cf_c126c2131ef3);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFontFamily2 {
    type Target = IDWriteFontFamily1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFontFamily2, windows_core::IUnknown, super::dwrite::IDWriteFontList, super::dwrite::IDWriteFontFamily, IDWriteFontFamily1);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontFamily2 {
    pub unsafe fn GetMatchingFonts(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontList2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFamily2_Vtbl {
    pub base__: IDWriteFontFamily1_Vtbl,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteFontFamily2_Impl: IDWriteFontFamily1_Impl {
    fn GetMatchingFonts(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontList2>;
    fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontFamily2_Vtbl {
    pub const fn new<Identity: IDWriteFontFamily2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMatchingFonts<Identity: IDWriteFontFamily2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFamily2_Impl::GetMatchingFonts(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                    Ok(ok__) => {
                        matchingfonts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontSet<Identity: IDWriteFontFamily2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFamily2_Impl::GetFontSet(this) {
                    Ok(ok__) => {
                        fontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFontFamily1_Vtbl::new::<Identity, OFFSET>(),
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
            GetFontSet: GetFontSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFamily2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontList as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFamily as windows_core::Interface>::IID || iid == &<IDWriteFontFamily1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteFontFamily2 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFontList1, IDWriteFontList1_Vtbl, 0xda20d8ef_812a_4c43_9802_62ec4abd7ade);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFontList1 {
    type Target = super::dwrite::IDWriteFontList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFontList1, windows_core::IUnknown, super::dwrite::IDWriteFontList);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontList1 {
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        unsafe { (windows_core::Interface::vtable(self).GetFontLocality)(windows_core::Interface::as_raw(self), listindex) }
    }
    #[cfg(all(feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub unsafe fn GetFont(&self, listindex: u32) -> windows_core::Result<IDWriteFont3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontList1_Vtbl {
    pub base__: super::dwrite::IDWriteFontList_Vtbl,
    pub GetFontLocality: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_LOCALITY,
    #[cfg(all(feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite_1", feature = "Win32_dwrite_2")))]
    GetFont: usize,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteFontList1_Impl: super::dwrite::IDWriteFontList_Impl {
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
    fn GetFont(&self, listindex: u32) -> windows_core::Result<IDWriteFont3>;
    fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontList1_Vtbl {
    pub const fn new<Identity: IDWriteFontList1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontLocality<Identity: IDWriteFontList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontList1_Impl::GetFontLocality(this, core::mem::transmute_copy(&listindex))
            }
        }
        unsafe extern "system" fn GetFont<Identity: IDWriteFontList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontList1_Impl::GetFont(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        font.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: IDWriteFontList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontList1_Impl::GetFontFaceReference(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite::IDWriteFontList_Vtbl::new::<Identity, OFFSET>(),
            GetFontLocality: GetFontLocality::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontList1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteFontList1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFontList2, IDWriteFontList2_Vtbl, 0xc0763a34_77af_445a_b735_08c37b0a5bf5);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFontList2 {
    type Target = IDWriteFontList1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFontList2, windows_core::IUnknown, super::dwrite::IDWriteFontList, IDWriteFontList1);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontList2 {
    pub unsafe fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontList2_Vtbl {
    pub base__: IDWriteFontList1_Vtbl,
    pub GetFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteFontList2_Impl: IDWriteFontList1_Impl {
    fn GetFontSet(&self) -> windows_core::Result<IDWriteFontSet1>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontList2_Vtbl {
    pub const fn new<Identity: IDWriteFontList2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontSet<Identity: IDWriteFontList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontList2_Impl::GetFontSet(this) {
                    Ok(ok__) => {
                        fontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDWriteFontList1_Vtbl::new::<Identity, OFFSET>(), GetFontSet: GetFontSet::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontList2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontList as windows_core::Interface>::IID || iid == &<IDWriteFontList1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteFontList2 {}
windows_core::imp::define_interface!(IDWriteFontResource, IDWriteFontResource_Vtbl, 0x1f803a76_6871_48e8_987f_b975551c50f2);
windows_core::imp::interface_hierarchy!(IDWriteFontResource, windows_core::IUnknown);
impl IDWriteFontResource {
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetFontFile(&self) -> windows_core::Result<super::dwrite::IDWriteFontFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontFaceIndex(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontFaceIndex)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontAxisCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDefaultFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDefaultFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()) }
    }
    pub unsafe fn GetFontAxisRanges(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisRanges)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap()) }
    }
    pub unsafe fn GetFontAxisAttributes(&self, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisAttributes)(windows_core::Interface::as_raw(self), axisindex) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetAxisNames(&self, axisindex: u32) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAxisNames)(windows_core::Interface::as_raw(self), axisindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAxisValueNameCount(&self, axisindex: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAxisValueNameCount)(windows_core::Interface::as_raw(self), axisindex) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetAxisValueNames(&self, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut Option<super::dwrite::IDWriteLocalizedStrings>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAxisValueNames)(windows_core::Interface::as_raw(self), axisindex, axisvalueindex, fontaxisrange as _, core::mem::transmute(names)) }
    }
    pub unsafe fn HasVariations(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasVariations)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub unsafe fn CreateFontFace(&self, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontFace5> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), fontsimulations, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn CreateFontFaceReference(&self, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<IDWriteFontFaceReference1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFaceReference)(windows_core::Interface::as_raw(self), fontsimulations, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_dwrite")]
    pub GetFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetFontFile: usize,
    pub GetFontFaceIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontAxisCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDefaultFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
    pub GetFontAxisRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_RANGE, u32) -> windows_core::HRESULT,
    pub GetFontAxisAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_FONT_AXIS_ATTRIBUTES,
    #[cfg(feature = "Win32_dwrite")]
    pub GetAxisNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetAxisNames: usize,
    pub GetAxisValueNameCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    #[cfg(feature = "Win32_dwrite")]
    pub GetAxisValueNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DWRITE_FONT_AXIS_RANGE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetAxisValueNames: usize,
    pub HasVariations: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, super::dwrite::DWRITE_FONT_SIMULATIONS, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2")))]
    CreateFontFace: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub CreateFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, super::dwrite::DWRITE_FONT_SIMULATIONS, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    CreateFontFaceReference: usize,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteFontResource_Impl: windows_core::IUnknownImpl {
    fn GetFontFile(&self) -> windows_core::Result<super::dwrite::IDWriteFontFile>;
    fn GetFontFaceIndex(&self) -> u32;
    fn GetFontAxisCount(&self) -> u32;
    fn GetDefaultFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
    fn GetFontAxisRanges(&self, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> windows_core::Result<()>;
    fn GetFontAxisAttributes(&self, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES;
    fn GetAxisNames(&self, axisindex: u32) -> windows_core::Result<super::dwrite::IDWriteLocalizedStrings>;
    fn GetAxisValueNameCount(&self, axisindex: u32) -> u32;
    fn GetAxisValueNames(&self, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: windows_core::OutRef<super::dwrite::IDWriteLocalizedStrings>) -> windows_core::Result<()>;
    fn HasVariations(&self) -> windows_core::BOOL;
    fn CreateFontFace(&self, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontFace5>;
    fn CreateFontFaceReference(&self, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontFaceReference1>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontResource_Vtbl {
    pub const fn new<Identity: IDWriteFontResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontFile<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontResource_Impl::GetFontFile(this) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontResource_Impl::GetFontFaceIndex(this)
            }
        }
        unsafe extern "system" fn GetFontAxisCount<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontResource_Impl::GetFontAxisCount(this)
            }
        }
        unsafe extern "system" fn GetDefaultFontAxisValues<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontResource_Impl::GetDefaultFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
            }
        }
        unsafe extern "system" fn GetFontAxisRanges<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontResource_Impl::GetFontAxisRanges(this, core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&fontaxisrangecount)).into()
            }
        }
        unsafe extern "system" fn GetFontAxisAttributes<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontResource_Impl::GetFontAxisAttributes(this, core::mem::transmute_copy(&axisindex))
            }
        }
        unsafe extern "system" fn GetAxisNames<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisindex: u32, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontResource_Impl::GetAxisNames(this, core::mem::transmute_copy(&axisindex)) {
                    Ok(ok__) => {
                        names.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAxisValueNameCount<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisindex: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontResource_Impl::GetAxisValueNameCount(this, core::mem::transmute_copy(&axisindex))
            }
        }
        unsafe extern "system" fn GetAxisValueNames<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontResource_Impl::GetAxisValueNames(this, core::mem::transmute_copy(&axisindex), core::mem::transmute_copy(&axisvalueindex), core::mem::transmute_copy(&fontaxisrange), core::mem::transmute_copy(&names)).into()
            }
        }
        unsafe extern "system" fn HasVariations<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontResource_Impl::HasVariations(this)
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontResource_Impl::CreateFontFace(this, core::mem::transmute_copy(&fontsimulations), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFaceReference<Identity: IDWriteFontResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontResource_Impl::CreateFontFaceReference(this, core::mem::transmute_copy(&fontsimulations), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFile: GetFontFile::<Identity, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, OFFSET>,
            GetFontAxisCount: GetFontAxisCount::<Identity, OFFSET>,
            GetDefaultFontAxisValues: GetDefaultFontAxisValues::<Identity, OFFSET>,
            GetFontAxisRanges: GetFontAxisRanges::<Identity, OFFSET>,
            GetFontAxisAttributes: GetFontAxisAttributes::<Identity, OFFSET>,
            GetAxisNames: GetAxisNames::<Identity, OFFSET>,
            GetAxisValueNameCount: GetAxisValueNameCount::<Identity, OFFSET>,
            GetAxisValueNames: GetAxisValueNames::<Identity, OFFSET>,
            HasVariations: HasVariations::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            CreateFontFaceReference: CreateFontFaceReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontResource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteFontResource {}
windows_core::imp::define_interface!(IDWriteFontSet, IDWriteFontSet_Vtbl, 0x53585141_d9f8_4095_8321_d73cf6bd116b);
windows_core::imp::interface_hierarchy!(IDWriteFontSet, windows_core::IUnknown);
impl IDWriteFontSet {
    pub unsafe fn GetFontCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFaceReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindFontFaceReference)(windows_core::Interface::as_raw(self), fontfacereference.param().abi(), listindex as _, exists as _) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFace>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindFontFace)(windows_core::Interface::as_raw(self), fontface.param().abi(), listindex as _, exists as _) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetPropertyValues(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut windows_core::BOOL, values: *mut Option<super::dwrite::IDWriteLocalizedStrings>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyValues)(windows_core::Interface::as_raw(self), listindex, propertyid, exists as _, core::mem::transmute(values)) }
    }
    pub unsafe fn GetPropertyValues2<P1>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P1) -> windows_core::Result<IDWriteStringList>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyValues2)(windows_core::Interface::as_raw(self), propertyid, preferredlocalenames.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyValues3(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> windows_core::Result<IDWriteStringList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyValues3)(windows_core::Interface::as_raw(self), propertyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyOccurrenceCount)(windows_core::Interface::as_raw(self), property, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMatchingFonts(&self, properties: &[DWRITE_FONT_PROPERTY]) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetMatchingFonts2<P0>(&self, familyname: P0, fontweight: super::dwrite::DWRITE_FONT_WEIGHT, fontstretch: super::dwrite::DWRITE_FONT_STRETCH, fontstyle: super::dwrite::DWRITE_FONT_STYLE) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatchingFonts2)(windows_core::Interface::as_raw(self), familyname.param().abi(), fontweight, fontstretch, fontstyle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dwrite")]
    pub FindFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    FindFontFace: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub GetPropertyValues: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DWRITE_FONT_PROPERTY_ID, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetPropertyValues: usize,
    pub GetPropertyValues2: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_PROPERTY_ID, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyValues3: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_PROPERTY_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyOccurrenceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, *mut u32) -> windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dwrite")]
    pub GetMatchingFonts2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::dwrite::DWRITE_FONT_WEIGHT, super::dwrite::DWRITE_FONT_STRETCH, super::dwrite::DWRITE_FONT_STYLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetMatchingFonts2: usize,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteFontSet_Impl: windows_core::IUnknownImpl {
    fn GetFontCount(&self) -> u32;
    fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference>;
    fn FindFontFaceReference(&self, fontfacereference: windows_core::Ref<IDWriteFontFaceReference>, listindex: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn FindFontFace(&self, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, listindex: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetPropertyValues(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut windows_core::BOOL, values: windows_core::OutRef<super::dwrite::IDWriteLocalizedStrings>) -> windows_core::Result<()>;
    fn GetPropertyValues2(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: &windows_core::PCWSTR) -> windows_core::Result<IDWriteStringList>;
    fn GetPropertyValues3(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> windows_core::Result<IDWriteStringList>;
    fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> windows_core::Result<u32>;
    fn GetMatchingFonts(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::Result<IDWriteFontSet>;
    fn GetMatchingFonts2(&self, familyname: &windows_core::PCWSTR, fontweight: super::dwrite::DWRITE_FONT_WEIGHT, fontstretch: super::dwrite::DWRITE_FONT_STRETCH, fontstyle: super::dwrite::DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFontSet>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontSet_Vtbl {
    pub const fn new<Identity: IDWriteFontSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontCount<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet_Impl::GetFontCount(this)
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet_Impl::GetFontFaceReference(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFontFaceReference<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut core::ffi::c_void, listindex: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet_Impl::FindFontFaceReference(this, core::mem::transmute_copy(&fontfacereference), core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&exists)).into()
            }
        }
        unsafe extern "system" fn FindFontFace<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, listindex: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet_Impl::FindFontFace(this, core::mem::transmute_copy(&fontface), core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&exists)).into()
            }
        }
        unsafe extern "system" fn GetPropertyValues<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut windows_core::BOOL, values: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet_Impl::GetPropertyValues(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&exists), core::mem::transmute_copy(&values)).into()
            }
        }
        unsafe extern "system" fn GetPropertyValues2<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: windows_core::PCWSTR, values: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet_Impl::GetPropertyValues2(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&preferredlocalenames)) {
                    Ok(ok__) => {
                        values.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyValues3<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, values: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet_Impl::GetPropertyValues3(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        values.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyOccurrenceCount<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *const DWRITE_FONT_PROPERTY, propertyoccurrencecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet_Impl::GetPropertyOccurrenceCount(this, core::mem::transmute_copy(&property)) {
                    Ok(ok__) => {
                        propertyoccurrencecount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, filteredset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet_Impl::GetMatchingFonts(this, core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount)) {
                    Ok(ok__) => {
                        filteredset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMatchingFonts2<Identity: IDWriteFontSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: windows_core::PCWSTR, fontweight: super::dwrite::DWRITE_FONT_WEIGHT, fontstretch: super::dwrite::DWRITE_FONT_STRETCH, fontstyle: super::dwrite::DWRITE_FONT_STYLE, filteredset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet_Impl::GetMatchingFonts2(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&fontweight), core::mem::transmute_copy(&fontstretch), core::mem::transmute_copy(&fontstyle)) {
                    Ok(ok__) => {
                        filteredset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontCount: GetFontCount::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
            FindFontFaceReference: FindFontFaceReference::<Identity, OFFSET>,
            FindFontFace: FindFontFace::<Identity, OFFSET>,
            GetPropertyValues: GetPropertyValues::<Identity, OFFSET>,
            GetPropertyValues2: GetPropertyValues2::<Identity, OFFSET>,
            GetPropertyValues3: GetPropertyValues3::<Identity, OFFSET>,
            GetPropertyOccurrenceCount: GetPropertyOccurrenceCount::<Identity, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
            GetMatchingFonts2: GetMatchingFonts2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteFontSet {}
windows_core::imp::define_interface!(IDWriteFontSet1, IDWriteFontSet1_Vtbl, 0x7e9fda85_6c92_4053_bc47_7ae3530db4d3);
impl core::ops::Deref for IDWriteFontSet1 {
    type Target = IDWriteFontSet;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet1, windows_core::IUnknown, IDWriteFontSet);
impl IDWriteFontSet1 {
    pub unsafe fn GetMatchingFonts(&self, fontproperty: Option<*const DWRITE_FONT_PROPERTY>, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), fontproperty.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstFontResources(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstFontResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFilteredFonts(&self, properties: Option<&[DWRITE_FONT_PROPERTY]>, selectanyproperty: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilteredFonts)(windows_core::Interface::as_raw(self), core::mem::transmute(properties.map_or(core::ptr::null(), |slice| slice.as_ptr())), properties.map_or(0, |slice| slice.len().try_into().unwrap()), selectanyproperty.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFilteredFonts2(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilteredFonts2)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFilteredFonts3(&self, indices: &[u32]) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilteredFonts3)(windows_core::Interface::as_raw(self), core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFilteredFontIndices(&self, properties: &[DWRITE_FONT_PROPERTY], selectanyproperty: bool, indices: &mut [u32], actualindexcount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFilteredFontIndices)(windows_core::Interface::as_raw(self), core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), selectanyproperty.into(), core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount as _) }
    }
    pub unsafe fn GetFilteredFontIndices2(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: bool, indices: &mut [u32], actualindexcount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFilteredFontIndices2)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into(), core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount as _) }
    }
    pub unsafe fn GetFontAxisRanges(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisRanges)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount as _) }
    }
    pub unsafe fn GetFontAxisRanges2(&self, listindex: u32, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisRanges2)(windows_core::Interface::as_raw(self), listindex, core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount as _) }
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFaceReference)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontResource(&self, listindex: u32) -> windows_core::Result<IDWriteFontResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontResource)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub unsafe fn CreateFontFace(&self, listindex: u32) -> windows_core::Result<IDWriteFontFace5> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), listindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        unsafe { (windows_core::Interface::vtable(self).GetFontLocality)(windows_core::Interface::as_raw(self), listindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet1_Vtbl {
    pub base__: IDWriteFontSet_Vtbl,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, *const DWRITE_FONT_AXIS_VALUE, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstFontResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredFonts: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, u32, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredFonts2: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_RANGE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredFonts3: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredFontIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, u32, windows_core::BOOL, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFilteredFontIndices2: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_RANGE, u32, windows_core::BOOL, *mut u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFontAxisRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_RANGE, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFontAxisRanges2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_AXIS_RANGE, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontResource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2")))]
    CreateFontFace: usize,
    pub GetFontLocality: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_LOCALITY,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteFontSet1_Impl: IDWriteFontSet_Impl {
    fn GetMatchingFonts(&self, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFirstFontResources(&self) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: windows_core::BOOL) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts2(&self, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: windows_core::BOOL) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts3(&self, indices: *const u32, indexcount: u32) -> windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFontIndices(&self, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: windows_core::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> windows_core::Result<()>;
    fn GetFilteredFontIndices2(&self, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: windows_core::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> windows_core::Result<()>;
    fn GetFontAxisRanges(&self, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> windows_core::Result<()>;
    fn GetFontAxisRanges2(&self, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> windows_core::Result<()>;
    fn GetFontFaceReference(&self, listindex: u32) -> windows_core::Result<IDWriteFontFaceReference1>;
    fn CreateFontResource(&self, listindex: u32) -> windows_core::Result<IDWriteFontResource>;
    fn CreateFontFace(&self, listindex: u32) -> windows_core::Result<IDWriteFontFace5>;
    fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteFontSet1_Vtbl {
    pub const fn new<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMatchingFonts<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet1_Impl::GetMatchingFonts(this, core::mem::transmute_copy(&fontproperty), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)) {
                    Ok(ok__) => {
                        matchingfonts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFirstFontResources<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filteredfontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet1_Impl::GetFirstFontResources(this) {
                    Ok(ok__) => {
                        filteredfontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilteredFonts<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: windows_core::BOOL, filteredfontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet1_Impl::GetFilteredFonts(this, core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount), core::mem::transmute_copy(&selectanyproperty)) {
                    Ok(ok__) => {
                        filteredfontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilteredFonts2<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: windows_core::BOOL, filteredfontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet1_Impl::GetFilteredFonts2(this, core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&fontaxisrangecount), core::mem::transmute_copy(&selectanyrange)) {
                    Ok(ok__) => {
                        filteredfontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilteredFonts3<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indices: *const u32, indexcount: u32, filteredfontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet1_Impl::GetFilteredFonts3(this, core::mem::transmute_copy(&indices), core::mem::transmute_copy(&indexcount)) {
                    Ok(ok__) => {
                        filteredfontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilteredFontIndices<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: windows_core::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet1_Impl::GetFilteredFontIndices(this, core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount), core::mem::transmute_copy(&selectanyproperty), core::mem::transmute_copy(&indices), core::mem::transmute_copy(&maxindexcount), core::mem::transmute_copy(&actualindexcount)).into()
            }
        }
        unsafe extern "system" fn GetFilteredFontIndices2<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: windows_core::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet1_Impl::GetFilteredFontIndices2(this, core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&fontaxisrangecount), core::mem::transmute_copy(&selectanyrange), core::mem::transmute_copy(&indices), core::mem::transmute_copy(&maxindexcount), core::mem::transmute_copy(&actualindexcount)).into()
            }
        }
        unsafe extern "system" fn GetFontAxisRanges<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet1_Impl::GetFontAxisRanges(this, core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&maxfontaxisrangecount), core::mem::transmute_copy(&actualfontaxisrangecount)).into()
            }
        }
        unsafe extern "system" fn GetFontAxisRanges2<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet1_Impl::GetFontAxisRanges2(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&maxfontaxisrangecount), core::mem::transmute_copy(&actualfontaxisrangecount)).into()
            }
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet1_Impl::GetFontFaceReference(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        fontfacereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontResource<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet1_Impl::CreateFontResource(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        fontresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet1_Impl::CreateFontFace(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontLocality<Identity: IDWriteFontSet1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet1_Impl::GetFontLocality(this, core::mem::transmute_copy(&listindex))
            }
        }
        Self {
            base__: IDWriteFontSet_Vtbl::new::<Identity, OFFSET>(),
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
            GetFirstFontResources: GetFirstFontResources::<Identity, OFFSET>,
            GetFilteredFonts: GetFilteredFonts::<Identity, OFFSET>,
            GetFilteredFonts2: GetFilteredFonts2::<Identity, OFFSET>,
            GetFilteredFonts3: GetFilteredFonts3::<Identity, OFFSET>,
            GetFilteredFontIndices: GetFilteredFontIndices::<Identity, OFFSET>,
            GetFilteredFontIndices2: GetFilteredFontIndices2::<Identity, OFFSET>,
            GetFontAxisRanges: GetFontAxisRanges::<Identity, OFFSET>,
            GetFontAxisRanges2: GetFontAxisRanges2::<Identity, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            GetFontLocality: GetFontLocality::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet1 as windows_core::Interface>::IID || iid == &<IDWriteFontSet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteFontSet1 {}
windows_core::imp::define_interface!(IDWriteFontSet2, IDWriteFontSet2_Vtbl, 0xdc7ead19_e54c_43af_b2da_4e2b79ba3f7f);
impl core::ops::Deref for IDWriteFontSet2 {
    type Target = IDWriteFontSet1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet2, windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1);
impl IDWriteFontSet2 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetExpirationEvent(&self) -> super::winnt::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).GetExpirationEvent)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet2_Vtbl {
    pub base__: IDWriteFontSet1_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub GetExpirationEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::winnt::HANDLE,
    #[cfg(not(feature = "Win32_winnt"))]
    GetExpirationEvent: usize,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
pub trait IDWriteFontSet2_Impl: IDWriteFontSet1_Impl {
    fn GetExpirationEvent(&self) -> super::winnt::HANDLE;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
impl IDWriteFontSet2_Vtbl {
    pub const fn new<Identity: IDWriteFontSet2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExpirationEvent<Identity: IDWriteFontSet2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::winnt::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet2_Impl::GetExpirationEvent(this)
            }
        }
        Self { base__: IDWriteFontSet1_Vtbl::new::<Identity, OFFSET>(), GetExpirationEvent: GetExpirationEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet2 as windows_core::Interface>::IID || iid == &<IDWriteFontSet as windows_core::Interface>::IID || iid == &<IDWriteFontSet1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDWriteFontSet2 {}
windows_core::imp::define_interface!(IDWriteFontSet3, IDWriteFontSet3_Vtbl, 0x7c073ef2_a7f4_4045_8c32_8ab8ae640f90);
impl core::ops::Deref for IDWriteFontSet3 {
    type Target = IDWriteFontSet2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet3, windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1, IDWriteFontSet2);
impl IDWriteFontSet3 {
    pub unsafe fn GetFontSourceType(&self, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetFontSourceType)(windows_core::Interface::as_raw(self), fontindex) }
    }
    pub unsafe fn GetFontSourceNameLength(&self, listindex: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontSourceNameLength)(windows_core::Interface::as_raw(self), listindex) }
    }
    pub unsafe fn GetFontSourceName(&self, listindex: u32, stringbuffer: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontSourceName)(windows_core::Interface::as_raw(self), listindex, core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet3_Vtbl {
    pub base__: IDWriteFontSet2_Vtbl,
    pub GetFontSourceType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> DWRITE_FONT_SOURCE_TYPE,
    pub GetFontSourceNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetFontSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
pub trait IDWriteFontSet3_Impl: IDWriteFontSet2_Impl {
    fn GetFontSourceType(&self, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE;
    fn GetFontSourceNameLength(&self, listindex: u32) -> u32;
    fn GetFontSourceName(&self, listindex: u32, stringbuffer: *mut u16, stringbuffersize: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
impl IDWriteFontSet3_Vtbl {
    pub const fn new<Identity: IDWriteFontSet3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontSourceType<Identity: IDWriteFontSet3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet3_Impl::GetFontSourceType(this, core::mem::transmute_copy(&fontindex))
            }
        }
        unsafe extern "system" fn GetFontSourceNameLength<Identity: IDWriteFontSet3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet3_Impl::GetFontSourceNameLength(this, core::mem::transmute_copy(&listindex))
            }
        }
        unsafe extern "system" fn GetFontSourceName<Identity: IDWriteFontSet3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, stringbuffer: *mut u16, stringbuffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet3_Impl::GetFontSourceName(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&stringbuffer), core::mem::transmute_copy(&stringbuffersize)).into()
            }
        }
        Self {
            base__: IDWriteFontSet2_Vtbl::new::<Identity, OFFSET>(),
            GetFontSourceType: GetFontSourceType::<Identity, OFFSET>,
            GetFontSourceNameLength: GetFontSourceNameLength::<Identity, OFFSET>,
            GetFontSourceName: GetFontSourceName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet3 as windows_core::Interface>::IID || iid == &<IDWriteFontSet as windows_core::Interface>::IID || iid == &<IDWriteFontSet1 as windows_core::Interface>::IID || iid == &<IDWriteFontSet2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDWriteFontSet3 {}
windows_core::imp::define_interface!(IDWriteFontSet4, IDWriteFontSet4_Vtbl, 0xeec175fc_bea9_4c86_8b53_ccbdd7df0c82);
impl core::ops::Deref for IDWriteFontSet4 {
    type Target = IDWriteFontSet3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSet4, windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1, IDWriteFontSet2, IDWriteFontSet3);
impl IDWriteFontSet4 {
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn ConvertWeightStretchStyleToFontAxisValues(&self, inputaxisvalues: Option<&[DWRITE_FONT_AXIS_VALUE]>, fontweight: super::dwrite::DWRITE_FONT_WEIGHT, fontstretch: super::dwrite::DWRITE_FONT_STRETCH, fontstyle: super::dwrite::DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).ConvertWeightStretchStyleToFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(inputaxisvalues.map_or(core::ptr::null(), |slice| slice.as_ptr())), inputaxisvalues.map_or(0, |slice| slice.len().try_into().unwrap()), fontweight, fontstretch, fontstyle, fontsize, outputaxisvalues as _) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], allowedsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), familyname.param().abi(), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), allowedsimulations, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet4_Vtbl {
    pub base__: IDWriteFontSet3_Vtbl,
    #[cfg(feature = "Win32_dwrite")]
    pub ConvertWeightStretchStyleToFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32, super::dwrite::DWRITE_FONT_WEIGHT, super::dwrite::DWRITE_FONT_STRETCH, super::dwrite::DWRITE_FONT_STYLE, f32, *mut DWRITE_FONT_AXIS_VALUE) -> u32,
    #[cfg(not(feature = "Win32_dwrite"))]
    ConvertWeightStretchStyleToFontAxisValues: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const DWRITE_FONT_AXIS_VALUE, u32, super::dwrite::DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetMatchingFonts: usize,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
pub trait IDWriteFontSet4_Impl: IDWriteFontSet3_Impl {
    fn ConvertWeightStretchStyleToFontAxisValues(&self, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: super::dwrite::DWRITE_FONT_WEIGHT, fontstretch: super::dwrite::DWRITE_FONT_STRETCH, fontstyle: super::dwrite::DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32;
    fn GetMatchingFonts(&self, familyname: &windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontSet4>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
impl IDWriteFontSet4_Vtbl {
    pub const fn new<Identity: IDWriteFontSet4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConvertWeightStretchStyleToFontAxisValues<Identity: IDWriteFontSet4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: super::dwrite::DWRITE_FONT_WEIGHT, fontstretch: super::dwrite::DWRITE_FONT_STRETCH, fontstyle: super::dwrite::DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSet4_Impl::ConvertWeightStretchStyleToFontAxisValues(this, core::mem::transmute_copy(&inputaxisvalues), core::mem::transmute_copy(&inputaxiscount), core::mem::transmute_copy(&fontweight), core::mem::transmute_copy(&fontstretch), core::mem::transmute_copy(&fontstyle), core::mem::transmute_copy(&fontsize), core::mem::transmute_copy(&outputaxisvalues))
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: IDWriteFontSet4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, matchingfonts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSet4_Impl::GetMatchingFonts(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&allowedsimulations)) {
                    Ok(ok__) => {
                        matchingfonts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFontSet3_Vtbl::new::<Identity, OFFSET>(),
            ConvertWeightStretchStyleToFontAxisValues: ConvertWeightStretchStyleToFontAxisValues::<Identity, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSet4 as windows_core::Interface>::IID || iid == &<IDWriteFontSet as windows_core::Interface>::IID || iid == &<IDWriteFontSet1 as windows_core::Interface>::IID || iid == &<IDWriteFontSet2 as windows_core::Interface>::IID || iid == &<IDWriteFontSet3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDWriteFontSet4 {}
windows_core::imp::define_interface!(IDWriteFontSetBuilder, IDWriteFontSetBuilder_Vtbl, 0x2f642afe_9c68_4f40_b8be_457401afcb3d);
windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder, windows_core::IUnknown);
impl IDWriteFontSetBuilder {
    pub unsafe fn AddFontFaceReference<P0>(&self, fontfacereference: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFaceReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFontFaceReference)(windows_core::Interface::as_raw(self), fontfacereference.param().abi()) }
    }
    pub unsafe fn AddFontFaceReference2<P0>(&self, fontfacereference: P0, properties: &[DWRITE_FONT_PROPERTY]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFaceReference>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFontFaceReference2)(windows_core::Interface::as_raw(self), fontfacereference.param().abi(), core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap()) }
    }
    pub unsafe fn AddFontSet<P0>(&self, fontset: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFontSet)(windows_core::Interface::as_raw(self), fontset.param().abi()) }
    }
    pub unsafe fn CreateFontSet(&self) -> windows_core::Result<IDWriteFontSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSetBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFontFaceReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddFontFaceReference2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DWRITE_FONT_PROPERTY, u32) -> windows_core::HRESULT,
    pub AddFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteFontSetBuilder_Impl: windows_core::IUnknownImpl {
    fn AddFontFaceReference(&self, fontfacereference: windows_core::Ref<IDWriteFontFaceReference>) -> windows_core::Result<()>;
    fn AddFontFaceReference2(&self, fontfacereference: windows_core::Ref<IDWriteFontFaceReference>, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::Result<()>;
    fn AddFontSet(&self, fontset: windows_core::Ref<IDWriteFontSet>) -> windows_core::Result<()>;
    fn CreateFontSet(&self) -> windows_core::Result<IDWriteFontSet>;
}
impl IDWriteFontSetBuilder_Vtbl {
    pub const fn new<Identity: IDWriteFontSetBuilder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddFontFaceReference<Identity: IDWriteFontSetBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSetBuilder_Impl::AddFontFaceReference(this, core::mem::transmute_copy(&fontfacereference)).into()
            }
        }
        unsafe extern "system" fn AddFontFaceReference2<Identity: IDWriteFontSetBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacereference: *mut core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSetBuilder_Impl::AddFontFaceReference2(this, core::mem::transmute_copy(&fontfacereference), core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount)).into()
            }
        }
        unsafe extern "system" fn AddFontSet<Identity: IDWriteFontSetBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSetBuilder_Impl::AddFontSet(this, core::mem::transmute_copy(&fontset)).into()
            }
        }
        unsafe extern "system" fn CreateFontSet<Identity: IDWriteFontSetBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontSetBuilder_Impl::CreateFontSet(this) {
                    Ok(ok__) => {
                        fontset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFontFaceReference: AddFontFaceReference::<Identity, OFFSET>,
            AddFontFaceReference2: AddFontFaceReference2::<Identity, OFFSET>,
            AddFontSet: AddFontSet::<Identity, OFFSET>,
            CreateFontSet: CreateFontSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontSetBuilder {}
windows_core::imp::define_interface!(IDWriteFontSetBuilder1, IDWriteFontSetBuilder1_Vtbl, 0x3ff7715f_3cdc_4dc6_9b72_ec5621dccafd);
impl core::ops::Deref for IDWriteFontSetBuilder1 {
    type Target = IDWriteFontSetBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder1, windows_core::IUnknown, IDWriteFontSetBuilder);
impl IDWriteFontSetBuilder1 {
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn AddFontFile<P0>(&self, fontfile: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFile>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFontFile)(windows_core::Interface::as_raw(self), fontfile.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSetBuilder1_Vtbl {
    pub base__: IDWriteFontSetBuilder_Vtbl,
    #[cfg(feature = "Win32_dwrite")]
    pub AddFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    AddFontFile: usize,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteFontSetBuilder1_Impl: IDWriteFontSetBuilder_Impl {
    fn AddFontFile(&self, fontfile: windows_core::Ref<super::dwrite::IDWriteFontFile>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontSetBuilder1_Vtbl {
    pub const fn new<Identity: IDWriteFontSetBuilder1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddFontFile<Identity: IDWriteFontSetBuilder1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSetBuilder1_Impl::AddFontFile(this, core::mem::transmute_copy(&fontfile)).into()
            }
        }
        Self { base__: IDWriteFontSetBuilder_Vtbl::new::<Identity, OFFSET>(), AddFontFile: AddFontFile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder1 as windows_core::Interface>::IID || iid == &<IDWriteFontSetBuilder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteFontSetBuilder1 {}
windows_core::imp::define_interface!(IDWriteFontSetBuilder2, IDWriteFontSetBuilder2_Vtbl, 0xee5ba612_b131_463c_8f4f_3189b9401e45);
impl core::ops::Deref for IDWriteFontSetBuilder2 {
    type Target = IDWriteFontSetBuilder1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder2, windows_core::IUnknown, IDWriteFontSetBuilder, IDWriteFontSetBuilder1);
impl IDWriteFontSetBuilder2 {
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn AddFont<P0>(&self, fontfile: P0, fontfaceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], properties: &[DWRITE_FONT_PROPERTY]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFile>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFont)(windows_core::Interface::as_raw(self), fontfile.param().abi(), fontfaceindex, fontsimulations, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap()) }
    }
    pub unsafe fn AddFontFile<P0>(&self, filepath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFontFile)(windows_core::Interface::as_raw(self), filepath.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSetBuilder2_Vtbl {
    pub base__: IDWriteFontSetBuilder1_Vtbl,
    #[cfg(feature = "Win32_dwrite")]
    pub AddFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::dwrite::DWRITE_FONT_SIMULATIONS, *const DWRITE_FONT_AXIS_VALUE, u32, *const DWRITE_FONT_AXIS_RANGE, u32, *const DWRITE_FONT_PROPERTY, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    AddFont: usize,
    pub AddFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteFontSetBuilder2_Impl: IDWriteFontSetBuilder1_Impl {
    fn AddFont(&self, fontfile: windows_core::Ref<super::dwrite::IDWriteFontFile>, fontfaceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::Result<()>;
    fn AddFontFile(&self, filepath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontSetBuilder2_Vtbl {
    pub const fn new<Identity: IDWriteFontSetBuilder2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddFont<Identity: IDWriteFontSetBuilder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut core::ffi::c_void, fontfaceindex: u32, fontsimulations: super::dwrite::DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSetBuilder2_Impl::AddFont(this, core::mem::transmute_copy(&fontfile), core::mem::transmute_copy(&fontfaceindex), core::mem::transmute_copy(&fontsimulations), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&fontaxisranges), core::mem::transmute_copy(&fontaxisrangecount), core::mem::transmute_copy(&properties), core::mem::transmute_copy(&propertycount)).into()
            }
        }
        unsafe extern "system" fn AddFontFile<Identity: IDWriteFontSetBuilder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontSetBuilder2_Impl::AddFontFile(this, core::mem::transmute(&filepath)).into()
            }
        }
        Self {
            base__: IDWriteFontSetBuilder1_Vtbl::new::<Identity, OFFSET>(),
            AddFont: AddFont::<Identity, OFFSET>,
            AddFontFile: AddFontFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontSetBuilder2 as windows_core::Interface>::IID || iid == &<IDWriteFontSetBuilder as windows_core::Interface>::IID || iid == &<IDWriteFontSetBuilder1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteFontSetBuilder2 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteGdiInterop1, IDWriteGdiInterop1_Vtbl, 0x4556be70_3abd_4f70_90be_421780a6f515);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteGdiInterop1 {
    type Target = super::dwrite::IDWriteGdiInterop;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteGdiInterop1, windows_core::IUnknown, super::dwrite::IDWriteGdiInterop);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteGdiInterop1 {
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn CreateFontFromLOGFONT<P1>(&self, logfont: *const super::wingdi::LOGFONTW, fontcollection: P1) -> windows_core::Result<super::dwrite::IDWriteFont>
    where
        P1: windows_core::Param<super::dwrite::IDWriteFontCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFromLOGFONT)(windows_core::Interface::as_raw(self), logfont, fontcollection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn GetFontSignature<P0>(&self, font: P0, fontsignature: *mut super::wingdi::FONTSIGNATURE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFont>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFontSignature)(windows_core::Interface::as_raw(self), font.param().abi(), fontsignature as _) }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn GetFontSignature2<P0>(&self, fontface: P0, fontsignature: *mut super::wingdi::FONTSIGNATURE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFace>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFontSignature2)(windows_core::Interface::as_raw(self), fontface.param().abi(), fontsignature as _) }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn GetMatchingFontsByLOGFONT<P1>(&self, logfont: *const super::wingdi::LOGFONT, fontset: P1) -> windows_core::Result<IDWriteFontSet>
    where
        P1: windows_core::Param<IDWriteFontSet>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatchingFontsByLOGFONT)(windows_core::Interface::as_raw(self), logfont, fontset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteGdiInterop1_Vtbl {
    pub base__: super::dwrite::IDWriteGdiInterop_Vtbl,
    #[cfg(feature = "Win32_wingdi")]
    pub CreateFontFromLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wingdi::LOGFONTW, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    CreateFontFromLOGFONT: usize,
    #[cfg(feature = "Win32_wingdi")]
    pub GetFontSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wingdi::FONTSIGNATURE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    GetFontSignature: usize,
    #[cfg(feature = "Win32_wingdi")]
    pub GetFontSignature2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wingdi::FONTSIGNATURE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    GetFontSignature2: usize,
    #[cfg(feature = "Win32_wingdi")]
    pub GetMatchingFontsByLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wingdi::LOGFONT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    GetMatchingFontsByLOGFONT: usize,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub trait IDWriteGdiInterop1_Impl: super::dwrite::IDWriteGdiInterop_Impl {
    fn CreateFontFromLOGFONT(&self, logfont: *const super::wingdi::LOGFONTW, fontcollection: windows_core::Ref<super::dwrite::IDWriteFontCollection>) -> windows_core::Result<super::dwrite::IDWriteFont>;
    fn GetFontSignature(&self, font: windows_core::Ref<super::dwrite::IDWriteFont>, fontsignature: *mut super::wingdi::FONTSIGNATURE) -> windows_core::Result<()>;
    fn GetFontSignature2(&self, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, fontsignature: *mut super::wingdi::FONTSIGNATURE) -> windows_core::Result<()>;
    fn GetMatchingFontsByLOGFONT(&self, logfont: *const super::wingdi::LOGFONT, fontset: windows_core::Ref<IDWriteFontSet>) -> windows_core::Result<IDWriteFontSet>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl IDWriteGdiInterop1_Vtbl {
    pub const fn new<Identity: IDWriteGdiInterop1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFontFromLOGFONT<Identity: IDWriteGdiInterop1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfont: *const super::wingdi::LOGFONTW, fontcollection: *mut core::ffi::c_void, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteGdiInterop1_Impl::CreateFontFromLOGFONT(this, core::mem::transmute_copy(&logfont), core::mem::transmute_copy(&fontcollection)) {
                    Ok(ok__) => {
                        font.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontSignature<Identity: IDWriteGdiInterop1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, font: *mut core::ffi::c_void, fontsignature: *mut super::wingdi::FONTSIGNATURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteGdiInterop1_Impl::GetFontSignature(this, core::mem::transmute_copy(&font), core::mem::transmute_copy(&fontsignature)).into()
            }
        }
        unsafe extern "system" fn GetFontSignature2<Identity: IDWriteGdiInterop1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, fontsignature: *mut super::wingdi::FONTSIGNATURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteGdiInterop1_Impl::GetFontSignature2(this, core::mem::transmute_copy(&fontface), core::mem::transmute_copy(&fontsignature)).into()
            }
        }
        unsafe extern "system" fn GetMatchingFontsByLOGFONT<Identity: IDWriteGdiInterop1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfont: *const super::wingdi::LOGFONT, fontset: *mut core::ffi::c_void, filteredset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteGdiInterop1_Impl::GetMatchingFontsByLOGFONT(this, core::mem::transmute_copy(&logfont), core::mem::transmute_copy(&fontset)) {
                    Ok(ok__) => {
                        filteredset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite::IDWriteGdiInterop_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFromLOGFONT: CreateFontFromLOGFONT::<Identity, OFFSET>,
            GetFontSignature: GetFontSignature::<Identity, OFFSET>,
            GetFontSignature2: GetFontSignature2::<Identity, OFFSET>,
            GetMatchingFontsByLOGFONT: GetMatchingFontsByLOGFONT::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteGdiInterop1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteGdiInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl windows_core::RuntimeName for IDWriteGdiInterop1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteInMemoryFontFileLoader, IDWriteInMemoryFontFileLoader_Vtbl, 0xdc102f47_a12d_4b1c_822d_9e117e33043f);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteInMemoryFontFileLoader {
    type Target = super::dwrite::IDWriteFontFileLoader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteInMemoryFontFileLoader, windows_core::IUnknown, super::dwrite::IDWriteFontFileLoader);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteInMemoryFontFileLoader {
    pub unsafe fn CreateInMemoryFontFileReference<P0, P3>(&self, factory: P0, fontdata: *const core::ffi::c_void, fontdatasize: u32, ownerobject: P3) -> windows_core::Result<super::dwrite::IDWriteFontFile>
    where
        P0: windows_core::Param<super::dwrite::IDWriteFactory>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInMemoryFontFileReference)(windows_core::Interface::as_raw(self), factory.param().abi(), fontdata, fontdatasize, ownerobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFileCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFileCount)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteInMemoryFontFileLoader_Vtbl {
    pub base__: super::dwrite::IDWriteFontFileLoader_Vtbl,
    pub CreateInMemoryFontFileReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteInMemoryFontFileLoader_Impl: super::dwrite::IDWriteFontFileLoader_Impl {
    fn CreateInMemoryFontFileReference(&self, factory: windows_core::Ref<super::dwrite::IDWriteFactory>, fontdata: *const core::ffi::c_void, fontdatasize: u32, ownerobject: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<super::dwrite::IDWriteFontFile>;
    fn GetFileCount(&self) -> u32;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteInMemoryFontFileLoader_Vtbl {
    pub const fn new<Identity: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInMemoryFontFileReference<Identity: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, fontdata: *const core::ffi::c_void, fontdatasize: u32, ownerobject: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteInMemoryFontFileLoader_Impl::CreateInMemoryFontFileReference(this, core::mem::transmute_copy(&factory), core::mem::transmute_copy(&fontdata), core::mem::transmute_copy(&fontdatasize), core::mem::transmute_copy(&ownerobject)) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileCount<Identity: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteInMemoryFontFileLoader_Impl::GetFileCount(this)
            }
        }
        Self {
            base__: super::dwrite::IDWriteFontFileLoader_Vtbl::new::<Identity, OFFSET>(),
            CreateInMemoryFontFileReference: CreateInMemoryFontFileReference::<Identity, OFFSET>,
            GetFileCount: GetFileCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteInMemoryFontFileLoader as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFileLoader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteInMemoryFontFileLoader {}
windows_core::imp::define_interface!(IDWritePaintReader, IDWritePaintReader_Vtbl, 0x8128e912_3b97_42a5_ab6c_24aad3a86e54);
windows_core::imp::interface_hierarchy!(IDWritePaintReader, windows_core::IUnknown);
impl IDWritePaintReader {
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub unsafe fn SetCurrentGlyph(&self, glyphindex: u32, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32, clipbox: *mut super::dcommon::D2D_RECT_F, glyphattributes: Option<*mut DWRITE_PAINT_ATTRIBUTES>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentGlyph)(windows_core::Interface::as_raw(self), glyphindex, paintelement as _, structsize, clipbox as _, glyphattributes.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub unsafe fn SetTextColor(&self, textcolor: *const super::dwrite_2::DWRITE_COLOR_F) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextColor)(windows_core::Interface::as_raw(self), textcolor) }
    }
    pub unsafe fn SetColorPaletteIndex(&self, colorpaletteindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorPaletteIndex)(windows_core::Interface::as_raw(self), colorpaletteindex) }
    }
    #[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub unsafe fn SetCustomColorPalette(&self, paletteentries: &[super::dwrite_2::DWRITE_COLOR_F]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCustomColorPalette)(windows_core::Interface::as_raw(self), core::mem::transmute(paletteentries.as_ptr()), paletteentries.len().try_into().unwrap()) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub unsafe fn MoveToFirstChild(&self, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveToFirstChild)(windows_core::Interface::as_raw(self), paintelement as _, structsize) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub unsafe fn MoveToNextSibling(&self, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveToNextSibling)(windows_core::Interface::as_raw(self), paintelement as _, structsize) }
    }
    pub unsafe fn MoveToParent(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveToParent)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn GetGradientStops(&self, firstgradientstopindex: u32, gradientstops: &mut [super::d2d1::D2D1_GRADIENT_STOP]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGradientStops)(windows_core::Interface::as_raw(self), firstgradientstopindex, gradientstops.len().try_into().unwrap(), core::mem::transmute(gradientstops.as_ptr())) }
    }
    #[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub unsafe fn GetGradientStopColors(&self, firstgradientstopindex: u32, gradientstopcolors: &mut [DWRITE_PAINT_COLOR]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGradientStopColors)(windows_core::Interface::as_raw(self), firstgradientstopindex, gradientstopcolors.len().try_into().unwrap(), core::mem::transmute(gradientstopcolors.as_ptr())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWritePaintReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub SetCurrentGlyph: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_PAINT_ELEMENT, u32, *mut super::dcommon::D2D_RECT_F, *mut DWRITE_PAINT_ATTRIBUTES) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype")))]
    SetCurrentGlyph: usize,
    #[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub SetTextColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dwrite_2::DWRITE_COLOR_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype")))]
    SetTextColor: usize,
    pub SetColorPaletteIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub SetCustomColorPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dwrite_2::DWRITE_COLOR_F, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype")))]
    SetCustomColorPalette: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub MoveToFirstChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_PAINT_ELEMENT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype")))]
    MoveToFirstChild: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub MoveToNextSibling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_PAINT_ELEMENT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype")))]
    MoveToNextSibling: usize,
    pub MoveToParent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub GetGradientStops: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::d2d1::D2D1_GRADIENT_STOP) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    GetGradientStops: usize,
    #[cfg(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
    pub GetGradientStopColors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DWRITE_PAINT_COLOR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dwrite_2", feature = "Win32_dxgitype")))]
    GetGradientStopColors: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
pub trait IDWritePaintReader_Impl: windows_core::IUnknownImpl {
    fn SetCurrentGlyph(&self, glyphindex: u32, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32, clipbox: *mut super::dcommon::D2D_RECT_F, glyphattributes: *mut DWRITE_PAINT_ATTRIBUTES) -> windows_core::Result<()>;
    fn SetTextColor(&self, textcolor: *const super::dwrite_2::DWRITE_COLOR_F) -> windows_core::Result<()>;
    fn SetColorPaletteIndex(&self, colorpaletteindex: u32) -> windows_core::Result<()>;
    fn SetCustomColorPalette(&self, paletteentries: *const super::dwrite_2::DWRITE_COLOR_F, paletteentrycount: u32) -> windows_core::Result<()>;
    fn MoveToFirstChild(&self, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::Result<()>;
    fn MoveToNextSibling(&self, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::Result<()>;
    fn MoveToParent(&self) -> windows_core::Result<()>;
    fn GetGradientStops(&self, firstgradientstopindex: u32, gradientstopcount: u32, gradientstops: *mut super::d2d1::D2D1_GRADIENT_STOP) -> windows_core::Result<()>;
    fn GetGradientStopColors(&self, firstgradientstopindex: u32, gradientstopcount: u32, gradientstopcolors: *mut DWRITE_PAINT_COLOR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl IDWritePaintReader_Vtbl {
    pub const fn new<Identity: IDWritePaintReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCurrentGlyph<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphindex: u32, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32, clipbox: *mut super::dcommon::D2D_RECT_F, glyphattributes: *mut DWRITE_PAINT_ATTRIBUTES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::SetCurrentGlyph(this, core::mem::transmute_copy(&glyphindex), core::mem::transmute_copy(&paintelement), core::mem::transmute_copy(&structsize), core::mem::transmute_copy(&clipbox), core::mem::transmute_copy(&glyphattributes)).into()
            }
        }
        unsafe extern "system" fn SetTextColor<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textcolor: *const super::dwrite_2::DWRITE_COLOR_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::SetTextColor(this, core::mem::transmute_copy(&textcolor)).into()
            }
        }
        unsafe extern "system" fn SetColorPaletteIndex<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorpaletteindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::SetColorPaletteIndex(this, core::mem::transmute_copy(&colorpaletteindex)).into()
            }
        }
        unsafe extern "system" fn SetCustomColorPalette<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paletteentries: *const super::dwrite_2::DWRITE_COLOR_F, paletteentrycount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::SetCustomColorPalette(this, core::mem::transmute_copy(&paletteentries), core::mem::transmute_copy(&paletteentrycount)).into()
            }
        }
        unsafe extern "system" fn MoveToFirstChild<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::MoveToFirstChild(this, core::mem::transmute_copy(&paintelement), core::mem::transmute_copy(&structsize)).into()
            }
        }
        unsafe extern "system" fn MoveToNextSibling<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paintelement: *mut DWRITE_PAINT_ELEMENT, structsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::MoveToNextSibling(this, core::mem::transmute_copy(&paintelement), core::mem::transmute_copy(&structsize)).into()
            }
        }
        unsafe extern "system" fn MoveToParent<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::MoveToParent(this).into()
            }
        }
        unsafe extern "system" fn GetGradientStops<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstgradientstopindex: u32, gradientstopcount: u32, gradientstops: *mut super::d2d1::D2D1_GRADIENT_STOP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::GetGradientStops(this, core::mem::transmute_copy(&firstgradientstopindex), core::mem::transmute_copy(&gradientstopcount), core::mem::transmute_copy(&gradientstops)).into()
            }
        }
        unsafe extern "system" fn GetGradientStopColors<Identity: IDWritePaintReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstgradientstopindex: u32, gradientstopcount: u32, gradientstopcolors: *mut DWRITE_PAINT_COLOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePaintReader_Impl::GetGradientStopColors(this, core::mem::transmute_copy(&firstgradientstopindex), core::mem::transmute_copy(&gradientstopcount), core::mem::transmute_copy(&gradientstopcolors)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCurrentGlyph: SetCurrentGlyph::<Identity, OFFSET>,
            SetTextColor: SetTextColor::<Identity, OFFSET>,
            SetColorPaletteIndex: SetColorPaletteIndex::<Identity, OFFSET>,
            SetCustomColorPalette: SetCustomColorPalette::<Identity, OFFSET>,
            MoveToFirstChild: MoveToFirstChild::<Identity, OFFSET>,
            MoveToNextSibling: MoveToNextSibling::<Identity, OFFSET>,
            MoveToParent: MoveToParent::<Identity, OFFSET>,
            GetGradientStops: GetGradientStops::<Identity, OFFSET>,
            GetGradientStopColors: GetGradientStopColors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWritePaintReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_2", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for IDWritePaintReader {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteRemoteFontFileLoader, IDWriteRemoteFontFileLoader_Vtbl, 0x68648c83_6ede_46c0_ab46_20083a887fde);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteRemoteFontFileLoader {
    type Target = super::dwrite::IDWriteFontFileLoader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteRemoteFontFileLoader, windows_core::IUnknown, super::dwrite::IDWriteFontFileLoader);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteRemoteFontFileLoader {
    pub unsafe fn CreateRemoteStreamFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<IDWriteRemoteFontFileStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRemoteStreamFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLocalityFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<DWRITE_LOCALITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalityFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateFontFileReferenceFromUrl<P0, P1, P2>(&self, factory: P0, baseurl: P1, fontfileurl: P2) -> windows_core::Result<super::dwrite::IDWriteFontFile>
    where
        P0: windows_core::Param<super::dwrite::IDWriteFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFileReferenceFromUrl)(windows_core::Interface::as_raw(self), factory.param().abi(), baseurl.param().abi(), fontfileurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRemoteFontFileLoader_Vtbl {
    pub base__: super::dwrite::IDWriteFontFileLoader_Vtbl,
    pub CreateRemoteStreamFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalityFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut DWRITE_LOCALITY) -> windows_core::HRESULT,
    pub CreateFontFileReferenceFromUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteRemoteFontFileLoader_Impl: super::dwrite::IDWriteFontFileLoader_Impl {
    fn CreateRemoteStreamFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<IDWriteRemoteFontFileStream>;
    fn GetLocalityFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<DWRITE_LOCALITY>;
    fn CreateFontFileReferenceFromUrl(&self, factory: windows_core::Ref<super::dwrite::IDWriteFactory>, baseurl: &windows_core::PCWSTR, fontfileurl: &windows_core::PCWSTR) -> windows_core::Result<super::dwrite::IDWriteFontFile>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteRemoteFontFileLoader_Vtbl {
    pub const fn new<Identity: IDWriteRemoteFontFileLoader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateRemoteStreamFromKey<Identity: IDWriteRemoteFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteRemoteFontFileLoader_Impl::CreateRemoteStreamFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                    Ok(ok__) => {
                        fontfilestream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalityFromKey<Identity: IDWriteRemoteFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, locality: *mut DWRITE_LOCALITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteRemoteFontFileLoader_Impl::GetLocalityFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                    Ok(ok__) => {
                        locality.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFileReferenceFromUrl<Identity: IDWriteRemoteFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, baseurl: windows_core::PCWSTR, fontfileurl: windows_core::PCWSTR, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteRemoteFontFileLoader_Impl::CreateFontFileReferenceFromUrl(this, core::mem::transmute_copy(&factory), core::mem::transmute(&baseurl), core::mem::transmute(&fontfileurl)) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite::IDWriteFontFileLoader_Vtbl::new::<Identity, OFFSET>(),
            CreateRemoteStreamFromKey: CreateRemoteStreamFromKey::<Identity, OFFSET>,
            GetLocalityFromKey: GetLocalityFromKey::<Identity, OFFSET>,
            CreateFontFileReferenceFromUrl: CreateFontFileReferenceFromUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRemoteFontFileLoader as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFileLoader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteRemoteFontFileLoader {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteRemoteFontFileStream, IDWriteRemoteFontFileStream_Vtbl, 0x4db3757a_2c72_4ed9_b2b6_1ababe1aff9c);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteRemoteFontFileStream {
    type Target = super::dwrite::IDWriteFontFileStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteRemoteFontFileStream, windows_core::IUnknown, super::dwrite::IDWriteFontFileStream);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteRemoteFontFileStream {
    pub unsafe fn GetLocalFileSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFileFragmentLocality(&self, fileoffset: u64, fragmentsize: u64, islocal: *mut windows_core::BOOL, partialsize: *mut u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileFragmentLocality)(windows_core::Interface::as_raw(self), fileoffset, fragmentsize, islocal as _, partialsize as _) }
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        unsafe { (windows_core::Interface::vtable(self).GetLocality)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BeginDownload(&self, downloadoperationid: *const windows_core::GUID, filefragments: &[DWRITE_FILE_FRAGMENT]) -> windows_core::Result<IDWriteAsyncResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginDownload)(windows_core::Interface::as_raw(self), downloadoperationid, core::mem::transmute(filefragments.as_ptr()), filefragments.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRemoteFontFileStream_Vtbl {
    pub base__: super::dwrite::IDWriteFontFileStream_Vtbl,
    pub GetLocalFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetFileFragmentLocality: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, *mut windows_core::BOOL, *mut u64) -> windows_core::HRESULT,
    pub GetLocality: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_LOCALITY,
    pub BeginDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DWRITE_FILE_FRAGMENT, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteRemoteFontFileStream_Impl: super::dwrite::IDWriteFontFileStream_Impl {
    fn GetLocalFileSize(&self) -> windows_core::Result<u64>;
    fn GetFileFragmentLocality(&self, fileoffset: u64, fragmentsize: u64, islocal: *mut windows_core::BOOL, partialsize: *mut u64) -> windows_core::Result<()>;
    fn GetLocality(&self) -> DWRITE_LOCALITY;
    fn BeginDownload(&self, downloadoperationid: *const windows_core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32) -> windows_core::Result<IDWriteAsyncResult>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteRemoteFontFileStream_Vtbl {
    pub const fn new<Identity: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLocalFileSize<Identity: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localfilesize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteRemoteFontFileStream_Impl::GetLocalFileSize(this) {
                    Ok(ok__) => {
                        localfilesize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileFragmentLocality<Identity: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64, islocal: *mut windows_core::BOOL, partialsize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRemoteFontFileStream_Impl::GetFileFragmentLocality(this, core::mem::transmute_copy(&fileoffset), core::mem::transmute_copy(&fragmentsize), core::mem::transmute_copy(&islocal), core::mem::transmute_copy(&partialsize)).into()
            }
        }
        unsafe extern "system" fn GetLocality<Identity: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_LOCALITY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRemoteFontFileStream_Impl::GetLocality(this)
            }
        }
        unsafe extern "system" fn BeginDownload<Identity: IDWriteRemoteFontFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadoperationid: *const windows_core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32, asyncresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteRemoteFontFileStream_Impl::BeginDownload(this, core::mem::transmute_copy(&downloadoperationid), core::mem::transmute_copy(&filefragments), core::mem::transmute_copy(&fragmentcount)) {
                    Ok(ok__) => {
                        asyncresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite::IDWriteFontFileStream_Vtbl::new::<Identity, OFFSET>(),
            GetLocalFileSize: GetLocalFileSize::<Identity, OFFSET>,
            GetFileFragmentLocality: GetFileFragmentLocality::<Identity, OFFSET>,
            GetLocality: GetLocality::<Identity, OFFSET>,
            BeginDownload: BeginDownload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRemoteFontFileStream as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFileStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteRemoteFontFileStream {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteRenderingParams3, IDWriteRenderingParams3_Vtbl, 0xb7924baa_391b_412a_8c5c_e44cc2d867dc);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteRenderingParams3 {
    type Target = super::dwrite_2::IDWriteRenderingParams2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams3, windows_core::IUnknown, super::dwrite::IDWriteRenderingParams, super::dwrite_1::IDWriteRenderingParams1, super::dwrite_2::IDWriteRenderingParams2);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteRenderingParams3 {
    pub unsafe fn GetRenderingMode1(&self) -> DWRITE_RENDERING_MODE1 {
        unsafe { (windows_core::Interface::vtable(self).GetRenderingMode1)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRenderingParams3_Vtbl {
    pub base__: super::dwrite_2::IDWriteRenderingParams2_Vtbl,
    pub GetRenderingMode1: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_RENDERING_MODE1,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteRenderingParams3_Impl: super::dwrite_2::IDWriteRenderingParams2_Impl {
    fn GetRenderingMode1(&self) -> DWRITE_RENDERING_MODE1;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteRenderingParams3_Vtbl {
    pub const fn new<Identity: IDWriteRenderingParams3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRenderingMode1<Identity: IDWriteRenderingParams3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_RENDERING_MODE1 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRenderingParams3_Impl::GetRenderingMode1(this)
            }
        }
        Self { base__: super::dwrite_2::IDWriteRenderingParams2_Vtbl::new::<Identity, OFFSET>(), GetRenderingMode1: GetRenderingMode1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRenderingParams3 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteRenderingParams as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteRenderingParams1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteRenderingParams2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteRenderingParams3 {}
windows_core::imp::define_interface!(IDWriteStringList, IDWriteStringList_Vtbl, 0xcfee3140_1157_47ca_8b85_31bfcf3f2d0e);
windows_core::imp::interface_hierarchy!(IDWriteStringList, windows_core::IUnknown);
impl IDWriteStringList {
    pub unsafe fn GetCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLocaleNameLength(&self, listindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self), listindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocaleName(&self, listindex: u32, localename: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), listindex, core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap()) }
    }
    pub unsafe fn GetStringLength(&self, listindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self), listindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetString(&self, listindex: u32, stringbuffer: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), listindex, core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteStringList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, u32) -> windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, u32) -> windows_core::HRESULT,
}
pub trait IDWriteStringList_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> u32;
    fn GetLocaleNameLength(&self, listindex: u32) -> windows_core::Result<u32>;
    fn GetLocaleName(&self, listindex: u32, localename: *mut u16, size: u32) -> windows_core::Result<()>;
    fn GetStringLength(&self, listindex: u32) -> windows_core::Result<u32>;
    fn GetString(&self, listindex: u32, stringbuffer: *mut u16, stringbuffersize: u32) -> windows_core::Result<()>;
}
impl IDWriteStringList_Vtbl {
    pub const fn new<Identity: IDWriteStringList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteStringList_Impl::GetCount(this)
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, length: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteStringList_Impl::GetLocaleNameLength(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, localename: *mut u16, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteStringList_Impl::GetLocaleName(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&localename), core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn GetStringLength<Identity: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, length: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteStringList_Impl::GetStringLength(this, core::mem::transmute_copy(&listindex)) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: IDWriteStringList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listindex: u32, stringbuffer: *mut u16, stringbuffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteStringList_Impl::GetString(this, core::mem::transmute_copy(&listindex), core::mem::transmute_copy(&stringbuffer), core::mem::transmute_copy(&stringbuffersize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteStringList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteStringList {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteTextFormat2, IDWriteTextFormat2_Vtbl, 0xf67e0edd_9e3d_4ecc_8c32_4183253dfe70);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteTextFormat2 {
    type Target = super::dwrite_2::IDWriteTextFormat1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteTextFormat2, windows_core::IUnknown, super::dwrite::IDWriteTextFormat, super::dwrite_2::IDWriteTextFormat1);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
impl IDWriteTextFormat2 {
    pub unsafe fn SetLineSpacing(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLineSpacing)(windows_core::Interface::as_raw(self), linespacingoptions) }
    }
    pub unsafe fn GetLineSpacing(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLineSpacing)(windows_core::Interface::as_raw(self), linespacingoptions as _) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat2_Vtbl {
    pub base__: super::dwrite_2::IDWriteTextFormat1_Vtbl,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_LINE_SPACING) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteTextFormat2_Impl: super::dwrite_2::IDWriteTextFormat1_Impl {
    fn SetLineSpacing(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::Result<()>;
    fn GetLineSpacing(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteTextFormat2_Vtbl {
    pub const fn new<Identity: IDWriteTextFormat2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLineSpacing<Identity: IDWriteTextFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat2_Impl::SetLineSpacing(this, core::mem::transmute_copy(&linespacingoptions)).into()
            }
        }
        unsafe extern "system" fn GetLineSpacing<Identity: IDWriteTextFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat2_Impl::GetLineSpacing(this, core::mem::transmute_copy(&linespacingoptions)).into()
            }
        }
        Self {
            base__: super::dwrite_2::IDWriteTextFormat1_Vtbl::new::<Identity, OFFSET>(),
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat2 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextFormat as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteTextFormat1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteTextFormat2 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteTextFormat3, IDWriteTextFormat3_Vtbl, 0x6d3b5641_e550_430d_a85b_b7bf48a93427);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteTextFormat3 {
    type Target = IDWriteTextFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteTextFormat3, windows_core::IUnknown, super::dwrite::IDWriteTextFormat, super::dwrite_2::IDWriteTextFormat1, IDWriteTextFormat2);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
impl IDWriteTextFormat3 {
    pub unsafe fn SetFontAxisValues(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()) }
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisValueCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap()) }
    }
    pub unsafe fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES {
        unsafe { (windows_core::Interface::vtable(self).GetAutomaticFontAxes)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutomaticFontAxes)(windows_core::Interface::as_raw(self), automaticfontaxes) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat3_Vtbl {
    pub base__: IDWriteTextFormat2_Vtbl,
    pub SetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_AXIS_VALUE, u32) -> windows_core::HRESULT,
    pub GetAutomaticFontAxes: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES,
    pub SetAutomaticFontAxes: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteTextFormat3_Impl: IDWriteTextFormat2_Impl {
    fn SetFontAxisValues(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
    fn GetFontAxisValueCount(&self) -> u32;
    fn GetFontAxisValues(&self, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::Result<()>;
    fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES;
    fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteTextFormat3_Vtbl {
    pub const fn new<Identity: IDWriteTextFormat3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFontAxisValues<Identity: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat3_Impl::SetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
            }
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat3_Impl::GetFontAxisValueCount(this)
            }
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat3_Impl::GetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount)).into()
            }
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Identity: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat3_Impl::GetAutomaticFontAxes(this)
            }
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Identity: IDWriteTextFormat3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat3_Impl::SetAutomaticFontAxes(this, core::mem::transmute_copy(&automaticfontaxes)).into()
            }
        }
        Self {
            base__: IDWriteTextFormat2_Vtbl::new::<Identity, OFFSET>(),
            SetFontAxisValues: SetFontAxisValues::<Identity, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, OFFSET>,
            GetAutomaticFontAxes: GetAutomaticFontAxes::<Identity, OFFSET>,
            SetAutomaticFontAxes: SetAutomaticFontAxes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat3 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextFormat as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteTextFormat1 as windows_core::Interface>::IID || iid == &<IDWriteTextFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteTextFormat3 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteTextLayout3, IDWriteTextLayout3_Vtbl, 0x07ddcd52_020e_4de8_ac33_6c953d83f92d);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteTextLayout3 {
    type Target = super::dwrite_2::IDWriteTextLayout2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteTextLayout3, windows_core::IUnknown, super::dwrite::IDWriteTextFormat, super::dwrite::IDWriteTextLayout, super::dwrite_1::IDWriteTextLayout1, super::dwrite_2::IDWriteTextLayout2);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteTextLayout3 {
    pub unsafe fn InvalidateLayout(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvalidateLayout)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLineSpacing(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLineSpacing)(windows_core::Interface::as_raw(self), linespacingoptions) }
    }
    pub unsafe fn GetLineSpacing(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLineSpacing)(windows_core::Interface::as_raw(self), linespacingoptions as _) }
    }
    pub unsafe fn GetLineMetrics(&self, linemetrics: Option<&mut [DWRITE_LINE_METRICS1]>, actuallinecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLineMetrics)(windows_core::Interface::as_raw(self), core::mem::transmute(linemetrics.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount as _) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout3_Vtbl {
    pub base__: super::dwrite_2::IDWriteTextLayout2_Vtbl,
    pub InvalidateLayout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_LINE_SPACING) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT,
    pub GetLineMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_METRICS1, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteTextLayout3_Impl: super::dwrite_2::IDWriteTextLayout2_Impl {
    fn InvalidateLayout(&self) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::Result<()>;
    fn GetLineSpacing(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::Result<()>;
    fn GetLineMetrics(&self, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteTextLayout3_Vtbl {
    pub const fn new<Identity: IDWriteTextLayout3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvalidateLayout<Identity: IDWriteTextLayout3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout3_Impl::InvalidateLayout(this).into()
            }
        }
        unsafe extern "system" fn SetLineSpacing<Identity: IDWriteTextLayout3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout3_Impl::SetLineSpacing(this, core::mem::transmute_copy(&linespacingoptions)).into()
            }
        }
        unsafe extern "system" fn GetLineSpacing<Identity: IDWriteTextLayout3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout3_Impl::GetLineSpacing(this, core::mem::transmute_copy(&linespacingoptions)).into()
            }
        }
        unsafe extern "system" fn GetLineMetrics<Identity: IDWriteTextLayout3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout3_Impl::GetLineMetrics(this, core::mem::transmute_copy(&linemetrics), core::mem::transmute_copy(&maxlinecount), core::mem::transmute_copy(&actuallinecount)).into()
            }
        }
        Self {
            base__: super::dwrite_2::IDWriteTextLayout2_Vtbl::new::<Identity, OFFSET>(),
            InvalidateLayout: InvalidateLayout::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
            GetLineMetrics: GetLineMetrics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout3 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextFormat as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextLayout as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteTextLayout1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteTextLayout2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteTextLayout3 {}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::define_interface!(IDWriteTextLayout4, IDWriteTextLayout4_Vtbl, 0x05a9bf42_223f_4441_b5fb_8263685f55e9);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl core::ops::Deref for IDWriteTextLayout4 {
    type Target = IDWriteTextLayout3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
windows_core::imp::interface_hierarchy!(IDWriteTextLayout4, windows_core::IUnknown, super::dwrite::IDWriteTextFormat, super::dwrite::IDWriteTextLayout, super::dwrite_1::IDWriteTextLayout1, super::dwrite_2::IDWriteTextLayout2, IDWriteTextLayout3);
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteTextLayout4 {
    pub unsafe fn SetFontAxisValues(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], textrange: super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFontAxisValues)(windows_core::Interface::as_raw(self), core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn GetFontAxisValueCount(&self, currentposition: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisValueCount)(windows_core::Interface::as_raw(self), currentposition) }
    }
    pub unsafe fn GetFontAxisValues(&self, currentposition: u32, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE], textrange: Option<*mut super::dwrite::DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontAxisValues)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES {
        unsafe { (windows_core::Interface::vtable(self).GetAutomaticFontAxes)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutomaticFontAxes)(windows_core::Interface::as_raw(self), automaticfontaxes) }
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout4_Vtbl {
    pub base__: IDWriteTextLayout3_Vtbl,
    pub SetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_FONT_AXIS_VALUE, u32, super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_AXIS_VALUE, u32, *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetAutomaticFontAxes: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES,
    pub SetAutomaticFontAxes: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
pub trait IDWriteTextLayout4_Impl: IDWriteTextLayout3_Impl {
    fn SetFontAxisValues(&self, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: &super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontAxisValueCount(&self, currentposition: u32) -> u32;
    fn GetFontAxisValues(&self, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES;
    fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl IDWriteTextLayout4_Vtbl {
    pub const fn new<Identity: IDWriteTextLayout4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFontAxisValues<Identity: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout4_Impl::SetFontAxisValues(this, core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout4_Impl::GetFontAxisValueCount(this, core::mem::transmute_copy(&currentposition))
            }
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout4_Impl::GetFontAxisValues(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontaxisvalues), core::mem::transmute_copy(&fontaxisvaluecount), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Identity: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout4_Impl::GetAutomaticFontAxes(this)
            }
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Identity: IDWriteTextLayout4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout4_Impl::SetAutomaticFontAxes(this, core::mem::transmute_copy(&automaticfontaxes)).into()
            }
        }
        Self {
            base__: IDWriteTextLayout3_Vtbl::new::<Identity, OFFSET>(),
            SetFontAxisValues: SetFontAxisValues::<Identity, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, OFFSET>,
            GetAutomaticFontAxes: GetAutomaticFontAxes::<Identity, OFFSET>,
            SetAutomaticFontAxes: SetAutomaticFontAxes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout4 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextFormat as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextLayout as windows_core::Interface>::IID || iid == &<super::dwrite_1::IDWriteTextLayout1 as windows_core::Interface>::IID || iid == &<super::dwrite_2::IDWriteTextLayout2 as windows_core::Interface>::IID || iid == &<IDWriteTextLayout3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dwrite_1", feature = "Win32_dwrite_2"))]
impl windows_core::RuntimeName for IDWriteTextLayout4 {}
