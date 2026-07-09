#[inline]
pub unsafe fn DWriteCreateFactory(factorytype: DWRITE_FACTORY_TYPE, iid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
    windows_core::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DWriteCreateFactory(factorytype, iid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub const DWRITE_ALPHA_MAX: u32 = 255;
pub type DWRITE_BREAK_CONDITION = i32;
pub const DWRITE_BREAK_CONDITION_CAN_BREAK: DWRITE_BREAK_CONDITION = 1;
pub const DWRITE_BREAK_CONDITION_MAY_NOT_BREAK: DWRITE_BREAK_CONDITION = 2;
pub const DWRITE_BREAK_CONDITION_MUST_BREAK: DWRITE_BREAK_CONDITION = 3;
pub const DWRITE_BREAK_CONDITION_NEUTRAL: DWRITE_BREAK_CONDITION = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: f32,
    pub length: u16,
    pub _bitfield: u16,
}
pub const DWRITE_ERR_BASE: u32 = 20480;
pub type DWRITE_FACTORY_TYPE = i32;
pub const DWRITE_FACTORY_TYPE_ISOLATED: DWRITE_FACTORY_TYPE = 1;
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = 0;
pub type DWRITE_FLOW_DIRECTION = i32;
pub const DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP: DWRITE_FLOW_DIRECTION = 1;
pub const DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT: DWRITE_FLOW_DIRECTION = 2;
pub const DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT: DWRITE_FLOW_DIRECTION = 3;
pub const DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM: DWRITE_FLOW_DIRECTION = 0;
pub type DWRITE_FONT_FACE_TYPE = i32;
pub const DWRITE_FONT_FACE_TYPE_BITMAP: DWRITE_FONT_FACE_TYPE = 5;
pub const DWRITE_FONT_FACE_TYPE_CFF: DWRITE_FONT_FACE_TYPE = 0;
pub const DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION: DWRITE_FONT_FACE_TYPE = 2;
pub const DWRITE_FONT_FACE_TYPE_RAW_CFF: DWRITE_FONT_FACE_TYPE = 7;
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE: DWRITE_FONT_FACE_TYPE = 1;
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION: DWRITE_FONT_FACE_TYPE = 2;
pub const DWRITE_FONT_FACE_TYPE_TYPE1: DWRITE_FONT_FACE_TYPE = 3;
pub const DWRITE_FONT_FACE_TYPE_UNKNOWN: DWRITE_FONT_FACE_TYPE = 6;
pub const DWRITE_FONT_FACE_TYPE_VECTOR: DWRITE_FONT_FACE_TYPE = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_FONT_FEATURE {
    pub nameTag: DWRITE_FONT_FEATURE_TAG,
    pub parameter: u32,
}
pub type DWRITE_FONT_FEATURE_TAG = i32;
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS: DWRITE_FONT_FEATURE_TAG = 1953259886;
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH: DWRITE_FONT_FEATURE_TAG = 1953259880;
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS: DWRITE_FONT_FEATURE_TAG = 1668441697;
pub const DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING: DWRITE_FONT_FEATURE_TAG = 1886613603;
pub const DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS: DWRITE_FONT_FEATURE_TAG = 1702060387;
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES: DWRITE_FONT_FEATURE_TAG = 1953259875;
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1734962275;
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH: DWRITE_FONT_FEATURE_TAG = 1752658787;
pub const DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING: DWRITE_FONT_FEATURE_TAG = 1936880995;
pub const DWRITE_FONT_FEATURE_TAG_DEFAULT: DWRITE_FONT_FEATURE_TAG = 1953261156;
pub const DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1734962276;
pub const DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS: DWRITE_FONT_FEATURE_TAG = 1953527909;
pub const DWRITE_FONT_FEATURE_TAG_FRACTIONS: DWRITE_FONT_FEATURE_TAG = 1667330662;
pub const DWRITE_FONT_FEATURE_TAG_FULL_WIDTH: DWRITE_FONT_FEATURE_TAG = 1684633446;
pub const DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION: DWRITE_FONT_FEATURE_TAG = 1886217059;
pub const DWRITE_FONT_FEATURE_TAG_HALANT_FORMS: DWRITE_FONT_FEATURE_TAG = 1852596584;
pub const DWRITE_FONT_FEATURE_TAG_HALF_FORMS: DWRITE_FONT_FEATURE_TAG = 1718378856;
pub const DWRITE_FONT_FEATURE_TAG_HALF_WIDTH: DWRITE_FONT_FEATURE_TAG = 1684633448;
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS: DWRITE_FONT_FEATURE_TAG = 1953720680;
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1734962280;
pub const DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS: DWRITE_FONT_FEATURE_TAG = 1869246312;
pub const DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES: DWRITE_FONT_FEATURE_TAG = 1634626408;
pub const DWRITE_FONT_FEATURE_TAG_JIS04_FORMS: DWRITE_FONT_FEATURE_TAG = 875589738;
pub const DWRITE_FONT_FEATURE_TAG_JIS78_FORMS: DWRITE_FONT_FEATURE_TAG = 943157354;
pub const DWRITE_FONT_FEATURE_TAG_JIS83_FORMS: DWRITE_FONT_FEATURE_TAG = 859336810;
pub const DWRITE_FONT_FEATURE_TAG_JIS90_FORMS: DWRITE_FONT_FEATURE_TAG = 809070698;
pub const DWRITE_FONT_FEATURE_TAG_KERNING: DWRITE_FONT_FEATURE_TAG = 1852990827;
pub const DWRITE_FONT_FEATURE_TAG_LINING_FIGURES: DWRITE_FONT_FEATURE_TAG = 1836412524;
pub const DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS: DWRITE_FONT_FEATURE_TAG = 1818455916;
pub const DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING: DWRITE_FONT_FEATURE_TAG = 1802658157;
pub const DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING: DWRITE_FONT_FEATURE_TAG = 1802333037;
pub const DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK: DWRITE_FONT_FEATURE_TAG = 1802659693;
pub const DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS: DWRITE_FONT_FEATURE_TAG = 1801677934;
pub const DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES: DWRITE_FONT_FEATURE_TAG = 1836412527;
pub const DWRITE_FONT_FEATURE_TAG_ORDINALS: DWRITE_FONT_FEATURE_TAG = 1852076655;
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS: DWRITE_FONT_FEATURE_TAG = 1885430640;
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS: DWRITE_FONT_FEATURE_TAG = 1668297315;
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH: DWRITE_FONT_FEATURE_TAG = 1953259888;
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES: DWRITE_FONT_FEATURE_TAG = 1836412528;
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS: DWRITE_FONT_FEATURE_TAG = 1684633456;
pub const DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS: DWRITE_FONT_FEATURE_TAG = 1684633457;
pub const DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1734962290;
pub const DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS: DWRITE_FONT_FEATURE_TAG = 2036495730;
pub const DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS: DWRITE_FONT_FEATURE_TAG = 1718511987;
pub const DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS: DWRITE_FONT_FEATURE_TAG = 1819307379;
pub const DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO: DWRITE_FONT_FEATURE_TAG = 1869768058;
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS: DWRITE_FONT_FEATURE_TAG = 1885564275;
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS: DWRITE_FONT_FEATURE_TAG = 1668493923;
pub const DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES: DWRITE_FONT_FEATURE_TAG = 1634167148;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES: DWRITE_FONT_FEATURE_TAG = 1953259891;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1: DWRITE_FONT_FEATURE_TAG = 825258867;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10: DWRITE_FONT_FEATURE_TAG = 808547187;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11: DWRITE_FONT_FEATURE_TAG = 825324403;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12: DWRITE_FONT_FEATURE_TAG = 842101619;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13: DWRITE_FONT_FEATURE_TAG = 858878835;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14: DWRITE_FONT_FEATURE_TAG = 875656051;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15: DWRITE_FONT_FEATURE_TAG = 892433267;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16: DWRITE_FONT_FEATURE_TAG = 909210483;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17: DWRITE_FONT_FEATURE_TAG = 925987699;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18: DWRITE_FONT_FEATURE_TAG = 942764915;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19: DWRITE_FONT_FEATURE_TAG = 959542131;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2: DWRITE_FONT_FEATURE_TAG = 842036083;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20: DWRITE_FONT_FEATURE_TAG = 808612723;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3: DWRITE_FONT_FEATURE_TAG = 858813299;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4: DWRITE_FONT_FEATURE_TAG = 875590515;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5: DWRITE_FONT_FEATURE_TAG = 892367731;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6: DWRITE_FONT_FEATURE_TAG = 909144947;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7: DWRITE_FONT_FEATURE_TAG = 925922163;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8: DWRITE_FONT_FEATURE_TAG = 942699379;
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9: DWRITE_FONT_FEATURE_TAG = 959476595;
pub const DWRITE_FONT_FEATURE_TAG_SUBSCRIPT: DWRITE_FONT_FEATURE_TAG = 1935832435;
pub const DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT: DWRITE_FONT_FEATURE_TAG = 1936749939;
pub const DWRITE_FONT_FEATURE_TAG_SWASH: DWRITE_FONT_FEATURE_TAG = 1752397683;
pub const DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES: DWRITE_FONT_FEATURE_TAG = 1836412532;
pub const DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS: DWRITE_FONT_FEATURE_TAG = 1684633460;
pub const DWRITE_FONT_FEATURE_TAG_TITLING: DWRITE_FONT_FEATURE_TAG = 1819568500;
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS: DWRITE_FONT_FEATURE_TAG = 1684107892;
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS: DWRITE_FONT_FEATURE_TAG = 1835101812;
pub const DWRITE_FONT_FEATURE_TAG_UNICASE: DWRITE_FONT_FEATURE_TAG = 1667853941;
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION: DWRITE_FONT_FEATURE_TAG = 846492278;
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING: DWRITE_FONT_FEATURE_TAG = 1953654134;
pub type DWRITE_FONT_FILE_TYPE = i32;
pub const DWRITE_FONT_FILE_TYPE_BITMAP: DWRITE_FONT_FILE_TYPE = 7;
pub const DWRITE_FONT_FILE_TYPE_CFF: DWRITE_FONT_FILE_TYPE = 1;
pub const DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION: DWRITE_FONT_FILE_TYPE = 3;
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE: DWRITE_FONT_FILE_TYPE = 2;
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION: DWRITE_FONT_FILE_TYPE = 3;
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFB: DWRITE_FONT_FILE_TYPE = 5;
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFM: DWRITE_FONT_FILE_TYPE = 4;
pub const DWRITE_FONT_FILE_TYPE_UNKNOWN: DWRITE_FONT_FILE_TYPE = 0;
pub const DWRITE_FONT_FILE_TYPE_VECTOR: DWRITE_FONT_FILE_TYPE = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub type DWRITE_FONT_SIMULATIONS = u32;
pub const DWRITE_FONT_SIMULATIONS_BOLD: DWRITE_FONT_SIMULATIONS = 1;
pub const DWRITE_FONT_SIMULATIONS_NONE: DWRITE_FONT_SIMULATIONS = 0;
pub const DWRITE_FONT_SIMULATIONS_OBLIQUE: DWRITE_FONT_SIMULATIONS = 2;
pub type DWRITE_FONT_STRETCH = i32;
pub const DWRITE_FONT_STRETCH_CONDENSED: DWRITE_FONT_STRETCH = 3;
pub const DWRITE_FONT_STRETCH_EXPANDED: DWRITE_FONT_STRETCH = 7;
pub const DWRITE_FONT_STRETCH_EXTRA_CONDENSED: DWRITE_FONT_STRETCH = 2;
pub const DWRITE_FONT_STRETCH_EXTRA_EXPANDED: DWRITE_FONT_STRETCH = 8;
pub const DWRITE_FONT_STRETCH_MEDIUM: DWRITE_FONT_STRETCH = 5;
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = 5;
pub const DWRITE_FONT_STRETCH_SEMI_CONDENSED: DWRITE_FONT_STRETCH = 4;
pub const DWRITE_FONT_STRETCH_SEMI_EXPANDED: DWRITE_FONT_STRETCH = 6;
pub const DWRITE_FONT_STRETCH_ULTRA_CONDENSED: DWRITE_FONT_STRETCH = 1;
pub const DWRITE_FONT_STRETCH_ULTRA_EXPANDED: DWRITE_FONT_STRETCH = 9;
pub const DWRITE_FONT_STRETCH_UNDEFINED: DWRITE_FONT_STRETCH = 0;
pub type DWRITE_FONT_STYLE = i32;
pub const DWRITE_FONT_STYLE_ITALIC: DWRITE_FONT_STYLE = 2;
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = 0;
pub const DWRITE_FONT_STYLE_OBLIQUE: DWRITE_FONT_STYLE = 1;
pub type DWRITE_FONT_WEIGHT = i32;
pub const DWRITE_FONT_WEIGHT_BLACK: DWRITE_FONT_WEIGHT = 900;
pub const DWRITE_FONT_WEIGHT_BOLD: DWRITE_FONT_WEIGHT = 700;
pub const DWRITE_FONT_WEIGHT_DEMI_BOLD: DWRITE_FONT_WEIGHT = 600;
pub const DWRITE_FONT_WEIGHT_EXTRA_BLACK: DWRITE_FONT_WEIGHT = 950;
pub const DWRITE_FONT_WEIGHT_EXTRA_BOLD: DWRITE_FONT_WEIGHT = 800;
pub const DWRITE_FONT_WEIGHT_EXTRA_LIGHT: DWRITE_FONT_WEIGHT = 200;
pub const DWRITE_FONT_WEIGHT_HEAVY: DWRITE_FONT_WEIGHT = 900;
pub const DWRITE_FONT_WEIGHT_LIGHT: DWRITE_FONT_WEIGHT = 300;
pub const DWRITE_FONT_WEIGHT_MEDIUM: DWRITE_FONT_WEIGHT = 500;
pub const DWRITE_FONT_WEIGHT_NORMAL: DWRITE_FONT_WEIGHT = 400;
pub const DWRITE_FONT_WEIGHT_REGULAR: DWRITE_FONT_WEIGHT = 400;
pub const DWRITE_FONT_WEIGHT_SEMI_BOLD: DWRITE_FONT_WEIGHT = 600;
pub const DWRITE_FONT_WEIGHT_SEMI_LIGHT: DWRITE_FONT_WEIGHT = 350;
pub const DWRITE_FONT_WEIGHT_THIN: DWRITE_FONT_WEIGHT = 100;
pub const DWRITE_FONT_WEIGHT_ULTRA_BLACK: DWRITE_FONT_WEIGHT = 950;
pub const DWRITE_FONT_WEIGHT_ULTRA_BOLD: DWRITE_FONT_WEIGHT = 800;
pub const DWRITE_FONT_WEIGHT_ULTRA_LIGHT: DWRITE_FONT_WEIGHT = 200;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_METRICS {
    pub leftSideBearing: i32,
    pub advanceWidth: u32,
    pub rightSideBearing: i32,
    pub topSideBearing: i32,
    pub advanceHeight: u32,
    pub bottomSideBearing: i32,
    pub verticalOriginY: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: f32,
    pub ascenderOffset: f32,
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: core::mem::ManuallyDrop<Option<IDWriteFontFace>>,
    pub fontEmSize: f32,
    pub glyphCount: u32,
    pub glyphIndices: *const u16,
    pub glyphAdvances: *const f32,
    pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    pub isSideways: windows_core::BOOL,
    pub bidiLevel: u32,
}
impl Default for DWRITE_GLYPH_RUN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: *const u16,
    pub string: *const u16,
    pub stringLength: u32,
    pub clusterMap: *const u16,
    pub textPosition: u32,
}
impl Default for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidiLevel: u32,
    pub isText: windows_core::BOOL,
    pub isTrimmed: windows_core::BOOL,
}
pub const DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE: DWRITE_INFORMATIONAL_STRING_ID = 1;
pub const DWRITE_INFORMATIONAL_STRING_DESCRIPTION: DWRITE_INFORMATIONAL_STRING_ID = 7;
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER: DWRITE_INFORMATIONAL_STRING_ID = 5;
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER_URL: DWRITE_INFORMATIONAL_STRING_ID = 6;
pub const DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_INFORMATIONAL_STRING_ID = 20;
pub const DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL: DWRITE_INFORMATIONAL_STRING_ID = 8;
pub const DWRITE_INFORMATIONAL_STRING_FULL_NAME: DWRITE_INFORMATIONAL_STRING_ID = 16;
pub type DWRITE_INFORMATIONAL_STRING_ID = i32;
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION: DWRITE_INFORMATIONAL_STRING_ID = 9;
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL: DWRITE_INFORMATIONAL_STRING_ID = 10;
pub const DWRITE_INFORMATIONAL_STRING_MANUFACTURER: DWRITE_INFORMATIONAL_STRING_ID = 4;
pub const DWRITE_INFORMATIONAL_STRING_NONE: DWRITE_INFORMATIONAL_STRING_ID = 0;
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME: DWRITE_INFORMATIONAL_STRING_ID = 18;
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME: DWRITE_INFORMATIONAL_STRING_ID = 17;
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 13;
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 14;
pub const DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT: DWRITE_INFORMATIONAL_STRING_ID = 15;
pub const DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_INFORMATIONAL_STRING_ID = 21;
pub const DWRITE_INFORMATIONAL_STRING_TRADEMARK: DWRITE_INFORMATIONAL_STRING_ID = 3;
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 13;
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 14;
pub const DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS: DWRITE_INFORMATIONAL_STRING_ID = 2;
pub const DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_INFORMATIONAL_STRING_ID = 19;
pub const DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 11;
pub const DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = 12;
pub const DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME: DWRITE_INFORMATIONAL_STRING_ID = 19;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_INLINE_OBJECT_METRICS {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supportsSideways: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_BREAKPOINT {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_METRICS {
    pub length: u32,
    pub trailingWhitespaceLength: u32,
    pub newlineLength: u32,
    pub height: f32,
    pub baseline: f32,
    pub isTrimmed: windows_core::BOOL,
}
pub type DWRITE_LINE_SPACING_METHOD = i32;
pub const DWRITE_LINE_SPACING_METHOD_DEFAULT: DWRITE_LINE_SPACING_METHOD = 0;
pub const DWRITE_LINE_SPACING_METHOD_PROPORTIONAL: DWRITE_LINE_SPACING_METHOD = 2;
pub const DWRITE_LINE_SPACING_METHOD_UNIFORM: DWRITE_LINE_SPACING_METHOD = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
pub type DWRITE_NUMBER_SUBSTITUTION_METHOD = i32;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = 1;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE: DWRITE_NUMBER_SUBSTITUTION_METHOD = 0;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = 3;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE: DWRITE_NUMBER_SUBSTITUTION_METHOD = 2;
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
pub type DWRITE_PARAGRAPH_ALIGNMENT = i32;
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = 2;
pub const DWRITE_PARAGRAPH_ALIGNMENT_FAR: DWRITE_PARAGRAPH_ALIGNMENT = 1;
pub const DWRITE_PARAGRAPH_ALIGNMENT_NEAR: DWRITE_PARAGRAPH_ALIGNMENT = 0;
pub type DWRITE_PIXEL_GEOMETRY = i32;
pub const DWRITE_PIXEL_GEOMETRY_BGR: DWRITE_PIXEL_GEOMETRY = 2;
pub const DWRITE_PIXEL_GEOMETRY_FLAT: DWRITE_PIXEL_GEOMETRY = 0;
pub const DWRITE_PIXEL_GEOMETRY_RGB: DWRITE_PIXEL_GEOMETRY = 1;
pub type DWRITE_READING_DIRECTION = i32;
pub const DWRITE_READING_DIRECTION_BOTTOM_TO_TOP: DWRITE_READING_DIRECTION = 3;
pub const DWRITE_READING_DIRECTION_LEFT_TO_RIGHT: DWRITE_READING_DIRECTION = 0;
pub const DWRITE_READING_DIRECTION_RIGHT_TO_LEFT: DWRITE_READING_DIRECTION = 1;
pub const DWRITE_READING_DIRECTION_TOP_TO_BOTTOM: DWRITE_READING_DIRECTION = 2;
pub type DWRITE_RENDERING_MODE = i32;
pub const DWRITE_RENDERING_MODE_ALIASED: DWRITE_RENDERING_MODE = 1;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC: DWRITE_RENDERING_MODE = 2;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL: DWRITE_RENDERING_MODE = 3;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL: DWRITE_RENDERING_MODE = 4;
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE = 5;
pub const DWRITE_RENDERING_MODE_DEFAULT: DWRITE_RENDERING_MODE = 0;
pub const DWRITE_RENDERING_MODE_GDI_CLASSIC: DWRITE_RENDERING_MODE = 2;
pub const DWRITE_RENDERING_MODE_GDI_NATURAL: DWRITE_RENDERING_MODE = 3;
pub const DWRITE_RENDERING_MODE_NATURAL: DWRITE_RENDERING_MODE = 4;
pub const DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE = 5;
pub const DWRITE_RENDERING_MODE_OUTLINE: DWRITE_RENDERING_MODE = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_SCRIPT_ANALYSIS {
    pub script: u16,
    pub shapes: DWRITE_SCRIPT_SHAPES,
}
pub type DWRITE_SCRIPT_SHAPES = u32;
pub const DWRITE_SCRIPT_SHAPES_DEFAULT: DWRITE_SCRIPT_SHAPES = 0;
pub const DWRITE_SCRIPT_SHAPES_NO_VISUAL: DWRITE_SCRIPT_SHAPES = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_SHAPING_GLYPH_PROPERTIES {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_SHAPING_TEXT_PROPERTIES {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_STRIKETHROUGH {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: *const u16,
    pub measuringMode: super::dcommon::DWRITE_MEASURING_MODE,
}
#[cfg(feature = "Win32_dcommon")]
impl Default for DWRITE_STRIKETHROUGH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DWRITE_TEXTURE_ALIASED_1x1: DWRITE_TEXTURE_TYPE = 0;
pub const DWRITE_TEXTURE_CLEARTYPE_3x1: DWRITE_TEXTURE_TYPE = 1;
pub type DWRITE_TEXTURE_TYPE = i32;
pub type DWRITE_TEXT_ALIGNMENT = i32;
pub const DWRITE_TEXT_ALIGNMENT_CENTER: DWRITE_TEXT_ALIGNMENT = 2;
pub const DWRITE_TEXT_ALIGNMENT_JUSTIFIED: DWRITE_TEXT_ALIGNMENT = 3;
pub const DWRITE_TEXT_ALIGNMENT_LEADING: DWRITE_TEXT_ALIGNMENT = 0;
pub const DWRITE_TEXT_ALIGNMENT_TRAILING: DWRITE_TEXT_ALIGNMENT = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: u32,
    pub length: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TRIMMING {
    pub granularity: DWRITE_TRIMMING_GRANULARITY,
    pub delimiter: u32,
    pub delimiterCount: u32,
}
pub type DWRITE_TRIMMING_GRANULARITY = i32;
pub const DWRITE_TRIMMING_GRANULARITY_CHARACTER: DWRITE_TRIMMING_GRANULARITY = 1;
pub const DWRITE_TRIMMING_GRANULARITY_NONE: DWRITE_TRIMMING_GRANULARITY = 0;
pub const DWRITE_TRIMMING_GRANULARITY_WORD: DWRITE_TRIMMING_GRANULARITY = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_TYPOGRAPHIC_FEATURES {
    pub features: *mut DWRITE_FONT_FEATURE,
    pub featureCount: u32,
}
impl Default for DWRITE_TYPOGRAPHIC_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dcommon")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_UNDERLINE {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub runHeight: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: *const u16,
    pub measuringMode: super::dcommon::DWRITE_MEASURING_MODE,
}
#[cfg(feature = "Win32_dcommon")]
impl Default for DWRITE_UNDERLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DWRITE_WORD_WRAPPING = i32;
pub const DWRITE_WORD_WRAPPING_CHARACTER: DWRITE_WORD_WRAPPING = 4;
pub const DWRITE_WORD_WRAPPING_EMERGENCY_BREAK: DWRITE_WORD_WRAPPING = 2;
pub const DWRITE_WORD_WRAPPING_NO_WRAP: DWRITE_WORD_WRAPPING = 1;
pub const DWRITE_WORD_WRAPPING_WHOLE_WORD: DWRITE_WORD_WRAPPING = 3;
pub const DWRITE_WORD_WRAPPING_WRAP: DWRITE_WORD_WRAPPING = 0;
pub const FACILITY_DWRITE: u32 = 2200;
windows_core::imp::define_interface!(IDWriteBitmapRenderTarget, IDWriteBitmapRenderTarget_Vtbl, 0x5e5a32a3_8dff_4773_9ff6_0696eab77267);
windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget, windows_core::IUnknown);
impl IDWriteBitmapRenderTarget {
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
    pub unsafe fn DrawGlyphRun<P4>(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: P4, textcolor: super::windef::COLORREF, blackboxrect: Option<*mut super::windef::RECT>) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IDWriteRenderingParams>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), baselineoriginx, baselineoriginy, measuringmode, core::mem::transmute(glyphrun), renderingparams.param().abi(), textcolor, blackboxrect.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetMemoryDC(&self) -> super::windef::HDC {
        unsafe { (windows_core::Interface::vtable(self).GetMemoryDC)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPixelsPerDip(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetPixelsPerDip)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPixelsPerDip(&self, pixelsperdip: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelsPerDip)(windows_core::Interface::as_raw(self), pixelsperdip) }
    }
    pub unsafe fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentTransform)(windows_core::Interface::as_raw(self), transform as _) }
    }
    pub unsafe fn SetCurrentTransform(&self, transform: Option<*const DWRITE_MATRIX>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTransform)(windows_core::Interface::as_raw(self), transform.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetSize(&self) -> windows_core::Result<super::windef::SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), width, height) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteBitmapRenderTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, super::dcommon::DWRITE_MEASURING_MODE, *const DWRITE_GLYPH_RUN, *mut core::ffi::c_void, super::windef::COLORREF, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_windef")))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_windef")]
    pub GetMemoryDC: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::windef::HDC,
    #[cfg(not(feature = "Win32_windef"))]
    GetMemoryDC: usize,
    pub GetPixelsPerDip: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub SetPixelsPerDip: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetCurrentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_MATRIX) -> windows_core::HRESULT,
    pub SetCurrentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_MATRIX) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetSize: usize,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
