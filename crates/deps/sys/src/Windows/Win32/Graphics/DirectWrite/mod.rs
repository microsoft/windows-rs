#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn DWriteCreateFactory(factorytype: DWRITE_FACTORY_TYPE, iid: *const ::windows_sys::core::GUID, factory: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
pub const DWRITE_ALPHA_MAX: u32 = 255u32;
pub const DWRITE_AUTOMATIC_FONT_AXES_NONE: u32 = 0u32;
pub const DWRITE_AUTOMATIC_FONT_AXES_OPTICAL_SIZE: u32 = 1u32;
pub const DWRITE_BASELINE_DEFAULT: i32 = 0i32;
pub const DWRITE_BASELINE_ROMAN: i32 = 1i32;
pub const DWRITE_BASELINE_CENTRAL: i32 = 2i32;
pub const DWRITE_BASELINE_MATH: i32 = 3i32;
pub const DWRITE_BASELINE_HANGING: i32 = 4i32;
pub const DWRITE_BASELINE_IDEOGRAPHIC_BOTTOM: i32 = 5i32;
pub const DWRITE_BASELINE_IDEOGRAPHIC_TOP: i32 = 6i32;
pub const DWRITE_BASELINE_MINIMUM: i32 = 7i32;
pub const DWRITE_BASELINE_MAXIMUM: i32 = 8i32;
pub const DWRITE_BREAK_CONDITION_NEUTRAL: i32 = 0i32;
pub const DWRITE_BREAK_CONDITION_CAN_BREAK: i32 = 1i32;
pub const DWRITE_BREAK_CONDITION_MAY_NOT_BREAK: i32 = 2i32;
pub const DWRITE_BREAK_CONDITION_MUST_BREAK: i32 = 3i32;
#[repr(C)]
pub struct DWRITE_CARET_METRICS {
    pub slopeRise: i16,
    pub slopeRun: i16,
    pub offset: i16,
}
impl ::core::marker::Copy for DWRITE_CARET_METRICS {}
impl ::core::clone::Clone for DWRITE_CARET_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: f32,
    pub length: u16,
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_CLUSTER_METRICS {}
impl ::core::clone::Clone for DWRITE_CLUSTER_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for DWRITE_COLOR_F {}
impl ::core::clone::Clone for DWRITE_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_COLOR_GLYPH_RUN {
    pub glyphRun: DWRITE_GLYPH_RUN,
    pub glyphRunDescription: *mut DWRITE_GLYPH_RUN_DESCRIPTION,
    pub baselineOriginX: f32,
    pub baselineOriginY: f32,
    pub runColor: DWRITE_COLOR_F,
    pub paletteIndex: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_COLOR_GLYPH_RUN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_COLOR_GLYPH_RUN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_COLOR_GLYPH_RUN1 {
    pub Base: DWRITE_COLOR_GLYPH_RUN,
    pub glyphImageFormat: DWRITE_GLYPH_IMAGE_FORMATS,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_COLOR_GLYPH_RUN1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_COLOR_GLYPH_RUN1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_CONTAINER_TYPE_UNKNOWN: i32 = 0i32;
pub const DWRITE_CONTAINER_TYPE_WOFF: i32 = 1i32;
pub const DWRITE_CONTAINER_TYPE_WOFF2: i32 = 2i32;
pub const DWRITE_ERR_BASE: u32 = 20480u32;
pub const DWRITE_E_DOWNLOADCANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283954i32 as _);
pub const DWRITE_E_DOWNLOADFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283953i32 as _);
pub const DWRITE_E_REMOTEFONT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283955i32 as _);
pub const DWRITE_E_TOOMANYDOWNLOADS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283952i32 as _);
pub const DWRITE_FACTORY_TYPE_SHARED: i32 = 0i32;
pub const DWRITE_FACTORY_TYPE_ISOLATED: i32 = 1i32;
#[repr(C)]
pub struct DWRITE_FILE_FRAGMENT {
    pub fileOffset: u64,
    pub fragmentSize: u64,
}
impl ::core::marker::Copy for DWRITE_FILE_FRAGMENT {}
impl ::core::clone::Clone for DWRITE_FILE_FRAGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM: i32 = 0i32;
pub const DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP: i32 = 1i32;
pub const DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT: i32 = 2i32;
pub const DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT: i32 = 3i32;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_NONE: u32 = 0u32;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE: u32 = 1u32;
pub const DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN: u32 = 2u32;
#[repr(C)]
pub struct DWRITE_FONT_AXIS_RANGE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub minValue: f32,
    pub maxValue: f32,
}
impl ::core::marker::Copy for DWRITE_FONT_AXIS_RANGE {}
impl ::core::clone::Clone for DWRITE_FONT_AXIS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_FONT_AXIS_TAG_WEIGHT: u32 = 1952999287u32;
pub const DWRITE_FONT_AXIS_TAG_WIDTH: u32 = 1752458359u32;
pub const DWRITE_FONT_AXIS_TAG_SLANT: u32 = 1953393779u32;
pub const DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE: u32 = 2054385775u32;
pub const DWRITE_FONT_AXIS_TAG_ITALIC: u32 = 1818326121u32;
#[repr(C)]
pub struct DWRITE_FONT_AXIS_VALUE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub value: f32,
}
impl ::core::marker::Copy for DWRITE_FONT_AXIS_VALUE {}
impl ::core::clone::Clone for DWRITE_FONT_AXIS_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_FONT_FACE_TYPE_CFF: i32 = 0i32;
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE: i32 = 1i32;
pub const DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION: i32 = 2i32;
pub const DWRITE_FONT_FACE_TYPE_TYPE1: i32 = 3i32;
pub const DWRITE_FONT_FACE_TYPE_VECTOR: i32 = 4i32;
pub const DWRITE_FONT_FACE_TYPE_BITMAP: i32 = 5i32;
pub const DWRITE_FONT_FACE_TYPE_UNKNOWN: i32 = 6i32;
pub const DWRITE_FONT_FACE_TYPE_RAW_CFF: i32 = 7i32;
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION: i32 = 2i32;
pub const DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC: i32 = 0i32;
pub const DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE: i32 = 1i32;
#[repr(C)]
pub struct DWRITE_FONT_FEATURE {
    pub nameTag: DWRITE_FONT_FEATURE_TAG,
    pub parameter: u32,
}
impl ::core::marker::Copy for DWRITE_FONT_FEATURE {}
impl ::core::clone::Clone for DWRITE_FONT_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS: u32 = 1668441697u32;
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS: u32 = 1668297315u32;
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS: u32 = 1668493923u32;
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES: u32 = 1953259875u32;
pub const DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS: u32 = 1702060387u32;
pub const DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION: u32 = 1886217059u32;
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES: u32 = 1734962275u32;
pub const DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING: u32 = 1886613603u32;
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH: u32 = 1752658787u32;
pub const DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING: u32 = 1936880995u32;
pub const DWRITE_FONT_FEATURE_TAG_DEFAULT: u32 = 1953261156u32;
pub const DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES: u32 = 1734962276u32;
pub const DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS: u32 = 1953527909u32;
pub const DWRITE_FONT_FEATURE_TAG_FRACTIONS: u32 = 1667330662u32;
pub const DWRITE_FONT_FEATURE_TAG_FULL_WIDTH: u32 = 1684633446u32;
pub const DWRITE_FONT_FEATURE_TAG_HALF_FORMS: u32 = 1718378856u32;
pub const DWRITE_FONT_FEATURE_TAG_HALANT_FORMS: u32 = 1852596584u32;
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH: u32 = 1953259880u32;
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS: u32 = 1953720680u32;
pub const DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES: u32 = 1634626408u32;
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES: u32 = 1734962280u32;
pub const DWRITE_FONT_FEATURE_TAG_HALF_WIDTH: u32 = 1684633448u32;
pub const DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS: u32 = 1869246312u32;
pub const DWRITE_FONT_FEATURE_TAG_JIS04_FORMS: u32 = 875589738u32;
pub const DWRITE_FONT_FEATURE_TAG_JIS78_FORMS: u32 = 943157354u32;
pub const DWRITE_FONT_FEATURE_TAG_JIS83_FORMS: u32 = 859336810u32;
pub const DWRITE_FONT_FEATURE_TAG_JIS90_FORMS: u32 = 809070698u32;
pub const DWRITE_FONT_FEATURE_TAG_KERNING: u32 = 1852990827u32;
pub const DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES: u32 = 1634167148u32;
pub const DWRITE_FONT_FEATURE_TAG_LINING_FIGURES: u32 = 1836412524u32;
pub const DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS: u32 = 1818455916u32;
pub const DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING: u32 = 1802658157u32;
pub const DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK: u32 = 1802659693u32;
pub const DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING: u32 = 1802333037u32;
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS: u32 = 1953259886u32;
pub const DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS: u32 = 1801677934u32;
pub const DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES: u32 = 1836412527u32;
pub const DWRITE_FONT_FEATURE_TAG_ORDINALS: u32 = 1852076655u32;
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH: u32 = 1953259888u32;
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS: u32 = 1885430640u32;
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES: u32 = 1836412528u32;
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS: u32 = 1684633456u32;
pub const DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS: u32 = 1684633457u32;
pub const DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES: u32 = 1734962290u32;
pub const DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS: u32 = 2036495730u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES: u32 = 1953259891u32;
pub const DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS: u32 = 1718511987u32;
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS: u32 = 1885564275u32;
pub const DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS: u32 = 1819307379u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1: u32 = 825258867u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2: u32 = 842036083u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3: u32 = 858813299u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4: u32 = 875590515u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5: u32 = 892367731u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6: u32 = 909144947u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7: u32 = 925922163u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8: u32 = 942699379u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9: u32 = 959476595u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10: u32 = 808547187u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11: u32 = 825324403u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12: u32 = 842101619u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13: u32 = 858878835u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14: u32 = 875656051u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15: u32 = 892433267u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16: u32 = 909210483u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17: u32 = 925987699u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18: u32 = 942764915u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19: u32 = 959542131u32;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20: u32 = 808612723u32;
pub const DWRITE_FONT_FEATURE_TAG_SUBSCRIPT: u32 = 1935832435u32;
pub const DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT: u32 = 1936749939u32;
pub const DWRITE_FONT_FEATURE_TAG_SWASH: u32 = 1752397683u32;
pub const DWRITE_FONT_FEATURE_TAG_TITLING: u32 = 1819568500u32;
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS: u32 = 1835101812u32;
pub const DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES: u32 = 1836412532u32;
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS: u32 = 1684107892u32;
pub const DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS: u32 = 1684633460u32;
pub const DWRITE_FONT_FEATURE_TAG_UNICASE: u32 = 1667853941u32;
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING: u32 = 1953654134u32;
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION: u32 = 846492278u32;
pub const DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO: u32 = 1869768058u32;
pub const DWRITE_FONT_FILE_TYPE_UNKNOWN: i32 = 0i32;
pub const DWRITE_FONT_FILE_TYPE_CFF: i32 = 1i32;
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE: i32 = 2i32;
pub const DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION: i32 = 3i32;
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFM: i32 = 4i32;
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFB: i32 = 5i32;
pub const DWRITE_FONT_FILE_TYPE_VECTOR: i32 = 6i32;
pub const DWRITE_FONT_FILE_TYPE_BITMAP: i32 = 7i32;
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION: i32 = 3i32;
pub const DWRITE_FONT_LINE_GAP_USAGE_DEFAULT: i32 = 0i32;
pub const DWRITE_FONT_LINE_GAP_USAGE_DISABLED: i32 = 1i32;
pub const DWRITE_FONT_LINE_GAP_USAGE_ENABLED: i32 = 2i32;
#[repr(C)]
pub struct DWRITE_FONT_METRICS {
    pub designUnitsPerEm: u16,
    pub ascent: u16,
    pub descent: u16,
    pub lineGap: i16,
    pub capHeight: u16,
    pub xHeight: u16,
    pub underlinePosition: i16,
    pub underlineThickness: u16,
    pub strikethroughPosition: i16,
    pub strikethroughThickness: u16,
}
impl ::core::marker::Copy for DWRITE_FONT_METRICS {}
impl ::core::clone::Clone for DWRITE_FONT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_FONT_METRICS1 {
    pub __AnonymousBase_DWrite_1_L627_C38: DWRITE_FONT_METRICS,
    pub glyphBoxLeft: i16,
    pub glyphBoxTop: i16,
    pub glyphBoxRight: i16,
    pub glyphBoxBottom: i16,
    pub subscriptPositionX: i16,
    pub subscriptPositionY: i16,
    pub subscriptSizeX: i16,
    pub subscriptSizeY: i16,
    pub superscriptPositionX: i16,
    pub superscriptPositionY: i16,
    pub superscriptSizeX: i16,
    pub superscriptSizeY: i16,
    pub hasTypographicMetrics: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_FONT_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_FONT_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_FONT_PROPERTY {
    pub propertyId: DWRITE_FONT_PROPERTY_ID,
    pub propertyValue: super::super::Foundation::PWSTR,
    pub localeName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_FONT_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_FONT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_FONT_PROPERTY_ID_NONE: i32 = 0i32;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FAMILY_NAME: i32 = 1i32;
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FAMILY_NAME: i32 = 2i32;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FACE_NAME: i32 = 3i32;
pub const DWRITE_FONT_PROPERTY_ID_FULL_NAME: i32 = 4i32;
pub const DWRITE_FONT_PROPERTY_ID_WIN32_FAMILY_NAME: i32 = 5i32;
pub const DWRITE_FONT_PROPERTY_ID_POSTSCRIPT_NAME: i32 = 6i32;
pub const DWRITE_FONT_PROPERTY_ID_DESIGN_SCRIPT_LANGUAGE_TAG: i32 = 7i32;
pub const DWRITE_FONT_PROPERTY_ID_SUPPORTED_SCRIPT_LANGUAGE_TAG: i32 = 8i32;
pub const DWRITE_FONT_PROPERTY_ID_SEMANTIC_TAG: i32 = 9i32;
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT: i32 = 10i32;
pub const DWRITE_FONT_PROPERTY_ID_STRETCH: i32 = 11i32;
pub const DWRITE_FONT_PROPERTY_ID_STYLE: i32 = 12i32;
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FACE_NAME: i32 = 13i32;
pub const DWRITE_FONT_PROPERTY_ID_TOTAL: i32 = 13i32;
pub const DWRITE_FONT_PROPERTY_ID_TOTAL_RS3: i32 = 14i32;
pub const DWRITE_FONT_PROPERTY_ID_PREFERRED_FAMILY_NAME: i32 = 2i32;
pub const DWRITE_FONT_PROPERTY_ID_FAMILY_NAME: i32 = 1i32;
pub const DWRITE_FONT_PROPERTY_ID_FACE_NAME: i32 = 3i32;
pub const DWRITE_FONT_SIMULATIONS_NONE: u32 = 0u32;
pub const DWRITE_FONT_SIMULATIONS_BOLD: u32 = 1u32;
pub const DWRITE_FONT_SIMULATIONS_OBLIQUE: u32 = 2u32;
pub const DWRITE_FONT_SOURCE_TYPE_UNKNOWN: i32 = 0i32;
pub const DWRITE_FONT_SOURCE_TYPE_PER_MACHINE: i32 = 1i32;
pub const DWRITE_FONT_SOURCE_TYPE_PER_USER: i32 = 2i32;
pub const DWRITE_FONT_SOURCE_TYPE_APPX_PACKAGE: i32 = 3i32;
pub const DWRITE_FONT_SOURCE_TYPE_REMOTE_FONT_PROVIDER: i32 = 4i32;
pub const DWRITE_FONT_STRETCH_UNDEFINED: i32 = 0i32;
pub const DWRITE_FONT_STRETCH_ULTRA_CONDENSED: i32 = 1i32;
pub const DWRITE_FONT_STRETCH_EXTRA_CONDENSED: i32 = 2i32;
pub const DWRITE_FONT_STRETCH_CONDENSED: i32 = 3i32;
pub const DWRITE_FONT_STRETCH_SEMI_CONDENSED: i32 = 4i32;
pub const DWRITE_FONT_STRETCH_NORMAL: i32 = 5i32;
pub const DWRITE_FONT_STRETCH_MEDIUM: i32 = 5i32;
pub const DWRITE_FONT_STRETCH_SEMI_EXPANDED: i32 = 6i32;
pub const DWRITE_FONT_STRETCH_EXPANDED: i32 = 7i32;
pub const DWRITE_FONT_STRETCH_EXTRA_EXPANDED: i32 = 8i32;
pub const DWRITE_FONT_STRETCH_ULTRA_EXPANDED: i32 = 9i32;
pub const DWRITE_FONT_STYLE_NORMAL: i32 = 0i32;
pub const DWRITE_FONT_STYLE_OBLIQUE: i32 = 1i32;
pub const DWRITE_FONT_STYLE_ITALIC: i32 = 2i32;
pub const DWRITE_FONT_WEIGHT_THIN: i32 = 100i32;
pub const DWRITE_FONT_WEIGHT_EXTRA_LIGHT: i32 = 200i32;
pub const DWRITE_FONT_WEIGHT_ULTRA_LIGHT: i32 = 200i32;
pub const DWRITE_FONT_WEIGHT_LIGHT: i32 = 300i32;
pub const DWRITE_FONT_WEIGHT_SEMI_LIGHT: i32 = 350i32;
pub const DWRITE_FONT_WEIGHT_NORMAL: i32 = 400i32;
pub const DWRITE_FONT_WEIGHT_REGULAR: i32 = 400i32;
pub const DWRITE_FONT_WEIGHT_MEDIUM: i32 = 500i32;
pub const DWRITE_FONT_WEIGHT_DEMI_BOLD: i32 = 600i32;
pub const DWRITE_FONT_WEIGHT_SEMI_BOLD: i32 = 600i32;
pub const DWRITE_FONT_WEIGHT_BOLD: i32 = 700i32;
pub const DWRITE_FONT_WEIGHT_EXTRA_BOLD: i32 = 800i32;
pub const DWRITE_FONT_WEIGHT_ULTRA_BOLD: i32 = 800i32;
pub const DWRITE_FONT_WEIGHT_BLACK: i32 = 900i32;
pub const DWRITE_FONT_WEIGHT_HEAVY: i32 = 900i32;
pub const DWRITE_FONT_WEIGHT_EXTRA_BLACK: i32 = 950i32;
pub const DWRITE_FONT_WEIGHT_ULTRA_BLACK: i32 = 950i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct DWRITE_GLYPH_IMAGE_DATA {
    pub imageData: *mut ::core::ffi::c_void,
    pub imageDataSize: u32,
    pub uniqueDataId: u32,
    pub pixelsPerEm: u32,
    pub pixelSize: super::Direct2D::Common::D2D_SIZE_U,
    pub horizontalLeftOrigin: super::super::Foundation::POINT,
    pub horizontalRightOrigin: super::super::Foundation::POINT,
    pub verticalTopOrigin: super::super::Foundation::POINT,
    pub verticalBottomOrigin: super::super::Foundation::POINT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for DWRITE_GLYPH_IMAGE_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for DWRITE_GLYPH_IMAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_GLYPH_IMAGE_FORMATS_NONE: u32 = 0u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE: u32 = 1u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_CFF: u32 = 2u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_COLR: u32 = 4u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_SVG: u32 = 8u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_PNG: u32 = 16u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_JPEG: u32 = 32u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_TIFF: u32 = 64u32;
pub const DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8: u32 = 128u32;
#[repr(C)]
pub struct DWRITE_GLYPH_METRICS {
    pub leftSideBearing: i32,
    pub advanceWidth: u32,
    pub rightSideBearing: i32,
    pub topSideBearing: i32,
    pub advanceHeight: u32,
    pub bottomSideBearing: i32,
    pub verticalOriginY: i32,
}
impl ::core::marker::Copy for DWRITE_GLYPH_METRICS {}
impl ::core::clone::Clone for DWRITE_GLYPH_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: f32,
    pub ascenderOffset: f32,
}
impl ::core::marker::Copy for DWRITE_GLYPH_OFFSET {}
impl ::core::clone::Clone for DWRITE_GLYPH_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES: i32 = 0i32;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES: i32 = 1i32;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES: i32 = 2i32;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: IDWriteFontFace,
    pub fontEmSize: f32,
    pub glyphCount: u32,
    pub glyphIndices: *mut u16,
    pub glyphAdvances: *mut f32,
    pub glyphOffsets: *mut DWRITE_GLYPH_OFFSET,
    pub isSideways: super::super::Foundation::BOOL,
    pub bidiLevel: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_GLYPH_RUN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_GLYPH_RUN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: super::super::Foundation::PWSTR,
    pub string: super::super::Foundation::PWSTR,
    pub stringLength: u32,
    pub clusterMap: *mut u16,
    pub textPosition: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_GLYPH_RUN_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_GRID_FIT_MODE_DEFAULT: i32 = 0i32;
pub const DWRITE_GRID_FIT_MODE_DISABLED: i32 = 1i32;
pub const DWRITE_GRID_FIT_MODE_ENABLED: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidiLevel: u32,
    pub isText: super::super::Foundation::BOOL,
    pub isTrimmed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_HIT_TEST_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_HIT_TEST_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_INFORMATIONAL_STRING_NONE: i32 = 0i32;
pub const DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE: i32 = 1i32;
pub const DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS: i32 = 2i32;
pub const DWRITE_INFORMATIONAL_STRING_TRADEMARK: i32 = 3i32;
pub const DWRITE_INFORMATIONAL_STRING_MANUFACTURER: i32 = 4i32;
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER: i32 = 5i32;
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER_URL: i32 = 6i32;
pub const DWRITE_INFORMATIONAL_STRING_DESCRIPTION: i32 = 7i32;
pub const DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL: i32 = 8i32;
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION: i32 = 9i32;
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL: i32 = 10i32;
pub const DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES: i32 = 11i32;
pub const DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES: i32 = 12i32;
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES: i32 = 13i32;
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES: i32 = 14i32;
pub const DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT: i32 = 15i32;
pub const DWRITE_INFORMATIONAL_STRING_FULL_NAME: i32 = 16i32;
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME: i32 = 17i32;
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME: i32 = 18i32;
pub const DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME: i32 = 19i32;
pub const DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG: i32 = 20i32;
pub const DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG: i32 = 21i32;
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES: i32 = 13i32;
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES: i32 = 14i32;
pub const DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME: i32 = 19i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_INLINE_OBJECT_METRICS {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supportsSideways: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_INLINE_OBJECT_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_INLINE_OBJECT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_JUSTIFICATION_OPPORTUNITY {
    pub expansionMinimum: f32,
    pub expansionMaximum: f32,
    pub compressionMaximum: f32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DWRITE_JUSTIFICATION_OPPORTUNITY {}
impl ::core::clone::Clone for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_LINE_BREAKPOINT {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DWRITE_LINE_BREAKPOINT {}
impl ::core::clone::Clone for DWRITE_LINE_BREAKPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_LINE_METRICS {
    pub length: u32,
    pub trailingWhitespaceLength: u32,
    pub newlineLength: u32,
    pub height: f32,
    pub baseline: f32,
    pub isTrimmed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_LINE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_LINE_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_LINE_METRICS1 {
    pub Base: DWRITE_LINE_METRICS,
    pub leadingBefore: f32,
    pub leadingAfter: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_LINE_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_LINE_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_LINE_SPACING {
    pub method: DWRITE_LINE_SPACING_METHOD,
    pub height: f32,
    pub baseline: f32,
    pub leadingBefore: f32,
    pub fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE,
}
impl ::core::marker::Copy for DWRITE_LINE_SPACING {}
impl ::core::clone::Clone for DWRITE_LINE_SPACING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_LINE_SPACING_METHOD_DEFAULT: i32 = 0i32;
pub const DWRITE_LINE_SPACING_METHOD_UNIFORM: i32 = 1i32;
pub const DWRITE_LINE_SPACING_METHOD_PROPORTIONAL: i32 = 2i32;
pub const DWRITE_LOCALITY_REMOTE: i32 = 0i32;
pub const DWRITE_LOCALITY_PARTIAL: i32 = 1i32;
pub const DWRITE_LOCALITY_LOCAL: i32 = 2i32;
#[repr(C)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
impl ::core::marker::Copy for DWRITE_MATRIX {}
impl ::core::clone::Clone for DWRITE_MATRIX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_MEASURING_MODE_NATURAL: i32 = 0i32;
pub const DWRITE_MEASURING_MODE_GDI_CLASSIC: i32 = 1i32;
pub const DWRITE_MEASURING_MODE_GDI_NATURAL: i32 = 2i32;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE: i32 = 0i32;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL: i32 = 1i32;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE: i32 = 2i32;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL: i32 = 3i32;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL: i32 = 4i32;
pub const DWRITE_OPTICAL_ALIGNMENT_NONE: i32 = 0i32;
pub const DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS: i32 = 1i32;
pub const DWRITE_OUTLINE_THRESHOLD_ANTIALIASED: i32 = 0i32;
pub const DWRITE_OUTLINE_THRESHOLD_ALIASED: i32 = 1i32;
#[repr(C)]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl ::core::marker::Copy for DWRITE_OVERHANG_METRICS {}
impl ::core::clone::Clone for DWRITE_OVERHANG_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union DWRITE_PANOSE {
    pub values: [u8; 10],
    pub familyKind: u8,
    pub text: DWRITE_PANOSE_3,
    pub script: DWRITE_PANOSE_1,
    pub decorative: DWRITE_PANOSE_0,
    pub symbol: DWRITE_PANOSE_2,
}
impl ::core::marker::Copy for DWRITE_PANOSE {}
impl ::core::clone::Clone for DWRITE_PANOSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_PANOSE_0 {
    pub familyKind: u8,
    pub decorativeClass: u8,
    pub weight: u8,
    pub aspect: u8,
    pub contrast: u8,
    pub serifVariant: u8,
    pub fill: u8,
    pub lining: u8,
    pub decorativeTopology: u8,
    pub characterRange: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_0 {}
impl ::core::clone::Clone for DWRITE_PANOSE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_PANOSE_1 {
    pub familyKind: u8,
    pub toolKind: u8,
    pub weight: u8,
    pub spacing: u8,
    pub aspectRatio: u8,
    pub contrast: u8,
    pub scriptTopology: u8,
    pub scriptForm: u8,
    pub finials: u8,
    pub xAscent: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_1 {}
impl ::core::clone::Clone for DWRITE_PANOSE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_PANOSE_2 {
    pub familyKind: u8,
    pub symbolKind: u8,
    pub weight: u8,
    pub spacing: u8,
    pub aspectRatioAndContrast: u8,
    pub aspectRatio94: u8,
    pub aspectRatio119: u8,
    pub aspectRatio157: u8,
    pub aspectRatio163: u8,
    pub aspectRatio211: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_2 {}
impl ::core::clone::Clone for DWRITE_PANOSE_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_PANOSE_3 {
    pub familyKind: u8,
    pub serifStyle: u8,
    pub weight: u8,
    pub proportion: u8,
    pub contrast: u8,
    pub strokeVariation: u8,
    pub armStyle: u8,
    pub letterform: u8,
    pub midline: u8,
    pub xHeight: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_3 {}
impl ::core::clone::Clone for DWRITE_PANOSE_3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_PANOSE_ARM_STYLE_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_ARM_STYLE_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORIZONTAL: i32 = 2i32;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_WEDGE: i32 = 3i32;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERTICAL: i32 = 4i32;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_SINGLE_SERIF: i32 = 5i32;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_DOUBLE_SERIF: i32 = 6i32;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_HORIZONTAL: i32 = 7i32;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_WEDGE: i32 = 8i32;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_VERTICAL: i32 = 9i32;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_SINGLE_SERIF: i32 = 10i32;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_DOUBLE_SERIF: i32 = 11i32;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORZ: i32 = 2i32;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERT: i32 = 4i32;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_HORZ: i32 = 7i32;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_WEDGE: i32 = 8i32;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_VERT: i32 = 9i32;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_SINGLE_SERIF: i32 = 10i32;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_DOUBLE_SERIF: i32 = 11i32;
pub const DWRITE_PANOSE_ASPECT_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_ASPECT_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_ASPECT_SUPER_CONDENSED: i32 = 2i32;
pub const DWRITE_PANOSE_ASPECT_VERY_CONDENSED: i32 = 3i32;
pub const DWRITE_PANOSE_ASPECT_CONDENSED: i32 = 4i32;
pub const DWRITE_PANOSE_ASPECT_NORMAL: i32 = 5i32;
pub const DWRITE_PANOSE_ASPECT_EXTENDED: i32 = 6i32;
pub const DWRITE_PANOSE_ASPECT_VERY_EXTENDED: i32 = 7i32;
pub const DWRITE_PANOSE_ASPECT_SUPER_EXTENDED: i32 = 8i32;
pub const DWRITE_PANOSE_ASPECT_MONOSPACED: i32 = 9i32;
pub const DWRITE_PANOSE_ASPECT_RATIO_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_ASPECT_RATIO_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_CONDENSED: i32 = 2i32;
pub const DWRITE_PANOSE_ASPECT_RATIO_CONDENSED: i32 = 3i32;
pub const DWRITE_PANOSE_ASPECT_RATIO_NORMAL: i32 = 4i32;
pub const DWRITE_PANOSE_ASPECT_RATIO_EXPANDED: i32 = 5i32;
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_EXPANDED: i32 = 6i32;
pub const DWRITE_PANOSE_CHARACTER_RANGES_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_CHARACTER_RANGES_EXTENDED_COLLECTION: i32 = 2i32;
pub const DWRITE_PANOSE_CHARACTER_RANGES_LITERALS: i32 = 3i32;
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_LOWER_CASE: i32 = 4i32;
pub const DWRITE_PANOSE_CHARACTER_RANGES_SMALL_CAPS: i32 = 5i32;
pub const DWRITE_PANOSE_CONTRAST_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_CONTRAST_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_CONTRAST_NONE: i32 = 2i32;
pub const DWRITE_PANOSE_CONTRAST_VERY_LOW: i32 = 3i32;
pub const DWRITE_PANOSE_CONTRAST_LOW: i32 = 4i32;
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_LOW: i32 = 5i32;
pub const DWRITE_PANOSE_CONTRAST_MEDIUM: i32 = 6i32;
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_HIGH: i32 = 7i32;
pub const DWRITE_PANOSE_CONTRAST_HIGH: i32 = 8i32;
pub const DWRITE_PANOSE_CONTRAST_VERY_HIGH: i32 = 9i32;
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_LOW: i32 = 10i32;
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_MEDIUM: i32 = 11i32;
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_HIGH: i32 = 12i32;
pub const DWRITE_PANOSE_CONTRAST_BROKEN: i32 = 13i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_DERIVATIVE: i32 = 2i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_TOPOLOGY: i32 = 3i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ELEMENTS: i32 = 4i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ASPECT: i32 = 5i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_INITIALS: i32 = 6i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_CARTOON: i32 = 7i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_PICTURE_STEMS: i32 = 8i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ORNAMENTED: i32 = 9i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_TEXT_AND_BACKGROUND: i32 = 10i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_COLLAGE: i32 = 11i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_MONTAGE: i32 = 12i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_STANDARD: i32 = 2i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SQUARE: i32 = 3i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_MULTIPLE_SEGMENT: i32 = 4i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ART_DECO: i32 = 5i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UNEVEN_WEIGHTING: i32 = 6i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_ARMS: i32 = 7i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_FORMS: i32 = 8i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_LOMBARDIC_FORMS: i32 = 9i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UPPER_CASE_IN_LOWER_CASE: i32 = 10i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_IMPLIED_TOPOLOGY: i32 = 11i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_HORSESHOE_E_AND_A: i32 = 12i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_CURSIVE: i32 = 13i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_BLACKLETTER: i32 = 14i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SWASH_VARIANCE: i32 = 15i32;
pub const DWRITE_PANOSE_FAMILY_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_FAMILY_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_FAMILY_TEXT_DISPLAY: i32 = 2i32;
pub const DWRITE_PANOSE_FAMILY_SCRIPT: i32 = 3i32;
pub const DWRITE_PANOSE_FAMILY_DECORATIVE: i32 = 4i32;
pub const DWRITE_PANOSE_FAMILY_SYMBOL: i32 = 5i32;
pub const DWRITE_PANOSE_FAMILY_PICTORIAL: i32 = 5i32;
pub const DWRITE_PANOSE_FILL_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_FILL_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_FILL_STANDARD_SOLID_FILL: i32 = 2i32;
pub const DWRITE_PANOSE_FILL_NO_FILL: i32 = 3i32;
pub const DWRITE_PANOSE_FILL_PATTERNED_FILL: i32 = 4i32;
pub const DWRITE_PANOSE_FILL_COMPLEX_FILL: i32 = 5i32;
pub const DWRITE_PANOSE_FILL_SHAPED_FILL: i32 = 6i32;
pub const DWRITE_PANOSE_FILL_DRAWN_DISTRESSED: i32 = 7i32;
pub const DWRITE_PANOSE_FINIALS_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_FINIALS_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_FINIALS_NONE_NO_LOOPS: i32 = 2i32;
pub const DWRITE_PANOSE_FINIALS_NONE_CLOSED_LOOPS: i32 = 3i32;
pub const DWRITE_PANOSE_FINIALS_NONE_OPEN_LOOPS: i32 = 4i32;
pub const DWRITE_PANOSE_FINIALS_SHARP_NO_LOOPS: i32 = 5i32;
pub const DWRITE_PANOSE_FINIALS_SHARP_CLOSED_LOOPS: i32 = 6i32;
pub const DWRITE_PANOSE_FINIALS_SHARP_OPEN_LOOPS: i32 = 7i32;
pub const DWRITE_PANOSE_FINIALS_TAPERED_NO_LOOPS: i32 = 8i32;
pub const DWRITE_PANOSE_FINIALS_TAPERED_CLOSED_LOOPS: i32 = 9i32;
pub const DWRITE_PANOSE_FINIALS_TAPERED_OPEN_LOOPS: i32 = 10i32;
pub const DWRITE_PANOSE_FINIALS_ROUND_NO_LOOPS: i32 = 11i32;
pub const DWRITE_PANOSE_FINIALS_ROUND_CLOSED_LOOPS: i32 = 12i32;
pub const DWRITE_PANOSE_FINIALS_ROUND_OPEN_LOOPS: i32 = 13i32;
pub const DWRITE_PANOSE_LETTERFORM_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_LETTERFORM_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_CONTACT: i32 = 2i32;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_WEIGHTED: i32 = 3i32;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_BOXED: i32 = 4i32;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_FLATTENED: i32 = 5i32;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_ROUNDED: i32 = 6i32;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_OFF_CENTER: i32 = 7i32;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_SQUARE: i32 = 8i32;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_CONTACT: i32 = 9i32;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_WEIGHTED: i32 = 10i32;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_BOXED: i32 = 11i32;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_FLATTENED: i32 = 12i32;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_ROUNDED: i32 = 13i32;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_OFF_CENTER: i32 = 14i32;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_SQUARE: i32 = 15i32;
pub const DWRITE_PANOSE_LINING_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_LINING_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_LINING_NONE: i32 = 2i32;
pub const DWRITE_PANOSE_LINING_INLINE: i32 = 3i32;
pub const DWRITE_PANOSE_LINING_OUTLINE: i32 = 4i32;
pub const DWRITE_PANOSE_LINING_ENGRAVED: i32 = 5i32;
pub const DWRITE_PANOSE_LINING_SHADOW: i32 = 6i32;
pub const DWRITE_PANOSE_LINING_RELIEF: i32 = 7i32;
pub const DWRITE_PANOSE_LINING_BACKDROP: i32 = 8i32;
pub const DWRITE_PANOSE_MIDLINE_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_MIDLINE_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_MIDLINE_STANDARD_TRIMMED: i32 = 2i32;
pub const DWRITE_PANOSE_MIDLINE_STANDARD_POINTED: i32 = 3i32;
pub const DWRITE_PANOSE_MIDLINE_STANDARD_SERIFED: i32 = 4i32;
pub const DWRITE_PANOSE_MIDLINE_HIGH_TRIMMED: i32 = 5i32;
pub const DWRITE_PANOSE_MIDLINE_HIGH_POINTED: i32 = 6i32;
pub const DWRITE_PANOSE_MIDLINE_HIGH_SERIFED: i32 = 7i32;
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_TRIMMED: i32 = 8i32;
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_POINTED: i32 = 9i32;
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_SERIFED: i32 = 10i32;
pub const DWRITE_PANOSE_MIDLINE_LOW_TRIMMED: i32 = 11i32;
pub const DWRITE_PANOSE_MIDLINE_LOW_POINTED: i32 = 12i32;
pub const DWRITE_PANOSE_MIDLINE_LOW_SERIFED: i32 = 13i32;
pub const DWRITE_PANOSE_PROPORTION_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_PROPORTION_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_PROPORTION_OLD_STYLE: i32 = 2i32;
pub const DWRITE_PANOSE_PROPORTION_MODERN: i32 = 3i32;
pub const DWRITE_PANOSE_PROPORTION_EVEN_WIDTH: i32 = 4i32;
pub const DWRITE_PANOSE_PROPORTION_EXPANDED: i32 = 5i32;
pub const DWRITE_PANOSE_PROPORTION_CONDENSED: i32 = 6i32;
pub const DWRITE_PANOSE_PROPORTION_VERY_EXPANDED: i32 = 7i32;
pub const DWRITE_PANOSE_PROPORTION_VERY_CONDENSED: i32 = 8i32;
pub const DWRITE_PANOSE_PROPORTION_MONOSPACED: i32 = 9i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_NO_WRAPPING: i32 = 2i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_SOME_WRAPPING: i32 = 3i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_MORE_WRAPPING: i32 = 4i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_EXTREME_WRAPPING: i32 = 5i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_NO_WRAPPING: i32 = 6i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_SOME_WRAPPING: i32 = 7i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_MORE_WRAPPING: i32 = 8i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_EXTREME_WRAPPING: i32 = 9i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_NO_WRAPPING: i32 = 10i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_SOME_WRAPPING: i32 = 11i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_MORE_WRAPPING: i32 = 12i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_EXTREME_WRAPPING: i32 = 13i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_DISCONNECTED: i32 = 2i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_TRAILING: i32 = 3i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_CONNECTED: i32 = 4i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_DISCONNECTED: i32 = 5i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_TRAILING: i32 = 6i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_CONNECTED: i32 = 7i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_DISCONNECTED: i32 = 8i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_TRAILING: i32 = 9i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_CONNECTED: i32 = 10i32;
pub const DWRITE_PANOSE_SERIF_STYLE_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_SERIF_STYLE_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_SERIF_STYLE_COVE: i32 = 2i32;
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_COVE: i32 = 3i32;
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE_COVE: i32 = 4i32;
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SQUARE_COVE: i32 = 5i32;
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE: i32 = 6i32;
pub const DWRITE_PANOSE_SERIF_STYLE_THIN: i32 = 7i32;
pub const DWRITE_PANOSE_SERIF_STYLE_OVAL: i32 = 8i32;
pub const DWRITE_PANOSE_SERIF_STYLE_EXAGGERATED: i32 = 9i32;
pub const DWRITE_PANOSE_SERIF_STYLE_TRIANGLE: i32 = 10i32;
pub const DWRITE_PANOSE_SERIF_STYLE_NORMAL_SANS: i32 = 11i32;
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SANS: i32 = 12i32;
pub const DWRITE_PANOSE_SERIF_STYLE_PERPENDICULAR_SANS: i32 = 13i32;
pub const DWRITE_PANOSE_SERIF_STYLE_FLARED: i32 = 14i32;
pub const DWRITE_PANOSE_SERIF_STYLE_ROUNDED: i32 = 15i32;
pub const DWRITE_PANOSE_SERIF_STYLE_SCRIPT: i32 = 16i32;
pub const DWRITE_PANOSE_SERIF_STYLE_PERP_SANS: i32 = 13i32;
pub const DWRITE_PANOSE_SERIF_STYLE_BONE: i32 = 8i32;
pub const DWRITE_PANOSE_SPACING_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_SPACING_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_SPACING_PROPORTIONAL_SPACED: i32 = 2i32;
pub const DWRITE_PANOSE_SPACING_MONOSPACED: i32 = 3i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_VARIATION: i32 = 2i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_DIAGONAL: i32 = 3i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_TRANSITIONAL: i32 = 4i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_VERTICAL: i32 = 5i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_HORIZONTAL: i32 = 6i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_VERTICAL: i32 = 7i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_HORIZONTAL: i32 = 8i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_VERTICAL: i32 = 9i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_HORIZONTAL: i32 = 10i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_WIDTH: i32 = 2i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_EXCEPTIONALLY_WIDE: i32 = 3i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_SUPER_WIDE: i32 = 4i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_WIDE: i32 = 5i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_WIDE: i32 = 6i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NORMAL: i32 = 7i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NARROW: i32 = 8i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_NARROW: i32 = 9i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_MONTAGES: i32 = 2i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_PICTURES: i32 = 3i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_SHAPES: i32 = 4i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_SCIENTIFIC: i32 = 5i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_MUSIC: i32 = 6i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_EXPERT: i32 = 7i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_PATTERNS: i32 = 8i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_BOARDERS: i32 = 9i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_ICONS: i32 = 10i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_LOGOS: i32 = 11i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_INDUSTRY_SPECIFIC: i32 = 12i32;
pub const DWRITE_PANOSE_TOOL_KIND_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_TOOL_KIND_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_TOOL_KIND_FLAT_NIB: i32 = 2i32;
pub const DWRITE_PANOSE_TOOL_KIND_PRESSURE_POINT: i32 = 3i32;
pub const DWRITE_PANOSE_TOOL_KIND_ENGRAVED: i32 = 4i32;
pub const DWRITE_PANOSE_TOOL_KIND_BALL: i32 = 5i32;
pub const DWRITE_PANOSE_TOOL_KIND_BRUSH: i32 = 6i32;
pub const DWRITE_PANOSE_TOOL_KIND_ROUGH: i32 = 7i32;
pub const DWRITE_PANOSE_TOOL_KIND_FELT_PEN_BRUSH_TIP: i32 = 8i32;
pub const DWRITE_PANOSE_TOOL_KIND_WILD_BRUSH: i32 = 9i32;
pub const DWRITE_PANOSE_WEIGHT_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_WEIGHT_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_WEIGHT_VERY_LIGHT: i32 = 2i32;
pub const DWRITE_PANOSE_WEIGHT_LIGHT: i32 = 3i32;
pub const DWRITE_PANOSE_WEIGHT_THIN: i32 = 4i32;
pub const DWRITE_PANOSE_WEIGHT_BOOK: i32 = 5i32;
pub const DWRITE_PANOSE_WEIGHT_MEDIUM: i32 = 6i32;
pub const DWRITE_PANOSE_WEIGHT_DEMI: i32 = 7i32;
pub const DWRITE_PANOSE_WEIGHT_BOLD: i32 = 8i32;
pub const DWRITE_PANOSE_WEIGHT_HEAVY: i32 = 9i32;
pub const DWRITE_PANOSE_WEIGHT_BLACK: i32 = 10i32;
pub const DWRITE_PANOSE_WEIGHT_EXTRA_BLACK: i32 = 11i32;
pub const DWRITE_PANOSE_WEIGHT_NORD: i32 = 11i32;
pub const DWRITE_PANOSE_XASCENT_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_XASCENT_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_XASCENT_VERY_LOW: i32 = 2i32;
pub const DWRITE_PANOSE_XASCENT_LOW: i32 = 3i32;
pub const DWRITE_PANOSE_XASCENT_MEDIUM: i32 = 4i32;
pub const DWRITE_PANOSE_XASCENT_HIGH: i32 = 5i32;
pub const DWRITE_PANOSE_XASCENT_VERY_HIGH: i32 = 6i32;
pub const DWRITE_PANOSE_XHEIGHT_ANY: i32 = 0i32;
pub const DWRITE_PANOSE_XHEIGHT_NO_FIT: i32 = 1i32;
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_SMALL: i32 = 2i32;
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STANDARD: i32 = 3i32;
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_LARGE: i32 = 4i32;
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_SMALL: i32 = 5i32;
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STANDARD: i32 = 6i32;
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_LARGE: i32 = 7i32;
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STD: i32 = 3i32;
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STD: i32 = 6i32;
pub const DWRITE_PARAGRAPH_ALIGNMENT_NEAR: i32 = 0i32;
pub const DWRITE_PARAGRAPH_ALIGNMENT_FAR: i32 = 1i32;
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: i32 = 2i32;
pub const DWRITE_PIXEL_GEOMETRY_FLAT: i32 = 0i32;
pub const DWRITE_PIXEL_GEOMETRY_RGB: i32 = 1i32;
pub const DWRITE_PIXEL_GEOMETRY_BGR: i32 = 2i32;
pub const DWRITE_READING_DIRECTION_LEFT_TO_RIGHT: i32 = 0i32;
pub const DWRITE_READING_DIRECTION_RIGHT_TO_LEFT: i32 = 1i32;
pub const DWRITE_READING_DIRECTION_TOP_TO_BOTTOM: i32 = 2i32;
pub const DWRITE_READING_DIRECTION_BOTTOM_TO_TOP: i32 = 3i32;
pub const DWRITE_RENDERING_MODE_DEFAULT: i32 = 0i32;
pub const DWRITE_RENDERING_MODE_ALIASED: i32 = 1i32;
pub const DWRITE_RENDERING_MODE_GDI_CLASSIC: i32 = 2i32;
pub const DWRITE_RENDERING_MODE_GDI_NATURAL: i32 = 3i32;
pub const DWRITE_RENDERING_MODE_NATURAL: i32 = 4i32;
pub const DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC: i32 = 5i32;
pub const DWRITE_RENDERING_MODE_OUTLINE: i32 = 6i32;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC: i32 = 2i32;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL: i32 = 3i32;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL: i32 = 4i32;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC: i32 = 5i32;
pub const DWRITE_RENDERING_MODE1_DEFAULT: i32 = 0i32;
pub const DWRITE_RENDERING_MODE1_ALIASED: i32 = 1i32;
pub const DWRITE_RENDERING_MODE1_GDI_CLASSIC: i32 = 2i32;
pub const DWRITE_RENDERING_MODE1_GDI_NATURAL: i32 = 3i32;
pub const DWRITE_RENDERING_MODE1_NATURAL: i32 = 4i32;
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC: i32 = 5i32;
pub const DWRITE_RENDERING_MODE1_OUTLINE: i32 = 6i32;
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC_DOWNSAMPLED: i32 = 7i32;
#[repr(C)]
pub struct DWRITE_SCRIPT_ANALYSIS {
    pub script: u16,
    pub shapes: DWRITE_SCRIPT_SHAPES,
}
impl ::core::marker::Copy for DWRITE_SCRIPT_ANALYSIS {}
impl ::core::clone::Clone for DWRITE_SCRIPT_ANALYSIS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_SCRIPT_PROPERTIES {
    pub isoScriptCode: u32,
    pub isoScriptNumber: u32,
    pub clusterLookahead: u32,
    pub justificationCharacter: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DWRITE_SCRIPT_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SCRIPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_SCRIPT_SHAPES_DEFAULT: u32 = 0u32;
pub const DWRITE_SCRIPT_SHAPES_NO_VISUAL: u32 = 1u32;
#[repr(C)]
pub struct DWRITE_SHAPING_GLYPH_PROPERTIES {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_SHAPING_GLYPH_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_SHAPING_TEXT_PROPERTIES {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_SHAPING_TEXT_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_STRIKETHROUGH {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: super::super::Foundation::PWSTR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_STRIKETHROUGH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_STRIKETHROUGH {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_TEXTURE_ALIASED_1x1: i32 = 0i32;
pub const DWRITE_TEXTURE_CLEARTYPE_3x1: i32 = 1i32;
pub const DWRITE_TEXT_ALIGNMENT_LEADING: i32 = 0i32;
pub const DWRITE_TEXT_ALIGNMENT_TRAILING: i32 = 1i32;
pub const DWRITE_TEXT_ALIGNMENT_CENTER: i32 = 2i32;
pub const DWRITE_TEXT_ALIGNMENT_JUSTIFIED: i32 = 3i32;
pub const DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE: i32 = 0i32;
pub const DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE: i32 = 1i32;
#[repr(C)]
pub struct DWRITE_TEXT_METRICS {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub widthIncludingTrailingWhitespace: f32,
    pub height: f32,
    pub layoutWidth: f32,
    pub layoutHeight: f32,
    pub maxBidiReorderingDepth: u32,
    pub lineCount: u32,
}
impl ::core::marker::Copy for DWRITE_TEXT_METRICS {}
impl ::core::clone::Clone for DWRITE_TEXT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_TEXT_METRICS1 {
    pub Base: DWRITE_TEXT_METRICS,
    pub heightIncludingTrailingWhitespace: f32,
}
impl ::core::marker::Copy for DWRITE_TEXT_METRICS1 {}
impl ::core::clone::Clone for DWRITE_TEXT_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: u32,
    pub length: u32,
}
impl ::core::marker::Copy for DWRITE_TEXT_RANGE {}
impl ::core::clone::Clone for DWRITE_TEXT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_TRIMMING {
    pub granularity: DWRITE_TRIMMING_GRANULARITY,
    pub delimiter: u32,
    pub delimiterCount: u32,
}
impl ::core::marker::Copy for DWRITE_TRIMMING {}
impl ::core::clone::Clone for DWRITE_TRIMMING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_TRIMMING_GRANULARITY_NONE: i32 = 0i32;
pub const DWRITE_TRIMMING_GRANULARITY_CHARACTER: i32 = 1i32;
pub const DWRITE_TRIMMING_GRANULARITY_WORD: i32 = 2i32;
#[repr(C)]
pub struct DWRITE_TYPOGRAPHIC_FEATURES {
    pub features: *mut DWRITE_FONT_FEATURE,
    pub featureCount: u32,
}
impl ::core::marker::Copy for DWRITE_TYPOGRAPHIC_FEATURES {}
impl ::core::clone::Clone for DWRITE_TYPOGRAPHIC_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_UNDERLINE {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub runHeight: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: super::super::Foundation::PWSTR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_UNDERLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_UNDERLINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DWRITE_UNICODE_RANGE {
    pub first: u32,
    pub last: u32,
}
impl ::core::marker::Copy for DWRITE_UNICODE_RANGE {}
impl ::core::clone::Clone for DWRITE_UNICODE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT: i32 = 0i32;
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED: i32 = 1i32;
pub const DWRITE_WORD_WRAPPING_WRAP: i32 = 0i32;
pub const DWRITE_WORD_WRAPPING_NO_WRAP: i32 = 1i32;
pub const DWRITE_WORD_WRAPPING_EMERGENCY_BREAK: i32 = 2i32;
pub const DWRITE_WORD_WRAPPING_WHOLE_WORD: i32 = 3i32;
pub const DWRITE_WORD_WRAPPING_CHARACTER: i32 = 4i32;
pub const FACILITY_DWRITE: u32 = 2200u32;
#[repr(transparent)]
pub struct IDWriteAsyncResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteAsyncResult {}
impl ::core::clone::Clone for IDWriteAsyncResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteBitmapRenderTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteBitmapRenderTarget {}
impl ::core::clone::Clone for IDWriteBitmapRenderTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteBitmapRenderTarget1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteBitmapRenderTarget1 {}
impl ::core::clone::Clone for IDWriteBitmapRenderTarget1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteColorGlyphRunEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteColorGlyphRunEnumerator {}
impl ::core::clone::Clone for IDWriteColorGlyphRunEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteColorGlyphRunEnumerator1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteColorGlyphRunEnumerator1 {}
impl ::core::clone::Clone for IDWriteColorGlyphRunEnumerator1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFactory {}
impl ::core::clone::Clone for IDWriteFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFactory1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFactory1 {}
impl ::core::clone::Clone for IDWriteFactory1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFactory2 {}
impl ::core::clone::Clone for IDWriteFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFactory3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFactory3 {}
impl ::core::clone::Clone for IDWriteFactory3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFactory4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFactory4 {}
impl ::core::clone::Clone for IDWriteFactory4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFactory5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFactory5 {}
impl ::core::clone::Clone for IDWriteFactory5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFactory6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFactory6 {}
impl ::core::clone::Clone for IDWriteFactory6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFactory7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFactory7 {}
impl ::core::clone::Clone for IDWriteFactory7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFont(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFont {}
impl ::core::clone::Clone for IDWriteFont {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFont1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFont1 {}
impl ::core::clone::Clone for IDWriteFont1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFont2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFont2 {}
impl ::core::clone::Clone for IDWriteFont2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFont3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFont3 {}
impl ::core::clone::Clone for IDWriteFont3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontCollection {}
impl ::core::clone::Clone for IDWriteFontCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontCollection1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontCollection1 {}
impl ::core::clone::Clone for IDWriteFontCollection1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontCollection2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontCollection2 {}
impl ::core::clone::Clone for IDWriteFontCollection2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontCollection3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontCollection3 {}
impl ::core::clone::Clone for IDWriteFontCollection3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontCollectionLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontCollectionLoader {}
impl ::core::clone::Clone for IDWriteFontCollectionLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontDownloadListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontDownloadListener {}
impl ::core::clone::Clone for IDWriteFontDownloadListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontDownloadQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontDownloadQueue {}
impl ::core::clone::Clone for IDWriteFontDownloadQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFace {}
impl ::core::clone::Clone for IDWriteFontFace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFace1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFace1 {}
impl ::core::clone::Clone for IDWriteFontFace1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFace2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFace2 {}
impl ::core::clone::Clone for IDWriteFontFace2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFace3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFace3 {}
impl ::core::clone::Clone for IDWriteFontFace3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFace4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFace4 {}
impl ::core::clone::Clone for IDWriteFontFace4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFace5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFace5 {}
impl ::core::clone::Clone for IDWriteFontFace5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFace6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFace6 {}
impl ::core::clone::Clone for IDWriteFontFace6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFaceReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFaceReference {}
impl ::core::clone::Clone for IDWriteFontFaceReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFaceReference1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFaceReference1 {}
impl ::core::clone::Clone for IDWriteFontFaceReference1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFallback {}
impl ::core::clone::Clone for IDWriteFontFallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFallback1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFallback1 {}
impl ::core::clone::Clone for IDWriteFontFallback1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFallbackBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFallbackBuilder {}
impl ::core::clone::Clone for IDWriteFontFallbackBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFamily(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFamily {}
impl ::core::clone::Clone for IDWriteFontFamily {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFamily1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFamily1 {}
impl ::core::clone::Clone for IDWriteFontFamily1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFamily2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFamily2 {}
impl ::core::clone::Clone for IDWriteFontFamily2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFile {}
impl ::core::clone::Clone for IDWriteFontFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFileEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFileEnumerator {}
impl ::core::clone::Clone for IDWriteFontFileEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFileLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFileLoader {}
impl ::core::clone::Clone for IDWriteFontFileLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontFileStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontFileStream {}
impl ::core::clone::Clone for IDWriteFontFileStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontList {}
impl ::core::clone::Clone for IDWriteFontList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontList1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontList1 {}
impl ::core::clone::Clone for IDWriteFontList1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontList2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontList2 {}
impl ::core::clone::Clone for IDWriteFontList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontResource {}
impl ::core::clone::Clone for IDWriteFontResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontSet {}
impl ::core::clone::Clone for IDWriteFontSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontSet1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontSet1 {}
impl ::core::clone::Clone for IDWriteFontSet1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontSet2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontSet2 {}
impl ::core::clone::Clone for IDWriteFontSet2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontSet3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontSet3 {}
impl ::core::clone::Clone for IDWriteFontSet3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontSetBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontSetBuilder {}
impl ::core::clone::Clone for IDWriteFontSetBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontSetBuilder1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontSetBuilder1 {}
impl ::core::clone::Clone for IDWriteFontSetBuilder1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteFontSetBuilder2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteFontSetBuilder2 {}
impl ::core::clone::Clone for IDWriteFontSetBuilder2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteGdiInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteGdiInterop {}
impl ::core::clone::Clone for IDWriteGdiInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteGdiInterop1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteGdiInterop1 {}
impl ::core::clone::Clone for IDWriteGdiInterop1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteGlyphRunAnalysis(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteGlyphRunAnalysis {}
impl ::core::clone::Clone for IDWriteGlyphRunAnalysis {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteInMemoryFontFileLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteInMemoryFontFileLoader {}
impl ::core::clone::Clone for IDWriteInMemoryFontFileLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteInlineObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteInlineObject {}
impl ::core::clone::Clone for IDWriteInlineObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteLocalFontFileLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteLocalFontFileLoader {}
impl ::core::clone::Clone for IDWriteLocalFontFileLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteLocalizedStrings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteLocalizedStrings {}
impl ::core::clone::Clone for IDWriteLocalizedStrings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteNumberSubstitution(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteNumberSubstitution {}
impl ::core::clone::Clone for IDWriteNumberSubstitution {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWritePixelSnapping(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWritePixelSnapping {}
impl ::core::clone::Clone for IDWritePixelSnapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteRemoteFontFileLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteRemoteFontFileLoader {}
impl ::core::clone::Clone for IDWriteRemoteFontFileLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteRemoteFontFileStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteRemoteFontFileStream {}
impl ::core::clone::Clone for IDWriteRemoteFontFileStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteRenderingParams(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteRenderingParams {}
impl ::core::clone::Clone for IDWriteRenderingParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteRenderingParams1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteRenderingParams1 {}
impl ::core::clone::Clone for IDWriteRenderingParams1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteRenderingParams2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteRenderingParams2 {}
impl ::core::clone::Clone for IDWriteRenderingParams2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteRenderingParams3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteRenderingParams3 {}
impl ::core::clone::Clone for IDWriteRenderingParams3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteStringList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteStringList {}
impl ::core::clone::Clone for IDWriteStringList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextAnalysisSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextAnalysisSink {}
impl ::core::clone::Clone for IDWriteTextAnalysisSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextAnalysisSink1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextAnalysisSink1 {}
impl ::core::clone::Clone for IDWriteTextAnalysisSink1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextAnalysisSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextAnalysisSource {}
impl ::core::clone::Clone for IDWriteTextAnalysisSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextAnalysisSource1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextAnalysisSource1 {}
impl ::core::clone::Clone for IDWriteTextAnalysisSource1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextAnalyzer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextAnalyzer {}
impl ::core::clone::Clone for IDWriteTextAnalyzer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextAnalyzer1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextAnalyzer1 {}
impl ::core::clone::Clone for IDWriteTextAnalyzer1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextAnalyzer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextAnalyzer2 {}
impl ::core::clone::Clone for IDWriteTextAnalyzer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextFormat {}
impl ::core::clone::Clone for IDWriteTextFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextFormat1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextFormat1 {}
impl ::core::clone::Clone for IDWriteTextFormat1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextFormat2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextFormat2 {}
impl ::core::clone::Clone for IDWriteTextFormat2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextFormat3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextFormat3 {}
impl ::core::clone::Clone for IDWriteTextFormat3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextLayout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextLayout {}
impl ::core::clone::Clone for IDWriteTextLayout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextLayout1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextLayout1 {}
impl ::core::clone::Clone for IDWriteTextLayout1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextLayout2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextLayout2 {}
impl ::core::clone::Clone for IDWriteTextLayout2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextLayout3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextLayout3 {}
impl ::core::clone::Clone for IDWriteTextLayout3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextLayout4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextLayout4 {}
impl ::core::clone::Clone for IDWriteTextLayout4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextRenderer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextRenderer {}
impl ::core::clone::Clone for IDWriteTextRenderer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTextRenderer1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTextRenderer1 {}
impl ::core::clone::Clone for IDWriteTextRenderer1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDWriteTypography(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDWriteTypography {}
impl ::core::clone::Clone for IDWriteTypography {
    fn clone(&self) -> Self {
        *self
    }
}
