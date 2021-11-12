#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct FindSimilarFileIndexResults(i32);
pub struct FindSimilarResults(i32);
pub struct GeneratorParametersType(i32);
pub struct IFindSimilarResults(i32);
pub struct IRdcComparator(i32);
pub struct IRdcFileReader(i32);
pub struct IRdcFileWriter(i32);
pub struct IRdcGenerator(i32);
pub struct IRdcGeneratorFilterMaxParameters(i32);
pub struct IRdcGeneratorParameters(i32);
pub struct IRdcLibrary(i32);
pub struct IRdcSignatureReader(i32);
pub struct IRdcSimilarityGenerator(i32);
pub struct ISimilarity(i32);
pub struct ISimilarityFileIdTable(i32);
pub struct ISimilarityReportProgress(i32);
pub struct ISimilarityTableDumpState(i32);
pub struct ISimilarityTraitsMappedView(i32);
pub struct ISimilarityTraitsMapping(i32);
pub struct ISimilarityTraitsTable(i32);
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_DEFAULT_COMPAREBUFFER: u32 = 3200000u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_1: u32 = 48u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_N: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_1: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_N: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MAXIMUM_COMPAREBUFFER: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MAXIMUM_DEPTH: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MAXIMUM_HASHWINDOWSIZE: u32 = 96u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MAXIMUM_HORIZONSIZE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MAXIMUM_MATCHESREQUIRED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MAXIMUM_TRAITVALUE: u32 = 63u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MINIMUM_COMPAREBUFFER: u32 = 100000u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MINIMUM_COMPATIBLE_APP_VERSION: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MINIMUM_DEPTH: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MINIMUM_HASHWINDOWSIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MINIMUM_HORIZONSIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MINIMUM_INPUTBUFFERSIZE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_MINIMUM_MATCHESREQUIRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_SIGNATURE_HASHSIZE: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const MSRDC_VERSION: u32 = 65536u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const RDCE_TABLE_CORRUPT: u32 = 2147745794u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
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
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const SimilarityFileIdMaxSize: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_RemoteDifferentialCompression`*"]
pub const SimilarityFileIdMinSize: u32 = 4u32;
pub struct SimilarityFileIdTable(i32);
pub struct SimilarityMappedViewInfo(i32);
pub struct SimilarityReportProgress(i32);
pub struct SimilarityTableDumpState(i32);
pub struct SimilarityTraitsMappedView(i32);
pub struct SimilarityTraitsMapping(i32);
pub struct SimilarityTraitsTable(i32);