pub trait IDWriteBitmapRenderTarget_Impl: windows_core::IUnknownImpl {
    fn DrawGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: windows_core::Ref<IDWriteRenderingParams>, textcolor: super::windef::COLORREF, blackboxrect: *mut super::windef::RECT) -> windows_core::Result<()>;
    fn GetMemoryDC(&self) -> super::windef::HDC;
    fn GetPixelsPerDip(&self) -> f32;
    fn SetPixelsPerDip(&self, pixelsperdip: f32) -> windows_core::Result<()>;
    fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()>;
    fn SetCurrentTransform(&self, transform: *const DWRITE_MATRIX) -> windows_core::Result<()>;
    fn GetSize(&self) -> windows_core::Result<super::windef::SIZE>;
    fn Resize(&self, width: u32, height: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
impl IDWriteBitmapRenderTarget_Vtbl {
    pub const fn new<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DrawGlyphRun<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: *mut core::ffi::c_void, textcolor: super::windef::COLORREF, blackboxrect: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget_Impl::DrawGlyphRun(this, core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&renderingparams), core::mem::transmute_copy(&textcolor), core::mem::transmute_copy(&blackboxrect)).into()
            }
        }
        unsafe extern "system" fn GetMemoryDC<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::windef::HDC {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget_Impl::GetMemoryDC(this)
            }
        }
        unsafe extern "system" fn GetPixelsPerDip<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget_Impl::GetPixelsPerDip(this)
            }
        }
        unsafe extern "system" fn SetPixelsPerDip<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelsperdip: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget_Impl::SetPixelsPerDip(this, core::mem::transmute_copy(&pixelsperdip)).into()
            }
        }
        unsafe extern "system" fn GetCurrentTransform<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget_Impl::GetCurrentTransform(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn SetCurrentTransform<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const DWRITE_MATRIX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget_Impl::SetCurrentTransform(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn GetSize<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut super::windef::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteBitmapRenderTarget_Impl::GetSize(this) {
                    Ok(ok__) => {
                        size.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resize<Identity: IDWriteBitmapRenderTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget_Impl::Resize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            GetMemoryDC: GetMemoryDC::<Identity, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, OFFSET>,
            SetPixelsPerDip: SetPixelsPerDip::<Identity, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, OFFSET>,
            SetCurrentTransform: SetCurrentTransform::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteBitmapRenderTarget {}
windows_core::imp::define_interface!(IDWriteFactory, IDWriteFactory_Vtbl, 0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48);
windows_core::imp::interface_hierarchy!(IDWriteFactory, windows_core::IUnknown);
impl IDWriteFactory {
    pub unsafe fn GetSystemFontCollection(&self, fontcollection: *mut Option<IDWriteFontCollection>, checkforupdates: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(fontcollection), checkforupdates.into()) }
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const core::ffi::c_void, collectionkeysize: u32) -> windows_core::Result<IDWriteFontCollection>
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomFontCollection)(windows_core::Interface::as_raw(self), collectionloader.param().abi(), collectionkey, collectionkeysize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterFontCollectionLoader)(windows_core::Interface::as_raw(self), fontcollectionloader.param().abi()) }
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterFontCollectionLoader)(windows_core::Interface::as_raw(self), fontcollectionloader.param().abi()) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: Option<*const super::minwindef::FILETIME>) -> windows_core::Result<IDWriteFontFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFileReference)(windows_core::Interface::as_raw(self), filepath.param().abi(), lastwritetime.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomFontFileReference<P2>(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P2) -> windows_core::Result<IDWriteFontFile>
    where
        P2: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomFontFileReference)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), fontfacetype, fontfiles.len().try_into().unwrap(), core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRenderingParams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn CreateMonitorRenderingParams(&self, monitor: super::windef::HMONITOR) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMonitorRenderingParams)(windows_core::Interface::as_raw(self), monitor, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(windows_core::Interface::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterFontFileLoader)(windows_core::Interface::as_raw(self), fontfileloader.param().abi()) }
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterFontFileLoader)(windows_core::Interface::as_raw(self), fontfileloader.param().abi()) }
    }
    pub unsafe fn CreateTextFormat<P0, P1, P6>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P6) -> windows_core::Result<IDWriteTextFormat>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontCollection>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextFormat)(windows_core::Interface::as_raw(self), fontfamilyname.param().abi(), fontcollection.param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTypography)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetGdiInterop(&self) -> windows_core::Result<IDWriteGdiInterop> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGdiInterop)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTextLayout<P2>(&self, string: &[u16], textformat: P2, maxwidth: f32, maxheight: f32) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), maxwidth, maxheight, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGdiCompatibleTextLayout<P2>(&self, string: &[u16], textformat: P2, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, usegdinatural: bool) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGdiCompatibleTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), layoutwidth, layoutheight, pixelsperdip, transform.unwrap_or(core::mem::zeroed()) as _, usegdinatural.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> windows_core::Result<IDWriteInlineObject>
    where
        P0: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEllipsisTrimmingSign)(windows_core::Interface::as_raw(self), textformat.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> windows_core::Result<IDWriteTextAnalyzer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextAnalyzer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateNumberSubstitution<P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P1, ignoreuseroverride: bool) -> windows_core::Result<IDWriteNumberSubstitution>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNumberSubstitution)(windows_core::Interface::as_raw(self), substitutionmethod, localename.param().abi(), ignoreuseroverride.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<IDWriteGlyphRunAnalysis> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphrun), pixelsperdip, transform.unwrap_or(core::mem::zeroed()) as _, renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateCustomFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterFontCollectionLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterFontCollectionLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub CreateFontFileReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::minwindef::FILETIME, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    CreateFontFileReference: usize,
    pub CreateCustomFontFileReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FACE_TYPE, u32, *const *mut core::ffi::c_void, u32, DWRITE_FONT_SIMULATIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub CreateMonitorRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HMONITOR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    CreateMonitorRenderingParams: usize,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, DWRITE_PIXEL_GEOMETRY, DWRITE_RENDERING_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterFontFileLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterFontFileLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_FONT_STYLE, DWRITE_FONT_STRETCH, f32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTypography: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGdiInterop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, *mut core::ffi::c_void, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGdiCompatibleTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, *mut core::ffi::c_void, f32, f32, f32, *const DWRITE_MATRIX, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEllipsisTrimmingSign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextAnalyzer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNumberSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_NUMBER_SUBSTITUTION_METHOD, windows_core::PCWSTR, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dcommon")]
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_GLYPH_RUN, f32, *const DWRITE_MATRIX, DWRITE_RENDERING_MODE, super::dcommon::DWRITE_MEASURING_MODE, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    CreateGlyphRunAnalysis: usize,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory_Impl: windows_core::IUnknownImpl {
    fn GetSystemFontCollection(&self, fontcollection: windows_core::OutRef<IDWriteFontCollection>, checkforupdates: windows_core::BOOL) -> windows_core::Result<()>;
    fn CreateCustomFontCollection(&self, collectionloader: windows_core::Ref<IDWriteFontCollectionLoader>, collectionkey: *const core::ffi::c_void, collectionkeysize: u32) -> windows_core::Result<IDWriteFontCollection>;
    fn RegisterFontCollectionLoader(&self, fontcollectionloader: windows_core::Ref<IDWriteFontCollectionLoader>) -> windows_core::Result<()>;
    fn UnregisterFontCollectionLoader(&self, fontcollectionloader: windows_core::Ref<IDWriteFontCollectionLoader>) -> windows_core::Result<()>;
    fn CreateFontFileReference(&self, filepath: &windows_core::PCWSTR, lastwritetime: *const super::minwindef::FILETIME) -> windows_core::Result<IDWriteFontFile>;
    fn CreateCustomFontFileReference(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: windows_core::Ref<IDWriteFontFileLoader>) -> windows_core::Result<IDWriteFontFile>;
    fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const Option<IDWriteFontFile>, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> windows_core::Result<IDWriteFontFace>;
    fn CreateRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams>;
    fn CreateMonitorRenderingParams(&self, monitor: super::windef::HMONITOR) -> windows_core::Result<IDWriteRenderingParams>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> windows_core::Result<IDWriteRenderingParams>;
    fn RegisterFontFileLoader(&self, fontfileloader: windows_core::Ref<IDWriteFontFileLoader>) -> windows_core::Result<()>;
    fn UnregisterFontFileLoader(&self, fontfileloader: windows_core::Ref<IDWriteFontFileLoader>) -> windows_core::Result<()>;
    fn CreateTextFormat(&self, fontfamilyname: &windows_core::PCWSTR, fontcollection: windows_core::Ref<IDWriteFontCollection>, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: &windows_core::PCWSTR) -> windows_core::Result<IDWriteTextFormat>;
    fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography>;
    fn GetGdiInterop(&self) -> windows_core::Result<IDWriteGdiInterop>;
    fn CreateTextLayout(&self, string: *const u16, stringlength: u32, textformat: windows_core::Ref<IDWriteTextFormat>, maxwidth: f32, maxheight: f32) -> windows_core::Result<IDWriteTextLayout>;
    fn CreateGdiCompatibleTextLayout(&self, string: *const u16, stringlength: u32, textformat: windows_core::Ref<IDWriteTextFormat>, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: windows_core::BOOL) -> windows_core::Result<IDWriteTextLayout>;
    fn CreateEllipsisTrimmingSign(&self, textformat: windows_core::Ref<IDWriteTextFormat>) -> windows_core::Result<IDWriteInlineObject>;
    fn CreateTextAnalyzer(&self) -> windows_core::Result<IDWriteTextAnalyzer>;
    fn CreateNumberSubstitution(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: &windows_core::PCWSTR, ignoreuseroverride: windows_core::BOOL) -> windows_core::Result<IDWriteNumberSubstitution>;
    fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> windows_core::Result<IDWriteGlyphRunAnalysis>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory_Vtbl {
    pub const fn new<Identity: IDWriteFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSystemFontCollection<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void, checkforupdates: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::GetSystemFontCollection(this, core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&checkforupdates)).into()
            }
        }
        unsafe extern "system" fn CreateCustomFontCollection<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectionloader: *mut core::ffi::c_void, collectionkey: *const core::ffi::c_void, collectionkeysize: u32, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomFontCollection(this, core::mem::transmute_copy(&collectionloader), core::mem::transmute_copy(&collectionkey), core::mem::transmute_copy(&collectionkeysize)) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterFontCollectionLoader<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollectionloader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::RegisterFontCollectionLoader(this, core::mem::transmute_copy(&fontcollectionloader)).into()
            }
        }
        unsafe extern "system" fn UnregisterFontCollectionLoader<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollectionloader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::UnregisterFontCollectionLoader(this, core::mem::transmute_copy(&fontcollectionloader)).into()
            }
        }
        unsafe extern "system" fn CreateFontFileReference<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: windows_core::PCWSTR, lastwritetime: *const super::minwindef::FILETIME, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateFontFileReference(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&lastwritetime)) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomFontFileReference<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomFontFileReference(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize), core::mem::transmute_copy(&fontfileloader)) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const *mut core::ffi::c_void, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateFontFace(this, core::mem::transmute_copy(&fontfacetype), core::mem::transmute_copy(&numberoffiles), core::mem::transmute_copy(&fontfiles), core::mem::transmute_copy(&faceindex), core::mem::transmute_copy(&fontfacesimulationflags)) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRenderingParams<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateRenderingParams(this) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMonitorRenderingParams<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: super::windef::HMONITOR, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateMonitorRenderingParams(this, core::mem::transmute_copy(&monitor)) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomRenderingParams(this, core::mem::transmute_copy(&gamma), core::mem::transmute_copy(&enhancedcontrast), core::mem::transmute_copy(&cleartypelevel), core::mem::transmute_copy(&pixelgeometry), core::mem::transmute_copy(&renderingmode)) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterFontFileLoader<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfileloader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::RegisterFontFileLoader(this, core::mem::transmute_copy(&fontfileloader)).into()
            }
        }
        unsafe extern "system" fn UnregisterFontFileLoader<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfileloader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::UnregisterFontFileLoader(this, core::mem::transmute_copy(&fontfileloader)).into()
            }
        }
        unsafe extern "system" fn CreateTextFormat<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilyname: windows_core::PCWSTR, fontcollection: *mut core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: windows_core::PCWSTR, textformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextFormat(this, core::mem::transmute(&fontfamilyname), core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&fontweight), core::mem::transmute_copy(&fontstyle), core::mem::transmute_copy(&fontstretch), core::mem::transmute_copy(&fontsize), core::mem::transmute(&localename)) {
                    Ok(ok__) => {
                        textformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTypography<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typography: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTypography(this) {
                    Ok(ok__) => {
                        typography.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGdiInterop<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdiinterop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::GetGdiInterop(this) {
                    Ok(ok__) => {
                        gdiinterop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTextLayout<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: *const u16, stringlength: u32, textformat: *mut core::ffi::c_void, maxwidth: f32, maxheight: f32, textlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextLayout(this, core::mem::transmute_copy(&string), core::mem::transmute_copy(&stringlength), core::mem::transmute_copy(&textformat), core::mem::transmute_copy(&maxwidth), core::mem::transmute_copy(&maxheight)) {
                    Ok(ok__) => {
                        textlayout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGdiCompatibleTextLayout<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: *const u16, stringlength: u32, textformat: *mut core::ffi::c_void, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: windows_core::BOOL, textlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateGdiCompatibleTextLayout(this, core::mem::transmute_copy(&string), core::mem::transmute_copy(&stringlength), core::mem::transmute_copy(&textformat), core::mem::transmute_copy(&layoutwidth), core::mem::transmute_copy(&layoutheight), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&usegdinatural)) {
                    Ok(ok__) => {
                        textlayout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEllipsisTrimmingSign<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textformat: *mut core::ffi::c_void, trimmingsign: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateEllipsisTrimmingSign(this, core::mem::transmute_copy(&textformat)) {
                    Ok(ok__) => {
                        trimmingsign.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTextAnalyzer<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textanalyzer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextAnalyzer(this) {
                    Ok(ok__) => {
                        textanalyzer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateNumberSubstitution<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: windows_core::PCWSTR, ignoreuseroverride: windows_core::BOOL, numbersubstitution: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateNumberSubstitution(this, core::mem::transmute_copy(&substitutionmethod), core::mem::transmute(&localename), core::mem::transmute_copy(&ignoreuseroverride)) {
                    Ok(ok__) => {
                        numbersubstitution.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Identity: IDWriteFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateGlyphRunAnalysis(this, core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&renderingmode), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy)) {
                    Ok(ok__) => {
                        glyphrunanalysis.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
            CreateCustomFontCollection: CreateCustomFontCollection::<Identity, OFFSET>,
            RegisterFontCollectionLoader: RegisterFontCollectionLoader::<Identity, OFFSET>,
            UnregisterFontCollectionLoader: UnregisterFontCollectionLoader::<Identity, OFFSET>,
            CreateFontFileReference: CreateFontFileReference::<Identity, OFFSET>,
            CreateCustomFontFileReference: CreateCustomFontFileReference::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            CreateRenderingParams: CreateRenderingParams::<Identity, OFFSET>,
            CreateMonitorRenderingParams: CreateMonitorRenderingParams::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
            RegisterFontFileLoader: RegisterFontFileLoader::<Identity, OFFSET>,
            UnregisterFontFileLoader: UnregisterFontFileLoader::<Identity, OFFSET>,
            CreateTextFormat: CreateTextFormat::<Identity, OFFSET>,
            CreateTypography: CreateTypography::<Identity, OFFSET>,
            GetGdiInterop: GetGdiInterop::<Identity, OFFSET>,
            CreateTextLayout: CreateTextLayout::<Identity, OFFSET>,
            CreateGdiCompatibleTextLayout: CreateGdiCompatibleTextLayout::<Identity, OFFSET>,
            CreateEllipsisTrimmingSign: CreateEllipsisTrimmingSign::<Identity, OFFSET>,
            CreateTextAnalyzer: CreateTextAnalyzer::<Identity, OFFSET>,
            CreateNumberSubstitution: CreateNumberSubstitution::<Identity, OFFSET>,
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory {}
windows_core::imp::define_interface!(IDWriteFont, IDWriteFont_Vtbl, 0xacd16696_8c14_4f5d_877e_fe3fc1d32737);
windows_core::imp::interface_hierarchy!(IDWriteFont, windows_core::IUnknown);
impl IDWriteFont {
    pub unsafe fn GetFontFamily(&self) -> windows_core::Result<IDWriteFontFamily> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        unsafe { (windows_core::Interface::vtable(self).GetWeight)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        unsafe { (windows_core::Interface::vtable(self).GetStretch)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        unsafe { (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsSymbolFont(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsSymbolFont)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFaceNames(&self) -> windows_core::Result<IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFaceNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut Option<IDWriteLocalizedStrings>, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInformationalStrings)(windows_core::Interface::as_raw(self), informationalstringid, core::mem::transmute(informationalstrings), exists as _) }
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        unsafe { (windows_core::Interface::vtable(self).GetSimulations)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), fontmetrics as _);
        }
    }
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasCharacter)(windows_core::Interface::as_raw(self), unicodevalue, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFont_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub IsSymbolFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetFaceNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInformationalStrings: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_INFORMATIONAL_STRING_ID, *mut *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSimulations: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS),
    pub HasCharacter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteFont_Impl: windows_core::IUnknownImpl {
    fn GetFontFamily(&self) -> windows_core::Result<IDWriteFontFamily>;
    fn GetWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetStyle(&self) -> DWRITE_FONT_STYLE;
    fn IsSymbolFont(&self) -> windows_core::BOOL;
    fn GetFaceNames(&self) -> windows_core::Result<IDWriteLocalizedStrings>;
    fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: windows_core::OutRef<IDWriteLocalizedStrings>, exists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS;
    fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS);
    fn HasCharacter(&self, unicodevalue: u32) -> windows_core::Result<windows_core::BOOL>;
    fn CreateFontFace(&self) -> windows_core::Result<IDWriteFontFace>;
}
impl IDWriteFont_Vtbl {
    pub const fn new<Identity: IDWriteFont_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontFamily<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFont_Impl::GetFontFamily(this) {
                    Ok(ok__) => {
                        fontfamily.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWeight<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont_Impl::GetWeight(this)
            }
        }
        unsafe extern "system" fn GetStretch<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont_Impl::GetStretch(this)
            }
        }
        unsafe extern "system" fn GetStyle<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STYLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont_Impl::GetStyle(this)
            }
        }
        unsafe extern "system" fn IsSymbolFont<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont_Impl::IsSymbolFont(this)
            }
        }
        unsafe extern "system" fn GetFaceNames<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFont_Impl::GetFaceNames(this) {
                    Ok(ok__) => {
                        names.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInformationalStrings<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut core::ffi::c_void, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont_Impl::GetInformationalStrings(this, core::mem::transmute_copy(&informationalstringid), core::mem::transmute_copy(&informationalstrings), core::mem::transmute_copy(&exists)).into()
            }
        }
        unsafe extern "system" fn GetSimulations<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont_Impl::GetSimulations(this)
            }
        }
        unsafe extern "system" fn GetMetrics<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont_Impl::GetMetrics(this, core::mem::transmute_copy(&fontmetrics));
            }
        }
        unsafe extern "system" fn HasCharacter<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unicodevalue: u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFont_Impl::HasCharacter(this, core::mem::transmute_copy(&unicodevalue)) {
                    Ok(ok__) => {
                        exists.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFace<Identity: IDWriteFont_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFont_Impl::CreateFontFace(this) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
            GetWeight: GetWeight::<Identity, OFFSET>,
            GetStretch: GetStretch::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            IsSymbolFont: IsSymbolFont::<Identity, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, OFFSET>,
            GetInformationalStrings: GetInformationalStrings::<Identity, OFFSET>,
            GetSimulations: GetSimulations::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            HasCharacter: HasCharacter::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFont as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFont {}
windows_core::imp::define_interface!(IDWriteFontCollection, IDWriteFontCollection_Vtbl, 0xa84cee02_3eea_4eee_a827_87c1a02a0fcc);
windows_core::imp::interface_hierarchy!(IDWriteFontCollection, windows_core::IUnknown);
impl IDWriteFontCollection {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindFamilyName)(windows_core::Interface::as_raw(self), familyname.param().abi(), index as _, exists as _) }
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> windows_core::Result<IDWriteFont>
    where
        P0: windows_core::Param<IDWriteFontFace>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFromFontFace)(windows_core::Interface::as_raw(self), fontface.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontFamilyCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFontFromFontFace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteFontCollection_Impl: windows_core::IUnknownImpl {
    fn GetFontFamilyCount(&self) -> u32;
    fn GetFontFamily(&self, index: u32) -> windows_core::Result<IDWriteFontFamily>;
    fn FindFamilyName(&self, familyname: &windows_core::PCWSTR, index: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFontFromFontFace(&self, fontface: windows_core::Ref<IDWriteFontFace>) -> windows_core::Result<IDWriteFont>;
}
impl IDWriteFontCollection_Vtbl {
    pub const fn new<Identity: IDWriteFontCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontFamilyCount<Identity: IDWriteFontCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontCollection_Impl::GetFontFamilyCount(this)
            }
        }
        unsafe extern "system" fn GetFontFamily<Identity: IDWriteFontCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, fontfamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontCollection_Impl::GetFontFamily(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        fontfamily.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFamilyName<Identity: IDWriteFontCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: windows_core::PCWSTR, index: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontCollection_Impl::FindFamilyName(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&index), core::mem::transmute_copy(&exists)).into()
            }
        }
        unsafe extern "system" fn GetFontFromFontFace<Identity: IDWriteFontCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontCollection_Impl::GetFontFromFontFace(this, core::mem::transmute_copy(&fontface)) {
                    Ok(ok__) => {
                        font.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontFamilyCount: GetFontFamilyCount::<Identity, OFFSET>,
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
            FindFamilyName: FindFamilyName::<Identity, OFFSET>,
            GetFontFromFontFace: GetFontFromFontFace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontCollection {}
windows_core::imp::define_interface!(IDWriteFontCollectionLoader, IDWriteFontCollectionLoader_Vtbl, 0xcca920e4_52f0_492b_bfa8_29c72ee0a468);
windows_core::imp::interface_hierarchy!(IDWriteFontCollectionLoader, windows_core::IUnknown);
impl IDWriteFontCollectionLoader {
    pub unsafe fn CreateEnumeratorFromKey<P0>(&self, factory: P0, collectionkey: *const core::ffi::c_void, collectionkeysize: u32) -> windows_core::Result<IDWriteFontFileEnumerator>
    where
        P0: windows_core::Param<IDWriteFactory>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEnumeratorFromKey)(windows_core::Interface::as_raw(self), factory.param().abi(), collectionkey, collectionkeysize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollectionLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateEnumeratorFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteFontCollectionLoader_Impl: windows_core::IUnknownImpl {
    fn CreateEnumeratorFromKey(&self, factory: windows_core::Ref<IDWriteFactory>, collectionkey: *const core::ffi::c_void, collectionkeysize: u32) -> windows_core::Result<IDWriteFontFileEnumerator>;
}
impl IDWriteFontCollectionLoader_Vtbl {
    pub const fn new<Identity: IDWriteFontCollectionLoader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEnumeratorFromKey<Identity: IDWriteFontCollectionLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, collectionkey: *const core::ffi::c_void, collectionkeysize: u32, fontfileenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontCollectionLoader_Impl::CreateEnumeratorFromKey(this, core::mem::transmute_copy(&factory), core::mem::transmute_copy(&collectionkey), core::mem::transmute_copy(&collectionkeysize)) {
                    Ok(ok__) => {
                        fontfileenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateEnumeratorFromKey: CreateEnumeratorFromKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontCollectionLoader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontCollectionLoader {}
windows_core::imp::define_interface!(IDWriteFontFace, IDWriteFontFace_Vtbl, 0x5f49804d_7024_4d43_bfa9_d25984f53849);
windows_core::imp::interface_hierarchy!(IDWriteFontFace, windows_core::IUnknown);
impl IDWriteFontFace {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: Option<*mut Option<IDWriteFontFile>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFiles)(windows_core::Interface::as_raw(self), numberoffiles as _, fontfiles.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        unsafe { (windows_core::Interface::vtable(self).GetSimulations)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsSymbolFont(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsSymbolFont)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), fontfacemetrics as _);
        }
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesignGlyphMetrics(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesignGlyphMetrics)(windows_core::Interface::as_raw(self), glyphindices, glyphcount, glyphmetrics as _, issideways.into()) }
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphIndices)(windows_core::Interface::as_raw(self), codepoints, codepointcount, glyphindices as _) }
    }
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut core::ffi::c_void, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TryGetFontTable)(windows_core::Interface::as_raw(self), opentypetabletag, tabledata as _, tablesize as _, tablecontext as _, exists as _) }
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseFontTable)(windows_core::Interface::as_raw(self), tablecontext);
        }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn GetGlyphRunOutline<P7>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: Option<*const f32>, glyphoffsets: Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: bool, isrighttoleft: bool, geometrysink: P7) -> windows_core::HRESULT
    where
        P7: windows_core::Param<super::d2d1::ID2D1SimplifiedGeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphRunOutline)(windows_core::Interface::as_raw(self), emsize, glyphindices, glyphadvances.unwrap_or(core::mem::zeroed()) as _, glyphoffsets.unwrap_or(core::mem::zeroed()) as _, glyphcount, issideways.into(), isrighttoleft.into(), geometrysink.param().abi()) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetRecommendedRenderingMode<P3>(&self, emsize: f32, pixelsperdip: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: P3) -> windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P3: windows_core::Param<IDWriteRenderingParams>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, measuringmode, renderingparams.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGdiCompatibleMetrics)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, transform.unwrap_or(core::mem::zeroed()) as _, fontfacemetrics as _) }
    }
    pub unsafe fn GetGdiCompatibleGlyphMetrics(&self, emsize: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, usegdinatural: bool, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGdiCompatibleGlyphMetrics)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, transform.unwrap_or(core::mem::zeroed()) as _, usegdinatural.into(), glyphindices, glyphcount, glyphmetrics as _, issideways.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE,
    pub GetFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSimulations: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS,
    pub IsSymbolFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS),
    pub GetGlyphCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    pub GetDesignGlyphMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, *mut DWRITE_GLYPH_METRICS, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32, *mut u16) -> windows_core::HRESULT,
    pub TryGetFontTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ReleaseFontTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
    #[cfg(feature = "Win32_d2d1")]
    pub GetGlyphRunOutline: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const u16, *const f32, *const DWRITE_GLYPH_OFFSET, u32, windows_core::BOOL, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    GetGlyphRunOutline: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, super::dcommon::DWRITE_MEASURING_MODE, *mut core::ffi::c_void, *mut DWRITE_RENDERING_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetRecommendedRenderingMode: usize,
    pub GetGdiCompatibleMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const DWRITE_MATRIX, *mut DWRITE_FONT_METRICS) -> windows_core::HRESULT,
    pub GetGdiCompatibleGlyphMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const DWRITE_MATRIX, windows_core::BOOL, *const u16, u32, *mut DWRITE_GLYPH_METRICS, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
