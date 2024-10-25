pub const OPC_CACHE_ON_ACCESS: OPC_READ_FLAGS = 2i32;
pub const OPC_CANONICALIZATION_C14N: OPC_CANONICALIZATION_METHOD = 1i32;
pub const OPC_CANONICALIZATION_C14N_WITH_COMMENTS: OPC_CANONICALIZATION_METHOD = 2i32;
pub const OPC_CANONICALIZATION_NONE: OPC_CANONICALIZATION_METHOD = 0i32;
pub const OPC_CERTIFICATE_IN_CERTIFICATE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = 0i32;
pub const OPC_CERTIFICATE_IN_SIGNATURE_PART: OPC_CERTIFICATE_EMBEDDING_OPTION = 1i32;
pub const OPC_CERTIFICATE_NOT_EMBEDDED: OPC_CERTIFICATE_EMBEDDING_OPTION = 2i32;
pub const OPC_COMPRESSION_FAST: OPC_COMPRESSION_OPTIONS = 2i32;
pub const OPC_COMPRESSION_MAXIMUM: OPC_COMPRESSION_OPTIONS = 1i32;
pub const OPC_COMPRESSION_NONE: OPC_COMPRESSION_OPTIONS = -1i32;
pub const OPC_COMPRESSION_NORMAL: OPC_COMPRESSION_OPTIONS = 0i32;
pub const OPC_COMPRESSION_SUPERFAST: OPC_COMPRESSION_OPTIONS = 3i32;
pub const OPC_E_CONFLICTING_SETTINGS: windows_core::HRESULT = 0x80510014_u32 as _;
pub const OPC_E_COULD_NOT_RECOVER: windows_core::HRESULT = 0x8051004E_u32 as _;
pub const OPC_E_DS_DEFAULT_DIGEST_METHOD_NOT_SET: windows_core::HRESULT = 0x80510047_u32 as _;
pub const OPC_E_DS_DIGEST_VALUE_ERROR: windows_core::HRESULT = 0x8051001A_u32 as _;
pub const OPC_E_DS_DUPLICATE_PACKAGE_OBJECT_REFERENCES: windows_core::HRESULT = 0x8051002D_u32 as _;
pub const OPC_E_DS_DUPLICATE_SIGNATURE_ORIGIN_RELATIONSHIP: windows_core::HRESULT = 0x8051001B_u32 as _;
pub const OPC_E_DS_DUPLICATE_SIGNATURE_PROPERTY_ELEMENT: windows_core::HRESULT = 0x80510028_u32 as _;
pub const OPC_E_DS_EXTERNAL_SIGNATURE: windows_core::HRESULT = 0x8051001E_u32 as _;
pub const OPC_E_DS_EXTERNAL_SIGNATURE_REFERENCE: windows_core::HRESULT = 0x8051002F_u32 as _;
pub const OPC_E_DS_INVALID_CANONICALIZATION_METHOD: windows_core::HRESULT = 0x80510022_u32 as _;
pub const OPC_E_DS_INVALID_CERTIFICATE_RELATIONSHIP: windows_core::HRESULT = 0x8051001D_u32 as _;
pub const OPC_E_DS_INVALID_OPC_SIGNATURE_TIME_FORMAT: windows_core::HRESULT = 0x80510024_u32 as _;
pub const OPC_E_DS_INVALID_RELATIONSHIPS_SIGNING_OPTION: windows_core::HRESULT = 0x80510023_u32 as _;
pub const OPC_E_DS_INVALID_RELATIONSHIP_TRANSFORM_XML: windows_core::HRESULT = 0x80510021_u32 as _;
pub const OPC_E_DS_INVALID_SIGNATURE_COUNT: windows_core::HRESULT = 0x8051002B_u32 as _;
pub const OPC_E_DS_INVALID_SIGNATURE_ORIGIN_RELATIONSHIP: windows_core::HRESULT = 0x8051001C_u32 as _;
pub const OPC_E_DS_INVALID_SIGNATURE_XML: windows_core::HRESULT = 0x8051002A_u32 as _;
pub const OPC_E_DS_MISSING_CANONICALIZATION_TRANSFORM: windows_core::HRESULT = 0x80510032_u32 as _;
pub const OPC_E_DS_MISSING_CERTIFICATE_PART: windows_core::HRESULT = 0x80510056_u32 as _;
pub const OPC_E_DS_MISSING_PACKAGE_OBJECT_REFERENCE: windows_core::HRESULT = 0x8051002E_u32 as _;
pub const OPC_E_DS_MISSING_SIGNATURE_ALGORITHM: windows_core::HRESULT = 0x8051002C_u32 as _;
pub const OPC_E_DS_MISSING_SIGNATURE_ORIGIN_PART: windows_core::HRESULT = 0x8051001F_u32 as _;
pub const OPC_E_DS_MISSING_SIGNATURE_PART: windows_core::HRESULT = 0x80510020_u32 as _;
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTIES_ELEMENT: windows_core::HRESULT = 0x80510026_u32 as _;
pub const OPC_E_DS_MISSING_SIGNATURE_PROPERTY_ELEMENT: windows_core::HRESULT = 0x80510027_u32 as _;
pub const OPC_E_DS_MISSING_SIGNATURE_TIME_PROPERTY: windows_core::HRESULT = 0x80510029_u32 as _;
pub const OPC_E_DS_MULTIPLE_RELATIONSHIP_TRANSFORMS: windows_core::HRESULT = 0x80510031_u32 as _;
pub const OPC_E_DS_PACKAGE_REFERENCE_URI_RESERVED: windows_core::HRESULT = 0x80510025_u32 as _;
pub const OPC_E_DS_REFERENCE_MISSING_CONTENT_TYPE: windows_core::HRESULT = 0x80510030_u32 as _;
pub const OPC_E_DS_SIGNATURE_CORRUPT: windows_core::HRESULT = 0x80510019_u32 as _;
pub const OPC_E_DS_SIGNATURE_METHOD_NOT_SET: windows_core::HRESULT = 0x80510046_u32 as _;
pub const OPC_E_DS_SIGNATURE_ORIGIN_EXISTS: windows_core::HRESULT = 0x80510054_u32 as _;
pub const OPC_E_DS_SIGNATURE_PROPERTY_MISSING_TARGET: windows_core::HRESULT = 0x80510045_u32 as _;
pub const OPC_E_DS_SIGNATURE_REFERENCE_MISSING_URI: windows_core::HRESULT = 0x80510043_u32 as _;
pub const OPC_E_DS_UNSIGNED_PACKAGE: windows_core::HRESULT = 0x80510055_u32 as _;
pub const OPC_E_DUPLICATE_DEFAULT_EXTENSION: windows_core::HRESULT = 0x8051000F_u32 as _;
pub const OPC_E_DUPLICATE_OVERRIDE_PART: windows_core::HRESULT = 0x8051000D_u32 as _;
pub const OPC_E_DUPLICATE_PART: windows_core::HRESULT = 0x8051000B_u32 as _;
pub const OPC_E_DUPLICATE_PIECE: windows_core::HRESULT = 0x80510015_u32 as _;
pub const OPC_E_DUPLICATE_RELATIONSHIP: windows_core::HRESULT = 0x80510013_u32 as _;
pub const OPC_E_ENUM_CANNOT_MOVE_NEXT: windows_core::HRESULT = 0x80510051_u32 as _;
pub const OPC_E_ENUM_CANNOT_MOVE_PREVIOUS: windows_core::HRESULT = 0x80510052_u32 as _;
pub const OPC_E_ENUM_COLLECTION_CHANGED: windows_core::HRESULT = 0x80510050_u32 as _;
pub const OPC_E_ENUM_INVALID_POSITION: windows_core::HRESULT = 0x80510053_u32 as _;
pub const OPC_E_INVALID_CONTENT_TYPE: windows_core::HRESULT = 0x80510044_u32 as _;
pub const OPC_E_INVALID_CONTENT_TYPE_XML: windows_core::HRESULT = 0x80510006_u32 as _;
pub const OPC_E_INVALID_DEFAULT_EXTENSION: windows_core::HRESULT = 0x8051000E_u32 as _;
pub const OPC_E_INVALID_OVERRIDE_PART_NAME: windows_core::HRESULT = 0x8051000C_u32 as _;
pub const OPC_E_INVALID_PIECE: windows_core::HRESULT = 0x80510016_u32 as _;
pub const OPC_E_INVALID_RELATIONSHIP_ID: windows_core::HRESULT = 0x80510010_u32 as _;
pub const OPC_E_INVALID_RELATIONSHIP_TARGET: windows_core::HRESULT = 0x80510012_u32 as _;
pub const OPC_E_INVALID_RELATIONSHIP_TARGET_MODE: windows_core::HRESULT = 0x8051004D_u32 as _;
pub const OPC_E_INVALID_RELATIONSHIP_TYPE: windows_core::HRESULT = 0x80510011_u32 as _;
pub const OPC_E_INVALID_RELS_XML: windows_core::HRESULT = 0x8051000A_u32 as _;
pub const OPC_E_INVALID_XML_ENCODING: windows_core::HRESULT = 0x80510042_u32 as _;
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ATTRIBUTES: windows_core::HRESULT = 0x8051004B_u32 as _;
pub const OPC_E_MC_INCONSISTENT_PRESERVE_ELEMENTS: windows_core::HRESULT = 0x8051004C_u32 as _;
pub const OPC_E_MC_INCONSISTENT_PROCESS_CONTENT: windows_core::HRESULT = 0x8051004A_u32 as _;
pub const OPC_E_MC_INVALID_ATTRIBUTES_ON_IGNORABLE_ELEMENT: windows_core::HRESULT = 0x80510040_u32 as _;
pub const OPC_E_MC_INVALID_ENUM_TYPE: windows_core::HRESULT = 0x8051003C_u32 as _;
pub const OPC_E_MC_INVALID_PREFIX_LIST: windows_core::HRESULT = 0x80510037_u32 as _;
pub const OPC_E_MC_INVALID_QNAME_LIST: windows_core::HRESULT = 0x80510038_u32 as _;
pub const OPC_E_MC_INVALID_XMLNS_ATTRIBUTE: windows_core::HRESULT = 0x80510041_u32 as _;
pub const OPC_E_MC_MISSING_CHOICE: windows_core::HRESULT = 0x8051003B_u32 as _;
pub const OPC_E_MC_MISSING_REQUIRES_ATTR: windows_core::HRESULT = 0x80510035_u32 as _;
pub const OPC_E_MC_MULTIPLE_FALLBACK_ELEMENTS: windows_core::HRESULT = 0x80510049_u32 as _;
pub const OPC_E_MC_NESTED_ALTERNATE_CONTENT: windows_core::HRESULT = 0x80510039_u32 as _;
pub const OPC_E_MC_UNEXPECTED_ATTR: windows_core::HRESULT = 0x80510036_u32 as _;
pub const OPC_E_MC_UNEXPECTED_CHOICE: windows_core::HRESULT = 0x8051003A_u32 as _;
pub const OPC_E_MC_UNEXPECTED_ELEMENT: windows_core::HRESULT = 0x80510033_u32 as _;
pub const OPC_E_MC_UNEXPECTED_REQUIRES_ATTR: windows_core::HRESULT = 0x80510034_u32 as _;
pub const OPC_E_MC_UNKNOWN_NAMESPACE: windows_core::HRESULT = 0x8051003E_u32 as _;
pub const OPC_E_MC_UNKNOWN_PREFIX: windows_core::HRESULT = 0x8051003F_u32 as _;
pub const OPC_E_MISSING_CONTENT_TYPES: windows_core::HRESULT = 0x80510007_u32 as _;
pub const OPC_E_MISSING_PIECE: windows_core::HRESULT = 0x80510017_u32 as _;
pub const OPC_E_NONCONFORMING_CONTENT_TYPES_XML: windows_core::HRESULT = 0x80510008_u32 as _;
pub const OPC_E_NONCONFORMING_RELS_XML: windows_core::HRESULT = 0x80510009_u32 as _;
pub const OPC_E_NONCONFORMING_URI: windows_core::HRESULT = 0x80510001_u32 as _;
pub const OPC_E_NO_SUCH_PART: windows_core::HRESULT = 0x80510018_u32 as _;
pub const OPC_E_NO_SUCH_RELATIONSHIP: windows_core::HRESULT = 0x80510048_u32 as _;
pub const OPC_E_NO_SUCH_SETTINGS: windows_core::HRESULT = 0x80510057_u32 as _;
pub const OPC_E_PART_CANNOT_BE_DIRECTORY: windows_core::HRESULT = 0x80510004_u32 as _;
pub const OPC_E_RELATIONSHIP_URI_REQUIRED: windows_core::HRESULT = 0x80510003_u32 as _;
pub const OPC_E_RELATIVE_URI_REQUIRED: windows_core::HRESULT = 0x80510002_u32 as _;
pub const OPC_E_UNEXPECTED_CONTENT_TYPE: windows_core::HRESULT = 0x80510005_u32 as _;
pub const OPC_E_UNSUPPORTED_PACKAGE: windows_core::HRESULT = 0x8051004F_u32 as _;
pub const OPC_E_ZIP_CENTRAL_DIRECTORY_TOO_LARGE: windows_core::HRESULT = 0x80511009_u32 as _;
pub const OPC_E_ZIP_COMMENT_TOO_LARGE: windows_core::HRESULT = 0x8051100C_u32 as _;
pub const OPC_E_ZIP_COMPRESSION_FAILED: windows_core::HRESULT = 0x80511003_u32 as _;
pub const OPC_E_ZIP_CORRUPTED_ARCHIVE: windows_core::HRESULT = 0x80511002_u32 as _;
pub const OPC_E_ZIP_DECOMPRESSION_FAILED: windows_core::HRESULT = 0x80511004_u32 as _;
pub const OPC_E_ZIP_DUPLICATE_NAME: windows_core::HRESULT = 0x8051100B_u32 as _;
pub const OPC_E_ZIP_EXTRA_FIELDS_TOO_LARGE: windows_core::HRESULT = 0x8051100D_u32 as _;
pub const OPC_E_ZIP_FILE_HEADER_TOO_LARGE: windows_core::HRESULT = 0x8051100E_u32 as _;
pub const OPC_E_ZIP_INCONSISTENT_DIRECTORY: windows_core::HRESULT = 0x80511006_u32 as _;
pub const OPC_E_ZIP_INCONSISTENT_FILEITEM: windows_core::HRESULT = 0x80511005_u32 as _;
pub const OPC_E_ZIP_INCORRECT_DATA_SIZE: windows_core::HRESULT = 0x80511001_u32 as _;
pub const OPC_E_ZIP_MISSING_DATA_DESCRIPTOR: windows_core::HRESULT = 0x80511007_u32 as _;
pub const OPC_E_ZIP_MISSING_END_OF_CENTRAL_DIRECTORY: windows_core::HRESULT = 0x8051100F_u32 as _;
pub const OPC_E_ZIP_NAME_TOO_LARGE: windows_core::HRESULT = 0x8051100A_u32 as _;
pub const OPC_E_ZIP_REQUIRES_64_BIT: windows_core::HRESULT = 0x80511010_u32 as _;
pub const OPC_E_ZIP_UNSUPPORTEDARCHIVE: windows_core::HRESULT = 0x80511008_u32 as _;
pub const OPC_READ_DEFAULT: OPC_READ_FLAGS = 0i32;
pub const OPC_RELATIONSHIP_SELECT_BY_ID: OPC_RELATIONSHIP_SELECTOR = 0i32;
pub const OPC_RELATIONSHIP_SELECT_BY_TYPE: OPC_RELATIONSHIP_SELECTOR = 1i32;
pub const OPC_RELATIONSHIP_SIGN_PART: OPC_RELATIONSHIPS_SIGNING_OPTION = 1i32;
pub const OPC_RELATIONSHIP_SIGN_USING_SELECTORS: OPC_RELATIONSHIPS_SIGNING_OPTION = 0i32;
pub const OPC_SIGNATURE_INVALID: OPC_SIGNATURE_VALIDATION_RESULT = -1i32;
pub const OPC_SIGNATURE_TIME_FORMAT_DAYS: OPC_SIGNATURE_TIME_FORMAT = 3i32;
pub const OPC_SIGNATURE_TIME_FORMAT_MILLISECONDS: OPC_SIGNATURE_TIME_FORMAT = 0i32;
pub const OPC_SIGNATURE_TIME_FORMAT_MINUTES: OPC_SIGNATURE_TIME_FORMAT = 2i32;
pub const OPC_SIGNATURE_TIME_FORMAT_MONTHS: OPC_SIGNATURE_TIME_FORMAT = 4i32;
pub const OPC_SIGNATURE_TIME_FORMAT_SECONDS: OPC_SIGNATURE_TIME_FORMAT = 1i32;
pub const OPC_SIGNATURE_TIME_FORMAT_YEARS: OPC_SIGNATURE_TIME_FORMAT = 5i32;
pub const OPC_SIGNATURE_VALID: OPC_SIGNATURE_VALIDATION_RESULT = 0i32;
pub const OPC_STREAM_IO_READ: OPC_STREAM_IO_MODE = 1i32;
pub const OPC_STREAM_IO_WRITE: OPC_STREAM_IO_MODE = 2i32;
pub const OPC_URI_TARGET_MODE_EXTERNAL: OPC_URI_TARGET_MODE = 1i32;
pub const OPC_URI_TARGET_MODE_INTERNAL: OPC_URI_TARGET_MODE = 0i32;
pub const OPC_VALIDATE_ON_LOAD: OPC_READ_FLAGS = 1i32;
pub const OPC_WRITE_DEFAULT: OPC_WRITE_FLAGS = 0i32;
pub const OPC_WRITE_FORCE_ZIP32: OPC_WRITE_FLAGS = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_CANONICALIZATION_METHOD(pub i32);
impl windows_core::TypeKind for OPC_CANONICALIZATION_METHOD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_CERTIFICATE_EMBEDDING_OPTION(pub i32);
impl windows_core::TypeKind for OPC_CERTIFICATE_EMBEDDING_OPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_COMPRESSION_OPTIONS(pub i32);
impl windows_core::TypeKind for OPC_COMPRESSION_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_READ_FLAGS(pub i32);
impl windows_core::TypeKind for OPC_READ_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_RELATIONSHIPS_SIGNING_OPTION(pub i32);
impl windows_core::TypeKind for OPC_RELATIONSHIPS_SIGNING_OPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_RELATIONSHIP_SELECTOR(pub i32);
impl windows_core::TypeKind for OPC_RELATIONSHIP_SELECTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_SIGNATURE_TIME_FORMAT(pub i32);
impl windows_core::TypeKind for OPC_SIGNATURE_TIME_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_SIGNATURE_VALIDATION_RESULT(pub i32);
impl windows_core::TypeKind for OPC_SIGNATURE_VALIDATION_RESULT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_STREAM_IO_MODE(pub i32);
impl windows_core::TypeKind for OPC_STREAM_IO_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_URI_TARGET_MODE(pub i32);
impl windows_core::TypeKind for OPC_URI_TARGET_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OPC_WRITE_FLAGS(pub i32);
impl windows_core::TypeKind for OPC_WRITE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
pub const OpcFactory: windows_core::GUID = windows_core::GUID::from_u128(0x6b2d6ba0_9f3e_4f27_920b_313cc426a39e);
