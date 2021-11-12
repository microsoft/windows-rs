#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
    pub fn DWriteCreateFactory(factorytype: DWRITE_FACTORY_TYPE, iid: *const ::windows_sys::core::GUID, factory: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_ALPHA_MAX: u32 = 255u32;
pub struct DWRITE_AUTOMATIC_FONT_AXES(i32);
pub struct DWRITE_BASELINE(i32);
pub struct DWRITE_BREAK_CONDITION(i32);
pub struct DWRITE_CARET_METRICS(i32);
pub struct DWRITE_CLUSTER_METRICS(i32);
pub struct DWRITE_COLOR_F(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_COLOR_GLYPH_RUN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_COLOR_GLYPH_RUN1(i32);
pub struct DWRITE_CONTAINER_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_ERR_BASE: u32 = 20480u32;
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_E_DOWNLOADCANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283954i32 as _);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_E_DOWNLOADFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283953i32 as _);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_E_REMOTEFONT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283955i32 as _);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_E_TOOMANYDOWNLOADS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283952i32 as _);
pub struct DWRITE_FACTORY_TYPE(i32);
pub struct DWRITE_FILE_FRAGMENT(i32);
pub struct DWRITE_FLOW_DIRECTION(i32);
pub struct DWRITE_FONT_AXIS_ATTRIBUTES(i32);
pub struct DWRITE_FONT_AXIS_RANGE(i32);
pub struct DWRITE_FONT_AXIS_TAG(i32);
pub struct DWRITE_FONT_AXIS_VALUE(i32);
pub struct DWRITE_FONT_FACE_TYPE(i32);
pub struct DWRITE_FONT_FAMILY_MODEL(i32);
pub struct DWRITE_FONT_FEATURE(i32);
pub struct DWRITE_FONT_FEATURE_TAG(i32);
pub struct DWRITE_FONT_FILE_TYPE(i32);
pub struct DWRITE_FONT_LINE_GAP_USAGE(i32);
pub struct DWRITE_FONT_METRICS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_FONT_METRICS1(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_FONT_PROPERTY(i32);
pub struct DWRITE_FONT_PROPERTY_ID(i32);
pub struct DWRITE_FONT_SIMULATIONS(i32);
pub struct DWRITE_FONT_SOURCE_TYPE(i32);
pub struct DWRITE_FONT_STRETCH(i32);
pub struct DWRITE_FONT_STYLE(i32);
pub struct DWRITE_FONT_WEIGHT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct DWRITE_GLYPH_IMAGE_DATA(i32);
pub struct DWRITE_GLYPH_IMAGE_FORMATS(i32);
pub struct DWRITE_GLYPH_METRICS(i32);
pub struct DWRITE_GLYPH_OFFSET(i32);
pub struct DWRITE_GLYPH_ORIENTATION_ANGLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_GLYPH_RUN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION(i32);
pub struct DWRITE_GRID_FIT_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_HIT_TEST_METRICS(i32);
pub struct DWRITE_INFORMATIONAL_STRING_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_INLINE_OBJECT_METRICS(i32);
pub struct DWRITE_JUSTIFICATION_OPPORTUNITY(i32);
pub struct DWRITE_LINE_BREAKPOINT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_LINE_METRICS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_LINE_METRICS1(i32);
pub struct DWRITE_LINE_SPACING(i32);
pub struct DWRITE_LINE_SPACING_METHOD(i32);
pub struct DWRITE_LOCALITY(i32);
pub struct DWRITE_MATRIX(i32);
pub struct DWRITE_MEASURING_MODE(i32);
pub struct DWRITE_NUMBER_SUBSTITUTION_METHOD(i32);
pub struct DWRITE_OPTICAL_ALIGNMENT(i32);
pub struct DWRITE_OUTLINE_THRESHOLD(i32);
pub struct DWRITE_OVERHANG_METRICS(i32);
pub struct DWRITE_PANOSE(i32);
pub struct DWRITE_PANOSE_ARM_STYLE(i32);
pub struct DWRITE_PANOSE_ASPECT(i32);
pub struct DWRITE_PANOSE_ASPECT_RATIO(i32);
pub struct DWRITE_PANOSE_CHARACTER_RANGES(i32);
pub struct DWRITE_PANOSE_CONTRAST(i32);
pub struct DWRITE_PANOSE_DECORATIVE_CLASS(i32);
pub struct DWRITE_PANOSE_DECORATIVE_TOPOLOGY(i32);
pub struct DWRITE_PANOSE_FAMILY(i32);
pub struct DWRITE_PANOSE_FILL(i32);
pub struct DWRITE_PANOSE_FINIALS(i32);
pub struct DWRITE_PANOSE_LETTERFORM(i32);
pub struct DWRITE_PANOSE_LINING(i32);
pub struct DWRITE_PANOSE_MIDLINE(i32);
pub struct DWRITE_PANOSE_PROPORTION(i32);
pub struct DWRITE_PANOSE_SCRIPT_FORM(i32);
pub struct DWRITE_PANOSE_SCRIPT_TOPOLOGY(i32);
pub struct DWRITE_PANOSE_SERIF_STYLE(i32);
pub struct DWRITE_PANOSE_SPACING(i32);
pub struct DWRITE_PANOSE_STROKE_VARIATION(i32);
pub struct DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(i32);
pub struct DWRITE_PANOSE_SYMBOL_KIND(i32);
pub struct DWRITE_PANOSE_TOOL_KIND(i32);
pub struct DWRITE_PANOSE_WEIGHT(i32);
pub struct DWRITE_PANOSE_XASCENT(i32);
pub struct DWRITE_PANOSE_XHEIGHT(i32);
pub struct DWRITE_PARAGRAPH_ALIGNMENT(i32);
pub struct DWRITE_PIXEL_GEOMETRY(i32);
pub struct DWRITE_READING_DIRECTION(i32);
pub struct DWRITE_RENDERING_MODE(i32);
pub struct DWRITE_RENDERING_MODE1(i32);
pub struct DWRITE_SCRIPT_ANALYSIS(i32);
pub struct DWRITE_SCRIPT_PROPERTIES(i32);
pub struct DWRITE_SCRIPT_SHAPES(i32);
pub struct DWRITE_SHAPING_GLYPH_PROPERTIES(i32);
pub struct DWRITE_SHAPING_TEXT_PROPERTIES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_STRIKETHROUGH(i32);
pub struct DWRITE_TEXTURE_TYPE(i32);
pub struct DWRITE_TEXT_ALIGNMENT(i32);
pub struct DWRITE_TEXT_ANTIALIAS_MODE(i32);
pub struct DWRITE_TEXT_METRICS(i32);
pub struct DWRITE_TEXT_METRICS1(i32);
pub struct DWRITE_TEXT_RANGE(i32);
pub struct DWRITE_TRIMMING(i32);
pub struct DWRITE_TRIMMING_GRANULARITY(i32);
pub struct DWRITE_TYPOGRAPHIC_FEATURES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_UNDERLINE(i32);
pub struct DWRITE_UNICODE_RANGE(i32);
pub struct DWRITE_VERTICAL_GLYPH_ORIENTATION(i32);
pub struct DWRITE_WORD_WRAPPING(i32);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const FACILITY_DWRITE: u32 = 2200u32;
pub struct IDWriteAsyncResult(pub *mut ::core::ffi::c_void);
pub struct IDWriteBitmapRenderTarget(pub *mut ::core::ffi::c_void);
pub struct IDWriteBitmapRenderTarget1(pub *mut ::core::ffi::c_void);
pub struct IDWriteColorGlyphRunEnumerator(pub *mut ::core::ffi::c_void);
pub struct IDWriteColorGlyphRunEnumerator1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFactory(pub *mut ::core::ffi::c_void);
pub struct IDWriteFactory1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFactory2(pub *mut ::core::ffi::c_void);
pub struct IDWriteFactory3(pub *mut ::core::ffi::c_void);
pub struct IDWriteFactory4(pub *mut ::core::ffi::c_void);
pub struct IDWriteFactory5(pub *mut ::core::ffi::c_void);
pub struct IDWriteFactory6(pub *mut ::core::ffi::c_void);
pub struct IDWriteFactory7(pub *mut ::core::ffi::c_void);
pub struct IDWriteFont(pub *mut ::core::ffi::c_void);
pub struct IDWriteFont1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFont2(pub *mut ::core::ffi::c_void);
pub struct IDWriteFont3(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontCollection(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontCollection1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontCollection2(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontCollection3(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontCollectionLoader(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontDownloadListener(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontDownloadQueue(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFace(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFace1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFace2(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFace3(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFace4(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFace5(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFace6(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFaceReference(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFaceReference1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFallback(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFallback1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFallbackBuilder(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFamily(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFamily1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFamily2(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFile(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFileEnumerator(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFileLoader(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontFileStream(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontList(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontList1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontList2(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontResource(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontSet(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontSet1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontSet2(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontSet3(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontSetBuilder(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontSetBuilder1(pub *mut ::core::ffi::c_void);
pub struct IDWriteFontSetBuilder2(pub *mut ::core::ffi::c_void);
pub struct IDWriteGdiInterop(pub *mut ::core::ffi::c_void);
pub struct IDWriteGdiInterop1(pub *mut ::core::ffi::c_void);
pub struct IDWriteGlyphRunAnalysis(pub *mut ::core::ffi::c_void);
pub struct IDWriteInMemoryFontFileLoader(pub *mut ::core::ffi::c_void);
pub struct IDWriteInlineObject(pub *mut ::core::ffi::c_void);
pub struct IDWriteLocalFontFileLoader(pub *mut ::core::ffi::c_void);
pub struct IDWriteLocalizedStrings(pub *mut ::core::ffi::c_void);
pub struct IDWriteNumberSubstitution(pub *mut ::core::ffi::c_void);
pub struct IDWritePixelSnapping(pub *mut ::core::ffi::c_void);
pub struct IDWriteRemoteFontFileLoader(pub *mut ::core::ffi::c_void);
pub struct IDWriteRemoteFontFileStream(pub *mut ::core::ffi::c_void);
pub struct IDWriteRenderingParams(pub *mut ::core::ffi::c_void);
pub struct IDWriteRenderingParams1(pub *mut ::core::ffi::c_void);
pub struct IDWriteRenderingParams2(pub *mut ::core::ffi::c_void);
pub struct IDWriteRenderingParams3(pub *mut ::core::ffi::c_void);
pub struct IDWriteStringList(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextAnalysisSink(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextAnalysisSink1(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextAnalysisSource(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextAnalysisSource1(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextAnalyzer(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextAnalyzer1(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextAnalyzer2(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextFormat(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextFormat1(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextFormat2(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextFormat3(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextLayout(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextLayout1(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextLayout2(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextLayout3(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextLayout4(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextRenderer(pub *mut ::core::ffi::c_void);
pub struct IDWriteTextRenderer1(pub *mut ::core::ffi::c_void);
pub struct IDWriteTypography(pub *mut ::core::ffi::c_void);