pub trait IDWriteFontFace_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> DWRITE_FONT_FACE_TYPE;
    fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: windows_core::OutRef<IDWriteFontFile>) -> windows_core::Result<()>;
    fn GetIndex(&self) -> u32;
    fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS;
    fn IsSymbolFont(&self) -> windows_core::BOOL;
    fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS);
    fn GetGlyphCount(&self) -> u16;
    fn GetDesignGlyphMetrics(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> windows_core::Result<()>;
    fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut core::ffi::c_void, exists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn ReleaseFontTable(&self, tablecontext: *const core::ffi::c_void);
    fn GetGlyphRunOutline(&self, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, geometrysink: windows_core::Ref<super::d2d1::ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn GetRecommendedRenderingMode(&self, emsize: f32, pixelsperdip: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: windows_core::Ref<IDWriteRenderingParams>) -> windows_core::Result<DWRITE_RENDERING_MODE>;
    fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> windows_core::Result<()>;
    fn GetGdiCompatibleGlyphMetrics(&self, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: windows_core::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl IDWriteFontFace_Vtbl {
    pub const fn new<Identity: IDWriteFontFace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetFiles<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberoffiles: *mut u32, fontfiles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetFiles(this, core::mem::transmute_copy(&numberoffiles), core::mem::transmute_copy(&fontfiles)).into()
            }
        }
        unsafe extern "system" fn GetIndex<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetIndex(this)
            }
        }
        unsafe extern "system" fn GetSimulations<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetSimulations(this)
            }
        }
        unsafe extern "system" fn IsSymbolFont<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::IsSymbolFont(this)
            }
        }
        unsafe extern "system" fn GetMetrics<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetMetrics(this, core::mem::transmute_copy(&fontfacemetrics));
            }
        }
        unsafe extern "system" fn GetGlyphCount<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u16 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetGlyphCount(this)
            }
        }
        unsafe extern "system" fn GetDesignGlyphMetrics<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetDesignGlyphMetrics(this, core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphmetrics), core::mem::transmute_copy(&issideways)).into()
            }
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetGlyphIndices(this, core::mem::transmute_copy(&codepoints), core::mem::transmute_copy(&codepointcount), core::mem::transmute_copy(&glyphindices)).into()
            }
        }
        unsafe extern "system" fn TryGetFontTable<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opentypetabletag: u32, tabledata: *mut *mut core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut core::ffi::c_void, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::TryGetFontTable(this, core::mem::transmute_copy(&opentypetabletag), core::mem::transmute_copy(&tabledata), core::mem::transmute_copy(&tablesize), core::mem::transmute_copy(&tablecontext), core::mem::transmute_copy(&exists)).into()
            }
        }
        unsafe extern "system" fn ReleaseFontTable<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tablecontext: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::ReleaseFontTable(this, core::mem::transmute_copy(&tablecontext));
            }
        }
        unsafe extern "system" fn GetGlyphRunOutline<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetGlyphRunOutline(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphadvances), core::mem::transmute_copy(&glyphoffsets), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft), core::mem::transmute_copy(&geometrysink)).into()
            }
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingparams: *mut core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace_Impl::GetRecommendedRenderingMode(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&renderingparams)) {
                    Ok(ok__) => {
                        renderingmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetGdiCompatibleMetrics(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&fontfacemetrics)).into()
            }
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphMetrics<Identity: IDWriteFontFace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: windows_core::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace_Impl::GetGdiCompatibleGlyphMetrics(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&usegdinatural), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphmetrics), core::mem::transmute_copy(&issideways)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetFiles: GetFiles::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            GetSimulations: GetSimulations::<Identity, OFFSET>,
            IsSymbolFont: IsSymbolFont::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetGlyphCount: GetGlyphCount::<Identity, OFFSET>,
            GetDesignGlyphMetrics: GetDesignGlyphMetrics::<Identity, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, OFFSET>,
            TryGetFontTable: TryGetFontTable::<Identity, OFFSET>,
            ReleaseFontTable: ReleaseFontTable::<Identity, OFFSET>,
            GetGlyphRunOutline: GetGlyphRunOutline::<Identity, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, OFFSET>,
            GetGdiCompatibleMetrics: GetGdiCompatibleMetrics::<Identity, OFFSET>,
            GetGdiCompatibleGlyphMetrics: GetGdiCompatibleGlyphMetrics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl windows_core::RuntimeName for IDWriteFontFace {}
