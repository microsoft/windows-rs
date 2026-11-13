windows_core::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const IID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
windows_core::link!("kernel32.dll" "system" fn GetUserDefaultLocaleName(lplocalename : windows_core::PWSTR, cchlocalename : i32) -> i32);
windows_core::link!("user32.dll" "system" fn SystemParametersInfoW(uiaction : UINT, uiparam : UINT, pvparam : PVOID, fwinini : UINT) -> windows_core::BOOL);
pub type BYTE = u8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: FLOAT,
    pub length: UINT16,
    pub _bitfield: u16,
}
impl DWRITE_CLUSTER_METRICS {
    pub fn canWrapLineAfter(&self) -> UINT16 {
        (self._bitfield) & 1
    }
    pub fn set_canWrapLineAfter(&mut self, value: UINT16) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn isWhitespace(&self) -> UINT16 {
        (self._bitfield >> 1) & 1
    }
    pub fn set_isWhitespace(&mut self, value: UINT16) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn isNewline(&self) -> UINT16 {
        (self._bitfield >> 2) & 1
    }
    pub fn set_isNewline(&mut self, value: UINT16) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn isSoftHyphen(&self) -> UINT16 {
        (self._bitfield >> 3) & 1
    }
    pub fn set_isSoftHyphen(&mut self, value: UINT16) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn isRightToLeft(&self) -> UINT16 {
        (self._bitfield >> 4) & 1
    }
    pub fn set_isRightToLeft(&mut self, value: UINT16) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn padding(&self) -> UINT16 {
        self._bitfield >> 5
    }
    pub fn set_padding(&mut self, value: UINT16) {
        self._bitfield = (self._bitfield & !(2047 << 5)) | ((value & 2047) << 5);
    }
}
pub type DWRITE_COLOR_F = D3DCOLORVALUE;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DWRITE_COLOR_GLYPH_RUN {
    pub glyphRun: DWRITE_GLYPH_RUN,
    pub glyphRunDescription: *mut DWRITE_GLYPH_RUN_DESCRIPTION,
    pub baselineOriginX: FLOAT,
    pub baselineOriginY: FLOAT,
    pub runColor: DWRITE_COLOR_F,
    pub paletteIndex: UINT16,
}
pub const DWRITE_E_NOCOLOR: windows_core::HRESULT = windows_core::HRESULT(0x8898500C_u32 as _);
pub type DWRITE_FACTORY_TYPE = i32;
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = 0;
pub type DWRITE_FLOW_DIRECTION = i32;
pub const DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP: DWRITE_FLOW_DIRECTION = 1;
pub const DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT: DWRITE_FLOW_DIRECTION = 2;
pub const DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT: DWRITE_FLOW_DIRECTION = 3;
pub const DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM: DWRITE_FLOW_DIRECTION = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_FONT_FEATURE {
    pub nameTag: DWRITE_FONT_FEATURE_TAG,
    pub parameter: UINT32,
}
pub type DWRITE_FONT_FEATURE_TAG = i32;
pub type DWRITE_FONT_LINE_GAP_USAGE = i32;
pub const DWRITE_FONT_LINE_GAP_USAGE_DEFAULT: DWRITE_FONT_LINE_GAP_USAGE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_FONT_METRICS {
    pub designUnitsPerEm: UINT16,
    pub ascent: UINT16,
    pub descent: UINT16,
    pub lineGap: INT16,
    pub capHeight: UINT16,
    pub xHeight: UINT16,
    pub underlinePosition: INT16,
    pub underlineThickness: UINT16,
    pub strikethroughPosition: INT16,
    pub strikethroughThickness: UINT16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_FONT_METRICS1 {
    pub base: DWRITE_FONT_METRICS,
    pub glyphBoxLeft: INT16,
    pub glyphBoxTop: INT16,
    pub glyphBoxRight: INT16,
    pub glyphBoxBottom: INT16,
    pub subscriptPositionX: INT16,
    pub subscriptPositionY: INT16,
    pub subscriptSizeX: INT16,
    pub subscriptSizeY: INT16,
    pub superscriptPositionX: INT16,
    pub superscriptPositionY: INT16,
    pub superscriptSizeX: INT16,
    pub superscriptSizeY: INT16,
    pub hasTypographicMetrics: windows_core::BOOL,
}
pub type DWRITE_FONT_STRETCH = i32;
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = 5;
pub type DWRITE_FONT_STYLE = i32;
pub const DWRITE_FONT_STYLE_ITALIC: DWRITE_FONT_STYLE = 2;
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = 0;
pub const DWRITE_FONT_STYLE_OBLIQUE: DWRITE_FONT_STYLE = 1;
pub type DWRITE_FONT_WEIGHT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_GLYPH_METRICS {
    pub leftSideBearing: INT32,
    pub advanceWidth: UINT32,
    pub rightSideBearing: INT32,
    pub topSideBearing: INT32,
    pub advanceHeight: UINT32,
    pub bottomSideBearing: INT32,
    pub verticalOriginY: INT32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: FLOAT,
    pub ascenderOffset: FLOAT,
}
pub type DWRITE_GLYPH_ORIENTATION_ANGLE = i32;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 0;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 2;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 3;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 1;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: core::mem::ManuallyDrop<Option<IDWriteFontFace>>,
    pub fontEmSize: FLOAT,
    pub glyphCount: UINT32,
    pub glyphIndices: *const UINT16,
    pub glyphAdvances: *const FLOAT,
    pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    pub isSideways: windows_core::BOOL,
    pub bidiLevel: UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: *const WCHAR,
    pub string: *const WCHAR,
    pub stringLength: UINT32,
    pub clusterMap: *const UINT16,
    pub textPosition: UINT32,
}
pub type DWRITE_GRID_FIT_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: UINT32,
    pub length: UINT32,
    pub left: FLOAT,
    pub top: FLOAT,
    pub width: FLOAT,
    pub height: FLOAT,
    pub bidiLevel: UINT32,
    pub isText: windows_core::BOOL,
    pub isTrimmed: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_METRICS {
    pub length: UINT32,
    pub trailingWhitespaceLength: UINT32,
    pub newlineLength: UINT32,
    pub height: FLOAT,
    pub baseline: FLOAT,
    pub isTrimmed: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_SPACING {
    pub method: DWRITE_LINE_SPACING_METHOD,
    pub height: FLOAT,
    pub baseline: FLOAT,
    pub leadingBefore: FLOAT,
    pub fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE,
}
pub type DWRITE_LINE_SPACING_METHOD = i32;
pub const DWRITE_LINE_SPACING_METHOD_DEFAULT: DWRITE_LINE_SPACING_METHOD = 0;
pub const DWRITE_LINE_SPACING_METHOD_PROPORTIONAL: DWRITE_LINE_SPACING_METHOD = 2;
pub const DWRITE_LINE_SPACING_METHOD_UNIFORM: DWRITE_LINE_SPACING_METHOD = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_MATRIX {
    pub m11: FLOAT,
    pub m12: FLOAT,
    pub m21: FLOAT,
    pub m22: FLOAT,
    pub dx: FLOAT,
    pub dy: FLOAT,
}
pub type DWRITE_MEASURING_MODE = i32;
pub const DWRITE_MEASURING_MODE_NATURAL: DWRITE_MEASURING_MODE = 0;
pub type DWRITE_OUTLINE_THRESHOLD = i32;
pub const DWRITE_OUTLINE_THRESHOLD_ANTIALIASED: DWRITE_OUTLINE_THRESHOLD = 0;
pub type DWRITE_PARAGRAPH_ALIGNMENT = i32;
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = 2;
pub const DWRITE_PARAGRAPH_ALIGNMENT_FAR: DWRITE_PARAGRAPH_ALIGNMENT = 1;
pub const DWRITE_PARAGRAPH_ALIGNMENT_NEAR: DWRITE_PARAGRAPH_ALIGNMENT = 0;
pub type DWRITE_PIXEL_GEOMETRY = i32;
pub const DWRITE_PIXEL_GEOMETRY_BGR: DWRITE_PIXEL_GEOMETRY = 2;
pub const DWRITE_PIXEL_GEOMETRY_RGB: DWRITE_PIXEL_GEOMETRY = 1;
pub type DWRITE_READING_DIRECTION = i32;
pub const DWRITE_READING_DIRECTION_BOTTOM_TO_TOP: DWRITE_READING_DIRECTION = 3;
pub const DWRITE_READING_DIRECTION_LEFT_TO_RIGHT: DWRITE_READING_DIRECTION = 0;
pub const DWRITE_READING_DIRECTION_RIGHT_TO_LEFT: DWRITE_READING_DIRECTION = 1;
pub const DWRITE_READING_DIRECTION_TOP_TO_BOTTOM: DWRITE_READING_DIRECTION = 2;
pub type DWRITE_RENDERING_MODE1 = i32;
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE1 = 5;
pub const DWRITE_RENDERING_MODE1_OUTLINE: DWRITE_RENDERING_MODE1 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_STRIKETHROUGH {
    pub width: FLOAT,
    pub thickness: FLOAT,
    pub offset: FLOAT,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: *const WCHAR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
pub const DWRITE_TEXTURE_ALIASED_1x1: DWRITE_TEXTURE_TYPE = 0;
pub const DWRITE_TEXTURE_CLEARTYPE_3x1: DWRITE_TEXTURE_TYPE = 1;
pub type DWRITE_TEXTURE_TYPE = i32;
pub type DWRITE_TEXT_ALIGNMENT = i32;
pub const DWRITE_TEXT_ALIGNMENT_CENTER: DWRITE_TEXT_ALIGNMENT = 2;
pub const DWRITE_TEXT_ALIGNMENT_JUSTIFIED: DWRITE_TEXT_ALIGNMENT = 3;
pub const DWRITE_TEXT_ALIGNMENT_LEADING: DWRITE_TEXT_ALIGNMENT = 0;
pub const DWRITE_TEXT_ALIGNMENT_TRAILING: DWRITE_TEXT_ALIGNMENT = 1;
pub type DWRITE_TEXT_ANTIALIAS_MODE = i32;
pub const DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE: DWRITE_TEXT_ANTIALIAS_MODE = 0;
pub const DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE: DWRITE_TEXT_ANTIALIAS_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TEXT_METRICS {
    pub left: FLOAT,
    pub top: FLOAT,
    pub width: FLOAT,
    pub widthIncludingTrailingWhitespace: FLOAT,
    pub height: FLOAT,
    pub layoutWidth: FLOAT,
    pub layoutHeight: FLOAT,
    pub maxBidiReorderingDepth: UINT32,
    pub lineCount: UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TEXT_METRICS1 {
    pub base: DWRITE_TEXT_METRICS,
    pub heightIncludingTrailingWhitespace: FLOAT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: UINT32,
    pub length: UINT32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_TRIMMING {
    pub granularity: DWRITE_TRIMMING_GRANULARITY,
    pub delimiter: UINT32,
    pub delimiterCount: UINT32,
}
pub type DWRITE_TRIMMING_GRANULARITY = i32;
pub const DWRITE_TRIMMING_GRANULARITY_CHARACTER: DWRITE_TRIMMING_GRANULARITY = 1;
pub const DWRITE_TRIMMING_GRANULARITY_NONE: DWRITE_TRIMMING_GRANULARITY = 0;
pub const DWRITE_TRIMMING_GRANULARITY_WORD: DWRITE_TRIMMING_GRANULARITY = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_UNDERLINE {
    pub width: FLOAT,
    pub thickness: FLOAT,
    pub offset: FLOAT,
    pub runHeight: FLOAT,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: *const WCHAR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_UNICODE_RANGE {
    pub first: UINT32,
    pub last: UINT32,
}
pub type DWRITE_VERTICAL_GLYPH_ORIENTATION = i32;
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT: DWRITE_VERTICAL_GLYPH_ORIENTATION = 0;
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED: DWRITE_VERTICAL_GLYPH_ORIENTATION = 1;
pub type DWRITE_WORD_WRAPPING = i32;
pub const DWRITE_WORD_WRAPPING_NO_WRAP: DWRITE_WORD_WRAPPING = 1;
pub const DWRITE_WORD_WRAPPING_WRAP: DWRITE_WORD_WRAPPING = 0;
pub const E_NOT_SUFFICIENT_BUFFER: windows_core::HRESULT =
    windows_core::HRESULT(0x8007007A_u32 as _);
pub const FE_FONTSMOOTHINGCLEARTYPE: i32 = 2;
pub type FLOAT = f32;
windows_core::imp::define_interface!(
    IDWriteColorGlyphRunEnumerator,
    IDWriteColorGlyphRunEnumerator_Vtbl,
    0xd31fbe17_f157_41a2_8d24_cb779e0560e8
);
windows_core::imp::interface_hierarchy!(IDWriteColorGlyphRunEnumerator, windows_core::IUnknown);
impl IDWriteColorGlyphRunEnumerator {
    pub(crate) unsafe fn MoveNext(&self, hasrun: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).MoveNext)(
                windows_core::Interface::as_raw(self),
                hasrun as _,
            )
        }
    }
    pub(crate) unsafe fn GetCurrentRun(
        &self,
        colorglyphrun: *mut *const DWRITE_COLOR_GLYPH_RUN,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetCurrentRun)(
                windows_core::Interface::as_raw(self),
                colorglyphrun as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteColorGlyphRunEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetCurrentRun: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *const DWRITE_COLOR_GLYPH_RUN,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDWriteFactory,
    IDWriteFactory_Vtbl,
    0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48
);
windows_core::imp::interface_hierarchy!(IDWriteFactory, windows_core::IUnknown);
impl IDWriteFactory {
    pub(crate) unsafe fn CreateRenderingParams(
        &self,
    ) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRenderingParams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn RegisterFontFileLoader<P0>(
        &self,
        fontfileloader: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RegisterFontFileLoader)(
                windows_core::Interface::as_raw(self),
                fontfileloader.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn CreateTextFormat<P1>(
        &self,
        fontfamilyname: *const WCHAR,
        fontcollection: P1,
        fontweight: DWRITE_FONT_WEIGHT,
        fontstyle: DWRITE_FONT_STYLE,
        fontstretch: DWRITE_FONT_STRETCH,
        fontsize: FLOAT,
        localename: *const WCHAR,
    ) -> windows_core::Result<IDWriteTextFormat>
    where
        P1: windows_core::Param<IDWriteFontCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextFormat)(
                windows_core::Interface::as_raw(self),
                fontfamilyname,
                fontcollection.param().abi(),
                fontweight,
                fontstyle,
                fontstretch,
                fontsize,
                localename,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTypography)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateTextLayout<P2>(
        &self,
        string: *const WCHAR,
        stringlength: UINT32,
        textformat: P2,
        maxwidth: FLOAT,
        maxheight: FLOAT,
    ) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextLayout)(
                windows_core::Interface::as_raw(self),
                string,
                stringlength,
                textformat.param().abi(),
                maxwidth,
                maxheight,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateEllipsisTrimmingSign<P0>(
        &self,
        textformat: P0,
    ) -> windows_core::Result<IDWriteInlineObject>
    where
        P0: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEllipsisTrimmingSign)(
                windows_core::Interface::as_raw(self),
                textformat.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetSystemFontCollection: usize,
    CreateCustomFontCollection: usize,
    RegisterFontCollectionLoader: usize,
    UnregisterFontCollectionLoader: usize,
    CreateFontFileReference: usize,
    CreateCustomFontFileReference: usize,
    CreateFontFace: usize,
    pub CreateRenderingParams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateMonitorRenderingParams: usize,
    CreateCustomRenderingParams: usize,
    pub RegisterFontFileLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    UnregisterFontFileLoader: usize,
    pub CreateTextFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const WCHAR,
        *mut core::ffi::c_void,
        DWRITE_FONT_WEIGHT,
        DWRITE_FONT_STYLE,
        DWRITE_FONT_STRETCH,
        FLOAT,
        *const WCHAR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTypography: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetGdiInterop: usize,
    pub CreateTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const WCHAR,
        UINT32,
        *mut core::ffi::c_void,
        FLOAT,
        FLOAT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateGdiCompatibleTextLayout: usize,
    pub CreateEllipsisTrimmingSign: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateTextAnalyzer: usize,
    CreateNumberSubstitution: usize,
    CreateGlyphRunAnalysis: usize,
}
windows_core::imp::define_interface!(
    IDWriteFactory1,
    IDWriteFactory1_Vtbl,
    0x30572f99_dac6_41db_a16e_0486307e606a
);
impl core::ops::Deref for IDWriteFactory1 {
    type Target = IDWriteFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFactory1, windows_core::IUnknown, IDWriteFactory);
#[repr(C)]
pub struct IDWriteFactory1_Vtbl {
    pub base__: IDWriteFactory_Vtbl,
    GetEudcFontCollection: usize,
    CreateCustomRenderingParams: usize,
}
windows_core::imp::define_interface!(
    IDWriteFactory2,
    IDWriteFactory2_Vtbl,
    0x0439fc60_ca44_4994_8dee_3a9af7b732ec
);
impl core::ops::Deref for IDWriteFactory2 {
    type Target = IDWriteFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteFactory2,
    windows_core::IUnknown,
    IDWriteFactory,
    IDWriteFactory1
);
impl IDWriteFactory2 {
    pub(crate) unsafe fn GetSystemFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemFontFallback)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateFontFallbackBuilder(
        &self,
    ) -> windows_core::Result<IDWriteFontFallbackBuilder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFallbackBuilder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn TranslateColorGlyphRun(
        &self,
        baselineoriginx: FLOAT,
        baselineoriginy: FLOAT,
        glyphrun: *const DWRITE_GLYPH_RUN,
        glyphrundescription: Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>,
        measuringmode: DWRITE_MEASURING_MODE,
        worldtodevicetransform: Option<*const DWRITE_MATRIX>,
        colorpaletteindex: UINT32,
    ) -> windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TranslateColorGlyphRun)(
                windows_core::Interface::as_raw(self),
                baselineoriginx,
                baselineoriginy,
                glyphrun,
                glyphrundescription.unwrap_or(core::mem::zeroed()) as _,
                measuringmode,
                worldtodevicetransform.unwrap_or(core::mem::zeroed()) as _,
                colorpaletteindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFactory2_Vtbl {
    pub base__: IDWriteFactory1_Vtbl,
    pub GetSystemFontFallback: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFontFallbackBuilder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TranslateColorGlyphRun: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FLOAT,
        FLOAT,
        *const DWRITE_GLYPH_RUN,
        *const DWRITE_GLYPH_RUN_DESCRIPTION,
        DWRITE_MEASURING_MODE,
        *const DWRITE_MATRIX,
        UINT32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateCustomRenderingParams: usize,
    CreateGlyphRunAnalysis: usize,
}
windows_core::imp::define_interface!(
    IDWriteFactory3,
    IDWriteFactory3_Vtbl,
    0x9a1b41c3_d3bb_466a_87fc_fe67556a3b65
);
impl core::ops::Deref for IDWriteFactory3 {
    type Target = IDWriteFactory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteFactory3,
    windows_core::IUnknown,
    IDWriteFactory,
    IDWriteFactory1,
    IDWriteFactory2
);
impl IDWriteFactory3 {
    pub(crate) unsafe fn CreateGlyphRunAnalysis(
        &self,
        glyphrun: *const DWRITE_GLYPH_RUN,
        transform: Option<*const DWRITE_MATRIX>,
        renderingmode: DWRITE_RENDERING_MODE1,
        measuringmode: DWRITE_MEASURING_MODE,
        gridfitmode: DWRITE_GRID_FIT_MODE,
        antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE,
        baselineoriginx: FLOAT,
        baselineoriginy: FLOAT,
    ) -> windows_core::Result<IDWriteGlyphRunAnalysis> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(
                windows_core::Interface::as_raw(self),
                glyphrun,
                transform.unwrap_or(core::mem::zeroed()) as _,
                renderingmode,
                measuringmode,
                gridfitmode,
                antialiasmode,
                baselineoriginx,
                baselineoriginy,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateFontCollectionFromFontSet<P0>(
        &self,
        fontset: P0,
    ) -> windows_core::Result<IDWriteFontCollection1>
    where
        P0: windows_core::Param<IDWriteFontSet>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontCollectionFromFontSet)(
                windows_core::Interface::as_raw(self),
                fontset.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetSystemFontCollection(
        &self,
        includedownloadablefonts: bool,
        fontcollection: *mut Option<IDWriteFontCollection1>,
        checkforupdates: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetSystemFontCollection)(
                windows_core::Interface::as_raw(self),
                includedownloadablefonts.into(),
                fontcollection as _,
                checkforupdates.into(),
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteFactory3_Vtbl {
    pub base__: IDWriteFactory2_Vtbl,
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DWRITE_GLYPH_RUN,
        *const DWRITE_MATRIX,
        DWRITE_RENDERING_MODE1,
        DWRITE_MEASURING_MODE,
        DWRITE_GRID_FIT_MODE,
        DWRITE_TEXT_ANTIALIAS_MODE,
        FLOAT,
        FLOAT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateCustomRenderingParams: usize,
    CreateFontFaceReference: usize,
    CreateFontFaceReference2: usize,
    GetSystemFontSet: usize,
    CreateFontSetBuilder: usize,
    pub CreateFontCollectionFromFontSet: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSystemFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    GetFontDownloadQueue: usize,
}
windows_core::imp::define_interface!(
    IDWriteFactory4,
    IDWriteFactory4_Vtbl,
    0x4b0b5bd3_0797_4549_8ac5_fe915cc53856
);
impl core::ops::Deref for IDWriteFactory4 {
    type Target = IDWriteFactory3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteFactory4,
    windows_core::IUnknown,
    IDWriteFactory,
    IDWriteFactory1,
    IDWriteFactory2,
    IDWriteFactory3
);
#[repr(C)]
pub struct IDWriteFactory4_Vtbl {
    pub base__: IDWriteFactory3_Vtbl,
    TranslateColorGlyphRun: usize,
    ComputeGlyphOrigins: usize,
    ComputeGlyphOrigins2: usize,
}
windows_core::imp::define_interface!(
    IDWriteFactory5,
    IDWriteFactory5_Vtbl,
    0x958db99a_be2a_4f09_af7d_65189803d1d3
);
impl core::ops::Deref for IDWriteFactory5 {
    type Target = IDWriteFactory4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteFactory5,
    windows_core::IUnknown,
    IDWriteFactory,
    IDWriteFactory1,
    IDWriteFactory2,
    IDWriteFactory3,
    IDWriteFactory4
);
impl IDWriteFactory5 {
    pub(crate) unsafe fn CreateFontSetBuilder(
        &self,
    ) -> windows_core::Result<IDWriteFontSetBuilder1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontSetBuilder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateInMemoryFontFileLoader(
        &self,
    ) -> windows_core::Result<IDWriteInMemoryFontFileLoader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInMemoryFontFileLoader)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFactory5_Vtbl {
    pub base__: IDWriteFactory4_Vtbl,
    pub CreateFontSetBuilder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateInMemoryFontFileLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateHttpFontFileLoader: usize,
    AnalyzeContainerType: usize,
    UnpackFontFile: usize,
}
windows_core::imp::define_interface!(
    IDWriteFont,
    IDWriteFont_Vtbl,
    0xacd16696_8c14_4f5d_877e_fe3fc1d32737
);
windows_core::imp::interface_hierarchy!(IDWriteFont, windows_core::IUnknown);
impl IDWriteFont {
    pub(crate) unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFont_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetFontFamily: usize,
    GetWeight: usize,
    GetStretch: usize,
    GetStyle: usize,
    IsSymbolFont: usize,
    GetFaceNames: usize,
    GetInformationalStrings: usize,
    GetSimulations: usize,
    GetMetrics: usize,
    HasCharacter: usize,
    pub CreateFontFace: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDWriteFontCollection,
    IDWriteFontCollection_Vtbl,
    0xa84cee02_3eea_4eee_a827_87c1a02a0fcc
);
windows_core::imp::interface_hierarchy!(IDWriteFontCollection, windows_core::IUnknown);
impl IDWriteFontCollection {
    pub(crate) unsafe fn GetFontFamilyCount(&self) -> UINT32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontFamilyCount)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetFontFamily(
        &self,
        index: UINT32,
    ) -> windows_core::Result<IDWriteFontFamily> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFamily)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn FindFamilyName(
        &self,
        familyname: *const WCHAR,
        index: *mut UINT32,
        exists: *mut windows_core::BOOL,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).FindFamilyName)(
                windows_core::Interface::as_raw(self),
                familyname,
                index as _,
                exists as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteFontCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontFamilyCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> UINT32,
    pub GetFontFamily: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UINT32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FindFamilyName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const WCHAR,
        *mut UINT32,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    GetFontFromFontFace: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontCollection1,
    IDWriteFontCollection1_Vtbl,
    0x53585141_d9f8_4095_8321_d73cf6bd116c
);
impl core::ops::Deref for IDWriteFontCollection1 {
    type Target = IDWriteFontCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteFontCollection1,
    windows_core::IUnknown,
    IDWriteFontCollection
);
#[repr(C)]
pub struct IDWriteFontCollection1_Vtbl {
    pub base__: IDWriteFontCollection_Vtbl,
    GetFontSet: usize,
    GetFontFamily: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontFace,
    IDWriteFontFace_Vtbl,
    0x5f49804d_7024_4d43_bfa9_d25984f53849
);
windows_core::imp::interface_hierarchy!(IDWriteFontFace, windows_core::IUnknown);
impl IDWriteFontFace {
    pub(crate) unsafe fn GetDesignGlyphMetrics(
        &self,
        glyphindices: *const UINT16,
        glyphcount: UINT32,
        glyphmetrics: *mut DWRITE_GLYPH_METRICS,
        issideways: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesignGlyphMetrics)(
                windows_core::Interface::as_raw(self),
                glyphindices,
                glyphcount,
                glyphmetrics as _,
                issideways.into(),
            )
        }
    }
    pub(crate) unsafe fn GetGlyphIndices(
        &self,
        codepoints: *const UINT32,
        codepointcount: UINT32,
        glyphindices: *mut UINT16,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetGlyphIndices)(
                windows_core::Interface::as_raw(self),
                codepoints,
                codepointcount,
                glyphindices as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteFontFace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetType: usize,
    GetFiles: usize,
    GetIndex: usize,
    GetSimulations: usize,
    IsSymbolFont: usize,
    GetMetrics: usize,
    GetGlyphCount: usize,
    pub GetDesignGlyphMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const UINT16,
        UINT32,
        *mut DWRITE_GLYPH_METRICS,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const UINT32,
        UINT32,
        *mut UINT16,
    ) -> windows_core::HRESULT,
    TryGetFontTable: usize,
    ReleaseFontTable: usize,
    GetGlyphRunOutline: usize,
    GetRecommendedRenderingMode: usize,
    GetGdiCompatibleMetrics: usize,
    GetGdiCompatibleGlyphMetrics: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontFace1,
    IDWriteFontFace1_Vtbl,
    0xa71efdb4_9fdb_4838_ad90_cfc3be8c3daf
);
impl core::ops::Deref for IDWriteFontFace1 {
    type Target = IDWriteFontFace;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFace1, windows_core::IUnknown, IDWriteFontFace);
impl IDWriteFontFace1 {
    pub(crate) unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(
                windows_core::Interface::as_raw(self),
                fontmetrics as _,
            );
        }
    }
}
#[repr(C)]
pub struct IDWriteFontFace1_Vtbl {
    pub base__: IDWriteFontFace_Vtbl,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS1),
    GetGdiCompatibleMetrics: usize,
    GetCaretMetrics: usize,
    GetUnicodeRanges: usize,
    IsMonospacedFont: usize,
    GetDesignGlyphAdvances: usize,
    GetGdiCompatibleGlyphAdvances: usize,
    GetKerningPairAdjustments: usize,
    HasKerningPairs: usize,
    GetRecommendedRenderingMode: usize,
    GetVerticalGlyphVariants: usize,
    HasVerticalGlyphVariants: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontFace2,
    IDWriteFontFace2_Vtbl,
    0xd8b768ff_64bc_4e66_982b_ec8e87f693f7
);
impl core::ops::Deref for IDWriteFontFace2 {
    type Target = IDWriteFontFace1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteFontFace2,
    windows_core::IUnknown,
    IDWriteFontFace,
    IDWriteFontFace1
);
impl IDWriteFontFace2 {
    pub(crate) unsafe fn IsColorFont(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsColorFont)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
}
#[repr(C)]
pub struct IDWriteFontFace2_Vtbl {
    pub base__: IDWriteFontFace1_Vtbl,
    pub IsColorFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    GetColorPaletteCount: usize,
    GetPaletteEntryCount: usize,
    GetPaletteEntries: usize,
    GetRecommendedRenderingMode: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontFace3,
    IDWriteFontFace3_Vtbl,
    0xd37d7598_09be_4222_a236_2081341cc1f2
);
impl core::ops::Deref for IDWriteFontFace3 {
    type Target = IDWriteFontFace2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteFontFace3,
    windows_core::IUnknown,
    IDWriteFontFace,
    IDWriteFontFace1,
    IDWriteFontFace2
);
impl IDWriteFontFace3 {
    pub(crate) unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        unsafe {
            (windows_core::Interface::vtable(self).GetWeight)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        unsafe {
            (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GetFamilyNames(&self) -> windows_core::Result<IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyNames)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetRecommendedRenderingMode<P7>(
        &self,
        fontemsize: FLOAT,
        dpix: FLOAT,
        dpiy: FLOAT,
        transform: Option<*const DWRITE_MATRIX>,
        issideways: bool,
        outlinethreshold: DWRITE_OUTLINE_THRESHOLD,
        measuringmode: DWRITE_MEASURING_MODE,
        renderingparams: P7,
        renderingmode: *mut DWRITE_RENDERING_MODE1,
        gridfitmode: *mut DWRITE_GRID_FIT_MODE,
    ) -> windows_core::HRESULT
    where
        P7: windows_core::Param<IDWriteRenderingParams>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(
                windows_core::Interface::as_raw(self),
                fontemsize,
                dpix,
                dpiy,
                transform.unwrap_or(core::mem::zeroed()) as _,
                issideways.into(),
                outlinethreshold,
                measuringmode,
                renderingparams.param().abi(),
                renderingmode as _,
                gridfitmode as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteFontFace3_Vtbl {
    pub base__: IDWriteFontFace2_Vtbl,
    GetFontFaceReference: usize,
    GetPanose: usize,
    pub GetWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    GetStretch: usize,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub GetFamilyNames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetFaceNames: usize,
    GetInformationalStrings: usize,
    HasCharacter: usize,
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FLOAT,
        FLOAT,
        FLOAT,
        *const DWRITE_MATRIX,
        windows_core::BOOL,
        DWRITE_OUTLINE_THRESHOLD,
        DWRITE_MEASURING_MODE,
        *mut core::ffi::c_void,
        *mut DWRITE_RENDERING_MODE1,
        *mut DWRITE_GRID_FIT_MODE,
    ) -> windows_core::HRESULT,
    IsCharacterLocal: usize,
    IsGlyphLocal: usize,
    AreCharactersLocal: usize,
    AreGlyphsLocal: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontFallback,
    IDWriteFontFallback_Vtbl,
    0xefa008f9_f7a1_48bf_b05c_f224713cc0ff
);
windows_core::imp::interface_hierarchy!(IDWriteFontFallback, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    MapCharacters: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontFallbackBuilder,
    IDWriteFontFallbackBuilder_Vtbl,
    0xfd882d06_8aba_4fb8_b849_8be8b73e14de
);
windows_core::imp::interface_hierarchy!(IDWriteFontFallbackBuilder, windows_core::IUnknown);
impl IDWriteFontFallbackBuilder {
    pub(crate) unsafe fn AddMapping<P4>(
        &self,
        ranges: *const DWRITE_UNICODE_RANGE,
        rangescount: UINT32,
        targetfamilynames: *const *const WCHAR,
        targetfamilynamescount: UINT32,
        fontcollection: P4,
        localename: Option<*const WCHAR>,
        basefamilyname: Option<*const WCHAR>,
        scale: FLOAT,
    ) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IDWriteFontCollection>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddMapping)(
                windows_core::Interface::as_raw(self),
                ranges,
                rangescount,
                targetfamilynames,
                targetfamilynamescount,
                fontcollection.param().abi(),
                localename.unwrap_or(core::mem::zeroed()) as _,
                basefamilyname.unwrap_or(core::mem::zeroed()) as _,
                scale,
            )
        }
    }
    pub(crate) unsafe fn AddMappings<P0>(&self, fontfallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFallback>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddMappings)(
                windows_core::Interface::as_raw(self),
                fontfallback.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn CreateFontFallback(&self) -> windows_core::Result<IDWriteFontFallback> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFallback)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFontFallbackBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddMapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DWRITE_UNICODE_RANGE,
        UINT32,
        *const *const WCHAR,
        UINT32,
        *mut core::ffi::c_void,
        *const WCHAR,
        *const WCHAR,
        FLOAT,
    ) -> windows_core::HRESULT,
    pub AddMappings: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFontFallback: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDWriteFontFamily,
    IDWriteFontFamily_Vtbl,
    0xda20d8ef_812a_4c43_9802_62ec4abd7add
);
impl core::ops::Deref for IDWriteFontFamily {
    type Target = IDWriteFontList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFamily, windows_core::IUnknown, IDWriteFontList);
