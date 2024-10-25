pub const MSRDC_DEFAULT_COMPAREBUFFER: u32 = 3200000u32;
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_1: u32 = 48u32;
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_N: u32 = 2u32;
pub const MSRDC_DEFAULT_HORIZONSIZE_1: u32 = 1024u32;
pub const MSRDC_DEFAULT_HORIZONSIZE_N: u32 = 128u32;
pub const MSRDC_MAXIMUM_COMPAREBUFFER: u32 = 1073741824u32;
pub const MSRDC_MAXIMUM_DEPTH: u32 = 8u32;
pub const MSRDC_MAXIMUM_HASHWINDOWSIZE: u32 = 96u32;
pub const MSRDC_MAXIMUM_HORIZONSIZE: u32 = 16384u32;
pub const MSRDC_MAXIMUM_MATCHESREQUIRED: u32 = 16u32;
pub const MSRDC_MAXIMUM_TRAITVALUE: u32 = 63u32;
pub const MSRDC_MINIMUM_COMPAREBUFFER: u32 = 100000u32;
pub const MSRDC_MINIMUM_COMPATIBLE_APP_VERSION: u32 = 65536u32;
pub const MSRDC_MINIMUM_DEPTH: u32 = 1u32;
pub const MSRDC_MINIMUM_HASHWINDOWSIZE: u32 = 2u32;
pub const MSRDC_MINIMUM_HORIZONSIZE: u32 = 128u32;
pub const MSRDC_MINIMUM_INPUTBUFFERSIZE: u32 = 1024u32;
pub const MSRDC_MINIMUM_MATCHESREQUIRED: u32 = 1u32;
pub const MSRDC_SIGNATURE_HASHSIZE: u32 = 16u32;
pub const MSRDC_VERSION: u32 = 65536u32;
pub const RDCE_TABLE_CORRUPT: u32 = 2147745794u32;
pub const RDCE_TABLE_FULL: u32 = 2147745793u32;
pub const RDCGENTYPE_FilterMax: GeneratorParametersType = 1i32;
pub const RDCGENTYPE_Unused: GeneratorParametersType = 0i32;
pub const RDCMAPPING_ReadOnly: RdcMappingAccessMode = 1i32;
pub const RDCMAPPING_ReadWrite: RdcMappingAccessMode = 2i32;
pub const RDCMAPPING_Undefined: RdcMappingAccessMode = 0i32;
pub const RDCNEED_SEED: RdcNeedType = 2i32;
pub const RDCNEED_SEED_MAX: RdcNeedType = 255i32;
pub const RDCNEED_SOURCE: RdcNeedType = 0i32;
pub const RDCNEED_TARGET: RdcNeedType = 1i32;
pub const RDCTABLE_Existing: RdcCreatedTables = 1i32;
pub const RDCTABLE_InvalidOrUnknown: RdcCreatedTables = 0i32;
pub const RDCTABLE_New: RdcCreatedTables = 2i32;
pub const RDC_Aborted: RDC_ErrorCode = 9i32;
pub const RDC_ApplicationError: RDC_ErrorCode = 8i32;
pub const RDC_DataMissingOrCorrupt: RDC_ErrorCode = 5i32;
pub const RDC_DataTooManyRecords: RDC_ErrorCode = 6i32;
pub const RDC_FileChecksumMismatch: RDC_ErrorCode = 7i32;
pub const RDC_HeaderMissingOrCorrupt: RDC_ErrorCode = 3i32;
pub const RDC_HeaderVersionNewer: RDC_ErrorCode = 1i32;
pub const RDC_HeaderVersionOlder: RDC_ErrorCode = 2i32;
pub const RDC_HeaderWrongType: RDC_ErrorCode = 4i32;
pub const RDC_NoError: RDC_ErrorCode = 0i32;
pub const RDC_Win32Error: RDC_ErrorCode = 10i32;
pub const SimilarityFileIdMaxSize: u32 = 32u32;
pub const SimilarityFileIdMinSize: u32 = 4u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GeneratorParametersType(pub i32);
impl windows_core::TypeKind for GeneratorParametersType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RDC_ErrorCode(pub i32);
impl windows_core::TypeKind for RDC_ErrorCode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RdcCreatedTables(pub i32);
impl windows_core::TypeKind for RdcCreatedTables {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RdcMappingAccessMode(pub i32);
impl windows_core::TypeKind for RdcMappingAccessMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RdcNeedType(pub i32);
impl windows_core::TypeKind for RdcNeedType {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FindSimilarFileIndexResults {
    pub m_FileIndex: u32,
    pub m_MatchCount: u32,
}
impl Default for FindSimilarFileIndexResults {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FindSimilarFileIndexResults {
    type TypeKind = windows_core::CopyType;
}
pub const FindSimilarResults: windows_core::GUID = windows_core::GUID::from_u128(0x96236a93_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcBufferPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut u8,
}
impl Default for RdcBufferPointer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RdcBufferPointer {
    type TypeKind = windows_core::CopyType;
}
pub const RdcComparator: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8b_9dbc_11da_9e3f_0011114ae311);
pub const RdcFileReader: windows_core::GUID = windows_core::GUID::from_u128(0x96236a89_9dbc_11da_9e3f_0011114ae311);
pub const RdcGenerator: windows_core::GUID = windows_core::GUID::from_u128(0x96236a88_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorFilterMaxParameters: windows_core::GUID = windows_core::GUID::from_u128(0x96236a87_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorParameters: windows_core::GUID = windows_core::GUID::from_u128(0x96236a86_9dbc_11da_9e3f_0011114ae311);
pub const RdcLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x96236a85_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcNeed {
    pub m_BlockType: RdcNeedType,
    pub m_FileOffset: u64,
    pub m_BlockLength: u64,
}
impl Default for RdcNeed {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RdcNeed {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcNeedPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcNeed,
}
impl Default for RdcNeedPointer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RdcNeedPointer {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcSignature {
    pub m_Signature: [u8; 16],
    pub m_BlockLength: u16,
}
impl Default for RdcSignature {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RdcSignature {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RdcSignaturePointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcSignature,
}
impl Default for RdcSignaturePointer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RdcSignaturePointer {
    type TypeKind = windows_core::CopyType;
}
pub const RdcSignatureReader: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8a_9dbc_11da_9e3f_0011114ae311);
pub const RdcSimilarityGenerator: windows_core::GUID = windows_core::GUID::from_u128(0x96236a92_9dbc_11da_9e3f_0011114ae311);
pub const Similarity: windows_core::GUID = windows_core::GUID::from_u128(0x96236a91_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimilarityData {
    pub m_Data: [u8; 16],
}
impl Default for SimilarityData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SimilarityData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimilarityDumpData {
    pub m_FileIndex: u32,
    pub m_Data: SimilarityData,
}
impl Default for SimilarityDumpData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SimilarityDumpData {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimilarityFileId {
    pub m_FileId: [u8; 32],
}
impl Default for SimilarityFileId {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SimilarityFileId {
    type TypeKind = windows_core::CopyType;
}
pub const SimilarityFileIdTable: windows_core::GUID = windows_core::GUID::from_u128(0x96236a90_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimilarityMappedViewInfo {
    pub m_Data: *mut u8,
    pub m_Length: u32,
}
impl Default for SimilarityMappedViewInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SimilarityMappedViewInfo {
    type TypeKind = windows_core::CopyType;
}
pub const SimilarityReportProgress: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8d_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTableDumpState: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8e_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsMappedView: windows_core::GUID = windows_core::GUID::from_u128(0x96236a95_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsMapping: windows_core::GUID = windows_core::GUID::from_u128(0x96236a94_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsTable: windows_core::GUID = windows_core::GUID::from_u128(0x96236a8f_9dbc_11da_9e3f_0011114ae311);
