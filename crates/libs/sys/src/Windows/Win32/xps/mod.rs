#[cfg(all(feature = "objidlbase", feature = "winnt"))]
windows_link::link!("xpsprint.dll" "system" fn StartXpsPrintJob(printername : windows_sys::core::PCWSTR, jobname : windows_sys::core::PCWSTR, outputfilename : windows_sys::core::PCWSTR, progressevent : super::HANDLE, completionevent : super::HANDLE, printablepageson : *const u8, printablepagesoncount : u32, xpsprintjob : *mut *mut core::ffi::c_void, documentstream : *mut *mut core::ffi::c_void, printticketstream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("xpsprint.dll" "system" fn StartXpsPrintJob1(printername : windows_sys::core::PCWSTR, jobname : windows_sys::core::PCWSTR, outputfilename : windows_sys::core::PCWSTR, progressevent : super::HANDLE, completionevent : super::HANDLE, xpsprintjob : *mut *mut core::ffi::c_void, printcontentreceiver : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl Default for XPS_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XPS_COLOR_0 {
    pub sRGB: XPS_COLOR_0_0,
    pub scRGB: XPS_COLOR_0_1,
    pub context: XPS_COLOR_0_2,
}
impl Default for XPS_COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_COLOR_0_0 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_COLOR_0_1 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XPS_COLOR_0_2 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl Default for XPS_COLOR_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type XPS_COLOR_INTERPOLATION = i32;
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = 1;
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = 2;
pub type XPS_COLOR_TYPE = i32;
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = 3;
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = 2;
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
pub type XPS_DASH_CAP = i32;
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = 1;
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = 2;
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = 3;
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = 4;
pub const XPS_E_ALREADY_OWNED: i32 = -2142108413;
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: i32 = -2142108407;
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: i32 = -2142108409;
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: i32 = -2142108408;
pub const XPS_E_CARET_OUTSIDE_STRING: i32 = -2142108923;
pub const XPS_E_CARET_OUT_OF_ORDER: i32 = -2142108922;
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: i32 = -2142108410;
pub const XPS_E_DICTIONARY_ITEM_NAMED: i32 = -2142108671;
pub const XPS_E_DUPLICATE_NAMES: i32 = -2142109175;
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: i32 = -2142109184;
pub const XPS_E_INDEX_OUT_OF_RANGE: i32 = -2142108416;
pub const XPS_E_INVALID_BLEED_BOX: i32 = -2142109692;
pub const XPS_E_INVALID_CONTENT_BOX: i32 = -2142109685;
pub const XPS_E_INVALID_CONTENT_TYPE: i32 = -2142109682;
pub const XPS_E_INVALID_FLOAT: i32 = -2142109689;
pub const XPS_E_INVALID_FONT_URI: i32 = -2142109686;
pub const XPS_E_INVALID_LANGUAGE: i32 = -2142109696;
pub const XPS_E_INVALID_LOOKUP_TYPE: i32 = -2142109690;
pub const XPS_E_INVALID_MARKUP: i32 = -2142109684;
pub const XPS_E_INVALID_NAME: i32 = -2142109695;
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: i32 = -2142109681;
pub const XPS_E_INVALID_PAGE_SIZE: i32 = -2142109693;
pub const XPS_E_INVALID_RESOURCE_KEY: i32 = -2142109694;
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: i32 = -2142109691;
pub const XPS_E_INVALID_XML_ENCODING: i32 = -2142109683;
pub const XPS_E_MAPPING_OUTSIDE_INDICES: i32 = -2142108924;
pub const XPS_E_MAPPING_OUTSIDE_STRING: i32 = -2142108925;
pub const XPS_E_MAPPING_OUT_OF_ORDER: i32 = -2142108926;
pub const XPS_E_MISSING_COLORPROFILE: i32 = -2142109436;
pub const XPS_E_MISSING_DISCARDCONTROL: i32 = -2142109422;
pub const XPS_E_MISSING_DOCUMENT: i32 = -2142109431;
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: i32 = -2142109432;
pub const XPS_E_MISSING_FONTURI: i32 = -2142109433;
pub const XPS_E_MISSING_GLYPHS: i32 = -2142109438;
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: i32 = -2142109426;
pub const XPS_E_MISSING_LOOKUP: i32 = -2142109439;
pub const XPS_E_MISSING_NAME: i32 = -2142109440;
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: i32 = -2142109428;
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: i32 = -2142109427;
pub const XPS_E_MISSING_PART_REFERENCE: i32 = -2142109424;
pub const XPS_E_MISSING_PART_STREAM: i32 = -2142109421;
pub const XPS_E_MISSING_REFERRED_DOCUMENT: i32 = -2142109430;
pub const XPS_E_MISSING_REFERRED_PAGE: i32 = -2142109429;
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: i32 = -2142109435;
pub const XPS_E_MISSING_RESOURCE_KEY: i32 = -2142109425;
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: i32 = -2142109434;
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: i32 = -2142109423;
pub const XPS_E_MISSING_SEGMENT_DATA: i32 = -2142109437;
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: i32 = -2142109182;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: i32 = -2142109178;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: i32 = -2142109177;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: i32 = -2142109179;
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: i32 = -2142109176;
pub const XPS_E_MULTIPLE_RESOURCES: i32 = -2142109183;
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: i32 = -2142109180;
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: i32 = -2142109181;
pub const XPS_E_NEGATIVE_FLOAT: i32 = -2142108918;
pub const XPS_E_NESTED_REMOTE_DICTIONARY: i32 = -2142108670;
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: i32 = -2142108405;
pub const XPS_E_NO_CUSTOM_OBJECTS: i32 = -2142108414;
pub const XPS_E_ODD_BIDILEVEL: i32 = -2142108921;
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: i32 = -2142108920;
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: i32 = -2142108404;
pub const XPS_E_RELATIONSHIP_EXTERNAL: i32 = -2142108406;
pub const XPS_E_RESOURCE_NOT_OWNED: i32 = -2142108412;
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: i32 = -2142108919;
pub const XPS_E_STRING_TOO_LONG: i32 = -2142108928;
pub const XPS_E_TOO_MANY_INDICES: i32 = -2142108927;
pub const XPS_E_UNAVAILABLE_PACKAGE: i32 = -2142109420;
pub const XPS_E_UNEXPECTED_COLORPROFILE: i32 = -2142108411;
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: i32 = -2142109688;
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: i32 = -2142109680;
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: i32 = -2142109679;
pub const XPS_E_VISUAL_CIRCULAR_REF: i32 = -2142108415;
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: i32 = -2142108672;
pub type XPS_FILL_RULE = i32;
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = 1;
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = 2;
pub type XPS_FONT_EMBEDDING = i32;
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = 1;
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = 2;
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = 3;
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
pub type XPS_IMAGE_TYPE = i32;
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = 1;
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = 5;
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = 2;
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = 3;
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = 4;
pub type XPS_INTERLEAVING = i32;
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = 1;
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = 2;
pub const XPS_JOB_CANCELLED: XPS_JOB_COMPLETION = 2;
pub const XPS_JOB_COMPLETED: XPS_JOB_COMPLETION = 1;
pub type XPS_JOB_COMPLETION = i32;
pub const XPS_JOB_FAILED: XPS_JOB_COMPLETION = 3;
pub const XPS_JOB_IN_PROGRESS: XPS_JOB_COMPLETION = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_JOB_STATUS {
    pub jobId: u32,
    pub currentDocument: i32,
    pub currentPage: i32,
    pub currentPageTotal: i32,
    pub completion: XPS_JOB_COMPLETION,
    pub jobStatus: windows_sys::core::HRESULT,
}
pub type XPS_LINE_CAP = i32;
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = 1;
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = 2;
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = 3;
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = 4;
pub type XPS_LINE_JOIN = i32;
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = 2;
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = 1;
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
pub type XPS_OBJECT_TYPE = i32;
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = 1;
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = 5;
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = 2;
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = 7;
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = 8;
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = 4;
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = 3;
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = 9;
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = 6;
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = 10;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
pub type XPS_SEGMENT_STROKE_PATTERN = i32;
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = 1;
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = 3;
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = 2;
pub type XPS_SEGMENT_TYPE = i32;
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = 1;
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = 2;
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = 3;
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = 4;
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = 5;
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = 6;
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = 7;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
pub type XPS_SPREAD_METHOD = i32;
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = 1;
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = 2;
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = 3;
pub type XPS_STYLE_SIMULATION = i32;
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = 3;
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = 4;
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = 2;
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = 1;
pub type XPS_THUMBNAIL_SIZE = i32;
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = 4;
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = 3;
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = 2;
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = 1;
pub type XPS_TILE_MODE = i32;
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = 3;
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = 5;
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = 4;
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = 1;
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = 2;
pub const XpsOMObjectFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
pub const XpsOMThumbnailGenerator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
