#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
pub const DC_BINNAMES: PRINTER_DEVICE_CAPABILITIES = 12u16;
pub const DC_BINS: PRINTER_DEVICE_CAPABILITIES = 6u16;
pub const DC_COLLATE: PRINTER_DEVICE_CAPABILITIES = 22u16;
pub const DC_COLORDEVICE: PRINTER_DEVICE_CAPABILITIES = 32u16;
pub const DC_COPIES: PRINTER_DEVICE_CAPABILITIES = 18u16;
pub const DC_DRIVER: PRINTER_DEVICE_CAPABILITIES = 11u16;
pub const DC_DUPLEX: PRINTER_DEVICE_CAPABILITIES = 7u16;
pub const DC_ENUMRESOLUTIONS: PRINTER_DEVICE_CAPABILITIES = 13u16;
pub const DC_EXTRA: PRINTER_DEVICE_CAPABILITIES = 9u16;
pub const DC_FIELDS: PRINTER_DEVICE_CAPABILITIES = 1u16;
pub const DC_FILEDEPENDENCIES: PRINTER_DEVICE_CAPABILITIES = 14u16;
pub const DC_MAXEXTENT: PRINTER_DEVICE_CAPABILITIES = 5u16;
pub const DC_MEDIAREADY: PRINTER_DEVICE_CAPABILITIES = 29u16;
pub const DC_MEDIATYPENAMES: PRINTER_DEVICE_CAPABILITIES = 34u16;
pub const DC_MEDIATYPES: PRINTER_DEVICE_CAPABILITIES = 35u16;
pub const DC_MINEXTENT: PRINTER_DEVICE_CAPABILITIES = 4u16;
pub const DC_NUP: PRINTER_DEVICE_CAPABILITIES = 33u16;
pub const DC_ORIENTATION: PRINTER_DEVICE_CAPABILITIES = 17u16;
pub const DC_PAPERNAMES: PRINTER_DEVICE_CAPABILITIES = 16u16;
pub const DC_PAPERS: PRINTER_DEVICE_CAPABILITIES = 2u16;
pub const DC_PAPERSIZE: PRINTER_DEVICE_CAPABILITIES = 3u16;
pub const DC_PERSONALITY: PRINTER_DEVICE_CAPABILITIES = 25u16;
pub const DC_PRINTERMEM: PRINTER_DEVICE_CAPABILITIES = 28u16;
pub const DC_PRINTRATE: PRINTER_DEVICE_CAPABILITIES = 26u16;
pub const DC_PRINTRATEPPM: PRINTER_DEVICE_CAPABILITIES = 31u16;
pub const DC_PRINTRATEUNIT: PRINTER_DEVICE_CAPABILITIES = 27u16;
pub const DC_SIZE: PRINTER_DEVICE_CAPABILITIES = 8u16;
pub const DC_STAPLE: PRINTER_DEVICE_CAPABILITIES = 30u16;
pub const DC_TRUETYPE: PRINTER_DEVICE_CAPABILITIES = 15u16;
pub const DC_VERSION: PRINTER_DEVICE_CAPABILITIES = 10u16;
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = 12u16;
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = 101u16;
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = 14u16;
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = 16u16;
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = 1u16;
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = 9u16;
pub const PSINJECT_COMMENTS: PSINJECT_POINT = 11u16;
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = 5u16;
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = 6u16;
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = 10u16;
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = 21u16;
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = 13u16;
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = 107u16;
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = 102u16;
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = 15u16;
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = 17u16;
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = 20u16;
pub const PSINJECT_EOF: PSINJECT_POINT = 19u16;
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = 8u16;
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = 106u16;
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = 100u16;
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = 7u16;
pub const PSINJECT_PAGES: PSINJECT_POINT = 4u16;
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = 3u16;
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = 103u16;
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = 104u16;
pub const PSINJECT_PSADOBE: PSINJECT_POINT = 2u16;
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = 105u16;
pub const PSINJECT_TRAILER: PSINJECT_POINT = 18u16;
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = 201u16;
pub const PSINJECT_VMSAVE: PSINJECT_POINT = 200u16;
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = 1u32;
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = 1i32;
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = 2i32;
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = 3i32;
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = 2i32;
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = 1i32;
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = 1i32;
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = 2i32;
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = 3i32;
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = 4i32;
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = 3i32;
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = 1i32;
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = 2i32;
pub const XPS_E_ABSOLUTE_REFERENCE: windows_core::HRESULT = 0x80520601_u32 as _;
pub const XPS_E_ALREADY_OWNED: windows_core::HRESULT = 0x80520503_u32 as _;
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: windows_core::HRESULT = 0x80520509_u32 as _;
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: windows_core::HRESULT = 0x80520507_u32 as _;
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: windows_core::HRESULT = 0x80520508_u32 as _;
pub const XPS_E_CARET_OUTSIDE_STRING: windows_core::HRESULT = 0x80520305_u32 as _;
pub const XPS_E_CARET_OUT_OF_ORDER: windows_core::HRESULT = 0x80520306_u32 as _;
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: windows_core::HRESULT = 0x80520506_u32 as _;
pub const XPS_E_DICTIONARY_ITEM_NAMED: windows_core::HRESULT = 0x80520401_u32 as _;
pub const XPS_E_DUPLICATE_NAMES: windows_core::HRESULT = 0x80520209_u32 as _;
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: windows_core::HRESULT = 0x80520200_u32 as _;
pub const XPS_E_INDEX_OUT_OF_RANGE: windows_core::HRESULT = 0x80520500_u32 as _;
pub const XPS_E_INVALID_BLEED_BOX: windows_core::HRESULT = 0x80520004_u32 as _;
pub const XPS_E_INVALID_CONTENT_BOX: windows_core::HRESULT = 0x8052000B_u32 as _;
pub const XPS_E_INVALID_CONTENT_TYPE: windows_core::HRESULT = 0x8052000E_u32 as _;
pub const XPS_E_INVALID_FLOAT: windows_core::HRESULT = 0x80520007_u32 as _;
pub const XPS_E_INVALID_FONT_URI: windows_core::HRESULT = 0x8052000A_u32 as _;
pub const XPS_E_INVALID_LANGUAGE: windows_core::HRESULT = 0x80520000_u32 as _;
pub const XPS_E_INVALID_LOOKUP_TYPE: windows_core::HRESULT = 0x80520006_u32 as _;
pub const XPS_E_INVALID_MARKUP: windows_core::HRESULT = 0x8052000C_u32 as _;
pub const XPS_E_INVALID_NAME: windows_core::HRESULT = 0x80520001_u32 as _;
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: windows_core::HRESULT = 0x80520602_u32 as _;
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: windows_core::HRESULT = 0x80520600_u32 as _;
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: windows_core::HRESULT = 0x8052000F_u32 as _;
pub const XPS_E_INVALID_PAGE_SIZE: windows_core::HRESULT = 0x80520003_u32 as _;
pub const XPS_E_INVALID_RESOURCE_KEY: windows_core::HRESULT = 0x80520002_u32 as _;
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: windows_core::HRESULT = 0x8052038B_u32 as _;
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: windows_core::HRESULT = 0x80520005_u32 as _;
pub const XPS_E_INVALID_XML_ENCODING: windows_core::HRESULT = 0x8052000D_u32 as _;
pub const XPS_E_MAPPING_OUTSIDE_INDICES: windows_core::HRESULT = 0x80520304_u32 as _;
pub const XPS_E_MAPPING_OUTSIDE_STRING: windows_core::HRESULT = 0x80520303_u32 as _;
pub const XPS_E_MAPPING_OUT_OF_ORDER: windows_core::HRESULT = 0x80520302_u32 as _;
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: windows_core::HRESULT = 0x80520389_u32 as _;
pub const XPS_E_MISSING_COLORPROFILE: windows_core::HRESULT = 0x80520104_u32 as _;
pub const XPS_E_MISSING_DISCARDCONTROL: windows_core::HRESULT = 0x80520112_u32 as _;
pub const XPS_E_MISSING_DOCUMENT: windows_core::HRESULT = 0x80520109_u32 as _;
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: windows_core::HRESULT = 0x80520108_u32 as _;
pub const XPS_E_MISSING_FONTURI: windows_core::HRESULT = 0x80520107_u32 as _;
pub const XPS_E_MISSING_GLYPHS: windows_core::HRESULT = 0x80520102_u32 as _;
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: windows_core::HRESULT = 0x8052010E_u32 as _;
pub const XPS_E_MISSING_LOOKUP: windows_core::HRESULT = 0x80520101_u32 as _;
pub const XPS_E_MISSING_NAME: windows_core::HRESULT = 0x80520100_u32 as _;
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: windows_core::HRESULT = 0x8052010C_u32 as _;
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: windows_core::HRESULT = 0x8052010D_u32 as _;
pub const XPS_E_MISSING_PART_REFERENCE: windows_core::HRESULT = 0x80520110_u32 as _;
pub const XPS_E_MISSING_PART_STREAM: windows_core::HRESULT = 0x80520113_u32 as _;
pub const XPS_E_MISSING_REFERRED_DOCUMENT: windows_core::HRESULT = 0x8052010A_u32 as _;
pub const XPS_E_MISSING_REFERRED_PAGE: windows_core::HRESULT = 0x8052010B_u32 as _;
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: windows_core::HRESULT = 0x80520105_u32 as _;
pub const XPS_E_MISSING_RESOURCE_KEY: windows_core::HRESULT = 0x8052010F_u32 as _;
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: windows_core::HRESULT = 0x80520106_u32 as _;
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: windows_core::HRESULT = 0x80520111_u32 as _;
pub const XPS_E_MISSING_SEGMENT_DATA: windows_core::HRESULT = 0x80520103_u32 as _;
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: windows_core::HRESULT = 0x80520202_u32 as _;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: windows_core::HRESULT = 0x80520206_u32 as _;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: windows_core::HRESULT = 0x80520207_u32 as _;
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: windows_core::HRESULT = 0x80520205_u32 as _;
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: windows_core::HRESULT = 0x80520208_u32 as _;
pub const XPS_E_MULTIPLE_RESOURCES: windows_core::HRESULT = 0x80520201_u32 as _;
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: windows_core::HRESULT = 0x80520204_u32 as _;
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: windows_core::HRESULT = 0x80520203_u32 as _;
pub const XPS_E_NEGATIVE_FLOAT: windows_core::HRESULT = 0x8052030A_u32 as _;
pub const XPS_E_NESTED_REMOTE_DICTIONARY: windows_core::HRESULT = 0x80520402_u32 as _;
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: windows_core::HRESULT = 0x8052050B_u32 as _;
pub const XPS_E_NO_CUSTOM_OBJECTS: windows_core::HRESULT = 0x80520502_u32 as _;
pub const XPS_E_OBJECT_DETACHED: windows_core::HRESULT = 0x8052038A_u32 as _;
pub const XPS_E_ODD_BIDILEVEL: windows_core::HRESULT = 0x80520307_u32 as _;
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: windows_core::HRESULT = 0x80520308_u32 as _;
pub const XPS_E_PACKAGE_ALREADY_OPENED: windows_core::HRESULT = 0x80520387_u32 as _;
pub const XPS_E_PACKAGE_NOT_OPENED: windows_core::HRESULT = 0x80520386_u32 as _;
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: windows_core::HRESULT = 0x8052050C_u32 as _;
pub const XPS_E_RELATIONSHIP_EXTERNAL: windows_core::HRESULT = 0x8052050A_u32 as _;
pub const XPS_E_RESOURCE_NOT_OWNED: windows_core::HRESULT = 0x80520504_u32 as _;
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: windows_core::HRESULT = 0x80520309_u32 as _;
pub const XPS_E_SIGNATUREID_DUP: windows_core::HRESULT = 0x80520388_u32 as _;
pub const XPS_E_SIGREQUESTID_DUP: windows_core::HRESULT = 0x80520385_u32 as _;
pub const XPS_E_STRING_TOO_LONG: windows_core::HRESULT = 0x80520300_u32 as _;
pub const XPS_E_TOO_MANY_INDICES: windows_core::HRESULT = 0x80520301_u32 as _;
pub const XPS_E_UNAVAILABLE_PACKAGE: windows_core::HRESULT = 0x80520114_u32 as _;
pub const XPS_E_UNEXPECTED_COLORPROFILE: windows_core::HRESULT = 0x80520505_u32 as _;
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: windows_core::HRESULT = 0x80520008_u32 as _;
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: windows_core::HRESULT = 0x80520010_u32 as _;
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: windows_core::HRESULT = 0x80520011_u32 as _;
pub const XPS_E_VISUAL_CIRCULAR_REF: windows_core::HRESULT = 0x80520501_u32 as _;
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: windows_core::HRESULT = 0x80520400_u32 as _;
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = 1i32;
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = 2i32;
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = 1i32;
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = 2i32;
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = 3i32;
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = 4i32;
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = 1i32;
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = 5i32;
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = 2i32;
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = 3i32;
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = 4i32;
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = 1i32;
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = 2i32;
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = 1i32;
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = 2i32;
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = 3i32;
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = 4i32;
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = 2i32;
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = 1i32;
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = 3i32;
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = 1i32;
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = 5i32;
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = 2i32;
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = 7i32;
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = 8i32;
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = 4i32;
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = 3i32;
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = 9i32;
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = 6i32;
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = 10i32;
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = 1i32;
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = 3i32;
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = 2i32;
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = 1i32;
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = 2i32;
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = 3i32;
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = 4i32;
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = 5i32;
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = 6i32;
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = 7i32;
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = 3i32;
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = 2i32;
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = 1i32;
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = 4i32;
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = 5i32;
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = 1i32;
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = 0i32;
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = 15i32;
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = 1i32;
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = 8i32;
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = 0i32;
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = 4i32;
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = 2i32;
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = 1i32;
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = 2i32;
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = 3i32;
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = 3i32;
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = 4i32;
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = 2i32;
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = 1i32;
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = 4i32;
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = 3i32;
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = 2i32;
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = 1i32;
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = 3i32;
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = 5i32;
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = 4i32;
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = 1i32;
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = 2i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PRINTER_DEVICE_CAPABILITIES(pub u16);
impl windows_core::TypeKind for PRINTER_DEVICE_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PRINT_WINDOW_FLAGS(pub u32);
impl windows_core::TypeKind for PRINT_WINDOW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PSINJECT_POINT(pub u16);
impl windows_core::TypeKind for PSINJECT_POINT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_COLOR_INTERPOLATION(pub i32);
impl windows_core::TypeKind for XPS_COLOR_INTERPOLATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_COLOR_TYPE(pub i32);
impl windows_core::TypeKind for XPS_COLOR_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_DASH_CAP(pub i32);
impl windows_core::TypeKind for XPS_DASH_CAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_DOCUMENT_TYPE(pub i32);
impl windows_core::TypeKind for XPS_DOCUMENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_FILL_RULE(pub i32);
impl windows_core::TypeKind for XPS_FILL_RULE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_FONT_EMBEDDING(pub i32);
impl windows_core::TypeKind for XPS_FONT_EMBEDDING {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_IMAGE_TYPE(pub i32);
impl windows_core::TypeKind for XPS_IMAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_INTERLEAVING(pub i32);
impl windows_core::TypeKind for XPS_INTERLEAVING {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_LINE_CAP(pub i32);
impl windows_core::TypeKind for XPS_LINE_CAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_LINE_JOIN(pub i32);
impl windows_core::TypeKind for XPS_LINE_JOIN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for XPS_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_SEGMENT_STROKE_PATTERN(pub i32);
impl windows_core::TypeKind for XPS_SEGMENT_STROKE_PATTERN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_SEGMENT_TYPE(pub i32);
impl windows_core::TypeKind for XPS_SEGMENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_SIGNATURE_STATUS(pub i32);
impl windows_core::TypeKind for XPS_SIGNATURE_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_SIGN_FLAGS(pub i32);
impl windows_core::TypeKind for XPS_SIGN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_SIGN_POLICY(pub i32);
impl windows_core::TypeKind for XPS_SIGN_POLICY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_SPREAD_METHOD(pub i32);
impl windows_core::TypeKind for XPS_SPREAD_METHOD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_STYLE_SIMULATION(pub i32);
impl windows_core::TypeKind for XPS_STYLE_SIMULATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_THUMBNAIL_SIZE(pub i32);
impl windows_core::TypeKind for XPS_THUMBNAIL_SIZE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XPS_TILE_MODE(pub i32);
impl windows_core::TypeKind for XPS_TILE_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: windows_core::PCSTR,
    pub lpszOutput: windows_core::PCSTR,
    pub lpszDatatype: windows_core::PCSTR,
    pub fwType: u32,
}
impl Default for DOCINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DOCINFOA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: windows_core::PCWSTR,
    pub lpszOutput: windows_core::PCWSTR,
    pub lpszDatatype: windows_core::PCWSTR,
    pub fwType: u32,
}
impl Default for DOCINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DOCINFOW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRAWPATRECT {
    pub ptPosition: super::super::Foundation::POINT,
    pub ptSize: super::super::Foundation::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
impl Default for DRAWPATRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DRAWPATRECT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
impl Default for PSFEATURE_CUSTPAPER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PSFEATURE_CUSTPAPER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: super::super::Foundation::BOOL,
    pub bSetPageDevice: super::super::Foundation::BOOL,
}
impl Default for PSFEATURE_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PSFEATURE_OUTPUT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: PSINJECT_POINT,
    pub PageNumber: u16,
}
impl Default for PSINJECTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PSINJECTDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl Default for XPS_COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_COLOR {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl windows_core::TypeKind for XPS_COLOR_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_COLOR_0_2 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl Default for XPS_COLOR_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_COLOR_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_COLOR_0_0 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl Default for XPS_COLOR_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_COLOR_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_COLOR_0_1 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl Default for XPS_COLOR_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_COLOR_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
impl Default for XPS_DASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_DASH {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
impl Default for XPS_GLYPH_INDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_GLYPH_INDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
impl Default for XPS_GLYPH_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_GLYPH_MAPPING {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
impl Default for XPS_MATRIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_MATRIX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
impl Default for XPS_POINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_POINT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl Default for XPS_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_RECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
impl Default for XPS_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for XPS_SIZE {
    type TypeKind = windows_core::CopyType;
}
pub const XpsOMObjectFactory: windows_core::GUID = windows_core::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
pub const XpsOMThumbnailGenerator: windows_core::GUID = windows_core::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
pub const XpsSignatureManager: windows_core::GUID = windows_core::GUID::from_u128(0xb0c43320_2315_44a2_b70a_0943a140a8ee);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type ABORTPROC = Option<unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL>;