impl IDWriteFontFamily {
    pub(crate) unsafe fn GetFamilyNames(&self) -> windows_core::Result<IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyNames)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetFirstMatchingFont(
        &self,
        weight: DWRITE_FONT_WEIGHT,
        stretch: DWRITE_FONT_STRETCH,
        style: DWRITE_FONT_STYLE,
    ) -> windows_core::Result<IDWriteFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstMatchingFont)(
                windows_core::Interface::as_raw(self),
                weight,
                stretch,
                style,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFontFamily_Vtbl {
    pub base__: IDWriteFontList_Vtbl,
    pub GetFamilyNames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFirstMatchingFont: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_WEIGHT,
        DWRITE_FONT_STRETCH,
        DWRITE_FONT_STYLE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetMatchingFonts: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontFile,
    IDWriteFontFile_Vtbl,
    0x739d886a_cef5_47dc_8769_1a8b41bebbb0
);
windows_core::imp::interface_hierarchy!(IDWriteFontFile, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetReferenceKey: usize,
    GetLoader: usize,
    Analyze: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontFileLoader,
    IDWriteFontFileLoader_Vtbl,
    0x727cad4e_d6af_4c9e_8a08_d695b11caa49
);
windows_core::imp::interface_hierarchy!(IDWriteFontFileLoader, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFileLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateStreamFromKey: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontList,
    IDWriteFontList_Vtbl,
    0x1a0d8438_1d97_4ec1_aef9_a2fb86ed6acb
);
windows_core::imp::interface_hierarchy!(IDWriteFontList, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetFontCollection: usize,
    GetFontCount: usize,
    GetFont: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontSet,
    IDWriteFontSet_Vtbl,
    0x53585141_d9f8_4095_8321_d73cf6bd116b
);
windows_core::imp::interface_hierarchy!(IDWriteFontSet, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontSet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetFontCount: usize,
    GetFontFaceReference: usize,
    FindFontFaceReference: usize,
    FindFontFace: usize,
    GetPropertyValues: usize,
    GetPropertyValues2: usize,
    GetPropertyValues3: usize,
    GetPropertyOccurrenceCount: usize,
    GetMatchingFonts: usize,
    GetMatchingFonts2: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontSetBuilder,
    IDWriteFontSetBuilder_Vtbl,
    0x2f642afe_9c68_4f40_b8be_457401afcb3d
);
windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder, windows_core::IUnknown);
impl IDWriteFontSetBuilder {
    pub(crate) unsafe fn CreateFontSet(&self) -> windows_core::Result<IDWriteFontSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontSet)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFontSetBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    AddFontFaceReference: usize,
    AddFontFaceReference2: usize,
    AddFontSet: usize,
    pub CreateFontSet: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDWriteFontSetBuilder1,
    IDWriteFontSetBuilder1_Vtbl,
    0x3ff7715f_3cdc_4dc6_9b72_ec5621dccafd
);
impl core::ops::Deref for IDWriteFontSetBuilder1 {
    type Target = IDWriteFontSetBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteFontSetBuilder1,
    windows_core::IUnknown,
    IDWriteFontSetBuilder
);
impl IDWriteFontSetBuilder1 {
    pub(crate) unsafe fn AddFontFile<P0>(&self, fontfile: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFile>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddFontFile)(
                windows_core::Interface::as_raw(self),
                fontfile.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteFontSetBuilder1_Vtbl {
    pub base__: IDWriteFontSetBuilder_Vtbl,
    pub AddFontFile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDWriteGlyphRunAnalysis,
    IDWriteGlyphRunAnalysis_Vtbl,
    0x7d97dbf7_e085_42d4_81e3_6a883bded118
);
windows_core::imp::interface_hierarchy!(IDWriteGlyphRunAnalysis, windows_core::IUnknown);
impl IDWriteGlyphRunAnalysis {
    pub(crate) unsafe fn GetAlphaTextureBounds(
        &self,
        texturetype: DWRITE_TEXTURE_TYPE,
        texturebounds: *mut RECT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetAlphaTextureBounds)(
                windows_core::Interface::as_raw(self),
                texturetype,
                texturebounds as _,
            )
        }
    }
    pub(crate) unsafe fn CreateAlphaTexture(
        &self,
        texturetype: DWRITE_TEXTURE_TYPE,
        texturebounds: *const RECT,
        alphavalues: *mut BYTE,
        buffersize: UINT32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateAlphaTexture)(
                windows_core::Interface::as_raw(self),
                texturetype,
                texturebounds,
                alphavalues as _,
                buffersize,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteGlyphRunAnalysis_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAlphaTextureBounds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_TEXTURE_TYPE,
        *mut RECT,
    ) -> windows_core::HRESULT,
    pub CreateAlphaTexture: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_TEXTURE_TYPE,
        *const RECT,
        *mut BYTE,
        UINT32,
    ) -> windows_core::HRESULT,
    GetAlphaBlendParams: usize,
}
windows_core::imp::define_interface!(
    IDWriteInMemoryFontFileLoader,
    IDWriteInMemoryFontFileLoader_Vtbl,
    0xdc102f47_a12d_4b1c_822d_9e117e33043f
);
impl core::ops::Deref for IDWriteInMemoryFontFileLoader {
    type Target = IDWriteFontFileLoader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteInMemoryFontFileLoader,
    windows_core::IUnknown,
    IDWriteFontFileLoader
);
impl IDWriteInMemoryFontFileLoader {
    pub(crate) unsafe fn CreateInMemoryFontFileReference<P0, P3>(
        &self,
        factory: P0,
        fontdata: *const core::ffi::c_void,
        fontdatasize: UINT32,
        ownerobject: P3,
    ) -> windows_core::Result<IDWriteFontFile>
    where
        P0: windows_core::Param<IDWriteFactory>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInMemoryFontFileReference)(
                windows_core::Interface::as_raw(self),
                factory.param().abi(),
                fontdata,
                fontdatasize,
                ownerobject.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteInMemoryFontFileLoader_Vtbl {
    pub base__: IDWriteFontFileLoader_Vtbl,
    pub CreateInMemoryFontFileReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        UINT32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetFileCount: usize,
}
windows_core::imp::define_interface!(
    IDWriteInlineObject,
    IDWriteInlineObject_Vtbl,
    0x8339fde3_106f_47ab_8373_1c6295eb10b3
);
windows_core::imp::interface_hierarchy!(IDWriteInlineObject, windows_core::IUnknown);
impl IDWriteInlineObject {
    pub(crate) unsafe fn Draw<P1, P6>(
        &self,
        clientdrawingcontext: Option<*const core::ffi::c_void>,
        renderer: P1,
        originx: FLOAT,
        originy: FLOAT,
        issideways: bool,
        isrighttoleft: bool,
        clientdrawingeffect: P6,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDWriteTextRenderer>,
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(
                windows_core::Interface::as_raw(self),
                clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _,
                renderer.param().abi(),
                originx,
                originy,
                issideways.into(),
                isrighttoleft.into(),
                clientdrawingeffect.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteInlineObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut core::ffi::c_void,
        FLOAT,
        FLOAT,
        windows_core::BOOL,
        windows_core::BOOL,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetMetrics: usize,
    GetOverhangMetrics: usize,
    GetBreakConditions: usize,
}
windows_core::imp::define_interface!(
    IDWriteLocalizedStrings,
    IDWriteLocalizedStrings_Vtbl,
    0x08256209_099a_4b34_b86d_c22b110e7771
);
windows_core::imp::interface_hierarchy!(IDWriteLocalizedStrings, windows_core::IUnknown);
impl IDWriteLocalizedStrings {
    pub(crate) unsafe fn FindLocaleName(
        &self,
        localename: *const WCHAR,
        index: *mut UINT32,
        exists: *mut windows_core::BOOL,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).FindLocaleName)(
                windows_core::Interface::as_raw(self),
                localename,
                index as _,
                exists as _,
            )
        }
    }
    pub(crate) unsafe fn GetStringLength(
        &self,
        index: UINT32,
        length: *mut UINT32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetStringLength)(
                windows_core::Interface::as_raw(self),
                index,
                length as _,
            )
        }
    }
    pub(crate) unsafe fn GetString(
        &self,
        index: UINT32,
        stringbuffer: *mut WCHAR,
        size: UINT32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetString)(
                windows_core::Interface::as_raw(self),
                index,
                stringbuffer as _,
                size,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteLocalizedStrings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetCount: usize,
    pub FindLocaleName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const WCHAR,
        *mut UINT32,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
    pub GetStringLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UINT32,
        *mut UINT32,
    ) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UINT32,
        *mut WCHAR,
        UINT32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDWritePixelSnapping,
    IDWritePixelSnapping_Vtbl,
    0xeaf3a2da_ecf4_4d24_b644_b34f6842024b
);
windows_core::imp::interface_hierarchy!(IDWritePixelSnapping, windows_core::IUnknown);
#[repr(C)]
pub struct IDWritePixelSnapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsPixelSnappingDisabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetCurrentTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut DWRITE_MATRIX,
    ) -> windows_core::HRESULT,
    pub GetPixelsPerDip: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut FLOAT,
    ) -> windows_core::HRESULT,
}
pub trait IDWritePixelSnapping_Impl: windows_core::IUnknownImpl {
    fn IsPixelSnappingDisabled(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        isdisabled: *mut windows_core::BOOL,
    ) -> windows_core::HRESULT;
    fn GetCurrentTransform(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        transform: *mut DWRITE_MATRIX,
    ) -> windows_core::HRESULT;
    fn GetPixelsPerDip(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        pixelsperdip: *mut FLOAT,
    ) -> windows_core::HRESULT;
}
impl IDWritePixelSnapping_Vtbl {
    pub(crate) const fn new<Identity: IDWritePixelSnapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPixelSnappingDisabled<
            Identity: IDWritePixelSnapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            isdisabled: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePixelSnapping_Impl::IsPixelSnappingDisabled(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&isdisabled),
                )
            }
        }
        unsafe extern "system" fn GetCurrentTransform<
            Identity: IDWritePixelSnapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            transform: *mut DWRITE_MATRIX,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePixelSnapping_Impl::GetCurrentTransform(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&transform),
                )
            }
        }
        unsafe extern "system" fn GetPixelsPerDip<
            Identity: IDWritePixelSnapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            pixelsperdip: *mut FLOAT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePixelSnapping_Impl::GetPixelsPerDip(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&pixelsperdip),
                )
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsPixelSnappingDisabled: IsPixelSnappingDisabled::<Identity, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, OFFSET>,
        }
    }
    pub(crate) fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWritePixelSnapping as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWritePixelSnapping {}
