#[cfg(all(feature = "objidlbase", feature = "winnt"))]
windows_link::link!("xpsprint.dll" "system" fn StartXpsPrintJob(printername : windows_sys::core::PCWSTR, jobname : windows_sys::core::PCWSTR, outputfilename : windows_sys::core::PCWSTR, progressevent : super::HANDLE, completionevent : super::HANDLE, printablepageson : *const u8, printablepagesoncount : u32, xpsprintjob : *mut *mut core::ffi::c_void, documentstream : *mut *mut core::ffi::c_void, printticketstream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("xpsprint.dll" "system" fn StartXpsPrintJob1(printername : windows_sys::core::PCWSTR, jobname : windows_sys::core::PCWSTR, outputfilename : windows_sys::core::PCWSTR, progressevent : super::HANDLE, completionevent : super::HANDLE, xpsprintjob : *mut *mut core::ffi::c_void, printcontentreceiver : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028,
}
#[cfg(feature = "minwindef")]
impl Default for XPS_COLOR {
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct XPS_DASH {
    pub length: super::FLOAT,
    pub gap: super::FLOAT,
}
pub type XPS_DASH_CAP = i32;
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = 1;
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = 2;
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = 3;
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = 4;
pub const XPS_E_ALREADY_OWNED: windows_sys::core::HRESULT = 0x80520503_u32 as _;
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: windows_sys::core::HRESULT = 0x80520509_u32 as _;
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: windows_sys::core::HRESULT = 0x80520507_u32 as _;
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: windows_sys::core::HRESULT = 0x80520508_u32 as _;
pub const XPS_E_CARET_OUTSIDE_STRING: windows_sys::core::HRESULT = 0x80520305_u32 as _;
pub const XPS_E_CARET_OUT_OF_ORDER: windows_sys::core::HRESULT = 0x80520306_u32 as _;
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: windows_sys::core::HRESULT = 0x80520506_u32 as _;
pub const XPS_E_DICTIONARY_ITEM_NAMED: windows_sys::core::HRESULT = 0x80520401_u32 as _;
pub const XPS_E_DUPLICATE_NAMES: windows_sys::core::HRESULT = 0x80520209_u32 as _;
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: windows_sys::core::HRESULT = 0x80520200_u32 as _;
pub const XPS_E_INDEX_OUT_OF_RANGE: windows_sys::core::HRESULT = 0x80520500_u32 as _;
pub const XPS_E_INVALID_BLEED_BOX: windows_sys::core::HRESULT = 0x80520004_u32 as _;
pub const XPS_E_INVALID_CONTENT_BOX: windows_sys::core::HRESULT = 0x8052000B_u32 as _;
pub const XPS_E_INVALID_CONTENT_TYPE: windows_sys::core::HRESULT = 0x8052000E_u32 as _;
pub const XPS_E_INVALID_FLOAT: windows_sys::core::HRESULT = 0x80520007_u32 as _;
pub const XPS_E_INVALID_FONT_URI: windows_sys::core::HRESULT = 0x8052000A_u32 as _;
pub const XPS_E_INVALID_LANGUAGE: windows_sys::core::HRESULT = 0x80520000_u32 as _;
pub const XPS_E_INVALID_LOOKUP_TYPE: windows_sys::core::HRESULT = 0x80520006_u32 as _;
pub const XPS_E_INVALID_MARKUP: windows_sys::core::HRESULT = 0x8052000C_u32 as _;
pub const XPS_E_INVALID_NAME: windows_sys::core::HRESULT = 0x80520001_u32 as _;
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: windows_sys::core::HRESULT = 0x8052000F_u32 as _;
pub const XPS_E_INVALID_PAGE_SIZE: windows_sys::core::HRESULT = 0x80520003_u32 as _;
pub const XPS_E_INVALID_RESOURCE_KEY: windows_sys::core::HRESULT = 0x80520002_u32 as _;
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: windows_sys::core::HRESULT = 0x80520005_u32 as _;
pub const XPS_E_INVALID_XML_ENCODING: windows_sys::core::HRESULT = 0x8052000D_u32 as _;
pub const XPS_E_MAPPING_OUTSIDE_INDICES: windows_sys::core::HRESULT = 0x80520304_u32 as _;
pub const XPS_E_MAPPING_OUTSIDE_STRING: windows_sys::core::HRESULT = 0x80520303_u32 as _;
pub const XPS_E_MAPPING_OUT_OF_ORDER: windows_sys::core::HRESULT = 0x80520302_u32 as _;
pub const XPS_E_MISSING_COLORPROFILE: windows_sys::core::HRESULT = 0x80520104_u32 as _;
pub const XPS_E_MISSING_DISCARDCONTROL: windows_sys::core::HRESULT = 0x80520112_u32 as _;
pub const XPS_E_MISSING_DOCUMENT: windows_sys::core::HRESULT = 0x80520109_u32 as _;
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: windows_sys::core::HRESULT = 0x80520108_u32 as _;
pub const XPS_E_MISSING_FONTURI: windows_sys::core::HRESULT = 0x80520107_u32 as _;
pub const XPS_E_MISSING_GLYPHS: windows_sys::core::HRESULT = 0x80520102_u32 as _;
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: windows_sys::core::HRESULT = 0x8052010E_u32 as _;
pub const XPS_E_MISSING_LOOKUP: windows_sys::core::HRESULT = 0x80520101_u32 as _;
pub const XPS_E_MISSING_NAME: windows_sys::core::HRESULT = 0x80520100_u32 as _;
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: windows_sys::core::HRESULT = 0x8052010C_u32 as _;
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: windows_sys::core::HRESULT = 0x8052010D_u32 as _;
pub const XPS_E_MISSING_PART_REFERENCE: windows_sys::core::HRESULT = 0x80520110_u32 as _;
pub const XPS_E_MISSING_PART_STREAM: windows_sys::core::HRESULT = 0x80520113_u32 as _;
pub const XPS_E_MISSING_REFERRED_DOCUMENT: windows_sys::core::HRESULT = 0x8052010A_u32 as _;
pub const XPS_E_MISSING_REFERRED_PAGE: windows_sys::core::HRESULT = 0x8052010B_u32 as _;
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: windows_sys::core::HRESULT = 0x80520105_u32 as _;
pub const XPS_E_MISSING_RESOURCE_KEY: windows_sys::core::HRESULT = 0x8052010F_u32 as _;
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: windows_sys::core::HRESULT = 0x80520106_u32 as _;
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: windows_sys::core::HRESULT = 0x80520111_u32 as _;
pub const XPS_E_MISSING_SEGMENT_DATA: windows_sys::core::HRESULT = 0x80520103_u32 as _;
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: windows_sys::core::HRESULT = 0x80520202_u32 as _;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: windows_sys::core::HRESULT = 0x80520206_u32 as _;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: windows_sys::core::HRESULT = 0x80520207_u32 as _;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: windows_sys::core::HRESULT = 0x80520205_u32 as _;
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: windows_sys::core::HRESULT = 0x80520208_u32 as _;
pub const XPS_E_MULTIPLE_RESOURCES: windows_sys::core::HRESULT = 0x80520201_u32 as _;
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: windows_sys::core::HRESULT = 0x80520204_u32 as _;
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: windows_sys::core::HRESULT = 0x80520203_u32 as _;
pub const XPS_E_NEGATIVE_FLOAT: windows_sys::core::HRESULT = 0x8052030A_u32 as _;
pub const XPS_E_NESTED_REMOTE_DICTIONARY: windows_sys::core::HRESULT = 0x80520402_u32 as _;
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: windows_sys::core::HRESULT = 0x8052050B_u32 as _;
pub const XPS_E_NO_CUSTOM_OBJECTS: windows_sys::core::HRESULT = 0x80520502_u32 as _;
pub const XPS_E_ODD_BIDILEVEL: windows_sys::core::HRESULT = 0x80520307_u32 as _;
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: windows_sys::core::HRESULT = 0x80520308_u32 as _;
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: windows_sys::core::HRESULT = 0x8052050C_u32 as _;
pub const XPS_E_RELATIONSHIP_EXTERNAL: windows_sys::core::HRESULT = 0x8052050A_u32 as _;
pub const XPS_E_RESOURCE_NOT_OWNED: windows_sys::core::HRESULT = 0x80520504_u32 as _;
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: windows_sys::core::HRESULT = 0x80520309_u32 as _;
pub const XPS_E_STRING_TOO_LONG: windows_sys::core::HRESULT = 0x80520300_u32 as _;
pub const XPS_E_TOO_MANY_INDICES: windows_sys::core::HRESULT = 0x80520301_u32 as _;
pub const XPS_E_UNAVAILABLE_PACKAGE: windows_sys::core::HRESULT = 0x80520114_u32 as _;
pub const XPS_E_UNEXPECTED_COLORPROFILE: windows_sys::core::HRESULT = 0x80520505_u32 as _;
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: windows_sys::core::HRESULT = 0x80520008_u32 as _;
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: windows_sys::core::HRESULT = 0x80520010_u32 as _;
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: windows_sys::core::HRESULT = 0x80520011_u32 as _;
pub const XPS_E_VISUAL_CIRCULAR_REF: windows_sys::core::HRESULT = 0x80520501_u32 as _;
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: windows_sys::core::HRESULT = 0x80520400_u32 as _;
pub type XPS_FILL_RULE = i32;
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = 1;
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = 2;
pub type XPS_FONT_EMBEDDING = i32;
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = 1;
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = 2;
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = 3;
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: super::FLOAT,
    pub horizontalOffset: super::FLOAT,
    pub verticalOffset: super::FLOAT,
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct XPS_MATRIX {
    pub m11: super::FLOAT,
    pub m12: super::FLOAT,
    pub m21: super::FLOAT,
    pub m22: super::FLOAT,
    pub m31: super::FLOAT,
    pub m32: super::FLOAT,
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct XPS_POINT {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct XPS_RECT {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
    pub width: super::FLOAT,
    pub height: super::FLOAT,
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct XPS_SIZE {
    pub width: super::FLOAT,
    pub height: super::FLOAT,
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
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028 {
    pub sRGB: __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028_0,
    pub scRGB: __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028_1,
    pub context: __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028_2,
}
#[cfg(feature = "minwindef")]
impl Default for __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028_0 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028_1 {
    pub alpha: super::FLOAT,
    pub red: super::FLOAT,
    pub green: super::FLOAT,
    pub blue: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028_2 {
    pub channelCount: u8,
    pub channels: [super::FLOAT; 9],
}
#[cfg(feature = "minwindef")]
impl Default for __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