windows_core::imp::define_interface!(IDWriteFontFamily, IDWriteFontFamily_Vtbl, 0xda20d8ef_812a_4c43_9802_62ec4abd7add);
impl core::ops::Deref for IDWriteFontFamily {
    type Target = IDWriteFontList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteFontFamily, windows_core::IUnknown, IDWriteFontList);
impl IDWriteFontFamily {
    pub unsafe fn GetFamilyNames(&self) -> windows_core::Result<IDWriteLocalizedStrings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFamilyNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstMatchingFont)(windows_core::Interface::as_raw(self), weight, stretch, style, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFontList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMatchingFonts)(windows_core::Interface::as_raw(self), weight, stretch, style, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFamily_Vtbl {
    pub base__: IDWriteFontList_Vtbl,
    pub GetFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstMatchingFont: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteFontFamily_Impl: IDWriteFontList_Impl {
    fn GetFamilyNames(&self) -> windows_core::Result<IDWriteLocalizedStrings>;
    fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFont>;
    fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> windows_core::Result<IDWriteFontList>;
}
impl IDWriteFontFamily_Vtbl {
    pub const fn new<Identity: IDWriteFontFamily_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFamilyNames<Identity: IDWriteFontFamily_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, names: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFamily_Impl::GetFamilyNames(this) {
                    Ok(ok__) => {
                        names.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFirstMatchingFont<Identity: IDWriteFontFamily_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfont: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFamily_Impl::GetFirstMatchingFont(this, core::mem::transmute_copy(&weight), core::mem::transmute_copy(&stretch), core::mem::transmute_copy(&style)) {
                    Ok(ok__) => {
                        matchingfont.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: IDWriteFontFamily_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfonts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFamily_Impl::GetMatchingFonts(this, core::mem::transmute_copy(&weight), core::mem::transmute_copy(&stretch), core::mem::transmute_copy(&style)) {
                    Ok(ok__) => {
                        matchingfonts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFontList_Vtbl::new::<Identity, OFFSET>(),
            GetFamilyNames: GetFamilyNames::<Identity, OFFSET>,
            GetFirstMatchingFont: GetFirstMatchingFont::<Identity, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFamily as windows_core::Interface>::IID || iid == &<IDWriteFontList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontFamily {}
windows_core::imp::define_interface!(IDWriteFontFile, IDWriteFontFile_Vtbl, 0x739d886a_cef5_47dc_8769_1a8b41bebbb0);
windows_core::imp::interface_hierarchy!(IDWriteFontFile, windows_core::IUnknown);
impl IDWriteFontFile {
    pub unsafe fn GetReferenceKey(&self, fontfilereferencekey: *mut *mut core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReferenceKey)(windows_core::Interface::as_raw(self), fontfilereferencekey as _, fontfilereferencekeysize as _) }
    }
    pub unsafe fn GetLoader(&self) -> windows_core::Result<IDWriteFontFileLoader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLoader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Analyze(&self, issupportedfonttype: *mut windows_core::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: Option<*mut DWRITE_FONT_FACE_TYPE>, numberoffaces: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Analyze)(windows_core::Interface::as_raw(self), issupportedfonttype as _, fontfiletype as _, fontfacetype.unwrap_or(core::mem::zeroed()) as _, numberoffaces as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetReferenceKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLoader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Analyze: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut DWRITE_FONT_FILE_TYPE, *mut DWRITE_FONT_FACE_TYPE, *mut u32) -> windows_core::HRESULT,
}
pub trait IDWriteFontFile_Impl: windows_core::IUnknownImpl {
    fn GetReferenceKey(&self, fontfilereferencekey: *mut *mut core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> windows_core::Result<()>;
    fn GetLoader(&self) -> windows_core::Result<IDWriteFontFileLoader>;
    fn Analyze(&self, issupportedfonttype: *mut windows_core::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> windows_core::Result<()>;
}
impl IDWriteFontFile_Vtbl {
    pub const fn new<Identity: IDWriteFontFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetReferenceKey<Identity: IDWriteFontFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *mut *mut core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFile_Impl::GetReferenceKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)).into()
            }
        }
        unsafe extern "system" fn GetLoader<Identity: IDWriteFontFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfileloader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFile_Impl::GetLoader(this) {
                    Ok(ok__) => {
                        fontfileloader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Analyze<Identity: IDWriteFontFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, issupportedfonttype: *mut windows_core::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFile_Impl::Analyze(this, core::mem::transmute_copy(&issupportedfonttype), core::mem::transmute_copy(&fontfiletype), core::mem::transmute_copy(&fontfacetype), core::mem::transmute_copy(&numberoffaces)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetReferenceKey: GetReferenceKey::<Identity, OFFSET>,
            GetLoader: GetLoader::<Identity, OFFSET>,
            Analyze: Analyze::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFile as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontFile {}
windows_core::imp::define_interface!(IDWriteFontFileEnumerator, IDWriteFontFileEnumerator_Vtbl, 0x72755049_5ff7_435d_8348_4be97cfa6c7c);
windows_core::imp::interface_hierarchy!(IDWriteFontFileEnumerator, windows_core::IUnknown);
impl IDWriteFontFileEnumerator {
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentFontFile(&self) -> windows_core::Result<IDWriteFontFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentFontFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFileEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrentFontFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteFontFileEnumerator_Impl: windows_core::IUnknownImpl {
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrentFontFile(&self) -> windows_core::Result<IDWriteFontFile>;
}
impl IDWriteFontFileEnumerator_Vtbl {
    pub const fn new<Identity: IDWriteFontFileEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveNext<Identity: IDWriteFontFileEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrentfile: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFileEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hascurrentfile.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentFontFile<Identity: IDWriteFontFileEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFileEnumerator_Impl::GetCurrentFontFile(this) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveNext: MoveNext::<Identity, OFFSET>,
            GetCurrentFontFile: GetCurrentFontFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFileEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontFileEnumerator {}
windows_core::imp::define_interface!(IDWriteFontFileLoader, IDWriteFontFileLoader_Vtbl, 0x727cad4e_d6af_4c9e_8a08_d695b11caa49);
windows_core::imp::interface_hierarchy!(IDWriteFontFileLoader, windows_core::IUnknown);
impl IDWriteFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<IDWriteFontFileStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStreamFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFileLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateStreamFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteFontFileLoader_Impl: windows_core::IUnknownImpl {
    fn CreateStreamFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<IDWriteFontFileStream>;
}
impl IDWriteFontFileLoader_Vtbl {
    pub const fn new<Identity: IDWriteFontFileLoader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStreamFromKey<Identity: IDWriteFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFileLoader_Impl::CreateStreamFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                    Ok(ok__) => {
                        fontfilestream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateStreamFromKey: CreateStreamFromKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFileLoader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontFileLoader {}
windows_core::imp::define_interface!(IDWriteFontFileStream, IDWriteFontFileStream_Vtbl, 0x6d4865fe_0ab8_4d91_8f62_5dd6be34a3e0);
windows_core::imp::interface_hierarchy!(IDWriteFontFileStream, windows_core::IUnknown);
impl IDWriteFontFileStream {
    pub unsafe fn ReadFileFragment(&self, fragmentstart: *mut *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadFileFragment)(windows_core::Interface::as_raw(self), fragmentstart as _, fileoffset, fragmentsize, fragmentcontext as _) }
    }
    pub unsafe fn ReleaseFileFragment(&self, fragmentcontext: *mut core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).ReleaseFileFragment)(windows_core::Interface::as_raw(self), fragmentcontext as _);
        }
    }
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLastWriteTime(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastWriteTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFileStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadFileFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u64, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseFileFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetLastWriteTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IDWriteFontFileStream_Impl: windows_core::IUnknownImpl {
    fn ReadFileFragment(&self, fragmentstart: *mut *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseFileFragment(&self, fragmentcontext: *mut core::ffi::c_void);
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn GetLastWriteTime(&self) -> windows_core::Result<u64>;
}
impl IDWriteFontFileStream_Vtbl {
    pub const fn new<Identity: IDWriteFontFileStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadFileFragment<Identity: IDWriteFontFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fragmentstart: *mut *mut core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFileStream_Impl::ReadFileFragment(this, core::mem::transmute_copy(&fragmentstart), core::mem::transmute_copy(&fileoffset), core::mem::transmute_copy(&fragmentsize), core::mem::transmute_copy(&fragmentcontext)).into()
            }
        }
        unsafe extern "system" fn ReleaseFileFragment<Identity: IDWriteFontFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fragmentcontext: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFileStream_Impl::ReleaseFileFragment(this, core::mem::transmute_copy(&fragmentcontext));
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: IDWriteFontFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFileStream_Impl::GetFileSize(this) {
                    Ok(ok__) => {
                        filesize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastWriteTime<Identity: IDWriteFontFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastwritetime: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFileStream_Impl::GetLastWriteTime(this) {
                    Ok(ok__) => {
                        lastwritetime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadFileFragment: ReadFileFragment::<Identity, OFFSET>,
            ReleaseFileFragment: ReleaseFileFragment::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            GetLastWriteTime: GetLastWriteTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFileStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontFileStream {}
windows_core::imp::define_interface!(IDWriteFontList, IDWriteFontList_Vtbl, 0x1a0d8438_1d97_4ec1_aef9_a2fb86ed6acb);
windows_core::imp::interface_hierarchy!(IDWriteFontList, windows_core::IUnknown);
impl IDWriteFontList {
    pub unsafe fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFont(&self, index: u32) -> windows_core::Result<IDWriteFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteFontList_Impl: windows_core::IUnknownImpl {
    fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection>;
    fn GetFontCount(&self) -> u32;
    fn GetFont(&self, index: u32) -> windows_core::Result<IDWriteFont>;
}
impl IDWriteFontList_Vtbl {
    pub const fn new<Identity: IDWriteFontList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFontCollection<Identity: IDWriteFontList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontList_Impl::GetFontCollection(this) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontCount<Identity: IDWriteFontList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontList_Impl::GetFontCount(this)
            }
        }
        unsafe extern "system" fn GetFont<Identity: IDWriteFontList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontList_Impl::GetFont(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        font.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFontCollection: GetFontCollection::<Identity, OFFSET>,
            GetFontCount: GetFontCount::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFontList {}
windows_core::imp::define_interface!(IDWriteGdiInterop, IDWriteGdiInterop_Vtbl, 0x1edd9491_9853_4299_898f_6432983b6f3a);
windows_core::imp::interface_hierarchy!(IDWriteGdiInterop, windows_core::IUnknown);
impl IDWriteGdiInterop {
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn CreateFontFromLOGFONT(&self, logfont: *const super::wingdi::LOGFONTW) -> windows_core::Result<IDWriteFont> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFromLOGFONT)(windows_core::Interface::as_raw(self), logfont, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn ConvertFontToLOGFONT<P0>(&self, font: P0, logfont: *mut super::wingdi::LOGFONTW, issystemfont: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFont>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertFontToLOGFONT)(windows_core::Interface::as_raw(self), font.param().abi(), logfont as _, issystemfont as _) }
    }
    #[cfg(feature = "Win32_wingdi")]
    pub unsafe fn ConvertFontFaceToLOGFONT<P0>(&self, font: P0, logfont: *mut super::wingdi::LOGFONTW) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFace>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertFontFaceToLOGFONT)(windows_core::Interface::as_raw(self), font.param().abi(), logfont as _) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn CreateFontFaceFromHdc(&self, hdc: super::windef::HDC) -> windows_core::Result<IDWriteFontFace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFaceFromHdc)(windows_core::Interface::as_raw(self), hdc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn CreateBitmapRenderTarget(&self, hdc: Option<super::windef::HDC>, width: u32, height: u32) -> windows_core::Result<IDWriteBitmapRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapRenderTarget)(windows_core::Interface::as_raw(self), hdc.unwrap_or(core::mem::zeroed()) as _, width, height, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteGdiInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wingdi")]
    pub CreateFontFromLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wingdi::LOGFONTW, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    CreateFontFromLOGFONT: usize,
    #[cfg(feature = "Win32_wingdi")]
    pub ConvertFontToLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wingdi::LOGFONTW, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    ConvertFontToLOGFONT: usize,
    #[cfg(feature = "Win32_wingdi")]
    pub ConvertFontFaceToLOGFONT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wingdi::LOGFONTW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wingdi"))]
    ConvertFontFaceToLOGFONT: usize,
    #[cfg(feature = "Win32_windef")]
    pub CreateFontFaceFromHdc: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    CreateFontFaceFromHdc: usize,
    #[cfg(feature = "Win32_windef")]
    pub CreateBitmapRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HDC, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    CreateBitmapRenderTarget: usize,
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
pub trait IDWriteGdiInterop_Impl: windows_core::IUnknownImpl {
    fn CreateFontFromLOGFONT(&self, logfont: *const super::wingdi::LOGFONTW) -> windows_core::Result<IDWriteFont>;
    fn ConvertFontToLOGFONT(&self, font: windows_core::Ref<IDWriteFont>, logfont: *mut super::wingdi::LOGFONTW, issystemfont: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn ConvertFontFaceToLOGFONT(&self, font: windows_core::Ref<IDWriteFontFace>, logfont: *mut super::wingdi::LOGFONTW) -> windows_core::Result<()>;
    fn CreateFontFaceFromHdc(&self, hdc: super::windef::HDC) -> windows_core::Result<IDWriteFontFace>;
    fn CreateBitmapRenderTarget(&self, hdc: super::windef::HDC, width: u32, height: u32) -> windows_core::Result<IDWriteBitmapRenderTarget>;
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
impl IDWriteGdiInterop_Vtbl {
    pub const fn new<Identity: IDWriteGdiInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFontFromLOGFONT<Identity: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfont: *const super::wingdi::LOGFONTW, font: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteGdiInterop_Impl::CreateFontFromLOGFONT(this, core::mem::transmute_copy(&logfont)) {
                    Ok(ok__) => {
                        font.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConvertFontToLOGFONT<Identity: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, font: *mut core::ffi::c_void, logfont: *mut super::wingdi::LOGFONTW, issystemfont: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteGdiInterop_Impl::ConvertFontToLOGFONT(this, core::mem::transmute_copy(&font), core::mem::transmute_copy(&logfont), core::mem::transmute_copy(&issystemfont)).into()
            }
        }
        unsafe extern "system" fn ConvertFontFaceToLOGFONT<Identity: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, font: *mut core::ffi::c_void, logfont: *mut super::wingdi::LOGFONTW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteGdiInterop_Impl::ConvertFontFaceToLOGFONT(this, core::mem::transmute_copy(&font), core::mem::transmute_copy(&logfont)).into()
            }
        }
        unsafe extern "system" fn CreateFontFaceFromHdc<Identity: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, fontface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteGdiInterop_Impl::CreateFontFaceFromHdc(this, core::mem::transmute_copy(&hdc)) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapRenderTarget<Identity: IDWriteGdiInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::windef::HDC, width: u32, height: u32, rendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteGdiInterop_Impl::CreateBitmapRenderTarget(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)) {
                    Ok(ok__) => {
                        rendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateFontFromLOGFONT: CreateFontFromLOGFONT::<Identity, OFFSET>,
            ConvertFontToLOGFONT: ConvertFontToLOGFONT::<Identity, OFFSET>,
            ConvertFontFaceToLOGFONT: ConvertFontFaceToLOGFONT::<Identity, OFFSET>,
            CreateFontFaceFromHdc: CreateFontFaceFromHdc::<Identity, OFFSET>,
            CreateBitmapRenderTarget: CreateBitmapRenderTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteGdiInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_wingdi"))]
impl windows_core::RuntimeName for IDWriteGdiInterop {}
windows_core::imp::define_interface!(IDWriteGlyphRunAnalysis, IDWriteGlyphRunAnalysis_Vtbl, 0x7d97dbf7_e085_42d4_81e3_6a883bded118);
windows_core::imp::interface_hierarchy!(IDWriteGlyphRunAnalysis, windows_core::IUnknown);
impl IDWriteGlyphRunAnalysis {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetAlphaTextureBounds(&self, texturetype: DWRITE_TEXTURE_TYPE) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlphaTextureBounds)(windows_core::Interface::as_raw(self), texturetype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn CreateAlphaTexture(&self, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::windef::RECT, alphavalues: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateAlphaTexture)(windows_core::Interface::as_raw(self), texturetype, texturebounds, core::mem::transmute(alphavalues.as_ptr()), alphavalues.len().try_into().unwrap()) }
    }
    pub unsafe fn GetAlphaBlendParams<P0>(&self, renderingparams: P0, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteRenderingParams>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAlphaBlendParams)(windows_core::Interface::as_raw(self), renderingparams.param().abi(), blendgamma as _, blendenhancedcontrast as _, blendcleartypelevel as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteGlyphRunAnalysis_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub GetAlphaTextureBounds: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXTURE_TYPE, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetAlphaTextureBounds: usize,
    #[cfg(feature = "Win32_windef")]
    pub CreateAlphaTexture: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXTURE_TYPE, *const super::windef::RECT, *mut u8, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    CreateAlphaTexture: usize,
    pub GetAlphaBlendParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait IDWriteGlyphRunAnalysis_Impl: windows_core::IUnknownImpl {
    fn GetAlphaTextureBounds(&self, texturetype: DWRITE_TEXTURE_TYPE) -> windows_core::Result<super::windef::RECT>;
    fn CreateAlphaTexture(&self, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::windef::RECT, alphavalues: *mut u8, buffersize: u32) -> windows_core::Result<()>;
    fn GetAlphaBlendParams(&self, renderingparams: windows_core::Ref<IDWriteRenderingParams>, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl IDWriteGlyphRunAnalysis_Vtbl {
    pub const fn new<Identity: IDWriteGlyphRunAnalysis_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAlphaTextureBounds<Identity: IDWriteGlyphRunAnalysis_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteGlyphRunAnalysis_Impl::GetAlphaTextureBounds(this, core::mem::transmute_copy(&texturetype)) {
                    Ok(ok__) => {
                        texturebounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAlphaTexture<Identity: IDWriteGlyphRunAnalysis_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::windef::RECT, alphavalues: *mut u8, buffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteGlyphRunAnalysis_Impl::CreateAlphaTexture(this, core::mem::transmute_copy(&texturetype), core::mem::transmute_copy(&texturebounds), core::mem::transmute_copy(&alphavalues), core::mem::transmute_copy(&buffersize)).into()
            }
        }
        unsafe extern "system" fn GetAlphaBlendParams<Identity: IDWriteGlyphRunAnalysis_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingparams: *mut core::ffi::c_void, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteGlyphRunAnalysis_Impl::GetAlphaBlendParams(this, core::mem::transmute_copy(&renderingparams), core::mem::transmute_copy(&blendgamma), core::mem::transmute_copy(&blendenhancedcontrast), core::mem::transmute_copy(&blendcleartypelevel)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAlphaTextureBounds: GetAlphaTextureBounds::<Identity, OFFSET>,
            CreateAlphaTexture: CreateAlphaTexture::<Identity, OFFSET>,
            GetAlphaBlendParams: GetAlphaBlendParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteGlyphRunAnalysis as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for IDWriteGlyphRunAnalysis {}
windows_core::imp::define_interface!(IDWriteInlineObject, IDWriteInlineObject_Vtbl, 0x8339fde3_106f_47ab_8373_1c6295eb10b3);
windows_core::imp::interface_hierarchy!(IDWriteInlineObject, windows_core::IUnknown);
impl IDWriteInlineObject {
    pub unsafe fn Draw<P1, P6>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, renderer: P1, originx: f32, originy: f32, issideways: bool, isrighttoleft: bool, clientdrawingeffect: P6) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDWriteTextRenderer>,
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, renderer.param().abi(), originx, originy, issideways.into(), isrighttoleft.into(), clientdrawingeffect.param().abi()) }
    }
    pub unsafe fn GetMetrics(&self) -> windows_core::Result<DWRITE_INLINE_OBJECT_METRICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverhangMetrics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBreakConditions(&self, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBreakConditions)(windows_core::Interface::as_raw(self), breakconditionbefore as _, breakconditionafter as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteInlineObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void, f32, f32, windows_core::BOOL, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_INLINE_OBJECT_METRICS) -> windows_core::HRESULT,
    pub GetOverhangMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_OVERHANG_METRICS) -> windows_core::HRESULT,
    pub GetBreakConditions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_BREAK_CONDITION, *mut DWRITE_BREAK_CONDITION) -> windows_core::HRESULT,
}
pub trait IDWriteInlineObject_Impl: windows_core::IUnknownImpl {
    fn Draw(&self, clientdrawingcontext: *const core::ffi::c_void, renderer: windows_core::Ref<IDWriteTextRenderer>, originx: f32, originy: f32, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetMetrics(&self) -> windows_core::Result<DWRITE_INLINE_OBJECT_METRICS>;
    fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetBreakConditions(&self, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> windows_core::Result<()>;
}
impl IDWriteInlineObject_Vtbl {
    pub const fn new<Identity: IDWriteInlineObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Draw<Identity: IDWriteInlineObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, renderer: *mut core::ffi::c_void, originx: f32, originy: f32, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteInlineObject_Impl::Draw(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&renderer), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft), core::mem::transmute_copy(&clientdrawingeffect)).into()
            }
        }
        unsafe extern "system" fn GetMetrics<Identity: IDWriteInlineObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteInlineObject_Impl::GetMetrics(this) {
                    Ok(ok__) => {
                        metrics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOverhangMetrics<Identity: IDWriteInlineObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteInlineObject_Impl::GetOverhangMetrics(this) {
                    Ok(ok__) => {
                        overhangs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBreakConditions<Identity: IDWriteInlineObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteInlineObject_Impl::GetBreakConditions(this, core::mem::transmute_copy(&breakconditionbefore), core::mem::transmute_copy(&breakconditionafter)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Draw: Draw::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, OFFSET>,
            GetBreakConditions: GetBreakConditions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteInlineObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteInlineObject {}
windows_core::imp::define_interface!(IDWriteLocalFontFileLoader, IDWriteLocalFontFileLoader_Vtbl, 0xb2d9f3ec_c9fe_4a11_a2ec_d86208f7c0a2);
impl core::ops::Deref for IDWriteLocalFontFileLoader {
    type Target = IDWriteFontFileLoader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteLocalFontFileLoader, windows_core::IUnknown, IDWriteFontFileLoader);
impl IDWriteLocalFontFileLoader {
    pub unsafe fn GetFilePathLengthFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilePathLengthFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFilePathFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, filepath: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFilePathFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap()) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetLastWriteTimeFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<super::minwindef::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastWriteTimeFromKey)(windows_core::Interface::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteLocalFontFileLoader_Vtbl {
    pub base__: IDWriteFontFileLoader_Vtbl,
    pub GetFilePathLengthFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetFilePathFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u16, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetLastWriteTimeFromKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetLastWriteTimeFromKey: usize,
}
#[cfg(feature = "Win32_minwindef")]
pub trait IDWriteLocalFontFileLoader_Impl: IDWriteFontFileLoader_Impl {
    fn GetFilePathLengthFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<u32>;
    fn GetFilePathFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, filepath: *mut u16, filepathsize: u32) -> windows_core::Result<()>;
    fn GetLastWriteTimeFromKey(&self, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32) -> windows_core::Result<super::minwindef::FILETIME>;
}
#[cfg(feature = "Win32_minwindef")]
impl IDWriteLocalFontFileLoader_Vtbl {
    pub const fn new<Identity: IDWriteLocalFontFileLoader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFilePathLengthFromKey<Identity: IDWriteLocalFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, filepathlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteLocalFontFileLoader_Impl::GetFilePathLengthFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                    Ok(ok__) => {
                        filepathlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilePathFromKey<Identity: IDWriteLocalFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, filepath: *mut u16, filepathsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteLocalFontFileLoader_Impl::GetFilePathFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize), core::mem::transmute_copy(&filepath), core::mem::transmute_copy(&filepathsize)).into()
            }
        }
        unsafe extern "system" fn GetLastWriteTimeFromKey<Identity: IDWriteLocalFontFileLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfilereferencekey: *const core::ffi::c_void, fontfilereferencekeysize: u32, lastwritetime: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteLocalFontFileLoader_Impl::GetLastWriteTimeFromKey(this, core::mem::transmute_copy(&fontfilereferencekey), core::mem::transmute_copy(&fontfilereferencekeysize)) {
                    Ok(ok__) => {
                        lastwritetime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDWriteFontFileLoader_Vtbl::new::<Identity, OFFSET>(),
            GetFilePathLengthFromKey: GetFilePathLengthFromKey::<Identity, OFFSET>,
            GetFilePathFromKey: GetFilePathFromKey::<Identity, OFFSET>,
            GetLastWriteTimeFromKey: GetLastWriteTimeFromKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteLocalFontFileLoader as windows_core::Interface>::IID || iid == &<IDWriteFontFileLoader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_minwindef")]
impl windows_core::RuntimeName for IDWriteLocalFontFileLoader {}
windows_core::imp::define_interface!(IDWriteLocalizedStrings, IDWriteLocalizedStrings_Vtbl, 0x08256209_099a_4b34_b86d_c22b110e7771);
windows_core::imp::interface_hierarchy!(IDWriteLocalizedStrings, windows_core::IUnknown);
impl IDWriteLocalizedStrings {
    pub unsafe fn GetCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FindLocaleName<P0>(&self, localename: P0, index: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindLocaleName)(windows_core::Interface::as_raw(self), localename.param().abi(), index as _, exists as _) }
    }
    pub unsafe fn GetLocaleNameLength(&self, index: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocaleName(&self, index: u32, localename: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), index, core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap()) }
    }
    pub unsafe fn GetStringLength(&self, index: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetString(&self, index: u32, stringbuffer: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), index, core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteLocalizedStrings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub FindLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, u32) -> windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, u32) -> windows_core::HRESULT,
}
pub trait IDWriteLocalizedStrings_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> u32;
    fn FindLocaleName(&self, localename: &windows_core::PCWSTR, index: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLocaleNameLength(&self, index: u32) -> windows_core::Result<u32>;
    fn GetLocaleName(&self, index: u32, localename: *mut u16, size: u32) -> windows_core::Result<()>;
    fn GetStringLength(&self, index: u32) -> windows_core::Result<u32>;
    fn GetString(&self, index: u32, stringbuffer: *mut u16, size: u32) -> windows_core::Result<()>;
}
impl IDWriteLocalizedStrings_Vtbl {
    pub const fn new<Identity: IDWriteLocalizedStrings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteLocalizedStrings_Impl::GetCount(this)
            }
        }
        unsafe extern "system" fn FindLocaleName<Identity: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localename: windows_core::PCWSTR, index: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteLocalizedStrings_Impl::FindLocaleName(this, core::mem::transmute(&localename), core::mem::transmute_copy(&index), core::mem::transmute_copy(&exists)).into()
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, length: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteLocalizedStrings_Impl::GetLocaleNameLength(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, localename: *mut u16, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteLocalizedStrings_Impl::GetLocaleName(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&localename), core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn GetStringLength<Identity: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, length: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteLocalizedStrings_Impl::GetStringLength(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: IDWriteLocalizedStrings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, stringbuffer: *mut u16, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteLocalizedStrings_Impl::GetString(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&stringbuffer), core::mem::transmute_copy(&size)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            FindLocaleName: FindLocaleName::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteLocalizedStrings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteLocalizedStrings {}
windows_core::imp::define_interface!(IDWriteNumberSubstitution, IDWriteNumberSubstitution_Vtbl, 0x14885cc9_bab0_4f90_b6ed_5c366a2cd03d);
windows_core::imp::interface_hierarchy!(IDWriteNumberSubstitution, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteNumberSubstitution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDWriteNumberSubstitution_Impl: windows_core::IUnknownImpl {}
impl IDWriteNumberSubstitution_Vtbl {
    pub const fn new<Identity: IDWriteNumberSubstitution_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteNumberSubstitution as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteNumberSubstitution {}
windows_core::imp::define_interface!(IDWritePixelSnapping, IDWritePixelSnapping_Vtbl, 0xeaf3a2da_ecf4_4d24_b644_b34f6842024b);
windows_core::imp::interface_hierarchy!(IDWritePixelSnapping, windows_core::IUnknown);
impl IDWritePixelSnapping {
    pub unsafe fn IsPixelSnappingDisabled(&self, clientdrawingcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPixelSnappingDisabled)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentTransform(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, transform: *mut DWRITE_MATRIX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentTransform)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, transform as _) }
    }
    pub unsafe fn GetPixelsPerDip(&self, clientdrawingcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelsPerDip)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWritePixelSnapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsPixelSnappingDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut DWRITE_MATRIX) -> windows_core::HRESULT,
    pub GetPixelsPerDip: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
pub trait IDWritePixelSnapping_Impl: windows_core::IUnknownImpl {
    fn IsPixelSnappingDisabled(&self, clientdrawingcontext: *const core::ffi::c_void) -> windows_core::Result<windows_core::BOOL>;
    fn GetCurrentTransform(&self, clientdrawingcontext: *const core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> windows_core::Result<()>;
    fn GetPixelsPerDip(&self, clientdrawingcontext: *const core::ffi::c_void) -> windows_core::Result<f32>;
}
impl IDWritePixelSnapping_Vtbl {
    pub const fn new<Identity: IDWritePixelSnapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPixelSnappingDisabled<Identity: IDWritePixelSnapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, isdisabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWritePixelSnapping_Impl::IsPixelSnappingDisabled(this, core::mem::transmute_copy(&clientdrawingcontext)) {
                    Ok(ok__) => {
                        isdisabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentTransform<Identity: IDWritePixelSnapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWritePixelSnapping_Impl::GetCurrentTransform(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn GetPixelsPerDip<Identity: IDWritePixelSnapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, pixelsperdip: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWritePixelSnapping_Impl::GetPixelsPerDip(this, core::mem::transmute_copy(&clientdrawingcontext)) {
                    Ok(ok__) => {
                        pixelsperdip.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsPixelSnappingDisabled: IsPixelSnappingDisabled::<Identity, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWritePixelSnapping as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWritePixelSnapping {}
windows_core::imp::define_interface!(IDWriteRenderingParams, IDWriteRenderingParams_Vtbl, 0x2f0da53a_2add_47cd_82ee_d9ec34688e75);
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams, windows_core::IUnknown);
impl IDWriteRenderingParams {
    pub unsafe fn GetGamma(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetGamma)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetEnhancedContrast)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetClearTypeLevel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        unsafe { (windows_core::Interface::vtable(self).GetPixelGeometry)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetRenderingMode)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRenderingParams_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGamma: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetEnhancedContrast: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetClearTypeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetPixelGeometry: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY,
    pub GetRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_RENDERING_MODE,
}
pub trait IDWriteRenderingParams_Impl: windows_core::IUnknownImpl {
    fn GetGamma(&self) -> f32;
    fn GetEnhancedContrast(&self) -> f32;
    fn GetClearTypeLevel(&self) -> f32;
    fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY;
    fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE;
}
impl IDWriteRenderingParams_Vtbl {
    pub const fn new<Identity: IDWriteRenderingParams_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGamma<Identity: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRenderingParams_Impl::GetGamma(this)
            }
        }
        unsafe extern "system" fn GetEnhancedContrast<Identity: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRenderingParams_Impl::GetEnhancedContrast(this)
            }
        }
        unsafe extern "system" fn GetClearTypeLevel<Identity: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRenderingParams_Impl::GetClearTypeLevel(this)
            }
        }
        unsafe extern "system" fn GetPixelGeometry<Identity: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRenderingParams_Impl::GetPixelGeometry(this)
            }
        }
        unsafe extern "system" fn GetRenderingMode<Identity: IDWriteRenderingParams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_RENDERING_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRenderingParams_Impl::GetRenderingMode(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGamma: GetGamma::<Identity, OFFSET>,
            GetEnhancedContrast: GetEnhancedContrast::<Identity, OFFSET>,
            GetClearTypeLevel: GetClearTypeLevel::<Identity, OFFSET>,
            GetPixelGeometry: GetPixelGeometry::<Identity, OFFSET>,
            GetRenderingMode: GetRenderingMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRenderingParams as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteRenderingParams {}
windows_core::imp::define_interface!(IDWriteTextAnalysisSink, IDWriteTextAnalysisSink_Vtbl, 0x5810cd44_0ca0_4701_b3fa_bec5182ae4f6);
windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSink, windows_core::IUnknown);
impl IDWriteTextAnalysisSink {
    pub unsafe fn SetScriptAnalysis(&self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScriptAnalysis)(windows_core::Interface::as_raw(self), textposition, textlength, scriptanalysis) }
    }
    pub unsafe fn SetLineBreakpoints(&self, textposition: u32, linebreakpoints: &[DWRITE_LINE_BREAKPOINT]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLineBreakpoints)(windows_core::Interface::as_raw(self), textposition, linebreakpoints.len().try_into().unwrap(), core::mem::transmute(linebreakpoints.as_ptr())) }
    }
    pub unsafe fn SetBidiLevel(&self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBidiLevel)(windows_core::Interface::as_raw(self), textposition, textlength, explicitlevel, resolvedlevel) }
    }
    pub unsafe fn SetNumberSubstitution<P2>(&self, textposition: u32, textlength: u32, numbersubstitution: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IDWriteNumberSubstitution>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNumberSubstitution)(windows_core::Interface::as_raw(self), textposition, textlength, numbersubstitution.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalysisSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetScriptAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const DWRITE_SCRIPT_ANALYSIS) -> windows_core::HRESULT,
    pub SetLineBreakpoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const DWRITE_LINE_BREAKPOINT) -> windows_core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u8, u8) -> windows_core::HRESULT,
    pub SetNumberSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteTextAnalysisSink_Impl: windows_core::IUnknownImpl {
    fn SetScriptAnalysis(&self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> windows_core::Result<()>;
    fn SetLineBreakpoints(&self, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> windows_core::Result<()>;
    fn SetBidiLevel(&self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> windows_core::Result<()>;
    fn SetNumberSubstitution(&self, textposition: u32, textlength: u32, numbersubstitution: windows_core::Ref<IDWriteNumberSubstitution>) -> windows_core::Result<()>;
}
impl IDWriteTextAnalysisSink_Vtbl {
    pub const fn new<Identity: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetScriptAnalysis<Identity: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSink_Impl::SetScriptAnalysis(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&scriptanalysis)).into()
            }
        }
        unsafe extern "system" fn SetLineBreakpoints<Identity: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSink_Impl::SetLineBreakpoints(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&linebreakpoints)).into()
            }
        }
        unsafe extern "system" fn SetBidiLevel<Identity: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSink_Impl::SetBidiLevel(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&explicitlevel), core::mem::transmute_copy(&resolvedlevel)).into()
            }
        }
        unsafe extern "system" fn SetNumberSubstitution<Identity: IDWriteTextAnalysisSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, numbersubstitution: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSink_Impl::SetNumberSubstitution(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&numbersubstitution)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScriptAnalysis: SetScriptAnalysis::<Identity, OFFSET>,
            SetLineBreakpoints: SetLineBreakpoints::<Identity, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, OFFSET>,
            SetNumberSubstitution: SetNumberSubstitution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteTextAnalysisSink {}
