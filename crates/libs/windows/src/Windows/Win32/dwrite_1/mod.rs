pub type DWRITE_BASELINE = i32;
pub const DWRITE_BASELINE_CENTRAL: DWRITE_BASELINE = 2;
pub const DWRITE_BASELINE_DEFAULT: DWRITE_BASELINE = 0;
pub const DWRITE_BASELINE_HANGING: DWRITE_BASELINE = 4;
pub const DWRITE_BASELINE_IDEOGRAPHIC_BOTTOM: DWRITE_BASELINE = 5;
pub const DWRITE_BASELINE_IDEOGRAPHIC_TOP: DWRITE_BASELINE = 6;
pub const DWRITE_BASELINE_MATH: DWRITE_BASELINE = 3;
pub const DWRITE_BASELINE_MAXIMUM: DWRITE_BASELINE = 8;
pub const DWRITE_BASELINE_MINIMUM: DWRITE_BASELINE = 7;
pub const DWRITE_BASELINE_ROMAN: DWRITE_BASELINE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_CARET_METRICS {
    pub slopeRise: i16,
    pub slopeRun: i16,
    pub offset: i16,
}
#[repr(C)]
#[cfg(feature = "Win32_dwrite")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_FONT_METRICS1 {
    pub Base: super::dwrite::DWRITE_FONT_METRICS,
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
    pub hasTypographicMetrics: windows_core::BOOL,
}
pub type DWRITE_GLYPH_ORIENTATION_ANGLE = i32;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 0;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 2;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 3;
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_JUSTIFICATION_OPPORTUNITY {
    pub expansionMinimum: f32,
    pub expansionMaximum: f32,
    pub compressionMaximum: f32,
    pub _bitfield: u32,
}
pub type DWRITE_OUTLINE_THRESHOLD = i32;
pub const DWRITE_OUTLINE_THRESHOLD_ALIASED: DWRITE_OUTLINE_THRESHOLD = 1;
pub const DWRITE_OUTLINE_THRESHOLD_ANTIALIASED: DWRITE_OUTLINE_THRESHOLD = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DWRITE_PANOSE {
    pub values: [u8; 10],
    pub familyKind: u8,
    pub text: DWRITE_PANOSE_0,
    pub script: DWRITE_PANOSE_1,
    pub decorative: DWRITE_PANOSE_2,
    pub symbol: DWRITE_PANOSE_3,
}
impl Default for DWRITE_PANOSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PANOSE_0 {
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PANOSE_2 {
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_PANOSE_3 {
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
pub type DWRITE_PANOSE_ARM_STYLE = i32;
pub const DWRITE_PANOSE_ARM_STYLE_ANY: DWRITE_PANOSE_ARM_STYLE = 0;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 11;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_HORZ: DWRITE_PANOSE_ARM_STYLE = 7;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 10;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_VERT: DWRITE_PANOSE_ARM_STYLE = 9;
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = 8;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 11;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_HORIZONTAL: DWRITE_PANOSE_ARM_STYLE = 7;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 10;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_VERTICAL: DWRITE_PANOSE_ARM_STYLE = 9;
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = 8;
pub const DWRITE_PANOSE_ARM_STYLE_NO_FIT: DWRITE_PANOSE_ARM_STYLE = 1;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 6;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORIZONTAL: DWRITE_PANOSE_ARM_STYLE = 2;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORZ: DWRITE_PANOSE_ARM_STYLE = 2;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = 5;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERT: DWRITE_PANOSE_ARM_STYLE = 4;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERTICAL: DWRITE_PANOSE_ARM_STYLE = 4;
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = 3;
pub type DWRITE_PANOSE_ASPECT = i32;
pub const DWRITE_PANOSE_ASPECT_ANY: DWRITE_PANOSE_ASPECT = 0;
pub const DWRITE_PANOSE_ASPECT_CONDENSED: DWRITE_PANOSE_ASPECT = 4;
pub const DWRITE_PANOSE_ASPECT_EXTENDED: DWRITE_PANOSE_ASPECT = 6;
pub const DWRITE_PANOSE_ASPECT_MONOSPACED: DWRITE_PANOSE_ASPECT = 9;
pub const DWRITE_PANOSE_ASPECT_NORMAL: DWRITE_PANOSE_ASPECT = 5;
pub const DWRITE_PANOSE_ASPECT_NO_FIT: DWRITE_PANOSE_ASPECT = 1;
pub type DWRITE_PANOSE_ASPECT_RATIO = i32;
pub const DWRITE_PANOSE_ASPECT_RATIO_ANY: DWRITE_PANOSE_ASPECT_RATIO = 0;
pub const DWRITE_PANOSE_ASPECT_RATIO_CONDENSED: DWRITE_PANOSE_ASPECT_RATIO = 3;
pub const DWRITE_PANOSE_ASPECT_RATIO_EXPANDED: DWRITE_PANOSE_ASPECT_RATIO = 5;
pub const DWRITE_PANOSE_ASPECT_RATIO_NORMAL: DWRITE_PANOSE_ASPECT_RATIO = 4;
pub const DWRITE_PANOSE_ASPECT_RATIO_NO_FIT: DWRITE_PANOSE_ASPECT_RATIO = 1;
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_CONDENSED: DWRITE_PANOSE_ASPECT_RATIO = 2;
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_EXPANDED: DWRITE_PANOSE_ASPECT_RATIO = 6;
pub const DWRITE_PANOSE_ASPECT_SUPER_CONDENSED: DWRITE_PANOSE_ASPECT = 2;
pub const DWRITE_PANOSE_ASPECT_SUPER_EXTENDED: DWRITE_PANOSE_ASPECT = 8;
pub const DWRITE_PANOSE_ASPECT_VERY_CONDENSED: DWRITE_PANOSE_ASPECT = 3;
pub const DWRITE_PANOSE_ASPECT_VERY_EXTENDED: DWRITE_PANOSE_ASPECT = 7;
pub type DWRITE_PANOSE_CHARACTER_RANGES = i32;
pub const DWRITE_PANOSE_CHARACTER_RANGES_ANY: DWRITE_PANOSE_CHARACTER_RANGES = 0;
pub const DWRITE_PANOSE_CHARACTER_RANGES_EXTENDED_COLLECTION: DWRITE_PANOSE_CHARACTER_RANGES = 2;
pub const DWRITE_PANOSE_CHARACTER_RANGES_LITERALS: DWRITE_PANOSE_CHARACTER_RANGES = 3;
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_FIT: DWRITE_PANOSE_CHARACTER_RANGES = 1;
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_LOWER_CASE: DWRITE_PANOSE_CHARACTER_RANGES = 4;
pub const DWRITE_PANOSE_CHARACTER_RANGES_SMALL_CAPS: DWRITE_PANOSE_CHARACTER_RANGES = 5;
pub type DWRITE_PANOSE_CONTRAST = i32;
pub const DWRITE_PANOSE_CONTRAST_ANY: DWRITE_PANOSE_CONTRAST = 0;
pub const DWRITE_PANOSE_CONTRAST_BROKEN: DWRITE_PANOSE_CONTRAST = 13;
pub const DWRITE_PANOSE_CONTRAST_HIGH: DWRITE_PANOSE_CONTRAST = 8;
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_HIGH: DWRITE_PANOSE_CONTRAST = 12;
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_LOW: DWRITE_PANOSE_CONTRAST = 10;
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_MEDIUM: DWRITE_PANOSE_CONTRAST = 11;
pub const DWRITE_PANOSE_CONTRAST_LOW: DWRITE_PANOSE_CONTRAST = 4;
pub const DWRITE_PANOSE_CONTRAST_MEDIUM: DWRITE_PANOSE_CONTRAST = 6;
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_HIGH: DWRITE_PANOSE_CONTRAST = 7;
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_LOW: DWRITE_PANOSE_CONTRAST = 5;
pub const DWRITE_PANOSE_CONTRAST_NONE: DWRITE_PANOSE_CONTRAST = 2;
pub const DWRITE_PANOSE_CONTRAST_NO_FIT: DWRITE_PANOSE_CONTRAST = 1;
pub const DWRITE_PANOSE_CONTRAST_VERY_HIGH: DWRITE_PANOSE_CONTRAST = 9;
pub const DWRITE_PANOSE_CONTRAST_VERY_LOW: DWRITE_PANOSE_CONTRAST = 3;
pub type DWRITE_PANOSE_DECORATIVE_CLASS = i32;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ANY: DWRITE_PANOSE_DECORATIVE_CLASS = 0;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_CARTOON: DWRITE_PANOSE_DECORATIVE_CLASS = 7;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_COLLAGE: DWRITE_PANOSE_DECORATIVE_CLASS = 11;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_DERIVATIVE: DWRITE_PANOSE_DECORATIVE_CLASS = 2;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_INITIALS: DWRITE_PANOSE_DECORATIVE_CLASS = 6;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_MONTAGE: DWRITE_PANOSE_DECORATIVE_CLASS = 12;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ASPECT: DWRITE_PANOSE_DECORATIVE_CLASS = 5;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ELEMENTS: DWRITE_PANOSE_DECORATIVE_CLASS = 4;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_TOPOLOGY: DWRITE_PANOSE_DECORATIVE_CLASS = 3;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NO_FIT: DWRITE_PANOSE_DECORATIVE_CLASS = 1;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ORNAMENTED: DWRITE_PANOSE_DECORATIVE_CLASS = 9;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_PICTURE_STEMS: DWRITE_PANOSE_DECORATIVE_CLASS = 8;
pub const DWRITE_PANOSE_DECORATIVE_CLASS_TEXT_AND_BACKGROUND: DWRITE_PANOSE_DECORATIVE_CLASS = 10;
pub type DWRITE_PANOSE_DECORATIVE_TOPOLOGY = i32;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ANY: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 0;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ART_DECO: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 5;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_BLACKLETTER: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 14;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_CURSIVE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 13;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_ARMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 7;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_FORMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 8;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_HORSESHOE_E_AND_A: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 12;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_IMPLIED_TOPOLOGY: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 11;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_LOMBARDIC_FORMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 9;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_MULTIPLE_SEGMENT: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 4;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_NO_FIT: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 1;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SQUARE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 3;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_STANDARD: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 2;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SWASH_VARIANCE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 15;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UNEVEN_WEIGHTING: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 6;
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UPPER_CASE_IN_LOWER_CASE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = 10;
pub type DWRITE_PANOSE_FAMILY = i32;
pub const DWRITE_PANOSE_FAMILY_ANY: DWRITE_PANOSE_FAMILY = 0;
pub const DWRITE_PANOSE_FAMILY_DECORATIVE: DWRITE_PANOSE_FAMILY = 4;
pub const DWRITE_PANOSE_FAMILY_NO_FIT: DWRITE_PANOSE_FAMILY = 1;
pub const DWRITE_PANOSE_FAMILY_PICTORIAL: DWRITE_PANOSE_FAMILY = 5;
pub const DWRITE_PANOSE_FAMILY_SCRIPT: DWRITE_PANOSE_FAMILY = 3;
pub const DWRITE_PANOSE_FAMILY_SYMBOL: DWRITE_PANOSE_FAMILY = 5;
pub const DWRITE_PANOSE_FAMILY_TEXT_DISPLAY: DWRITE_PANOSE_FAMILY = 2;
pub type DWRITE_PANOSE_FILL = i32;
pub const DWRITE_PANOSE_FILL_ANY: DWRITE_PANOSE_FILL = 0;
pub const DWRITE_PANOSE_FILL_COMPLEX_FILL: DWRITE_PANOSE_FILL = 5;
pub const DWRITE_PANOSE_FILL_DRAWN_DISTRESSED: DWRITE_PANOSE_FILL = 7;
pub const DWRITE_PANOSE_FILL_NO_FILL: DWRITE_PANOSE_FILL = 3;
pub const DWRITE_PANOSE_FILL_NO_FIT: DWRITE_PANOSE_FILL = 1;
pub const DWRITE_PANOSE_FILL_PATTERNED_FILL: DWRITE_PANOSE_FILL = 4;
pub const DWRITE_PANOSE_FILL_SHAPED_FILL: DWRITE_PANOSE_FILL = 6;
pub const DWRITE_PANOSE_FILL_STANDARD_SOLID_FILL: DWRITE_PANOSE_FILL = 2;
pub type DWRITE_PANOSE_FINIALS = i32;
pub const DWRITE_PANOSE_FINIALS_ANY: DWRITE_PANOSE_FINIALS = 0;
pub const DWRITE_PANOSE_FINIALS_NONE_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = 3;
pub const DWRITE_PANOSE_FINIALS_NONE_NO_LOOPS: DWRITE_PANOSE_FINIALS = 2;
pub const DWRITE_PANOSE_FINIALS_NONE_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = 4;
pub const DWRITE_PANOSE_FINIALS_NO_FIT: DWRITE_PANOSE_FINIALS = 1;
pub const DWRITE_PANOSE_FINIALS_ROUND_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = 12;
pub const DWRITE_PANOSE_FINIALS_ROUND_NO_LOOPS: DWRITE_PANOSE_FINIALS = 11;
pub const DWRITE_PANOSE_FINIALS_ROUND_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = 13;
pub const DWRITE_PANOSE_FINIALS_SHARP_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = 6;
pub const DWRITE_PANOSE_FINIALS_SHARP_NO_LOOPS: DWRITE_PANOSE_FINIALS = 5;
pub const DWRITE_PANOSE_FINIALS_SHARP_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = 7;
pub const DWRITE_PANOSE_FINIALS_TAPERED_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = 9;
pub const DWRITE_PANOSE_FINIALS_TAPERED_NO_LOOPS: DWRITE_PANOSE_FINIALS = 8;
pub const DWRITE_PANOSE_FINIALS_TAPERED_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = 10;
pub type DWRITE_PANOSE_LETTERFORM = i32;
pub const DWRITE_PANOSE_LETTERFORM_ANY: DWRITE_PANOSE_LETTERFORM = 0;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_BOXED: DWRITE_PANOSE_LETTERFORM = 4;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_CONTACT: DWRITE_PANOSE_LETTERFORM = 2;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_FLATTENED: DWRITE_PANOSE_LETTERFORM = 5;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_OFF_CENTER: DWRITE_PANOSE_LETTERFORM = 7;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_ROUNDED: DWRITE_PANOSE_LETTERFORM = 6;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_SQUARE: DWRITE_PANOSE_LETTERFORM = 8;
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_WEIGHTED: DWRITE_PANOSE_LETTERFORM = 3;
pub const DWRITE_PANOSE_LETTERFORM_NO_FIT: DWRITE_PANOSE_LETTERFORM = 1;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_BOXED: DWRITE_PANOSE_LETTERFORM = 11;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_CONTACT: DWRITE_PANOSE_LETTERFORM = 9;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_FLATTENED: DWRITE_PANOSE_LETTERFORM = 12;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_OFF_CENTER: DWRITE_PANOSE_LETTERFORM = 14;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_ROUNDED: DWRITE_PANOSE_LETTERFORM = 13;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_SQUARE: DWRITE_PANOSE_LETTERFORM = 15;
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_WEIGHTED: DWRITE_PANOSE_LETTERFORM = 10;
pub type DWRITE_PANOSE_LINING = i32;
pub const DWRITE_PANOSE_LINING_ANY: DWRITE_PANOSE_LINING = 0;
pub const DWRITE_PANOSE_LINING_BACKDROP: DWRITE_PANOSE_LINING = 8;
pub const DWRITE_PANOSE_LINING_ENGRAVED: DWRITE_PANOSE_LINING = 5;
pub const DWRITE_PANOSE_LINING_INLINE: DWRITE_PANOSE_LINING = 3;
pub const DWRITE_PANOSE_LINING_NONE: DWRITE_PANOSE_LINING = 2;
pub const DWRITE_PANOSE_LINING_NO_FIT: DWRITE_PANOSE_LINING = 1;
pub const DWRITE_PANOSE_LINING_OUTLINE: DWRITE_PANOSE_LINING = 4;
pub const DWRITE_PANOSE_LINING_RELIEF: DWRITE_PANOSE_LINING = 7;
pub const DWRITE_PANOSE_LINING_SHADOW: DWRITE_PANOSE_LINING = 6;
pub type DWRITE_PANOSE_MIDLINE = i32;
pub const DWRITE_PANOSE_MIDLINE_ANY: DWRITE_PANOSE_MIDLINE = 0;
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_POINTED: DWRITE_PANOSE_MIDLINE = 9;
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_SERIFED: DWRITE_PANOSE_MIDLINE = 10;
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_TRIMMED: DWRITE_PANOSE_MIDLINE = 8;
pub const DWRITE_PANOSE_MIDLINE_HIGH_POINTED: DWRITE_PANOSE_MIDLINE = 6;
pub const DWRITE_PANOSE_MIDLINE_HIGH_SERIFED: DWRITE_PANOSE_MIDLINE = 7;
pub const DWRITE_PANOSE_MIDLINE_HIGH_TRIMMED: DWRITE_PANOSE_MIDLINE = 5;
pub const DWRITE_PANOSE_MIDLINE_LOW_POINTED: DWRITE_PANOSE_MIDLINE = 12;
pub const DWRITE_PANOSE_MIDLINE_LOW_SERIFED: DWRITE_PANOSE_MIDLINE = 13;
pub const DWRITE_PANOSE_MIDLINE_LOW_TRIMMED: DWRITE_PANOSE_MIDLINE = 11;
pub const DWRITE_PANOSE_MIDLINE_NO_FIT: DWRITE_PANOSE_MIDLINE = 1;
pub const DWRITE_PANOSE_MIDLINE_STANDARD_POINTED: DWRITE_PANOSE_MIDLINE = 3;
pub const DWRITE_PANOSE_MIDLINE_STANDARD_SERIFED: DWRITE_PANOSE_MIDLINE = 4;
pub const DWRITE_PANOSE_MIDLINE_STANDARD_TRIMMED: DWRITE_PANOSE_MIDLINE = 2;
pub type DWRITE_PANOSE_PROPORTION = i32;
pub const DWRITE_PANOSE_PROPORTION_ANY: DWRITE_PANOSE_PROPORTION = 0;
pub const DWRITE_PANOSE_PROPORTION_CONDENSED: DWRITE_PANOSE_PROPORTION = 6;
pub const DWRITE_PANOSE_PROPORTION_EVEN_WIDTH: DWRITE_PANOSE_PROPORTION = 4;
pub const DWRITE_PANOSE_PROPORTION_EXPANDED: DWRITE_PANOSE_PROPORTION = 5;
pub const DWRITE_PANOSE_PROPORTION_MODERN: DWRITE_PANOSE_PROPORTION = 3;
pub const DWRITE_PANOSE_PROPORTION_MONOSPACED: DWRITE_PANOSE_PROPORTION = 9;
pub const DWRITE_PANOSE_PROPORTION_NO_FIT: DWRITE_PANOSE_PROPORTION = 1;
pub const DWRITE_PANOSE_PROPORTION_OLD_STYLE: DWRITE_PANOSE_PROPORTION = 2;
pub const DWRITE_PANOSE_PROPORTION_VERY_CONDENSED: DWRITE_PANOSE_PROPORTION = 8;
pub const DWRITE_PANOSE_PROPORTION_VERY_EXPANDED: DWRITE_PANOSE_PROPORTION = 7;
pub type DWRITE_PANOSE_SCRIPT_FORM = i32;
pub const DWRITE_PANOSE_SCRIPT_FORM_ANY: DWRITE_PANOSE_SCRIPT_FORM = 0;
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 13;
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 12;
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 10;
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 11;
pub const DWRITE_PANOSE_SCRIPT_FORM_NO_FIT: DWRITE_PANOSE_SCRIPT_FORM = 1;
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 9;
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 8;
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 6;
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 7;
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 5;
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 4;
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 2;
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = 3;
pub type DWRITE_PANOSE_SCRIPT_TOPOLOGY = i32;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ANY: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 0;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 10;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 8;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 9;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 7;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 5;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 6;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_NO_FIT: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 1;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 4;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 2;
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = 3;
pub type DWRITE_PANOSE_SERIF_STYLE = i32;
pub const DWRITE_PANOSE_SERIF_STYLE_ANY: DWRITE_PANOSE_SERIF_STYLE = 0;
pub const DWRITE_PANOSE_SERIF_STYLE_BONE: DWRITE_PANOSE_SERIF_STYLE = 8;
pub const DWRITE_PANOSE_SERIF_STYLE_COVE: DWRITE_PANOSE_SERIF_STYLE = 2;
pub const DWRITE_PANOSE_SERIF_STYLE_EXAGGERATED: DWRITE_PANOSE_SERIF_STYLE = 9;
pub const DWRITE_PANOSE_SERIF_STYLE_FLARED: DWRITE_PANOSE_SERIF_STYLE = 14;
pub const DWRITE_PANOSE_SERIF_STYLE_NORMAL_SANS: DWRITE_PANOSE_SERIF_STYLE = 11;
pub const DWRITE_PANOSE_SERIF_STYLE_NO_FIT: DWRITE_PANOSE_SERIF_STYLE = 1;
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_COVE: DWRITE_PANOSE_SERIF_STYLE = 3;
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SANS: DWRITE_PANOSE_SERIF_STYLE = 12;
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SQUARE_COVE: DWRITE_PANOSE_SERIF_STYLE = 5;
pub const DWRITE_PANOSE_SERIF_STYLE_OVAL: DWRITE_PANOSE_SERIF_STYLE = 8;
pub const DWRITE_PANOSE_SERIF_STYLE_PERPENDICULAR_SANS: DWRITE_PANOSE_SERIF_STYLE = 13;
pub const DWRITE_PANOSE_SERIF_STYLE_PERP_SANS: DWRITE_PANOSE_SERIF_STYLE = 13;
pub const DWRITE_PANOSE_SERIF_STYLE_ROUNDED: DWRITE_PANOSE_SERIF_STYLE = 15;
pub const DWRITE_PANOSE_SERIF_STYLE_SCRIPT: DWRITE_PANOSE_SERIF_STYLE = 16;
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE: DWRITE_PANOSE_SERIF_STYLE = 6;
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE_COVE: DWRITE_PANOSE_SERIF_STYLE = 4;
pub const DWRITE_PANOSE_SERIF_STYLE_THIN: DWRITE_PANOSE_SERIF_STYLE = 7;
pub const DWRITE_PANOSE_SERIF_STYLE_TRIANGLE: DWRITE_PANOSE_SERIF_STYLE = 10;
pub type DWRITE_PANOSE_SPACING = i32;
pub const DWRITE_PANOSE_SPACING_ANY: DWRITE_PANOSE_SPACING = 0;
pub const DWRITE_PANOSE_SPACING_MONOSPACED: DWRITE_PANOSE_SPACING = 3;
pub const DWRITE_PANOSE_SPACING_NO_FIT: DWRITE_PANOSE_SPACING = 1;
pub const DWRITE_PANOSE_SPACING_PROPORTIONAL_SPACED: DWRITE_PANOSE_SPACING = 2;
pub type DWRITE_PANOSE_STROKE_VARIATION = i32;
pub const DWRITE_PANOSE_STROKE_VARIATION_ANY: DWRITE_PANOSE_STROKE_VARIATION = 0;
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_DIAGONAL: DWRITE_PANOSE_STROKE_VARIATION = 3;
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = 6;
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_TRANSITIONAL: DWRITE_PANOSE_STROKE_VARIATION = 4;
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = 5;
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = 10;
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = 9;
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_FIT: DWRITE_PANOSE_STROKE_VARIATION = 1;
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_VARIATION: DWRITE_PANOSE_STROKE_VARIATION = 2;
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = 8;
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = 7;
pub type DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = i32;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_ANY: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 0;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_EXCEPTIONALLY_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 3;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NARROW: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 8;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NORMAL: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 7;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_FIT: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 1;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_WIDTH: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 2;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_SUPER_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 4;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_NARROW: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 9;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 5;
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = 6;
pub type DWRITE_PANOSE_SYMBOL_KIND = i32;
pub const DWRITE_PANOSE_SYMBOL_KIND_ANY: DWRITE_PANOSE_SYMBOL_KIND = 0;
pub const DWRITE_PANOSE_SYMBOL_KIND_BOARDERS: DWRITE_PANOSE_SYMBOL_KIND = 9;
pub const DWRITE_PANOSE_SYMBOL_KIND_EXPERT: DWRITE_PANOSE_SYMBOL_KIND = 7;
pub const DWRITE_PANOSE_SYMBOL_KIND_ICONS: DWRITE_PANOSE_SYMBOL_KIND = 10;
pub const DWRITE_PANOSE_SYMBOL_KIND_INDUSTRY_SPECIFIC: DWRITE_PANOSE_SYMBOL_KIND = 12;
pub const DWRITE_PANOSE_SYMBOL_KIND_LOGOS: DWRITE_PANOSE_SYMBOL_KIND = 11;
pub const DWRITE_PANOSE_SYMBOL_KIND_MONTAGES: DWRITE_PANOSE_SYMBOL_KIND = 2;
pub const DWRITE_PANOSE_SYMBOL_KIND_MUSIC: DWRITE_PANOSE_SYMBOL_KIND = 6;
pub const DWRITE_PANOSE_SYMBOL_KIND_NO_FIT: DWRITE_PANOSE_SYMBOL_KIND = 1;
pub const DWRITE_PANOSE_SYMBOL_KIND_PATTERNS: DWRITE_PANOSE_SYMBOL_KIND = 8;
pub const DWRITE_PANOSE_SYMBOL_KIND_PICTURES: DWRITE_PANOSE_SYMBOL_KIND = 3;
pub const DWRITE_PANOSE_SYMBOL_KIND_SCIENTIFIC: DWRITE_PANOSE_SYMBOL_KIND = 5;
pub const DWRITE_PANOSE_SYMBOL_KIND_SHAPES: DWRITE_PANOSE_SYMBOL_KIND = 4;
pub type DWRITE_PANOSE_TOOL_KIND = i32;
pub const DWRITE_PANOSE_TOOL_KIND_ANY: DWRITE_PANOSE_TOOL_KIND = 0;
pub const DWRITE_PANOSE_TOOL_KIND_BALL: DWRITE_PANOSE_TOOL_KIND = 5;
pub const DWRITE_PANOSE_TOOL_KIND_BRUSH: DWRITE_PANOSE_TOOL_KIND = 6;
pub const DWRITE_PANOSE_TOOL_KIND_ENGRAVED: DWRITE_PANOSE_TOOL_KIND = 4;
pub const DWRITE_PANOSE_TOOL_KIND_FELT_PEN_BRUSH_TIP: DWRITE_PANOSE_TOOL_KIND = 8;
pub const DWRITE_PANOSE_TOOL_KIND_FLAT_NIB: DWRITE_PANOSE_TOOL_KIND = 2;
pub const DWRITE_PANOSE_TOOL_KIND_NO_FIT: DWRITE_PANOSE_TOOL_KIND = 1;
pub const DWRITE_PANOSE_TOOL_KIND_PRESSURE_POINT: DWRITE_PANOSE_TOOL_KIND = 3;
pub const DWRITE_PANOSE_TOOL_KIND_ROUGH: DWRITE_PANOSE_TOOL_KIND = 7;
pub const DWRITE_PANOSE_TOOL_KIND_WILD_BRUSH: DWRITE_PANOSE_TOOL_KIND = 9;
pub type DWRITE_PANOSE_WEIGHT = i32;
pub const DWRITE_PANOSE_WEIGHT_ANY: DWRITE_PANOSE_WEIGHT = 0;
pub const DWRITE_PANOSE_WEIGHT_BLACK: DWRITE_PANOSE_WEIGHT = 10;
pub const DWRITE_PANOSE_WEIGHT_BOLD: DWRITE_PANOSE_WEIGHT = 8;
pub const DWRITE_PANOSE_WEIGHT_BOOK: DWRITE_PANOSE_WEIGHT = 5;
pub const DWRITE_PANOSE_WEIGHT_DEMI: DWRITE_PANOSE_WEIGHT = 7;
pub const DWRITE_PANOSE_WEIGHT_EXTRA_BLACK: DWRITE_PANOSE_WEIGHT = 11;
pub const DWRITE_PANOSE_WEIGHT_HEAVY: DWRITE_PANOSE_WEIGHT = 9;
pub const DWRITE_PANOSE_WEIGHT_LIGHT: DWRITE_PANOSE_WEIGHT = 3;
pub const DWRITE_PANOSE_WEIGHT_MEDIUM: DWRITE_PANOSE_WEIGHT = 6;
pub const DWRITE_PANOSE_WEIGHT_NORD: DWRITE_PANOSE_WEIGHT = 11;
pub const DWRITE_PANOSE_WEIGHT_NO_FIT: DWRITE_PANOSE_WEIGHT = 1;
pub const DWRITE_PANOSE_WEIGHT_THIN: DWRITE_PANOSE_WEIGHT = 4;
pub const DWRITE_PANOSE_WEIGHT_VERY_LIGHT: DWRITE_PANOSE_WEIGHT = 2;
pub type DWRITE_PANOSE_XASCENT = i32;
pub const DWRITE_PANOSE_XASCENT_ANY: DWRITE_PANOSE_XASCENT = 0;
pub const DWRITE_PANOSE_XASCENT_HIGH: DWRITE_PANOSE_XASCENT = 5;
pub const DWRITE_PANOSE_XASCENT_LOW: DWRITE_PANOSE_XASCENT = 3;
pub const DWRITE_PANOSE_XASCENT_MEDIUM: DWRITE_PANOSE_XASCENT = 4;
pub const DWRITE_PANOSE_XASCENT_NO_FIT: DWRITE_PANOSE_XASCENT = 1;
pub const DWRITE_PANOSE_XASCENT_VERY_HIGH: DWRITE_PANOSE_XASCENT = 6;
pub const DWRITE_PANOSE_XASCENT_VERY_LOW: DWRITE_PANOSE_XASCENT = 2;
pub type DWRITE_PANOSE_XHEIGHT = i32;
pub const DWRITE_PANOSE_XHEIGHT_ANY: DWRITE_PANOSE_XHEIGHT = 0;
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_LARGE: DWRITE_PANOSE_XHEIGHT = 4;
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_SMALL: DWRITE_PANOSE_XHEIGHT = 2;
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STANDARD: DWRITE_PANOSE_XHEIGHT = 3;
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STD: DWRITE_PANOSE_XHEIGHT = 3;
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_LARGE: DWRITE_PANOSE_XHEIGHT = 7;
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_SMALL: DWRITE_PANOSE_XHEIGHT = 5;
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STANDARD: DWRITE_PANOSE_XHEIGHT = 6;
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STD: DWRITE_PANOSE_XHEIGHT = 6;
pub const DWRITE_PANOSE_XHEIGHT_NO_FIT: DWRITE_PANOSE_XHEIGHT = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_SCRIPT_PROPERTIES {
    pub isoScriptCode: u32,
    pub isoScriptNumber: u32,
    pub clusterLookahead: u32,
    pub justificationCharacter: u32,
    pub _bitfield: u32,
}
pub type DWRITE_TEXT_ANTIALIAS_MODE = i32;
pub const DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE: DWRITE_TEXT_ANTIALIAS_MODE = 0;
pub const DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE: DWRITE_TEXT_ANTIALIAS_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_UNICODE_RANGE {
    pub first: u32,
    pub last: u32,
}
pub type DWRITE_VERTICAL_GLYPH_ORIENTATION = i32;
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT: DWRITE_VERTICAL_GLYPH_ORIENTATION = 0;
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED: DWRITE_VERTICAL_GLYPH_ORIENTATION = 1;
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteBitmapRenderTarget1, IDWriteBitmapRenderTarget1_Vtbl, 0x791e8298_3ef3_4230_9880_c9bdecc42064);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteBitmapRenderTarget1 {
    type Target = super::dwrite::IDWriteBitmapRenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget1, windows_core::IUnknown, super::dwrite::IDWriteBitmapRenderTarget);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteBitmapRenderTarget1 {
    pub unsafe fn GetTextAntialiasMode(&self) -> DWRITE_TEXT_ANTIALIAS_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetTextAntialiasMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetTextAntialiasMode(&self, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextAntialiasMode)(windows_core::Interface::as_raw(self), antialiasmode) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteBitmapRenderTarget1_Vtbl {
    pub base__: super::dwrite::IDWriteBitmapRenderTarget_Vtbl,
    pub GetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE,
    pub SetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_windef"))]
pub trait IDWriteBitmapRenderTarget1_Impl: super::dwrite::IDWriteBitmapRenderTarget_Impl {
    fn GetTextAntialiasMode(&self) -> DWRITE_TEXT_ANTIALIAS_MODE;
    fn SetTextAntialiasMode(&self, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_windef"))]
impl IDWriteBitmapRenderTarget1_Vtbl {
    pub const fn new<Identity: IDWriteBitmapRenderTarget1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTextAntialiasMode<Identity: IDWriteBitmapRenderTarget1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget1_Impl::GetTextAntialiasMode(this)
            }
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: IDWriteBitmapRenderTarget1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteBitmapRenderTarget1_Impl::SetTextAntialiasMode(this, core::mem::transmute_copy(&antialiasmode)).into()
            }
        }
        Self {
            base__: super::dwrite::IDWriteBitmapRenderTarget_Vtbl::new::<Identity, OFFSET>(),
            GetTextAntialiasMode: GetTextAntialiasMode::<Identity, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteBitmapRenderTarget1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteBitmapRenderTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteBitmapRenderTarget1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFactory1, IDWriteFactory1_Vtbl, 0x30572f99_dac6_41db_a16e_0486307e606a);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFactory1 {
    type Target = super::dwrite::IDWriteFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFactory1, windows_core::IUnknown, super::dwrite::IDWriteFactory);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFactory1 {
    pub unsafe fn GetEudcFontCollection(&self, fontcollection: *mut Option<super::dwrite::IDWriteFontCollection>, checkforupdates: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEudcFontCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(fontcollection), checkforupdates.into()) }
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: super::dwrite::DWRITE_RENDERING_MODE) -> windows_core::Result<IDWriteRenderingParams1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(windows_core::Interface::as_raw(self), gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory1_Vtbl {
    pub base__: super::dwrite::IDWriteFactory_Vtbl,
    pub GetEudcFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, super::dwrite::DWRITE_PIXEL_GEOMETRY, super::dwrite::DWRITE_RENDERING_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IDWriteFactory1_Impl: super::dwrite::IDWriteFactory_Impl {
    fn GetEudcFontCollection(&self, fontcollection: windows_core::OutRef<super::dwrite::IDWriteFontCollection>, checkforupdates: windows_core::BOOL) -> windows_core::Result<()>;
    fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: super::dwrite::DWRITE_RENDERING_MODE) -> windows_core::Result<IDWriteRenderingParams1>;
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IDWriteFactory1_Vtbl {
    pub const fn new<Identity: IDWriteFactory1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEudcFontCollection<Identity: IDWriteFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontcollection: *mut *mut core::ffi::c_void, checkforupdates: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory1_Impl::GetEudcFontCollection(this, core::mem::transmute_copy(&fontcollection), core::mem::transmute_copy(&checkforupdates)).into()
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: IDWriteFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: super::dwrite::DWRITE_PIXEL_GEOMETRY, renderingmode: super::dwrite::DWRITE_RENDERING_MODE, renderingparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory1_Impl::CreateCustomRenderingParams(this, core::mem::transmute_copy(&gamma), core::mem::transmute_copy(&enhancedcontrast), core::mem::transmute_copy(&enhancedcontrastgrayscale), core::mem::transmute_copy(&cleartypelevel), core::mem::transmute_copy(&pixelgeometry), core::mem::transmute_copy(&renderingmode)) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dwrite::IDWriteFactory_Vtbl::new::<Identity, OFFSET>(),
            GetEudcFontCollection: GetEudcFontCollection::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDWriteFactory1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFont1, IDWriteFont1_Vtbl, 0xacd16696_8c14_4f5d_877e_fe3fc1d32738);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFont1 {
    type Target = super::dwrite::IDWriteFont;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFont1, windows_core::IUnknown, super::dwrite::IDWriteFont);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFont1 {
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), fontmetrics as _);
        }
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPanose)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUnicodeRanges)(windows_core::Interface::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(unicoderanges.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), actualrangecount as _) }
    }
    pub unsafe fn IsMonospacedFont(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsMonospacedFont)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFont1_Vtbl {
    pub base__: super::dwrite::IDWriteFont_Vtbl,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS1),
    pub GetPanose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_PANOSE),
    pub GetUnicodeRanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_UNICODE_RANGE, *mut u32) -> windows_core::HRESULT,
    pub IsMonospacedFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteFont1_Impl: super::dwrite::IDWriteFont_Impl {
    fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1);
    fn GetPanose(&self, panose: *mut DWRITE_PANOSE);
    fn GetUnicodeRanges(&self, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> windows_core::Result<()>;
    fn IsMonospacedFont(&self) -> windows_core::BOOL;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFont1_Vtbl {
    pub const fn new<Identity: IDWriteFont1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetrics<Identity: IDWriteFont1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont1_Impl::GetMetrics(this, core::mem::transmute_copy(&fontmetrics));
            }
        }
        unsafe extern "system" fn GetPanose<Identity: IDWriteFont1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont1_Impl::GetPanose(this, core::mem::transmute_copy(&panose));
            }
        }
        unsafe extern "system" fn GetUnicodeRanges<Identity: IDWriteFont1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont1_Impl::GetUnicodeRanges(this, core::mem::transmute_copy(&maxrangecount), core::mem::transmute_copy(&unicoderanges), core::mem::transmute_copy(&actualrangecount)).into()
            }
        }
        unsafe extern "system" fn IsMonospacedFont<Identity: IDWriteFont1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFont1_Impl::IsMonospacedFont(this)
            }
        }
        Self {
            base__: super::dwrite::IDWriteFont_Vtbl::new::<Identity, OFFSET>(),
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetPanose: GetPanose::<Identity, OFFSET>,
            GetUnicodeRanges: GetUnicodeRanges::<Identity, OFFSET>,
            IsMonospacedFont: IsMonospacedFont::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFont1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFont as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteFont1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteFontFace1, IDWriteFontFace1_Vtbl, 0xa71efdb4_9fdb_4838_ad90_cfc3be8c3daf);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteFontFace1 {
    type Target = super::dwrite::IDWriteFontFace;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteFontFace1, windows_core::IUnknown, super::dwrite::IDWriteFontFace);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteFontFace1 {
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), fontmetrics as _);
        }
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: Option<*const super::dwrite::DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGdiCompatibleMetrics)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, transform.unwrap_or(core::mem::zeroed()) as _, fontmetrics as _) }
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretMetrics)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUnicodeRanges)(windows_core::Interface::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(unicoderanges.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), actualrangecount as _) }
    }
    pub unsafe fn IsMonospacedFont(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsMonospacedFont)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDesignGlyphAdvances(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesignGlyphAdvances)(windows_core::Interface::as_raw(self), glyphcount, glyphindices, glyphadvances as _, issideways.into()) }
    }
    pub unsafe fn GetGdiCompatibleGlyphAdvances(&self, emsize: f32, pixelsperdip: f32, transform: Option<*const super::dwrite::DWRITE_MATRIX>, usegdinatural: bool, issideways: bool, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGdiCompatibleGlyphAdvances)(windows_core::Interface::as_raw(self), emsize, pixelsperdip, transform.unwrap_or(core::mem::zeroed()) as _, usegdinatural.into(), issideways.into(), glyphcount, glyphindices, glyphadvances as _) }
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKerningPairAdjustments)(windows_core::Interface::as_raw(self), glyphcount, glyphindices, glyphadvanceadjustments as _) }
    }
    pub unsafe fn HasKerningPairs(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasKerningPairs)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetRecommendedRenderingMode(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: Option<*const super::dwrite::DWRITE_MATRIX>, issideways: bool, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) -> windows_core::Result<super::dwrite::DWRITE_RENDERING_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecommendedRenderingMode)(windows_core::Interface::as_raw(self), fontemsize, dpix, dpiy, transform.unwrap_or(core::mem::zeroed()) as _, issideways.into(), outlinethreshold, measuringmode, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVerticalGlyphVariants)(windows_core::Interface::as_raw(self), glyphcount, nominalglyphindices, verticalglyphindices as _) }
    }
    pub unsafe fn HasVerticalGlyphVariants(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasVerticalGlyphVariants)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace1_Vtbl {
    pub base__: super::dwrite::IDWriteFontFace_Vtbl,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_FONT_METRICS1),
    pub GetGdiCompatibleMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const super::dwrite::DWRITE_MATRIX, *mut DWRITE_FONT_METRICS1) -> windows_core::HRESULT,
    pub GetCaretMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_CARET_METRICS),
    pub GetUnicodeRanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_UNICODE_RANGE, *mut u32) -> windows_core::HRESULT,
    pub IsMonospacedFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDesignGlyphAdvances: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut i32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetGdiCompatibleGlyphAdvances: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, *const super::dwrite::DWRITE_MATRIX, windows_core::BOOL, windows_core::BOOL, u32, *const u16, *mut i32) -> windows_core::HRESULT,
    pub GetKerningPairAdjustments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut i32) -> windows_core::HRESULT,
    pub HasKerningPairs: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(feature = "Win32_dcommon")]
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, *const super::dwrite::DWRITE_MATRIX, windows_core::BOOL, DWRITE_OUTLINE_THRESHOLD, super::dcommon::DWRITE_MEASURING_MODE, *mut super::dwrite::DWRITE_RENDERING_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetRecommendedRenderingMode: usize,
    pub GetVerticalGlyphVariants: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut u16) -> windows_core::HRESULT,
    pub HasVerticalGlyphVariants: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite"))]
