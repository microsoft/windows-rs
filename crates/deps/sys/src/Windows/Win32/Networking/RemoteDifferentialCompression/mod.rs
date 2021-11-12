#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct FindSimilarFileIndexResults(i32);
pub struct FindSimilarResults(i32);
pub struct GeneratorParametersType(i32);
#[repr(transparent)]
pub struct IFindSimilarResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcComparator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcFileReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcFileWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcGeneratorFilterMaxParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcGeneratorParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcLibrary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcSignatureReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRdcSimilarityGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimilarity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimilarityFileIdTable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimilarityReportProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimilarityTableDumpState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimilarityTraitsMappedView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimilarityTraitsMapping(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimilarityTraitsTable(pub *mut ::core::ffi::c_void);
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
pub struct RDC_ErrorCode(i32);
pub struct RdcBufferPointer(i32);
pub struct RdcComparator(i32);
pub struct RdcCreatedTables(i32);
pub struct RdcFileReader(i32);
pub struct RdcGenerator(i32);
pub struct RdcGeneratorFilterMaxParameters(i32);
pub struct RdcGeneratorParameters(i32);
pub struct RdcLibrary(i32);
pub struct RdcMappingAccessMode(i32);
pub struct RdcNeed(i32);
pub struct RdcNeedPointer(i32);
pub struct RdcNeedType(i32);
pub struct RdcSignature(i32);
pub struct RdcSignaturePointer(i32);
pub struct RdcSignatureReader(i32);
pub struct RdcSimilarityGenerator(i32);
pub struct Similarity(i32);
pub struct SimilarityData(i32);
pub struct SimilarityDumpData(i32);
pub struct SimilarityFileId(i32);
pub const SimilarityFileIdMaxSize: u32 = 32u32;
pub const SimilarityFileIdMinSize: u32 = 4u32;
pub struct SimilarityFileIdTable(i32);
pub struct SimilarityMappedViewInfo(i32);
pub struct SimilarityReportProgress(i32);
pub struct SimilarityTableDumpState(i32);
pub struct SimilarityTraitsMappedView(i32);
pub struct SimilarityTraitsMapping(i32);
pub struct SimilarityTraitsTable(i32);