windows_core::imp::define_interface!(IDWriteTextAnalysisSource, IDWriteTextAnalysisSource_Vtbl, 0x688e1a58_5094_47c8_adc8_fbcea60ae92b);
windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSource, windows_core::IUnknown);
impl IDWriteTextAnalysisSource {
    pub unsafe fn GetTextAtPosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextAtPosition)(windows_core::Interface::as_raw(self), textposition, textstring as _, textlength as _) }
    }
    pub unsafe fn GetTextBeforePosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextBeforePosition)(windows_core::Interface::as_raw(self), textposition, textstring as _, textlength as _) }
    }
    pub unsafe fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        unsafe { (windows_core::Interface::vtable(self).GetParagraphReadingDirection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLocaleName(&self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), textposition, textlength as _, localename as _) }
    }
    pub unsafe fn GetNumberSubstitution(&self, textposition: u32, textlength: *mut u32, numbersubstitution: *mut Option<IDWriteNumberSubstitution>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNumberSubstitution)(windows_core::Interface::as_raw(self), textposition, textlength as _, core::mem::transmute(numbersubstitution)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalysisSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTextAtPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetTextBeforePosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u16, *mut u32) -> windows_core::HRESULT,
    pub GetParagraphReadingDirection: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_READING_DIRECTION,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut *mut u16) -> windows_core::HRESULT,
    pub GetNumberSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDWriteTextAnalysisSource_Impl: windows_core::IUnknownImpl {
    fn GetTextAtPosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::Result<()>;
    fn GetTextBeforePosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::Result<()>;
    fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION;
    fn GetLocaleName(&self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> windows_core::Result<()>;
    fn GetNumberSubstitution(&self, textposition: u32, textlength: *mut u32, numbersubstitution: windows_core::OutRef<IDWriteNumberSubstitution>) -> windows_core::Result<()>;
}
impl IDWriteTextAnalysisSource_Vtbl {
    pub const fn new<Identity: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTextAtPosition<Identity: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSource_Impl::GetTextAtPosition(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textstring), core::mem::transmute_copy(&textlength)).into()
            }
        }
        unsafe extern "system" fn GetTextBeforePosition<Identity: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSource_Impl::GetTextBeforePosition(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textstring), core::mem::transmute_copy(&textlength)).into()
            }
        }
        unsafe extern "system" fn GetParagraphReadingDirection<Identity: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSource_Impl::GetParagraphReadingDirection(this)
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSource_Impl::GetLocaleName(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&localename)).into()
            }
        }
        unsafe extern "system" fn GetNumberSubstitution<Identity: IDWriteTextAnalysisSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: *mut u32, numbersubstitution: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSource_Impl::GetNumberSubstitution(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&numbersubstitution)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTextAtPosition: GetTextAtPosition::<Identity, OFFSET>,
            GetTextBeforePosition: GetTextBeforePosition::<Identity, OFFSET>,
            GetParagraphReadingDirection: GetParagraphReadingDirection::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            GetNumberSubstitution: GetNumberSubstitution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteTextAnalysisSource {}