pub trait IDWriteFontFace1_Impl: super::dwrite::IDWriteFontFace_Impl {
    fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS1);
    fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: *const super::dwrite::DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> windows_core::Result<()>;
    fn GetCaretMetrics(&self, caretmetrics: *mut DWRITE_CARET_METRICS);
    fn GetUnicodeRanges(&self, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> windows_core::Result<()>;
    fn IsMonospacedFont(&self) -> windows_core::BOOL;
    fn GetDesignGlyphAdvances(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetGdiCompatibleGlyphAdvances(&self, emsize: f32, pixelsperdip: f32, transform: *const super::dwrite::DWRITE_MATRIX, usegdinatural: windows_core::BOOL, issideways: windows_core::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> windows_core::Result<()>;
    fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> windows_core::Result<()>;
    fn HasKerningPairs(&self) -> windows_core::BOOL;
    fn GetRecommendedRenderingMode(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const super::dwrite::DWRITE_MATRIX, issideways: windows_core::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) -> windows_core::Result<super::dwrite::DWRITE_RENDERING_MODE>;
    fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> windows_core::Result<()>;
    fn HasVerticalGlyphVariants(&self) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite"))]
impl IDWriteFontFace1_Vtbl {
    pub const fn new<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetrics<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::GetMetrics(this, core::mem::transmute_copy(&fontmetrics));
            }
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const super::dwrite::DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::GetGdiCompatibleMetrics(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&fontmetrics)).into()
            }
        }
        unsafe extern "system" fn GetCaretMetrics<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, caretmetrics: *mut DWRITE_CARET_METRICS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::GetCaretMetrics(this, core::mem::transmute_copy(&caretmetrics));
            }
        }
        unsafe extern "system" fn GetUnicodeRanges<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::GetUnicodeRanges(this, core::mem::transmute_copy(&maxrangecount), core::mem::transmute_copy(&unicoderanges), core::mem::transmute_copy(&actualrangecount)).into()
            }
        }
        unsafe extern "system" fn IsMonospacedFont<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::IsMonospacedFont(this)
            }
        }
        unsafe extern "system" fn GetDesignGlyphAdvances<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::GetDesignGlyphAdvances(this, core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphadvances), core::mem::transmute_copy(&issideways)).into()
            }
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphAdvances<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const super::dwrite::DWRITE_MATRIX, usegdinatural: windows_core::BOOL, issideways: windows_core::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::GetGdiCompatibleGlyphAdvances(this, core::mem::transmute_copy(&emsize), core::mem::transmute_copy(&pixelsperdip), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&usegdinatural), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphadvances)).into()
            }
        }
        unsafe extern "system" fn GetKerningPairAdjustments<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::GetKerningPairAdjustments(this, core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&glyphindices), core::mem::transmute_copy(&glyphadvanceadjustments)).into()
            }
        }
        unsafe extern "system" fn HasKerningPairs<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::HasKerningPairs(this)
            }
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const super::dwrite::DWRITE_MATRIX, issideways: windows_core::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, renderingmode: *mut super::dwrite::DWRITE_RENDERING_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFontFace1_Impl::GetRecommendedRenderingMode(this, core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&transform), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&outlinethreshold), core::mem::transmute_copy(&measuringmode)) {
                    Ok(ok__) => {
                        renderingmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVerticalGlyphVariants<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::GetVerticalGlyphVariants(this, core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&nominalglyphindices), core::mem::transmute_copy(&verticalglyphindices)).into()
            }
        }
        unsafe extern "system" fn HasVerticalGlyphVariants<Identity: IDWriteFontFace1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFontFace1_Impl::HasVerticalGlyphVariants(this)
            }
        }
        Self {
            base__: super::dwrite::IDWriteFontFace_Vtbl::new::<Identity, OFFSET>(),
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetGdiCompatibleMetrics: GetGdiCompatibleMetrics::<Identity, OFFSET>,
            GetCaretMetrics: GetCaretMetrics::<Identity, OFFSET>,
            GetUnicodeRanges: GetUnicodeRanges::<Identity, OFFSET>,
            IsMonospacedFont: IsMonospacedFont::<Identity, OFFSET>,
            GetDesignGlyphAdvances: GetDesignGlyphAdvances::<Identity, OFFSET>,
            GetGdiCompatibleGlyphAdvances: GetGdiCompatibleGlyphAdvances::<Identity, OFFSET>,
            GetKerningPairAdjustments: GetKerningPairAdjustments::<Identity, OFFSET>,
            HasKerningPairs: HasKerningPairs::<Identity, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, OFFSET>,
            GetVerticalGlyphVariants: GetVerticalGlyphVariants::<Identity, OFFSET>,
            HasVerticalGlyphVariants: HasVerticalGlyphVariants::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFontFace1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteFontFace as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_dwrite"))]
impl windows_core::RuntimeName for IDWriteFontFace1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteRenderingParams1, IDWriteRenderingParams1_Vtbl, 0x94413cf4_a6fc_4248_8b50_6674348fcad3);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteRenderingParams1 {
    type Target = super::dwrite::IDWriteRenderingParams;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams1, windows_core::IUnknown, super::dwrite::IDWriteRenderingParams);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteRenderingParams1 {
    pub unsafe fn GetGrayscaleEnhancedContrast(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetGrayscaleEnhancedContrast)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRenderingParams1_Vtbl {
    pub base__: super::dwrite::IDWriteRenderingParams_Vtbl,
    pub GetGrayscaleEnhancedContrast: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteRenderingParams1_Impl: super::dwrite::IDWriteRenderingParams_Impl {
    fn GetGrayscaleEnhancedContrast(&self) -> f32;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteRenderingParams1_Vtbl {
    pub const fn new<Identity: IDWriteRenderingParams1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGrayscaleEnhancedContrast<Identity: IDWriteRenderingParams1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteRenderingParams1_Impl::GetGrayscaleEnhancedContrast(this)
            }
        }
        Self {
            base__: super::dwrite::IDWriteRenderingParams_Vtbl::new::<Identity, OFFSET>(),
            GetGrayscaleEnhancedContrast: GetGrayscaleEnhancedContrast::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteRenderingParams1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteRenderingParams as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteRenderingParams1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteTextAnalysisSink1, IDWriteTextAnalysisSink1_Vtbl, 0xb0d941a0_85e7_4d8b_9fd3_5ced9934482a);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteTextAnalysisSink1 {
    type Target = super::dwrite::IDWriteTextAnalysisSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSink1, windows_core::IUnknown, super::dwrite::IDWriteTextAnalysisSink);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextAnalysisSink1 {
    pub unsafe fn SetGlyphOrientation(&self, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: bool, isrighttoleft: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGlyphOrientation)(windows_core::Interface::as_raw(self), textposition, textlength, glyphorientationangle, adjustedbidilevel, issideways.into(), isrighttoleft.into()) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalysisSink1_Vtbl {
    pub base__: super::dwrite::IDWriteTextAnalysisSink_Vtbl,
    pub SetGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, DWRITE_GLYPH_ORIENTATION_ANGLE, u8, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteTextAnalysisSink1_Impl: super::dwrite::IDWriteTextAnalysisSink_Impl {
    fn SetGlyphOrientation(&self, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextAnalysisSink1_Vtbl {
    pub const fn new<Identity: IDWriteTextAnalysisSink1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGlyphOrientation<Identity: IDWriteTextAnalysisSink1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: windows_core::BOOL, isrighttoleft: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSink1_Impl::SetGlyphOrientation(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&glyphorientationangle), core::mem::transmute_copy(&adjustedbidilevel), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&isrighttoleft)).into()
            }
        }
        Self { base__: super::dwrite::IDWriteTextAnalysisSink_Vtbl::new::<Identity, OFFSET>(), SetGlyphOrientation: SetGlyphOrientation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSink1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextAnalysisSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteTextAnalysisSink1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteTextAnalysisSource1, IDWriteTextAnalysisSource1_Vtbl, 0x639cfad8_0fb4_4b21_a58a_067920120009);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteTextAnalysisSource1 {
    type Target = super::dwrite::IDWriteTextAnalysisSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSource1, windows_core::IUnknown, super::dwrite::IDWriteTextAnalysisSource);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextAnalysisSource1 {
    pub unsafe fn GetVerticalGlyphOrientation(&self, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVerticalGlyphOrientation)(windows_core::Interface::as_raw(self), textposition, textlength as _, glyphorientation as _, bidilevel as _) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalysisSource1_Vtbl {
    pub base__: super::dwrite::IDWriteTextAnalysisSource_Vtbl,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, *mut u8) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteTextAnalysisSource1_Impl: super::dwrite::IDWriteTextAnalysisSource_Impl {
    fn GetVerticalGlyphOrientation(&self, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextAnalysisSource1_Vtbl {
    pub const fn new<Identity: IDWriteTextAnalysisSource1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: IDWriteTextAnalysisSource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalysisSource1_Impl::GetVerticalGlyphOrientation(this, core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&glyphorientation), core::mem::transmute_copy(&bidilevel)).into()
            }
        }
        Self {
            base__: super::dwrite::IDWriteTextAnalysisSource_Vtbl::new::<Identity, OFFSET>(),
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalysisSource1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextAnalysisSource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteTextAnalysisSource1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteTextAnalyzer1, IDWriteTextAnalyzer1_Vtbl, 0x80dad800_e21f_4e83_96ce_bfcce500db7c);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteTextAnalyzer1 {
    type Target = super::dwrite::IDWriteTextAnalyzer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer1, windows_core::IUnknown, super::dwrite::IDWriteTextAnalyzer);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextAnalyzer1 {
    pub unsafe fn ApplyCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, glyphcount: u32, clustermap: &[u16], glyphadvances: *const f32, glyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ApplyCharacterSpacing)(windows_core::Interface::as_raw(self), leadingspacing, trailingspacing, minimumadvancewidth, clustermap.len().try_into().unwrap(), glyphcount, core::mem::transmute(clustermap.as_ptr()), glyphadvances, glyphoffsets, glyphproperties, modifiedglyphadvances as _, modifiedglyphoffsets as _) }
    }
    pub unsafe fn GetBaseline<P0, P5>(&self, fontface: P0, baseline: DWRITE_BASELINE, isvertical: bool, issimulationallowed: bool, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: P5, baselinecoordinate: *mut i32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFace>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetBaseline)(windows_core::Interface::as_raw(self), fontface.param().abi(), baseline, isvertical.into(), issimulationallowed.into(), core::mem::transmute(scriptanalysis), localename.param().abi(), baselinecoordinate as _, exists as _) }
    }
    pub unsafe fn AnalyzeVerticalGlyphOrientation<P0, P3>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteTextAnalysisSource1>,
        P3: windows_core::Param<IDWriteTextAnalysisSink1>,
    {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeVerticalGlyphOrientation)(windows_core::Interface::as_raw(self), analysissource.param().abi(), textposition, textlength, analysissink.param().abi()) }
    }
    pub unsafe fn GetGlyphOrientationTransform(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: bool, transform: *mut super::dwrite::DWRITE_MATRIX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGlyphOrientationTransform)(windows_core::Interface::as_raw(self), glyphorientationangle, issideways.into(), transform as _) }
    }
    pub unsafe fn GetScriptProperties(&self, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScriptProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(scriptanalysis), scriptproperties as _) }
    }
    pub unsafe fn GetTextComplexity<P2>(&self, textstring: *const u16, textlength: u32, fontface: P2, istextsimple: *mut windows_core::BOOL, textlengthread: *mut u32, glyphindices: Option<*mut u16>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::dwrite::IDWriteFontFace>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTextComplexity)(windows_core::Interface::as_raw(self), textstring, textlength, fontface.param().abi(), istextsimple as _, textlengthread as _, glyphindices.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetJustificationOpportunities<P0>(&self, fontface: P0, fontemsize: f32, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: *const u16, clustermap: *const u16, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFace>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetJustificationOpportunities)(windows_core::Interface::as_raw(self), fontface.param().abi(), fontemsize, core::mem::transmute(scriptanalysis), textlength, glyphcount, textstring, clustermap, glyphproperties, justificationopportunities as _) }
    }
    pub unsafe fn JustifyGlyphAdvances(&self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: Option<*mut super::dwrite::DWRITE_GLYPH_OFFSET>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).JustifyGlyphAdvances)(windows_core::Interface::as_raw(self), linewidth, glyphcount, justificationopportunities, glyphadvances, glyphoffsets, justifiedglyphadvances as _, justifiedglyphoffsets.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetJustifiedGlyphs<P0>(&self, fontface: P0, fontemsize: f32, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: Option<*const u16>, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: Option<*mut u16>, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::dwrite::IDWriteFontFace>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetJustifiedGlyphs)(windows_core::Interface::as_raw(self), fontface.param().abi(), fontemsize, core::mem::transmute(scriptanalysis), textlength, glyphcount, maxglyphcount, clustermap.unwrap_or(core::mem::zeroed()) as _, glyphindices, glyphadvances, justifiedglyphadvances, justifiedglyphoffsets, glyphproperties, actualglyphcount as _, modifiedclustermap.unwrap_or(core::mem::zeroed()) as _, modifiedglyphindices as _, modifiedglyphadvances as _, modifiedglyphoffsets as _) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalyzer1_Vtbl {
    pub base__: super::dwrite::IDWriteTextAnalyzer_Vtbl,
    pub ApplyCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32, u32, *const u16, *const f32, *const super::dwrite::DWRITE_GLYPH_OFFSET, *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, *mut f32, *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
    pub GetBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DWRITE_BASELINE, windows_core::BOOL, windows_core::BOOL, super::dwrite::DWRITE_SCRIPT_ANALYSIS, windows_core::PCWSTR, *mut i32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub AnalyzeVerticalGlyphOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGlyphOrientationTransform: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_GLYPH_ORIENTATION_ANGLE, windows_core::BOOL, *mut super::dwrite::DWRITE_MATRIX) -> windows_core::HRESULT,
    pub GetScriptProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::dwrite::DWRITE_SCRIPT_ANALYSIS, *mut DWRITE_SCRIPT_PROPERTIES) -> windows_core::HRESULT,
    pub GetTextComplexity: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut u32, *mut u16) -> windows_core::HRESULT,
    pub GetJustificationOpportunities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, super::dwrite::DWRITE_SCRIPT_ANALYSIS, u32, u32, *const u16, *const u16, *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> windows_core::HRESULT,
    pub JustifyGlyphAdvances: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32, *const DWRITE_JUSTIFICATION_OPPORTUNITY, *const f32, *const super::dwrite::DWRITE_GLYPH_OFFSET, *mut f32, *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
    pub GetJustifiedGlyphs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, super::dwrite::DWRITE_SCRIPT_ANALYSIS, u32, u32, u32, *const u16, *const u16, *const f32, *const f32, *const super::dwrite::DWRITE_GLYPH_OFFSET, *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, *mut u32, *mut u16, *mut u16, *mut f32, *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteTextAnalyzer1_Impl: super::dwrite::IDWriteTextAnalyzer_Impl {
    fn ApplyCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
    fn GetBaseline(&self, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, baseline: DWRITE_BASELINE, isvertical: windows_core::BOOL, issimulationallowed: windows_core::BOOL, scriptanalysis: &super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: &windows_core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn AnalyzeVerticalGlyphOrientation(&self, analysissource: windows_core::Ref<IDWriteTextAnalysisSource1>, textposition: u32, textlength: u32, analysissink: windows_core::Ref<IDWriteTextAnalysisSink1>) -> windows_core::Result<()>;
    fn GetGlyphOrientationTransform(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: windows_core::BOOL, transform: *mut super::dwrite::DWRITE_MATRIX) -> windows_core::Result<()>;
    fn GetScriptProperties(&self, scriptanalysis: &super::dwrite::DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> windows_core::Result<()>;
    fn GetTextComplexity(&self, textstring: *const u16, textlength: u32, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, istextsimple: *mut windows_core::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> windows_core::Result<()>;
    fn GetJustificationOpportunities(&self, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, fontemsize: f32, scriptanalysis: &super::dwrite::DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: *const u16, clustermap: *const u16, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> windows_core::Result<()>;
    fn JustifyGlyphAdvances(&self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
    fn GetJustifiedGlyphs(&self, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, fontemsize: f32, scriptanalysis: &super::dwrite::DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextAnalyzer1_Vtbl {
    pub const fn new<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ApplyCharacterSpacing<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::ApplyCharacterSpacing(
                    this,
                    core::mem::transmute_copy(&leadingspacing),
                    core::mem::transmute_copy(&trailingspacing),
                    core::mem::transmute_copy(&minimumadvancewidth),
                    core::mem::transmute_copy(&textlength),
                    core::mem::transmute_copy(&glyphcount),
                    core::mem::transmute_copy(&clustermap),
                    core::mem::transmute_copy(&glyphadvances),
                    core::mem::transmute_copy(&glyphoffsets),
                    core::mem::transmute_copy(&glyphproperties),
                    core::mem::transmute_copy(&modifiedglyphadvances),
                    core::mem::transmute_copy(&modifiedglyphoffsets),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetBaseline<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, baseline: DWRITE_BASELINE, isvertical: windows_core::BOOL, issimulationallowed: windows_core::BOOL, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, localename: windows_core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::GetBaseline(this, core::mem::transmute_copy(&fontface), core::mem::transmute_copy(&baseline), core::mem::transmute_copy(&isvertical), core::mem::transmute_copy(&issimulationallowed), core::mem::transmute(&scriptanalysis), core::mem::transmute(&localename), core::mem::transmute_copy(&baselinecoordinate), core::mem::transmute_copy(&exists)).into()
            }
        }
        unsafe extern "system" fn AnalyzeVerticalGlyphOrientation<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, analysissource: *mut core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::AnalyzeVerticalGlyphOrientation(this, core::mem::transmute_copy(&analysissource), core::mem::transmute_copy(&textposition), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&analysissink)).into()
            }
        }
        unsafe extern "system" fn GetGlyphOrientationTransform<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: windows_core::BOOL, transform: *mut super::dwrite::DWRITE_MATRIX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::GetGlyphOrientationTransform(this, core::mem::transmute_copy(&glyphorientationangle), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn GetScriptProperties<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::GetScriptProperties(this, core::mem::transmute(&scriptanalysis), core::mem::transmute_copy(&scriptproperties)).into()
            }
        }
        unsafe extern "system" fn GetTextComplexity<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textstring: *const u16, textlength: u32, fontface: *mut core::ffi::c_void, istextsimple: *mut windows_core::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::GetTextComplexity(this, core::mem::transmute_copy(&textstring), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&fontface), core::mem::transmute_copy(&istextsimple), core::mem::transmute_copy(&textlengthread), core::mem::transmute_copy(&glyphindices)).into()
            }
        }
        unsafe extern "system" fn GetJustificationOpportunities<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, fontemsize: f32, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: *const u16, clustermap: *const u16, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::GetJustificationOpportunities(this, core::mem::transmute_copy(&fontface), core::mem::transmute_copy(&fontemsize), core::mem::transmute(&scriptanalysis), core::mem::transmute_copy(&textlength), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&textstring), core::mem::transmute_copy(&clustermap), core::mem::transmute_copy(&glyphproperties), core::mem::transmute_copy(&justificationopportunities)).into()
            }
        }
        unsafe extern "system" fn JustifyGlyphAdvances<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::JustifyGlyphAdvances(this, core::mem::transmute_copy(&linewidth), core::mem::transmute_copy(&glyphcount), core::mem::transmute_copy(&justificationopportunities), core::mem::transmute_copy(&glyphadvances), core::mem::transmute_copy(&glyphoffsets), core::mem::transmute_copy(&justifiedglyphadvances), core::mem::transmute_copy(&justifiedglyphoffsets)).into()
            }
        }
        unsafe extern "system" fn GetJustifiedGlyphs<Identity: IDWriteTextAnalyzer1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontface: *mut core::ffi::c_void, fontemsize: f32, scriptanalysis: super::dwrite::DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const super::dwrite::DWRITE_GLYPH_OFFSET, glyphproperties: *const super::dwrite::DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut super::dwrite::DWRITE_GLYPH_OFFSET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextAnalyzer1_Impl::GetJustifiedGlyphs(
                    this,
                    core::mem::transmute_copy(&fontface),
                    core::mem::transmute_copy(&fontemsize),
                    core::mem::transmute(&scriptanalysis),
                    core::mem::transmute_copy(&textlength),
                    core::mem::transmute_copy(&glyphcount),
                    core::mem::transmute_copy(&maxglyphcount),
                    core::mem::transmute_copy(&clustermap),
                    core::mem::transmute_copy(&glyphindices),
                    core::mem::transmute_copy(&glyphadvances),
                    core::mem::transmute_copy(&justifiedglyphadvances),
                    core::mem::transmute_copy(&justifiedglyphoffsets),
                    core::mem::transmute_copy(&glyphproperties),
                    core::mem::transmute_copy(&actualglyphcount),
                    core::mem::transmute_copy(&modifiedclustermap),
                    core::mem::transmute_copy(&modifiedglyphindices),
                    core::mem::transmute_copy(&modifiedglyphadvances),
                    core::mem::transmute_copy(&modifiedglyphoffsets),
                )
                .into()
            }
        }
        Self {
            base__: super::dwrite::IDWriteTextAnalyzer_Vtbl::new::<Identity, OFFSET>(),
            ApplyCharacterSpacing: ApplyCharacterSpacing::<Identity, OFFSET>,
            GetBaseline: GetBaseline::<Identity, OFFSET>,
            AnalyzeVerticalGlyphOrientation: AnalyzeVerticalGlyphOrientation::<Identity, OFFSET>,
            GetGlyphOrientationTransform: GetGlyphOrientationTransform::<Identity, OFFSET>,
            GetScriptProperties: GetScriptProperties::<Identity, OFFSET>,
            GetTextComplexity: GetTextComplexity::<Identity, OFFSET>,
            GetJustificationOpportunities: GetJustificationOpportunities::<Identity, OFFSET>,
            JustifyGlyphAdvances: JustifyGlyphAdvances::<Identity, OFFSET>,
            GetJustifiedGlyphs: GetJustifiedGlyphs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextAnalyzer1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextAnalyzer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteTextAnalyzer1 {}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::define_interface!(IDWriteTextLayout1, IDWriteTextLayout1_Vtbl, 0x9064d822_80a7_465c_a986_df65f78b8feb);
#[cfg(feature = "Win32_dwrite")]
impl core::ops::Deref for IDWriteTextLayout1 {
    type Target = super::dwrite::IDWriteTextLayout;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_dwrite")]
windows_core::imp::interface_hierarchy!(IDWriteTextLayout1, windows_core::IUnknown, super::dwrite::IDWriteTextFormat, super::dwrite::IDWriteTextLayout);
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextLayout1 {
    pub unsafe fn SetPairKerning(&self, ispairkerningenabled: bool, textrange: super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPairKerning)(windows_core::Interface::as_raw(self), ispairkerningenabled.into(), core::mem::transmute(textrange)) }
    }
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut windows_core::BOOL, textrange: Option<*mut super::dwrite::DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPairKerning)(windows_core::Interface::as_raw(self), currentposition, ispairkerningenabled as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCharacterSpacing)(windows_core::Interface::as_raw(self), leadingspacing, trailingspacing, minimumadvancewidth, core::mem::transmute(textrange)) }
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: Option<*mut super::dwrite::DWRITE_TEXT_RANGE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCharacterSpacing)(windows_core::Interface::as_raw(self), currentposition, leadingspacing as _, trailingspacing as _, minimumadvancewidth as _, textrange.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[cfg(feature = "Win32_dwrite")]
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout1_Vtbl {
    pub base__: super::dwrite::IDWriteTextLayout_Vtbl,
    pub SetPairKerning: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetPairKerning: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
    pub GetCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut f32, *mut f32, *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_dwrite")]
pub trait IDWriteTextLayout1_Impl: super::dwrite::IDWriteTextLayout_Impl {
    fn SetPairKerning(&self, ispairkerningenabled: windows_core::BOOL, textrange: &super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut windows_core::BOOL, textrange: *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: &super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
    fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_dwrite")]
impl IDWriteTextLayout1_Vtbl {
    pub const fn new<Identity: IDWriteTextLayout1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPairKerning<Identity: IDWriteTextLayout1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispairkerningenabled: windows_core::BOOL, textrange: super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout1_Impl::SetPairKerning(this, core::mem::transmute_copy(&ispairkerningenabled), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetPairKerning<Identity: IDWriteTextLayout1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, ispairkerningenabled: *mut windows_core::BOOL, textrange: *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout1_Impl::GetPairKerning(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&ispairkerningenabled), core::mem::transmute_copy(&textrange)).into()
            }
        }
        unsafe extern "system" fn SetCharacterSpacing<Identity: IDWriteTextLayout1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout1_Impl::SetCharacterSpacing(this, core::mem::transmute_copy(&leadingspacing), core::mem::transmute_copy(&trailingspacing), core::mem::transmute_copy(&minimumadvancewidth), core::mem::transmute(&textrange)).into()
            }
        }
        unsafe extern "system" fn GetCharacterSpacing<Identity: IDWriteTextLayout1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut super::dwrite::DWRITE_TEXT_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout1_Impl::GetCharacterSpacing(this, core::mem::transmute_copy(&currentposition), core::mem::transmute_copy(&leadingspacing), core::mem::transmute_copy(&trailingspacing), core::mem::transmute_copy(&minimumadvancewidth), core::mem::transmute_copy(&textrange)).into()
            }
        }
        Self {
            base__: super::dwrite::IDWriteTextLayout_Vtbl::new::<Identity, OFFSET>(),
            SetPairKerning: SetPairKerning::<Identity, OFFSET>,
            GetPairKerning: GetPairKerning::<Identity, OFFSET>,
            SetCharacterSpacing: SetCharacterSpacing::<Identity, OFFSET>,
            GetCharacterSpacing: GetCharacterSpacing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout1 as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextFormat as windows_core::Interface>::IID || iid == &<super::dwrite::IDWriteTextLayout as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_dwrite")]
impl windows_core::RuntimeName for IDWriteTextLayout1 {}