windows_core::imp::define_interface!(
    IDWriteRenderingParams,
    IDWriteRenderingParams_Vtbl,
    0x2f0da53a_2add_47cd_82ee_d9ec34688e75
);
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams, windows_core::IUnknown);
impl IDWriteRenderingParams {
    pub(crate) unsafe fn GetGamma(&self) -> FLOAT {
        unsafe {
            (windows_core::Interface::vtable(self).GetGamma)(windows_core::Interface::as_raw(self))
        }
    }
    pub(crate) unsafe fn GetEnhancedContrast(&self) -> FLOAT {
        unsafe {
            (windows_core::Interface::vtable(self).GetEnhancedContrast)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetClearTypeLevel(&self) -> FLOAT {
        unsafe {
            (windows_core::Interface::vtable(self).GetClearTypeLevel)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        unsafe {
            (windows_core::Interface::vtable(self).GetPixelGeometry)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteRenderingParams_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGamma: unsafe extern "system" fn(*mut core::ffi::c_void) -> FLOAT,
    pub GetEnhancedContrast: unsafe extern "system" fn(*mut core::ffi::c_void) -> FLOAT,
    pub GetClearTypeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> FLOAT,
    pub GetPixelGeometry:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY,
    GetRenderingMode: usize,
}
windows_core::imp::define_interface!(
    IDWriteRenderingParams1,
    IDWriteRenderingParams1_Vtbl,
    0x94413cf4_a6fc_4248_8b50_6674348fcad3
);
impl core::ops::Deref for IDWriteRenderingParams1 {
    type Target = IDWriteRenderingParams;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteRenderingParams1,
    windows_core::IUnknown,
    IDWriteRenderingParams
);
impl IDWriteRenderingParams1 {
    pub(crate) unsafe fn GetGrayscaleEnhancedContrast(&self) -> FLOAT {
        unsafe {
            (windows_core::Interface::vtable(self).GetGrayscaleEnhancedContrast)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteRenderingParams1_Vtbl {
    pub base__: IDWriteRenderingParams_Vtbl,
    pub GetGrayscaleEnhancedContrast: unsafe extern "system" fn(*mut core::ffi::c_void) -> FLOAT,
}
windows_core::imp::define_interface!(
    IDWriteTextFormat,
    IDWriteTextFormat_Vtbl,
    0x9c906818_31d7_4fd3_a151_7c5e225db55a
);
windows_core::imp::interface_hierarchy!(IDWriteTextFormat, windows_core::IUnknown);
impl IDWriteTextFormat {
    pub(crate) unsafe fn SetTextAlignment(
        &self,
        textalignment: DWRITE_TEXT_ALIGNMENT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextAlignment)(
                windows_core::Interface::as_raw(self),
                textalignment,
            )
        }
    }
    pub(crate) unsafe fn SetParagraphAlignment(
        &self,
        paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetParagraphAlignment)(
                windows_core::Interface::as_raw(self),
                paragraphalignment,
            )
        }
    }
    pub(crate) unsafe fn SetWordWrapping(
        &self,
        wordwrapping: DWRITE_WORD_WRAPPING,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetWordWrapping)(
                windows_core::Interface::as_raw(self),
                wordwrapping,
            )
        }
    }
    pub(crate) unsafe fn SetReadingDirection(
        &self,
        readingdirection: DWRITE_READING_DIRECTION,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetReadingDirection)(
                windows_core::Interface::as_raw(self),
                readingdirection,
            )
        }
    }
    pub(crate) unsafe fn SetFlowDirection(
        &self,
        flowdirection: DWRITE_FLOW_DIRECTION,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFlowDirection)(
                windows_core::Interface::as_raw(self),
                flowdirection,
            )
        }
    }
    pub(crate) unsafe fn SetIncrementalTabStop(
        &self,
        incrementaltabstop: FLOAT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIncrementalTabStop)(
                windows_core::Interface::as_raw(self),
                incrementaltabstop,
            )
        }
    }
    pub(crate) unsafe fn SetTrimming<P1>(
        &self,
        trimmingoptions: *const DWRITE_TRIMMING,
        trimmingsign: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDWriteInlineObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTrimming)(
                windows_core::Interface::as_raw(self),
                trimmingoptions,
                trimmingsign.param().abi(),
            )
        }
    }
    pub(crate) unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        unsafe {
            (windows_core::Interface::vtable(self).GetReadingDirection)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub(crate) unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        unsafe {
            (windows_core::Interface::vtable(self).GetFlowDirection)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTextAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_TEXT_ALIGNMENT,
    ) -> windows_core::HRESULT,
    pub SetParagraphAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_PARAGRAPH_ALIGNMENT,
    ) -> windows_core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_WORD_WRAPPING,
    ) -> windows_core::HRESULT,
    pub SetReadingDirection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_READING_DIRECTION,
    ) -> windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FLOW_DIRECTION,
    ) -> windows_core::HRESULT,
    pub SetIncrementalTabStop:
        unsafe extern "system" fn(*mut core::ffi::c_void, FLOAT) -> windows_core::HRESULT,
    pub SetTrimming: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DWRITE_TRIMMING,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    SetLineSpacing: usize,
    GetTextAlignment: usize,
    GetParagraphAlignment: usize,
    GetWordWrapping: usize,
    pub GetReadingDirection:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_READING_DIRECTION,
    pub GetFlowDirection:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FLOW_DIRECTION,
    GetIncrementalTabStop: usize,
    GetTrimming: usize,
    GetLineSpacing: usize,
    GetFontCollection: usize,
    GetFontFamilyNameLength: usize,
    GetFontFamilyName: usize,
    GetFontWeight: usize,
    GetFontStyle: usize,
    GetFontStretch: usize,
    GetFontSize: usize,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
}
windows_core::imp::define_interface!(
    IDWriteTextFormat1,
    IDWriteTextFormat1_Vtbl,
    0x5f174b49_0d8b_4cfb_8bca_f1cce9d06c67
);
impl core::ops::Deref for IDWriteTextFormat1 {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextFormat1,
    windows_core::IUnknown,
    IDWriteTextFormat
);
impl IDWriteTextFormat1 {
    pub(crate) unsafe fn SetVerticalGlyphOrientation(
        &self,
        glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetVerticalGlyphOrientation)(
                windows_core::Interface::as_raw(self),
                glyphorientation,
            )
        }
    }
    pub(crate) unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFallback>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontFallback)(
                windows_core::Interface::as_raw(self),
                fontfallback.param().abi(),
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextFormat1_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_VERTICAL_GLYPH_ORIENTATION,
    ) -> windows_core::HRESULT,
    GetVerticalGlyphOrientation: usize,
    SetLastLineWrapping: usize,
    GetLastLineWrapping: usize,
    SetOpticalAlignment: usize,
    GetOpticalAlignment: usize,
    pub SetFontFallback: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetFontFallback: usize,
}
windows_core::imp::define_interface!(
    IDWriteTextFormat2,
    IDWriteTextFormat2_Vtbl,
    0xf67e0edd_9e3d_4ecc_8c32_4183253dfe70
);
impl core::ops::Deref for IDWriteTextFormat2 {
    type Target = IDWriteTextFormat1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextFormat2,
    windows_core::IUnknown,
    IDWriteTextFormat,
    IDWriteTextFormat1
);
impl IDWriteTextFormat2 {
    pub(crate) unsafe fn SetLineSpacing(
        &self,
        linespacingoptions: *const DWRITE_LINE_SPACING,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetLineSpacing)(
                windows_core::Interface::as_raw(self),
                linespacingoptions,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextFormat2_Vtbl {
    pub base__: IDWriteTextFormat1_Vtbl,
    pub SetLineSpacing: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DWRITE_LINE_SPACING,
    ) -> windows_core::HRESULT,
    GetLineSpacing: usize,
}
windows_core::imp::define_interface!(
    IDWriteTextLayout,
    IDWriteTextLayout_Vtbl,
    0x53737037_6d14_410b_9bfe_0b182bb70961
);
impl core::ops::Deref for IDWriteTextLayout {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextLayout,
    windows_core::IUnknown,
    IDWriteTextFormat
);
impl IDWriteTextLayout {
    pub(crate) unsafe fn SetMaxWidth(&self, maxwidth: FLOAT) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxWidth)(
                windows_core::Interface::as_raw(self),
                maxwidth,
            )
        }
    }
    pub(crate) unsafe fn SetMaxHeight(&self, maxheight: FLOAT) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxHeight)(
                windows_core::Interface::as_raw(self),
                maxheight,
            )
        }
    }
    pub(crate) unsafe fn SetFontCollection<P0>(
        &self,
        fontcollection: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontCollection>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontCollection)(
                windows_core::Interface::as_raw(self),
                fontcollection.param().abi(),
                textrange,
            )
        }
    }
    pub(crate) unsafe fn SetFontFamilyName(
        &self,
        fontfamilyname: *const WCHAR,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontFamilyName)(
                windows_core::Interface::as_raw(self),
                fontfamilyname,
                textrange,
            )
        }
    }
    pub(crate) unsafe fn SetFontWeight(
        &self,
        fontweight: DWRITE_FONT_WEIGHT,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontWeight)(
                windows_core::Interface::as_raw(self),
                fontweight,
                textrange,
            )
        }
    }
    pub(crate) unsafe fn SetFontStyle(
        &self,
        fontstyle: DWRITE_FONT_STYLE,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontStyle)(
                windows_core::Interface::as_raw(self),
                fontstyle,
                textrange,
            )
        }
    }
    pub(crate) unsafe fn SetFontSize(
        &self,
        fontsize: FLOAT,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontSize)(
                windows_core::Interface::as_raw(self),
                fontsize,
                textrange,
            )
        }
    }
    pub(crate) unsafe fn SetUnderline(
        &self,
        hasunderline: bool,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnderline)(
                windows_core::Interface::as_raw(self),
                hasunderline.into(),
                textrange,
            )
        }
    }
    pub(crate) unsafe fn SetStrikethrough(
        &self,
        hasstrikethrough: bool,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetStrikethrough)(
                windows_core::Interface::as_raw(self),
                hasstrikethrough.into(),
                textrange,
            )
        }
    }
    pub(crate) unsafe fn SetTypography<P0>(
        &self,
        typography: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteTypography>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTypography)(
                windows_core::Interface::as_raw(self),
                typography.param().abi(),
                textrange,
            )
        }
    }
    pub(crate) unsafe fn SetLocaleName(
        &self,
        localename: *const WCHAR,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetLocaleName)(
                windows_core::Interface::as_raw(self),
                localename,
                textrange,
            )
        }
    }
    pub(crate) unsafe fn Draw<P1>(
        &self,
        clientdrawingcontext: Option<*const core::ffi::c_void>,
        renderer: P1,
        originx: FLOAT,
        originy: FLOAT,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDWriteTextRenderer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(
                windows_core::Interface::as_raw(self),
                clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _,
                renderer.param().abi(),
                originx,
                originy,
            )
        }
    }
    pub(crate) unsafe fn GetLineMetrics(
        &self,
        linemetrics: Option<*mut DWRITE_LINE_METRICS>,
        maxlinecount: UINT32,
        actuallinecount: *mut UINT32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetLineMetrics)(
                windows_core::Interface::as_raw(self),
                linemetrics.unwrap_or(core::mem::zeroed()) as _,
                maxlinecount,
                actuallinecount as _,
            )
        }
    }
    pub(crate) unsafe fn GetClusterMetrics(
        &self,
        clustermetrics: Option<*mut DWRITE_CLUSTER_METRICS>,
        maxclustercount: UINT32,
        actualclustercount: *mut UINT32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetClusterMetrics)(
                windows_core::Interface::as_raw(self),
                clustermetrics.unwrap_or(core::mem::zeroed()) as _,
                maxclustercount,
                actualclustercount as _,
            )
        }
    }
    pub(crate) unsafe fn HitTestPoint(
        &self,
        pointx: FLOAT,
        pointy: FLOAT,
        istrailinghit: *mut windows_core::BOOL,
        isinside: *mut windows_core::BOOL,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestPoint)(
                windows_core::Interface::as_raw(self),
                pointx,
                pointy,
                istrailinghit as _,
                isinside as _,
                hittestmetrics as _,
            )
        }
    }
    pub(crate) unsafe fn HitTestTextPosition(
        &self,
        textposition: UINT32,
        istrailinghit: bool,
        pointx: *mut FLOAT,
        pointy: *mut FLOAT,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestTextPosition)(
                windows_core::Interface::as_raw(self),
                textposition,
                istrailinghit.into(),
                pointx as _,
                pointy as _,
                hittestmetrics as _,
            )
        }
    }
    pub(crate) unsafe fn HitTestTextRange(
        &self,
        textposition: UINT32,
        textlength: UINT32,
        originx: FLOAT,
        originy: FLOAT,
        hittestmetrics: Option<*mut DWRITE_HIT_TEST_METRICS>,
        maxhittestmetricscount: UINT32,
        actualhittestmetricscount: *mut UINT32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestTextRange)(
                windows_core::Interface::as_raw(self),
                textposition,
                textlength,
                originx,
                originy,
                hittestmetrics.unwrap_or(core::mem::zeroed()) as _,
                maxhittestmetricscount,
                actualhittestmetricscount as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetMaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, FLOAT) -> windows_core::HRESULT,
    pub SetMaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FLOAT) -> windows_core::HRESULT,
    pub SetFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontFamilyName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const WCHAR,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_WEIGHT,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_STYLE,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    SetFontStretch: usize,
    pub SetFontSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FLOAT,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    SetDrawingEffect: usize,
    SetInlineObject: usize,
    pub SetTypography: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetLocaleName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const WCHAR,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    GetMaxWidth: usize,
    GetMaxHeight: usize,
    GetFontCollection: usize,
    GetFontFamilyNameLength: usize,
    GetFontFamilyName: usize,
    GetFontWeight: usize,
    GetFontStyle: usize,
    GetFontStretch: usize,
    GetFontSize: usize,
    GetUnderline: usize,
    GetStrikethrough: usize,
    GetDrawingEffect: usize,
    GetInlineObject: usize,
    GetTypography: usize,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
    pub Draw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut core::ffi::c_void,
        FLOAT,
        FLOAT,
    ) -> windows_core::HRESULT,
    pub GetLineMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_LINE_METRICS,
        UINT32,
        *mut UINT32,
    ) -> windows_core::HRESULT,
    GetMetrics: usize,
    GetOverhangMetrics: usize,
    pub GetClusterMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_CLUSTER_METRICS,
        UINT32,
        *mut UINT32,
    ) -> windows_core::HRESULT,
    DetermineMinWidth: usize,
    pub HitTestPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FLOAT,
        FLOAT,
        *mut windows_core::BOOL,
        *mut windows_core::BOOL,
        *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT,
    pub HitTestTextPosition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UINT32,
        windows_core::BOOL,
        *mut FLOAT,
        *mut FLOAT,
        *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT,
    pub HitTestTextRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UINT32,
        UINT32,
        FLOAT,
        FLOAT,
        *mut DWRITE_HIT_TEST_METRICS,
        UINT32,
        *mut UINT32,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDWriteTextLayout1,
    IDWriteTextLayout1_Vtbl,
    0x9064d822_80a7_465c_a986_df65f78b8feb
);
impl core::ops::Deref for IDWriteTextLayout1 {
    type Target = IDWriteTextLayout;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextLayout1,
    windows_core::IUnknown,
    IDWriteTextFormat,
    IDWriteTextLayout
);
#[repr(C)]
pub struct IDWriteTextLayout1_Vtbl {
    pub base__: IDWriteTextLayout_Vtbl,
    SetPairKerning: usize,
    GetPairKerning: usize,
    SetCharacterSpacing: usize,
    GetCharacterSpacing: usize,
}
windows_core::imp::define_interface!(
    IDWriteTextLayout2,
    IDWriteTextLayout2_Vtbl,
    0x1093c18f_8d5e_43f0_b064_0917311b525e
);
impl core::ops::Deref for IDWriteTextLayout2 {
    type Target = IDWriteTextLayout1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextLayout2,
    windows_core::IUnknown,
    IDWriteTextFormat,
    IDWriteTextLayout,
    IDWriteTextLayout1
);
impl IDWriteTextLayout2 {
    pub(crate) unsafe fn GetMetrics(
        &self,
        textmetrics: *mut DWRITE_TEXT_METRICS1,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(
                windows_core::Interface::as_raw(self),
                textmetrics as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextLayout2_Vtbl {
    pub base__: IDWriteTextLayout1_Vtbl,
    pub GetMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_TEXT_METRICS1,
    ) -> windows_core::HRESULT,
    SetVerticalGlyphOrientation: usize,
    GetVerticalGlyphOrientation: usize,
    SetLastLineWrapping: usize,
    GetLastLineWrapping: usize,
    SetOpticalAlignment: usize,
    GetOpticalAlignment: usize,
    SetFontFallback: usize,
    GetFontFallback: usize,
}
windows_core::imp::define_interface!(
    IDWriteTextRenderer,
    IDWriteTextRenderer_Vtbl,
    0xef8a8135_5cc6_45fe_8825_c5a0724eb819
);
impl core::ops::Deref for IDWriteTextRenderer {
    type Target = IDWritePixelSnapping;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextRenderer,
    windows_core::IUnknown,
    IDWritePixelSnapping
);
#[repr(C)]
pub struct IDWriteTextRenderer_Vtbl {
    pub base__: IDWritePixelSnapping_Vtbl,
    pub DrawGlyphRun: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        FLOAT,
        FLOAT,
        DWRITE_MEASURING_MODE,
        *const DWRITE_GLYPH_RUN,
        *const DWRITE_GLYPH_RUN_DESCRIPTION,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DrawUnderline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        FLOAT,
        FLOAT,
        *const DWRITE_UNDERLINE,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DrawStrikethrough: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        FLOAT,
        FLOAT,
        *const DWRITE_STRIKETHROUGH,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DrawInlineObject: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        FLOAT,
        FLOAT,
        *mut core::ffi::c_void,
        windows_core::BOOL,
        windows_core::BOOL,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDWriteTextRenderer_Impl: IDWritePixelSnapping_Impl {
    fn DrawGlyphRun(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: FLOAT,
        baselineoriginy: FLOAT,
        measuringmode: DWRITE_MEASURING_MODE,
        glyphrun: *const DWRITE_GLYPH_RUN,
        glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::HRESULT;
    fn DrawUnderline(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: FLOAT,
        baselineoriginy: FLOAT,
        underline: *const DWRITE_UNDERLINE,
        clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::HRESULT;
    fn DrawStrikethrough(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: FLOAT,
        baselineoriginy: FLOAT,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::HRESULT;
    fn DrawInlineObject(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        originx: FLOAT,
        originy: FLOAT,
        inlineobject: windows_core::Ref<IDWriteInlineObject>,
        issideways: windows_core::BOOL,
        isrighttoleft: windows_core::BOOL,
        clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::HRESULT;
}
impl IDWriteTextRenderer_Vtbl {
    pub(crate) const fn new<Identity: IDWriteTextRenderer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DrawGlyphRun<
            Identity: IDWriteTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            baselineoriginx: FLOAT,
            baselineoriginy: FLOAT,
            measuringmode: DWRITE_MEASURING_MODE,
            glyphrun: *const DWRITE_GLYPH_RUN,
            glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
            clientdrawingeffect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer_Impl::DrawGlyphRun(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&baselineoriginx),
                    core::mem::transmute_copy(&baselineoriginy),
                    core::mem::transmute_copy(&measuringmode),
                    core::mem::transmute_copy(&glyphrun),
                    core::mem::transmute_copy(&glyphrundescription),
                    core::mem::transmute_copy(&clientdrawingeffect),
                )
            }
        }
        unsafe extern "system" fn DrawUnderline<
            Identity: IDWriteTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            baselineoriginx: FLOAT,
            baselineoriginy: FLOAT,
            underline: *const DWRITE_UNDERLINE,
            clientdrawingeffect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer_Impl::DrawUnderline(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&baselineoriginx),
                    core::mem::transmute_copy(&baselineoriginy),
                    core::mem::transmute_copy(&underline),
                    core::mem::transmute_copy(&clientdrawingeffect),
                )
            }
        }
        unsafe extern "system" fn DrawStrikethrough<
            Identity: IDWriteTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            baselineoriginx: FLOAT,
            baselineoriginy: FLOAT,
            strikethrough: *const DWRITE_STRIKETHROUGH,
            clientdrawingeffect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer_Impl::DrawStrikethrough(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&baselineoriginx),
                    core::mem::transmute_copy(&baselineoriginy),
                    core::mem::transmute_copy(&strikethrough),
                    core::mem::transmute_copy(&clientdrawingeffect),
                )
            }
        }
        unsafe extern "system" fn DrawInlineObject<
            Identity: IDWriteTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            originx: FLOAT,
            originy: FLOAT,
            inlineobject: *mut core::ffi::c_void,
            issideways: windows_core::BOOL,
            isrighttoleft: windows_core::BOOL,
            clientdrawingeffect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer_Impl::DrawInlineObject(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&originx),
                    core::mem::transmute_copy(&originy),
                    core::mem::transmute_copy(&inlineobject),
                    core::mem::transmute_copy(&issideways),
                    core::mem::transmute_copy(&isrighttoleft),
                    core::mem::transmute_copy(&clientdrawingeffect),
                )
            }
        }
        Self {
            base__: IDWritePixelSnapping_Vtbl::new::<Identity, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawUnderline: DrawUnderline::<Identity, OFFSET>,
            DrawStrikethrough: DrawStrikethrough::<Identity, OFFSET>,
            DrawInlineObject: DrawInlineObject::<Identity, OFFSET>,
        }
    }
    pub(crate) fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextRenderer as windows_core::Interface>::IID
            || IDWritePixelSnapping_Vtbl::matches(iid)
    }
}
impl windows_core::RuntimeName for IDWriteTextRenderer {}
windows_core::imp::define_interface!(
    IDWriteTextRenderer1,
    IDWriteTextRenderer1_Vtbl,
    0xd3e0e934_22a0_427e_aae4_7d9574b59db1
);
impl core::ops::Deref for IDWriteTextRenderer1 {
    type Target = IDWriteTextRenderer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextRenderer1,
    windows_core::IUnknown,
    IDWritePixelSnapping,
    IDWriteTextRenderer
);
#[repr(C)]
pub struct IDWriteTextRenderer1_Vtbl {
    pub base__: IDWriteTextRenderer_Vtbl,
    pub DrawGlyphRun: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        FLOAT,
        FLOAT,
        DWRITE_GLYPH_ORIENTATION_ANGLE,
        DWRITE_MEASURING_MODE,
        *const DWRITE_GLYPH_RUN,
        *const DWRITE_GLYPH_RUN_DESCRIPTION,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DrawUnderline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        FLOAT,
        FLOAT,
        DWRITE_GLYPH_ORIENTATION_ANGLE,
        *const DWRITE_UNDERLINE,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DrawStrikethrough: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        FLOAT,
        FLOAT,
        DWRITE_GLYPH_ORIENTATION_ANGLE,
        *const DWRITE_STRIKETHROUGH,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DrawInlineObject: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        FLOAT,
        FLOAT,
        DWRITE_GLYPH_ORIENTATION_ANGLE,
        *mut core::ffi::c_void,
        windows_core::BOOL,
        windows_core::BOOL,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDWriteTextRenderer1_Impl: IDWriteTextRenderer_Impl {
    fn DrawGlyphRun(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: FLOAT,
        baselineoriginy: FLOAT,
        orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        measuringmode: DWRITE_MEASURING_MODE,
        glyphrun: *const DWRITE_GLYPH_RUN,
        glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::HRESULT;
    fn DrawUnderline(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: FLOAT,
        baselineoriginy: FLOAT,
        orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        underline: *const DWRITE_UNDERLINE,
        clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::HRESULT;
    fn DrawStrikethrough(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: FLOAT,
        baselineoriginy: FLOAT,
        orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::HRESULT;
    fn DrawInlineObject(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        originx: FLOAT,
        originy: FLOAT,
        orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        inlineobject: windows_core::Ref<IDWriteInlineObject>,
        issideways: windows_core::BOOL,
        isrighttoleft: windows_core::BOOL,
        clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::HRESULT;
}
impl IDWriteTextRenderer1_Vtbl {
    pub(crate) const fn new<Identity: IDWriteTextRenderer1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DrawGlyphRun<
            Identity: IDWriteTextRenderer1_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            baselineoriginx: FLOAT,
            baselineoriginy: FLOAT,
            orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
            measuringmode: DWRITE_MEASURING_MODE,
            glyphrun: *const DWRITE_GLYPH_RUN,
            glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
            clientdrawingeffect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer1_Impl::DrawGlyphRun(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&baselineoriginx),
                    core::mem::transmute_copy(&baselineoriginy),
                    core::mem::transmute_copy(&orientationangle),
                    core::mem::transmute_copy(&measuringmode),
                    core::mem::transmute_copy(&glyphrun),
                    core::mem::transmute_copy(&glyphrundescription),
                    core::mem::transmute_copy(&clientdrawingeffect),
                )
            }
        }
        unsafe extern "system" fn DrawUnderline<
            Identity: IDWriteTextRenderer1_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            baselineoriginx: FLOAT,
            baselineoriginy: FLOAT,
            orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
            underline: *const DWRITE_UNDERLINE,
            clientdrawingeffect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer1_Impl::DrawUnderline(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&baselineoriginx),
                    core::mem::transmute_copy(&baselineoriginy),
                    core::mem::transmute_copy(&orientationangle),
                    core::mem::transmute_copy(&underline),
                    core::mem::transmute_copy(&clientdrawingeffect),
                )
            }
        }
        unsafe extern "system" fn DrawStrikethrough<
            Identity: IDWriteTextRenderer1_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            baselineoriginx: FLOAT,
            baselineoriginy: FLOAT,
            orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
            strikethrough: *const DWRITE_STRIKETHROUGH,
            clientdrawingeffect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer1_Impl::DrawStrikethrough(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&baselineoriginx),
                    core::mem::transmute_copy(&baselineoriginy),
                    core::mem::transmute_copy(&orientationangle),
                    core::mem::transmute_copy(&strikethrough),
                    core::mem::transmute_copy(&clientdrawingeffect),
                )
            }
        }
        unsafe extern "system" fn DrawInlineObject<
            Identity: IDWriteTextRenderer1_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            originx: FLOAT,
            originy: FLOAT,
            orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
            inlineobject: *mut core::ffi::c_void,
            issideways: windows_core::BOOL,
            isrighttoleft: windows_core::BOOL,
            clientdrawingeffect: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer1_Impl::DrawInlineObject(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&originx),
                    core::mem::transmute_copy(&originy),
                    core::mem::transmute_copy(&orientationangle),
                    core::mem::transmute_copy(&inlineobject),
                    core::mem::transmute_copy(&issideways),
                    core::mem::transmute_copy(&isrighttoleft),
                    core::mem::transmute_copy(&clientdrawingeffect),
                )
            }
        }
        Self {
            base__: IDWriteTextRenderer_Vtbl::new::<Identity, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            DrawUnderline: DrawUnderline::<Identity, OFFSET>,
            DrawStrikethrough: DrawStrikethrough::<Identity, OFFSET>,
            DrawInlineObject: DrawInlineObject::<Identity, OFFSET>,
        }
    }
    pub(crate) fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextRenderer1 as windows_core::Interface>::IID
            || IDWriteTextRenderer_Vtbl::matches(iid)
    }
}
impl windows_core::RuntimeName for IDWriteTextRenderer1 {}
windows_core::imp::define_interface!(
    IDWriteTypography,
    IDWriteTypography_Vtbl,
    0x55f1112b_1dc2_4b3c_9541_f46894ed85b6
);
windows_core::imp::interface_hierarchy!(IDWriteTypography, windows_core::IUnknown);
impl IDWriteTypography {
    pub(crate) unsafe fn AddFontFeature(
        &self,
        fontfeature: DWRITE_FONT_FEATURE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).AddFontFeature)(
                windows_core::Interface::as_raw(self),
                fontfeature,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTypography_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFontFeature: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_FEATURE,
    ) -> windows_core::HRESULT,
    GetFontFeatureCount: usize,
    GetFontFeature: usize,
}
pub type IID = windows_core::GUID;
pub type INT16 = i16;
pub type INT32 = i32;
pub const LOCALE_NAME_MAX_LENGTH: i32 = 85;
pub type LONG = i32;
pub type PVOID = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
pub const SPI_GETFONTSMOOTHING: i32 = 74;
pub const SPI_GETFONTSMOOTHINGTYPE: i32 = 8202;
pub type UINT = u32;
pub type UINT16 = u16;
pub type UINT32 = u32;
pub type WCHAR = u16;