windows_core::imp::define_interface!(IDWriteTextAnalyzer, IDWriteTextAnalyzer_Vtbl, 0xb7e6163e_7f46_43b4_84b3_e4e6249c365d);
windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer, windows_core::IUnknown);
impl IDWriteTextAnalyzer {
    pub unsafe fn AnalyzeScript<P0, P3>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P3: windows_core::Param<IDWriteTextAnalysisSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeScript)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()) }
    }
    pub unsafe fn AnalyzeBidi<P0, P3>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P3: windows_core::Param<IDWriteTextAnalysisSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeBidi)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()) }
    }
    pub unsafe fn AnalyzeNumberSubstitution<P0, P3>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P3: windows_core::Param<IDWriteTextAnalysisSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeNumberSubstitution)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()) }
    }
    pub unsafe fn AnalyzeLineBreakpoints<P0, P3>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource>,
        P3: windows_core::Param<IDWriteTextAnalysisSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeLineBreakpoints)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()) }
    }
    pub unsafe fn GetGlyphs<P2, P6, P7>(&self, textstring: *const u16, textlength: u32, fontface: P2, issideways: bool, isrighttoleft: bool, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P6, numbersubstitution: P7, features: Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: Option<*const u32>, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IDWriteFontFace>,
        P6: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<IDWriteNumberSubstitution>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphs)(windows_core::Interface::as_raw(self), textstring, textlength, fontface.param().abi(), issideways.into(), isrighttoleft.into(), scriptanalysis, localename.param().abi(), numbersubstitution.param().abi(), features.unwrap_or(core::mem::zeroed()) as _, featurerangelengths.unwrap_or(core::mem::zeroed()) as _, featureranges, maxglyphcount, clustermap as _, textprops as _, glyphindices as _, glyphprops as _, actualglyphcount as _) }
    }
    pub unsafe fn GetGlyphPlacements<P7, P12>(&self, textstring: *const u16, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P7, fontemsize: f32, issideways: bool, isrighttoleft: bool, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P12, features: Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT
    where
        P7: windows_core::Param<IDWriteFontFace>,
        P12: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphPlacements)(windows_core::Interface::as_raw(self), textstring, clustermap, textprops as _, textlength, glyphindices, glyphprops, glyphcount, fontface.param().abi(), fontemsize, issideways.into(), isrighttoleft.into(), scriptanalysis, localename.param().abi(), features.unwrap_or(core::mem::zeroed()) as _, featurerangelengths.unwrap_or(core::mem::zeroed()) as _, featureranges, glyphadvances as _, glyphoffsets as _) }
    }
    pub unsafe fn GetGdiCompatibleGlyphPlacements<P7, P15>(&self, textstring: *const u16, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P7, fontemsize: f32, pixelsperdip: f32, transform: Option<*const DWRITE_MATRIX>, usegdinatural: bool, issideways: bool, isrighttoleft: bool, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P15, features: Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT
    where
        P7: windows_core::Param<IDWriteFontFace>,
        P15: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetGdiCompatibleGlyphPlacements)(
                windows_core::Interface::as_raw(self),
                textstring,
                clustermap,
                textprops,
                textlength,
                glyphindices,
                glyphprops,
                glyphcount,
                fontface.param().abi(),
                fontemsize,
                pixelsperdip,
                transform.unwrap_or(core::mem::zeroed()) as _,
                usegdinatural.into(),
                issideways.into(),
                isrighttoleft.into(),
                scriptanalysis,
                localename.param().abi(),
                features.unwrap_or(core::mem::zeroed()) as _,
                featurerangelengths.unwrap_or(core::mem::zeroed()) as _,
                featureranges,
                glyphadvances as _,
                glyphoffsets as _,
            )
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalyzer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AnalyzeScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AnalyzeBidi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AnalyzeNumberSubstitution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AnalyzeLineBreakpoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGlyphs: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *const DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, *mut core::ffi::c_void, *const *const DWRITE_TYPOGRAPHIC_FEATURES, *const u32, u32, u32, *mut u16, *mut DWRITE_SHAPING_TEXT_PROPERTIES, *mut u16, *mut DWRITE_SHAPING_GLYPH_PROPERTIES, *mut u32) -> windows_core::HRESULT,
    pub GetGlyphPlacements: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, *mut DWRITE_SHAPING_TEXT_PROPERTIES, u32, *const u16, *const DWRITE_SHAPING_GLYPH_PROPERTIES, u32, *mut core::ffi::c_void, f32, windows_core::BOOL, windows_core::BOOL, *const DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, *const *const DWRITE_TYPOGRAPHIC_FEATURES, *const u32, u32, *mut f32, *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
    pub GetGdiCompatibleGlyphPlacements: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, *const DWRITE_SHAPING_TEXT_PROPERTIES, u32, *const u16, *const DWRITE_SHAPING_GLYPH_PROPERTIES, u32, *mut core::ffi::c_void, f32, f32, *const DWRITE_MATRIX, windows_core::BOOL, windows_core::BOOL, windows_core::BOOL, *const DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, *const *const DWRITE_TYPOGRAPHIC_FEATURES, *const u32, u32, *mut f32, *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
}
pub trait IDWriteTextAnalyzer_Impl: windows_core::IUnknownImpl {
    fn AnalyzeScript(&self, analysissource: windows_core::Ref<IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: windows_core::Ref<IDWriteTextAnalysisSink>) -> windows_core::Result<()>;
    fn AnalyzeBidi(&self, analysissource: windows_core::Ref<IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: windows_core::Ref<IDWriteTextAnalysisSink>) -> windows_core::Result<()>;
    fn AnalyzeNumberSubstitution(&self, analysissource: windows_core::Ref<IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: windows_core::Ref<IDWriteTextAnalysisSink>) -> windows_core::Result<()>;
    fn AnalyzeLineBreakpoints(&self, analysissource: windows_core::Ref<IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: windows_core::Ref<IDWriteTextAnalysisSink>) -> windows_core::Result<()>;
    fn GetGlyphs(&self, textstring: *const u16, textlength: u32, fontface: windows_core::Ref<IDWriteFontFace>, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, numbersubstitution: windows_core::Ref<IDWriteNumberSubstitution>, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> windows_core::Result<()>;
    fn GetGlyphPlacements(&self, textstring: *const u16, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: windows_core::Ref<IDWriteFontFace>, fontemsize: f32, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
    fn GetGdiCompatibleGlyphPlacements(&self, textstring: *const u16, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: windows_core::Ref<IDWriteFontFace>, fontemsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: windows_core::BOOL, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
}
impl IDWriteTextAnalyzer_Vtbl {
    pub const fn new<Identity: IDWriteTextAnalyzer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AnalyzeScript<Identity: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer_Impl::AnalyzeScript(this, core::mem::transmute_copy(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&analysissink)).into()
            }
        }
        unsafe extern "system" fn AnalyzeBidi<Identity: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer_Impl::AnalyzeBidi(this, core::mem::transmute_copy(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&analysissink)).into()
            }
        }
        unsafe extern "system" fn AnalyzeNumberSubstitution<Identity: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer_Impl::AnalyzeNumberSubstitution(this, core::mem::transmute_copy(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&analysissink)).into()
            }
        }
        unsafe extern "system" fn AnalyzeLineBreakpoints<Identity: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer_Impl::AnalyzeLineBreakpoints(this, core::mem::transmute_copy(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&analysissink)).into()
            }
        }
        unsafe extern "system" fn GetGlyphs<Identity: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textstring: *const u16, textlength: u32, fontface: *mut core::ffi::c_void, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, numbersubstitution: *mut core::ffi::c_void, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer_Impl::GetGlyphs(
                    this,
                    core::mem::transmute_copy(&textstring),
                    core::mem::transmute_copy(&textlength),
                    core::mem::transmute_copy(&fontface),
                    core::mem::transmute_copy(&issideways),
                    core::mem::transmute_copy(&isrighttoleft),
                    core::mem::transmute_copy(&scriptanalysis),
                    core::mem::transmute(&localename),
                    core::mem::transmute_copy(&numbersubstitution),
                    core::mem::transmute_copy(&features),
                    core::mem::transmute_copy(&featurerangelengths),
                    core::mem::transmute_copy(&featureranges),
                    core::mem::transmute_copy(&maxglyphcount),
                    core::mem::transmute_copy(&clustermap),
                    core::mem::transmute_copy(&textprops),
                    core::mem::transmute_copy(&glyphindices),
                    core::mem::transmute_copy(&glyphprops),
                    core::mem::transmute_copy(&actualglyphcount),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetGlyphPlacements<Identity: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textstring: *const u16, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: *mut core::ffi::c_void, fontemsize: f32, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer_Impl::GetGlyphPlacements(
                    this,
                    core::mem::transmute_copy(&textstring),
                    core::mem::transmute_copy(&clustermap),
                    core::mem::transmute_copy(&textprops),
                    core::mem::transmute_copy(&textlength),
                    core::mem::transmute_copy(&glyphindices),
                    core::mem::transmute_copy(&glyphprops),
                    core::mem::transmute_copy(&glyphcount),
                    core::mem::transmute_copy(&fontface),
                    core::mem::transmute_copy(&fontemsize),
                    core::mem::transmute_copy(&issideways),
                    core::mem::transmute_copy(&isrighttoleft),
                    core::mem::transmute_copy(&scriptanalysis),
                    core::mem::transmute(&localename),
                    core::mem::transmute_copy(&features),
                    core::mem::transmute_copy(&featurerangelengths),
                    core::mem::transmute_copy(&featureranges),
                    core::mem::transmute_copy(&glyphadvances),
                    core::mem::transmute_copy(&glyphoffsets),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphPlacements<Identity: IDWriteTextAnalyzer_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            textstring: *const u16,
            clustermap: *const u16,
            textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
            textlength: u32,
            glyphindices: *const u16,
            glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
            glyphcount: u32,
            fontface: *mut core::ffi::c_void,
            fontemsize: f32,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            usegdinatural: windows_core::BOOL,
            issideways: windows_core::BOOL,
            isrighttoleft: windows_core::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: windows_core::PCWSTR,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            glyphadvances: *mut f32,
            glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer_Impl::GetGdiCompatibleGlyphPlacements(
                    this,
                    core::mem::transmute_copy(&textstring),
                    core::mem::transmute_copy(&clustermap),
                    core::mem::transmute_copy(&textprops),
                    core::mem::transmute_copy(&textlength),
                    core::mem::transmute_copy(&glyphindices),
                    core::mem::transmute_copy(&glyphprops),
                    core::mem::transmute_copy(&glyphcount),
                    core::mem::transmute_copy(&fontface),
                    core::mem::transmute_copy(&fontemsize),
                    core::mem::transmute_copy(&pixelsperdip),
                    core::mem::transmute_copy(&transform),
                    core::mem::transmute_copy(&usegdinatural),
                    core::mem::transmute_copy(&issideways),
                    core::mem::transmute_copy(&isrighttoleft),
                    core::mem::transmute_copy(&scriptanalysis),
                    core::mem::transmute(&localename),
                    core::mem::transmute_copy(&features),
                    core::mem::transmute_copy(&featurerangelengths),
                    core::mem::transmute_copy(&featureranges),
                    core::mem::transmute_copy(&glyphadvances),
                    core::mem::transmute_copy(&glyphoffsets),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AnalyzeScript: AnalyzeScript::<Identity, OFFSET>,
            AnalyzeBidi: AnalyzeBidi::<Identity, OFFSET>,
            AnalyzeNumberSubstitution: AnalyzeNumberSubstitution::<Identity, OFFSET>,
            AnalyzeLineBreakpoints: AnalyzeLineBreakpoints::<Identity, OFFSET>,
            GetGlyphs: GetGlyphs::<Identity, OFFSET>,
            GetGlyphPlacements: GetGlyphPlacements::<Identity, OFFSET>,
            GetGdiCompatibleGlyphPlacements: GetGdiCompatibleGlyphPlacements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteTextAnalyzer {}
windows_core::imp::define_interface!(IDWriteTextFormat, IDWriteTextFormat_Vtbl, 0x9c906818_31d7_4fd3_a151_7c5e225db55a);
windows_core::imp::interface_hierarchy!(IDWriteTextFormat, windows_core::IUnknown);
impl IDWriteTextFormat {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextAlignment)(windows_core::Interface::as_raw(self), textalignment) }
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParagraphAlignment)(windows_core::Interface::as_raw(self), paragraphalignment) }
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWordWrapping)(windows_core::Interface::as_raw(self), wordwrapping) }
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReadingDirection)(windows_core::Interface::as_raw(self), readingdirection) }
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlowDirection)(windows_core::Interface::as_raw(self), flowdirection) }
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIncrementalTabStop)(windows_core::Interface::as_raw(self), incrementaltabstop) }
    }
    pub unsafe fn SetTrimming<P1>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDWriteInlineObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTrimming)(windows_core::Interface::as_raw(self), trimmingoptions, trimmingsign.param().abi()) }
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLineSpacing)(windows_core::Interface::as_raw(self), linespacingmethod, linespacing, baseline) }
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        unsafe { (windows_core::Interface::vtable(self).GetTextAlignment)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        unsafe { (windows_core::Interface::vtable(self).GetParagraphAlignment)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        unsafe { (windows_core::Interface::vtable(self).GetWordWrapping)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        unsafe { (windows_core::Interface::vtable(self).GetReadingDirection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        unsafe { (windows_core::Interface::vtable(self).GetFlowDirection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetIncrementalTabStop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut Option<IDWriteInlineObject>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTrimming)(windows_core::Interface::as_raw(self), trimmingoptions as _, core::mem::transmute(trimmingsign)) }
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLineSpacing)(windows_core::Interface::as_raw(self), linespacingmethod as _, linespacing as _, baseline as _) }
    }
    pub unsafe fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyNameLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyName)(windows_core::Interface::as_raw(self), core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap()) }
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        unsafe { (windows_core::Interface::vtable(self).GetFontWeight)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        unsafe { (windows_core::Interface::vtable(self).GetFontStyle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        unsafe { (windows_core::Interface::vtable(self).GetFontStretch)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTextAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXT_ALIGNMENT) -> windows_core::HRESULT,
    pub SetParagraphAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_WORD_WRAPPING) -> windows_core::HRESULT,
    pub SetReadingDirection: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_READING_DIRECTION) -> windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FLOW_DIRECTION) -> windows_core::HRESULT,
    pub SetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTrimming: unsafe extern "system" fn(*mut core::ffi::c_void, *const DWRITE_TRIMMING, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_LINE_SPACING_METHOD, f32, f32) -> windows_core::HRESULT,
    pub GetTextAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT,
    pub GetParagraphAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT,
    pub GetWordWrapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_WORD_WRAPPING,
    pub GetReadingDirection: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_READING_DIRECTION,
    pub GetFlowDirection: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FLOW_DIRECTION,
    pub GetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetTrimming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_TRIMMING, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_SPACING_METHOD, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32) -> windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub GetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32) -> windows_core::HRESULT,
}
pub trait IDWriteTextFormat_Impl: windows_core::IUnknownImpl {
    fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> windows_core::Result<()>;
    fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::Result<()>;
    fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> windows_core::Result<()>;
    fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> windows_core::Result<()>;
    fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> windows_core::Result<()>;
    fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> windows_core::Result<()>;
    fn SetTrimming(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: windows_core::Ref<IDWriteInlineObject>) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> windows_core::Result<()>;
    fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT;
    fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT;
    fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING;
    fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION;
    fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION;
    fn GetIncrementalTabStop(&self) -> f32;
    fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: windows_core::OutRef<IDWriteInlineObject>) -> windows_core::Result<()>;
    fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> windows_core::Result<()>;
    fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection>;
    fn GetFontFamilyNameLength(&self) -> u32;
    fn GetFontFamilyName(&self, fontfamilyname: *mut u16, namesize: u32) -> windows_core::Result<()>;
    fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetFontStyle(&self) -> DWRITE_FONT_STYLE;
    fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetFontSize(&self) -> f32;
    fn GetLocaleNameLength(&self) -> u32;
    fn GetLocaleName(&self, localename: *mut u16, namesize: u32) -> windows_core::Result<()>;
}
impl IDWriteTextFormat_Vtbl {
    pub const fn new<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTextAlignment<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textalignment: DWRITE_TEXT_ALIGNMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetTextAlignment(this, core::mem::transmute_copy(&textalignment)).into()
            }
        }
        unsafe extern "system" fn SetParagraphAlignment<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetParagraphAlignment(this, core::mem::transmute_copy(&paragraphalignment)).into()
            }
        }
        unsafe extern "system" fn SetWordWrapping<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wordwrapping: DWRITE_WORD_WRAPPING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetWordWrapping(this, core::mem::transmute_copy(&wordwrapping)).into()
            }
        }
        unsafe extern "system" fn SetReadingDirection<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readingdirection: DWRITE_READING_DIRECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetReadingDirection(this, core::mem::transmute_copy(&readingdirection)).into()
            }
        }
        unsafe extern "system" fn SetFlowDirection<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flowdirection: DWRITE_FLOW_DIRECTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetFlowDirection(this, core::mem::transmute_copy(&flowdirection)).into()
            }
        }
        unsafe extern "system" fn SetIncrementalTabStop<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, incrementaltabstop: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetIncrementalTabStop(this, core::mem::transmute_copy(&incrementaltabstop)).into()
            }
        }
        unsafe extern "system" fn SetTrimming<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetTrimming(this, core::mem::transmute_copy(&trimmingoptions), core::mem::transmute_copy(&trimmingsign)).into()
            }
        }
        unsafe extern "system" fn SetLineSpacing<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetLineSpacing(this, core::mem::transmute_copy(&linespacingmethod), core::mem::transmute_copy(&linespacing), core::mem::transmute_copy(&baseline)).into()
            }
        }
        unsafe extern "system" fn GetTextAlignment<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetTextAlignment(this)
            }
        }
        unsafe extern "system" fn GetParagraphAlignment<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetParagraphAlignment(this)
            }
        }
        unsafe extern "system" fn GetWordWrapping<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_WORD_WRAPPING {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetWordWrapping(this)
            }
        }
        unsafe extern "system" fn GetReadingDirection<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetReadingDirection(this)
            }
        }
        unsafe extern "system" fn GetFlowDirection<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FLOW_DIRECTION {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFlowDirection(this)
            }
        }
        unsafe extern "system" fn GetIncrementalTabStop<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetIncrementalTabStop(this)
            }
        }
        unsafe extern "system" fn GetTrimming<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetTrimming(this, core::mem::transmute_copy(&trimmingoptions), core::mem::transmute_copy(&trimmingsign)).into()
            }
        }
        unsafe extern "system" fn GetLineSpacing<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetLineSpacing(this, core::mem::transmute_copy(&linespacingmethod), core::mem::transmute_copy(&linespacing), core::mem::transmute_copy(&baseline)).into()
            }
        }
        unsafe extern "system" fn GetFontCollection<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTextFormat_Impl::GetFontCollection(this) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontFamilyNameLength(this)
            }
        }
        unsafe extern "system" fn GetFontFamilyName<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilyname: *mut u16, namesize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontFamilyName(this, core::mem::transmute_copy(&fontfamilyname), core::mem::transmute_copy(&namesize)).into()
            }
        }
        unsafe extern "system" fn GetFontWeight<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontWeight(this)
            }
        }
        unsafe extern "system" fn GetFontStyle<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STYLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontStyle(this)
            }
        }
        unsafe extern "system" fn GetFontStretch<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontStretch(this)
            }
        }
        unsafe extern "system" fn GetFontSize<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontSize(this)
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetLocaleNameLength(this)
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localename: *mut u16, namesize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetLocaleName(this, core::mem::transmute_copy(&localename), core::mem::transmute_copy(&namesize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTextAlignment: SetTextAlignment::<Identity, OFFSET>,
            SetParagraphAlignment: SetParagraphAlignment::<Identity, OFFSET>,
            SetWordWrapping: SetWordWrapping::<Identity, OFFSET>,
            SetReadingDirection: SetReadingDirection::<Identity, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, OFFSET>,
            SetIncrementalTabStop: SetIncrementalTabStop::<Identity, OFFSET>,
            SetTrimming: SetTrimming::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetTextAlignment: GetTextAlignment::<Identity, OFFSET>,
            GetParagraphAlignment: GetParagraphAlignment::<Identity, OFFSET>,
            GetWordWrapping: GetWordWrapping::<Identity, OFFSET>,
            GetReadingDirection: GetReadingDirection::<Identity, OFFSET>,
            GetFlowDirection: GetFlowDirection::<Identity, OFFSET>,
            GetIncrementalTabStop: GetIncrementalTabStop::<Identity, OFFSET>,
            GetTrimming: GetTrimming::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
            GetFontCollection: GetFontCollection::<Identity, OFFSET>,
            GetFontFamilyNameLength: GetFontFamilyNameLength::<Identity, OFFSET>,
            GetFontFamilyName: GetFontFamilyName::<Identity, OFFSET>,
            GetFontWeight: GetFontWeight::<Identity, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, OFFSET>,
            GetFontStretch: GetFontStretch::<Identity, OFFSET>,
            GetFontSize: GetFontSize::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteTextFormat {}
windows_core::imp::define_interface!(IDWriteTextLayout, IDWriteTextLayout_Vtbl, 0x53737037_6d14_410b_9bfe_0b182bb70961);
impl core::ops::Deref for IDWriteTextLayout {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextLayout, windows_core::IUnknown, IDWriteTextFormat);
impl IDWriteTextLayout {
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxWidth)(windows_core::Interface::as_raw(self), maxwidth) }
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxHeight)(windows_core::Interface::as_raw(self), maxheight) }
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFontCollection)(windows_core::Interface::as_raw(self), fontcollection.param().abi(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFontFamilyName)(windows_core::Interface::as_raw(self), fontfamilyname.param().abi(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFontWeight)(windows_core::Interface::as_raw(self), fontweight, core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFontStyle)(windows_core::Interface::as_raw(self), fontstyle, core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFontStretch)(windows_core::Interface::as_raw(self), fontstretch, core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFontSize)(windows_core::Interface::as_raw(self), fontsize, core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetUnderline(&self, hasunderline: bool, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUnderline)(windows_core::Interface::as_raw(self), hasunderline.into(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetStrikethrough(&self, hasstrikethrough: bool, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStrikethrough)(windows_core::Interface::as_raw(self), hasstrikethrough.into(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDrawingEffect)(windows_core::Interface::as_raw(self), drawingeffect.param().abi(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteInlineObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInlineObject)(windows_core::Interface::as_raw(self), inlineobject.param().abi(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteTypography>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTypography)(windows_core::Interface::as_raw(self), typography.param().abi(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLocaleName)(windows_core::Interface::as_raw(self), localename.param().abi(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetMaxWidth)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetMaxHeight)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontCollection(&self, currentposition: u32, fontcollection: *mut Option<IDWriteFontCollection>, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontCollection)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(fontcollection), textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetFontFamilyNameLength(&self, currentposition: u32, namelength: *mut u32, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyNameLength)(windows_core::Interface::as_raw(self), currentposition, namelength as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetFontFamilyName(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyName)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap(), textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetFontWeight(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontWeight)(windows_core::Interface::as_raw(self), currentposition, fontweight as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetFontStyle(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontStyle)(windows_core::Interface::as_raw(self), currentposition, fontstyle as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetFontStretch(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontStretch)(windows_core::Interface::as_raw(self), currentposition, fontstretch as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetFontSize(&self, currentposition: u32, fontsize: *mut f32, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontSize)(windows_core::Interface::as_raw(self), currentposition, fontsize as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut windows_core::BOOL, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUnderline)(windows_core::Interface::as_raw(self), currentposition, hasunderline as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut windows_core::BOOL, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStrikethrough)(windows_core::Interface::as_raw(self), currentposition, hasstrikethrough as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut Option<windows_core::IUnknown>, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDrawingEffect)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(drawingeffect), textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut Option<IDWriteInlineObject>, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInlineObject)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(inlineobject), textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut Option<IDWriteTypography>, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTypography)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(typography), textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetLocaleNameLength(&self, currentposition: u32, namelength: *mut u32, textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self), currentposition, namelength as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetLocaleName(&self, currentposition: u32, localename: &mut [u16], textrange: Option<*mut DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), currentposition, core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap(), textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Draw<P1>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, renderer: P1, originx: f32, originy: f32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDWriteTextRenderer>,
    {
        unsafe { (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, renderer.param().abi(), originx, originy) }
    }
    pub unsafe fn GetLineMetrics(&self, linemetrics: Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLineMetrics)(windows_core::Interface::as_raw(self), core::mem::transmute(linemetrics.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount as _) }
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), textmetrics as _) }
    }
    pub unsafe fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverhangMetrics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClusterMetrics)(windows_core::Interface::as_raw(self), core::mem::transmute(clustermetrics.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualclustercount as _) }
    }
    pub unsafe fn DetermineMinWidth(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetermineMinWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut windows_core::BOOL, isinside: *mut windows_core::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HitTestPoint)(windows_core::Interface::as_raw(self), pointx, pointy, istrailinghit as _, isinside as _, hittestmetrics as _) }
    }
    pub unsafe fn HitTestTextPosition(&self, textposition: u32, istrailinghit: bool, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HitTestTextPosition)(windows_core::Interface::as_raw(self), textposition, istrailinghit.into(), pointx as _, pointy as _, hittestmetrics as _) }
    }
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HitTestTextRange)(windows_core::Interface::as_raw(self), textposition, textlength, originx, originy, core::mem::transmute(hittestmetrics.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualhittestmetricscount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_STYLE, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_STRETCH, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetDrawingEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetTypography: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_WEIGHT, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_STYLE, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_STRETCH, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetDrawingEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetTypography: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u16, u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub GetLineMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_LINE_METRICS, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_TEXT_METRICS) -> windows_core::HRESULT,
    pub GetOverhangMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_OVERHANG_METRICS) -> windows_core::HRESULT,
    pub GetClusterMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_CLUSTER_METRICS, u32, *mut u32) -> windows_core::HRESULT,
    pub DetermineMinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub HitTestPoint: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *mut windows_core::BOOL, *mut windows_core::BOOL, *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT,
    pub HitTestTextPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL, *mut f32, *mut f32, *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT,
    pub HitTestTextRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, f32, f32, *mut DWRITE_HIT_TEST_METRICS, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IDWriteTextLayout_Impl: IDWriteTextFormat_Impl {
    fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::Result<()>;
    fn SetMaxHeight(&self, maxheight: f32) -> windows_core::Result<()>;
    fn SetFontCollection(&self, fontcollection: windows_core::Ref<IDWriteFontCollection>, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontFamilyName(&self, fontfamilyname: &windows_core::PCWSTR, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetFontSize(&self, fontsize: f32, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetUnderline(&self, hasunderline: windows_core::BOOL, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetStrikethrough(&self, hasstrikethrough: windows_core::BOOL, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetDrawingEffect(&self, drawingeffect: windows_core::Ref<windows_core::IUnknown>, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetInlineObject(&self, inlineobject: windows_core::Ref<IDWriteInlineObject>, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetTypography(&self, typography: windows_core::Ref<IDWriteTypography>, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetLocaleName(&self, localename: &windows_core::PCWSTR, textrange: &DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetMaxWidth(&self) -> f32;
    fn GetMaxHeight(&self) -> f32;
    fn GetFontCollection(&self, currentposition: u32, fontcollection: windows_core::OutRef<IDWriteFontCollection>, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontFamilyNameLength(&self, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontFamilyName(&self, currentposition: u32, fontfamilyname: *mut u16, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontWeight(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontStyle(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontStretch(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetFontSize(&self, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetUnderline(&self, currentposition: u32, hasunderline: *mut windows_core::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut windows_core::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: windows_core::OutRef<windows_core::IUnknown>, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetInlineObject(&self, currentposition: u32, inlineobject: windows_core::OutRef<IDWriteInlineObject>, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetTypography(&self, currentposition: u32, typography: windows_core::OutRef<IDWriteTypography>, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetLocaleNameLength(&self, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetLocaleName(&self, currentposition: u32, localename: *mut u16, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn Draw(&self, clientdrawingcontext: *const core::ffi::c_void, renderer: windows_core::Ref<IDWriteTextRenderer>, originx: f32, originy: f32) -> windows_core::Result<()>;
    fn GetLineMetrics(&self, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> windows_core::Result<()>;
    fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::Result<()>;
    fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetClusterMetrics(&self, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> windows_core::Result<()>;
    fn DetermineMinWidth(&self) -> windows_core::Result<f32>;
    fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut windows_core::BOOL, isinside: *mut windows_core::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::Result<()>;
    fn HitTestTextPosition(&self, textposition: u32, istrailinghit: windows_core::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::Result<()>;
    fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> windows_core::Result<()>;
}
impl IDWriteTextLayout_Vtbl {
    pub const fn new<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMaxWidth<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxwidth: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetMaxWidth(this, core::mem::transmute_copy(&maxwidth)).into()
            }
        }
        unsafe extern "system" fn SetMaxHeight<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxheight: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetMaxHeight(this, core::mem::transmute_copy(&maxheight)).into()
            }
        }
        unsafe extern "system" fn SetFontCollection<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontCollection(this, core::mem::transmute_copy(&fontcollection), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetFontFamilyName<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamilyname: windows_core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontFamilyName(this, core::mem::transmute(&fontfamilyname), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetFontWeight<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontWeight(this, core::mem::transmute_copy(&fontweight), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetFontStyle<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontStyle(this, core::mem::transmute_copy(&fontstyle), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetFontStretch<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontStretch(this, core::mem::transmute_copy(&fontstretch), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetFontSize<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontSize(this, core::mem::transmute_copy(&fontsize), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasunderline: windows_core::BOOL, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetUnderline(this, core::mem::transmute_copy(&hasunderline), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetStrikethrough<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasstrikethrough: windows_core::BOOL, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetStrikethrough(this, core::mem::transmute_copy(&hasstrikethrough), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetDrawingEffect<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drawingeffect: *mut core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetDrawingEffect(this, core::mem::transmute_copy(&drawingeffect), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetInlineObject<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inlineobject: *mut core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetInlineObject(this, core::mem::transmute_copy(&inlineobject), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetTypography<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typography: *mut core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetTypography(this, core::mem::transmute_copy(&typography), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetLocaleName<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localename: windows_core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetLocaleName(this, core::mem::transmute(&localename), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetMaxWidth<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetMaxWidth(this)
            }
        }
        unsafe extern "system" fn GetMaxHeight<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetMaxHeight(this)
            }
        }
        unsafe extern "system" fn GetFontCollection<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontcollection: *mut *mut core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontCollection(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontFamilyNameLength(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&namelength), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetFontFamilyName<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontfamilyname: *mut u16, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontFamilyName(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontfamilyname), core::mem::transmute_copy(&namesize), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetFontWeight<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontWeight(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontweight), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetFontStyle<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontStyle(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontstyle), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetFontStretch<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontStretch(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontstretch), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetFontSize<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontSize(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&fontsize), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetUnderline<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, hasunderline: *mut windows_core::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetUnderline(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&hasunderline), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetStrikethrough<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, hasstrikethrough: *mut windows_core::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetStrikethrough(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&hasstrikethrough), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetDrawingEffect<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, drawingeffect: *mut *mut core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetDrawingEffect(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&drawingeffect), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetInlineObject<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, inlineobject: *mut *mut core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetInlineObject(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&inlineobject), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetTypography<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, typography: *mut *mut core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetTypography(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&typography), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetLocaleNameLength(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&namelength), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetLocaleName<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, localename: *mut u16, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetLocaleName(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&localename), core::mem::transmute_copy(&namesize), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn Draw<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, renderer: *mut core::ffi::c_void, originx: f32, originy: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::Draw(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&renderer), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy)).into()
            }
        }
        unsafe extern "system" fn GetLineMetrics<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetLineMetrics(this, core::mem::transmute_copy(&linemetrics), core::mem::transmute_copy(&maxlinecount), core::mem::transmute_copy(&actuallinecount)).into()
            }
        }
        unsafe extern "system" fn GetMetrics<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetMetrics(this, core::mem::transmute_copy(&textmetrics)).into()
            }
        }
        unsafe extern "system" fn GetOverhangMetrics<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTextLayout_Impl::GetOverhangMetrics(this) {
                    Ok(ok__) => {
                        overhangs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClusterMetrics<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetClusterMetrics(this, core::mem::transmute_copy(&clustermetrics), core::mem::transmute_copy(&maxclustercount), core::mem::transmute_copy(&actualclustercount)).into()
            }
        }
        unsafe extern "system" fn DetermineMinWidth<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minwidth: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTextLayout_Impl::DetermineMinWidth(this) {
                    Ok(ok__) => {
                        minwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HitTestPoint<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointx: f32, pointy: f32, istrailinghit: *mut windows_core::BOOL, isinside: *mut windows_core::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::HitTestPoint(this, core::mem::transmute_copy(&pointx), core::mem::transmute_copy(&pointy), core::mem::transmute_copy(&istrailinghit), core::mem::transmute_copy(&isinside), core::mem::transmute_copy(&hittestmetrics)).into()
            }
        }
        unsafe extern "system" fn HitTestTextPosition<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, istrailinghit: windows_core::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::HitTestTextPosition(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&istrailinghit), core::mem::transmute_copy(&pointx), core::mem::transmute_copy(&pointy), core::mem::transmute_copy(&hittestmetrics)).into()
            }
        }
        unsafe extern "system" fn HitTestTextRange<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::HitTestTextRange(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&hittestmetrics), core::mem::transmute_copy(&maxhittestmetricscount), core::mem::transmute_copy(&actualhittestmetricscount)).into()
            }
        }
        Self {
            base__: IDWriteTextFormat_Vtbl::new::<Identity, OFFSET>(),
            SetMaxWidth: SetMaxWidth::<Identity, OFFSET>,
            SetMaxHeight: SetMaxHeight::<Identity, OFFSET>,
            SetFontCollection: SetFontCollection::<Identity, OFFSET>,
            SetFontFamilyName: SetFontFamilyName::<Identity, OFFSET>,
            SetFontWeight: SetFontWeight::<Identity, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, OFFSET>,
            SetFontSize: SetFontSize::<Identity, OFFSET>,
            SetUnderline: SetUnderline::<Identity, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, OFFSET>,
            SetDrawingEffect: SetDrawingEffect::<Identity, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, OFFSET>,
            SetTypography: SetTypography::<Identity, OFFSET>,
            SetLocaleName: SetLocaleName::<Identity, OFFSET>,
            GetMaxWidth: GetMaxWidth::<Identity, OFFSET>,
            GetMaxHeight: GetMaxHeight::<Identity, OFFSET>,
            GetFontCollection: GetFontCollection::<Identity, OFFSET>,
            GetFontFamilyNameLength: GetFontFamilyNameLength::<Identity, OFFSET>,
            GetFontFamilyName: GetFontFamilyName::<Identity, OFFSET>,
            GetFontWeight: GetFontWeight::<Identity, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, OFFSET>,
            GetFontStretch: GetFontStretch::<Identity, OFFSET>,
            GetFontSize: GetFontSize::<Identity, OFFSET>,
            GetUnderline: GetUnderline::<Identity, OFFSET>,
            GetStrikethrough: GetStrikethrough::<Identity, OFFSET>,
            GetDrawingEffect: GetDrawingEffect::<Identity, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, OFFSET>,
            GetTypography: GetTypography::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            GetLineMetrics: GetLineMetrics::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, OFFSET>,
            GetClusterMetrics: GetClusterMetrics::<Identity, OFFSET>,
            DetermineMinWidth: DetermineMinWidth::<Identity, OFFSET>,
            HitTestPoint: HitTestPoint::<Identity, OFFSET>,
            HitTestTextPosition: HitTestTextPosition::<Identity, OFFSET>,
            HitTestTextRange: HitTestTextRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout as windows_core::Interface>::IID || iid == &<IDWriteTextFormat as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteTextLayout {}
windows_core::imp::define_interface!(IDWriteTextRenderer, IDWriteTextRenderer_Vtbl, 0xef8a8135_5cc6_45fe_8825_c5a0724eb819);
impl core::ops::Deref for IDWriteTextRenderer {
    type Target = IDWritePixelSnapping;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextRenderer, windows_core::IUnknown, IDWritePixelSnapping);
impl IDWriteTextRenderer {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawGlyphRun<P6>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: P6) -> windows_core::HRESULT
    where
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGlyphRun)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, baselineoriginx, baselineoriginy, measuringmode, core::mem::transmute(glyphrun), glyphrundescription, clientdrawingeffect.param().abi()) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawUnderline<P4>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawUnderline)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, baselineoriginx, baselineoriginy, underline, clientdrawingeffect.param().abi()) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawStrikethrough<P4>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawStrikethrough)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, baselineoriginx, baselineoriginy, strikethrough, clientdrawingeffect.param().abi()) }
    }
    pub unsafe fn DrawInlineObject<P3, P6>(&self, clientdrawingcontext: Option<*const core::ffi::c_void>, originx: f32, originy: f32, inlineobject: P3, issideways: bool, isrighttoleft: bool, clientdrawingeffect: P6) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IDWriteInlineObject>,
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawInlineObject)(windows_core::Interface::as_raw(self), clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _, originx, originy, inlineobject.param().abi(), issideways.into(), isrighttoleft.into(), clientdrawingeffect.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextRenderer_Vtbl {
    pub base__: IDWritePixelSnapping_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub DrawGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, super::dcommon::DWRITE_MEASURING_MODE, *const DWRITE_GLYPH_RUN, *const DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub DrawUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, *const DWRITE_UNDERLINE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawUnderline: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub DrawStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, *const DWRITE_STRIKETHROUGH, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawStrikethrough: usize,
    pub DrawInlineObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, f32, f32, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dcommon")]
pub trait IDWriteTextRenderer_Impl: IDWritePixelSnapping_Impl {
    fn DrawGlyphRun(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawUnderline(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawStrikethrough(&self, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DrawInlineObject(&self, clientdrawingcontext: *const core::ffi::c_void, originx: f32, originy: f32, inlineobject: windows_core::Ref<IDWriteInlineObject>, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, clientdrawingeffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dcommon")]
impl IDWriteTextRenderer_Vtbl {
    pub const fn new<Identity: IDWriteTextRenderer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DrawGlyphRun<Identity: IDWriteTextRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer_Impl::DrawGlyphRun(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&clientdrawingeffect)).into()
            }
        }
        unsafe extern "system" fn DrawUnderline<Identity: IDWriteTextRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer_Impl::DrawUnderline(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&underline), core::mem::transmute_copy(&clientdrawingeffect)).into()
            }
        }
        unsafe extern "system" fn DrawStrikethrough<Identity: IDWriteTextRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer_Impl::DrawStrikethrough(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&baselineoriginx), core::mem::transmute_copy(&baselineoriginy), core::mem::transmute_copy(&strikethrough), core::mem::transmute_copy(&clientdrawingeffect)).into()
            }
        }
        unsafe extern "system" fn DrawInlineObject<Identity: IDWriteTextRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientdrawingcontext: *const core::ffi::c_void, originx: f32, originy: f32, inlineobject: *mut core::ffi::c_void, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL, clientdrawingeffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextRenderer_Impl::DrawInlineObject(this, core::mem::transmute_copy(&clientdrawingcontext), core::mem::transmute_copy(&originx), core::mem::transmute_copy(&originy), core::mem::transmute_copy(&inlineobject), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft), core::mem::transmute_copy(&clientdrawingeffect)).into()
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
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextRenderer as windows_core::Interface>::IID || iid == &<IDWritePixelSnapping as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dcommon")]
impl windows_core::RuntimeName for IDWriteTextRenderer {}
windows_core::imp::define_interface!(IDWriteTypography, IDWriteTypography_Vtbl, 0x55f1112b_1dc2_4b3c_9541_f46894ed85b6);
windows_core::imp::interface_hierarchy!(IDWriteTypography, windows_core::IUnknown);
impl IDWriteTypography {
    pub unsafe fn AddFontFeature(&self, fontfeature: DWRITE_FONT_FEATURE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddFontFeature)(windows_core::Interface::as_raw(self), core::mem::transmute(fontfeature)) }
    }
    pub unsafe fn GetFontFeatureCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontFeatureCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontFeature(&self, fontfeatureindex: u32) -> windows_core::Result<DWRITE_FONT_FEATURE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFeature)(windows_core::Interface::as_raw(self), fontfeatureindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTypography_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFontFeature: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FEATURE) -> windows_core::HRESULT,
    pub GetFontFeatureCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFeature: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_FEATURE) -> windows_core::HRESULT,
}
pub trait IDWriteTypography_Impl: windows_core::IUnknownImpl {
    fn AddFontFeature(&self, fontfeature: &DWRITE_FONT_FEATURE) -> windows_core::Result<()>;
    fn GetFontFeatureCount(&self) -> u32;
    fn GetFontFeature(&self, fontfeatureindex: u32) -> windows_core::Result<DWRITE_FONT_FEATURE>;
}
impl IDWriteTypography_Vtbl {
    pub const fn new<Identity: IDWriteTypography_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddFontFeature<Identity: IDWriteTypography_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfeature: DWRITE_FONT_FEATURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTypography_Impl::AddFontFeature(this, core::mem::transmute(&fontfeature)).into()
            }
        }
        unsafe extern "system" fn GetFontFeatureCount<Identity: IDWriteTypography_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTypography_Impl::GetFontFeatureCount(this)
            }
        }
        unsafe extern "system" fn GetFontFeature<Identity: IDWriteTypography_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfeatureindex: u32, fontfeature: *mut DWRITE_FONT_FEATURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTypography_Impl::GetFontFeature(this, core::mem::transmute_copy(&fontfeatureindex)) {
                    Ok(ok__) => {
                        fontfeature.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFontFeature: AddFontFeature::<Identity, OFFSET>,
            GetFontFeatureCount: GetFontFeatureCount::<Identity, OFFSET>,
            GetFontFeature: GetFontFeature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTypography as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteTypography {}
