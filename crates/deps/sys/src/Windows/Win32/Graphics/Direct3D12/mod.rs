#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12CreateDevice(padapter: ::windows_sys::core::IUnknown, minimumfeaturelevel: super::Direct3D::D3D_FEATURE_LEVEL, riid: *const ::windows_sys::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12CreateRootSignatureDeserializer(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows_sys::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12CreateVersionedRootSignatureDeserializer(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows_sys::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12EnableExperimentalFeatures(numfeatures: u32, piids: *const ::windows_sys::core::GUID, pconfigurationstructs: *const ::core::ffi::c_void, pconfigurationstructsizes: *const u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12GetDebugInterface(riid: *const ::windows_sys::core::GUID, ppvdebug: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12GetInterface(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppvdebug: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12SerializeRootSignature(prootsignature: *const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, ppblob: *mut super::Direct3D::ID3DBlob, pperrorblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12SerializeVersionedRootSignature(prootsignature: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob: *mut super::Direct3D::ID3DBlob, pperrorblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_D3D12Debug: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4063570667,
    data2: 56708,
    data3: 18942,
    data4: [185, 123, 169, 220, 253, 204, 27, 79],
};
pub const CLSID_D3D12DeviceRemovedExtendedData: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1249229764,
    data2: 40948,
    data3: 19160,
    data4: [159, 24, 171, 174, 132, 220, 95, 242],
};
pub const CLSID_D3D12SDKConfiguration: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2094688970, data2: 41022, data3: 18888, data4: [148, 88, 3, 52, 210, 14, 7, 206] };
pub const CLSID_D3D12Tools: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3816953521, data2: 15500, data3: 18483, data4: [170, 9, 10, 6, 182, 93, 150, 200] };
pub const D3D12ExperimentalShaderModels: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1995790142,
    data2: 61754,
    data3: 16629,
    data4: [178, 151, 129, 206, 158, 24, 147, 63],
};
pub struct D3D12MessageFunc(i32);
pub const D3D12MetaCommand: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3342125438,
    data2: 32887,
    data3: 18632,
    data4: [159, 220, 217, 209, 221, 49, 221, 119],
};
pub const D3D12TiledResourceTier4: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3385094751,
    data2: 43034,
    data3: 20310,
    data4: [140, 91, 197, 16, 57, 214, 148, 251],
};
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
pub struct D3D12_AUTO_BREADCRUMB_NODE(i32);
pub struct D3D12_AUTO_BREADCRUMB_NODE1(i32);
pub struct D3D12_AUTO_BREADCRUMB_OP(i32);
pub struct D3D12_AXIS_SHADING_RATE(i32);
pub struct D3D12_BACKGROUND_PROCESSING_MODE(i32);
pub struct D3D12_BLEND(i32);
pub struct D3D12_BLEND_DESC(i32);
pub struct D3D12_BLEND_OP(i32);
pub struct D3D12_BOX(i32);
pub struct D3D12_BUFFER_RTV(i32);
pub struct D3D12_BUFFER_SRV(i32);
pub struct D3D12_BUFFER_SRV_FLAGS(i32);
pub struct D3D12_BUFFER_UAV(i32);
pub struct D3D12_BUFFER_UAV_FLAGS(i32);
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC(i32);
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS(i32);
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER(i32);
pub struct D3D12_CACHED_PIPELINE_STATE(i32);
pub struct D3D12_CLEAR_FLAGS(i32);
pub struct D3D12_CLEAR_VALUE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
pub struct D3D12_COLOR_WRITE_ENABLE(i32);
pub struct D3D12_COMMAND_LIST_FLAGS(i32);
pub struct D3D12_COMMAND_LIST_SUPPORT_FLAGS(i32);
pub struct D3D12_COMMAND_LIST_TYPE(i32);
pub struct D3D12_COMMAND_POOL_FLAGS(i32);
pub struct D3D12_COMMAND_QUEUE_DESC(i32);
pub struct D3D12_COMMAND_QUEUE_FLAGS(i32);
pub struct D3D12_COMMAND_QUEUE_PRIORITY(i32);
pub struct D3D12_COMMAND_RECORDER_FLAGS(i32);
pub struct D3D12_COMMAND_SIGNATURE_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
pub struct D3D12_COMPARISON_FUNC(i32);
pub struct D3D12_COMPUTE_PIPELINE_STATE_DESC(i32);
pub struct D3D12_CONSERVATIVE_RASTERIZATION_MODE(i32);
pub struct D3D12_CONSERVATIVE_RASTERIZATION_TIER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT: u32 = 256u32;
pub struct D3D12_CONSTANT_BUFFER_VIEW_DESC(i32);
pub struct D3D12_CPU_DESCRIPTOR_HANDLE(i32);
pub struct D3D12_CPU_PAGE_PROPERTY(i32);
pub struct D3D12_CROSS_NODE_SHARING_TIER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 240u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: u32 = 68u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 224u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: u32 = 72u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 208u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: u32 = 76u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 192u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: u32 = 84u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 176u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: u32 = 92u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 160u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: u32 = 100u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 144u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: u32 = 112u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 112u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: u32 = 144u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 96u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: u32 = 168u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 80u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: u32 = 204u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 48u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: u32 = 340u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: u32 = 768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_X: u32 = 768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_Y: u32 = 768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_4_X_UAV_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_TGSM_REGISTER_COUNT: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_TGSM_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREADGROUPID_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREADGROUPID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREADID_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREADID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREAD_GROUP_MAX_X: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREAD_GROUP_MAX_Y: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREAD_GROUP_MAX_Z: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREAD_GROUP_MIN_X: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREAD_GROUP_MIN_Y: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREAD_GROUP_MIN_Z: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: u32 = 16384u32;
pub struct D3D12_CULL_MODE(i32);
pub struct D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS(i32);
pub struct D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE(i32);
pub struct D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS(i32);
pub struct D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR(i32);
pub struct D3D12_DEBUG_DEVICE_PARAMETER_TYPE(i32);
pub struct D3D12_DEBUG_FEATURE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_DEPTH_BIAS: i32 = 0i32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_MAX_ANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: u32 = 5768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
pub struct D3D12_DEPTH_STENCILOP_DESC(i32);
pub struct D3D12_DEPTH_STENCIL_DESC(i32);
pub struct D3D12_DEPTH_STENCIL_DESC1(i32);
pub struct D3D12_DEPTH_STENCIL_VALUE(i32);
pub struct D3D12_DEPTH_STENCIL_VIEW_DESC(i32);
pub struct D3D12_DEPTH_WRITE_MASK(i32);
pub struct D3D12_DESCRIPTOR_HEAP_DESC(i32);
pub struct D3D12_DESCRIPTOR_HEAP_FLAGS(i32);
pub struct D3D12_DESCRIPTOR_HEAP_TYPE(i32);
pub struct D3D12_DESCRIPTOR_RANGE(i32);
pub struct D3D12_DESCRIPTOR_RANGE1(i32);
pub struct D3D12_DESCRIPTOR_RANGE_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = 4294967295u32;
pub struct D3D12_DESCRIPTOR_RANGE_TYPE(i32);
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA(i32);
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA1(i32);
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA2(i32);
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA3(i32);
pub struct D3D12_DISCARD_REGION(i32);
pub struct D3D12_DISPATCH_ARGUMENTS(i32);
pub struct D3D12_DISPATCH_MESH_ARGUMENTS(i32);
pub struct D3D12_DISPATCH_RAYS_DESC(i32);
pub struct D3D12_DRAW_ARGUMENTS(i32);
pub struct D3D12_DRAW_INDEXED_ARGUMENTS(i32);
pub struct D3D12_DRED_ALLOCATION_NODE(i32);
pub struct D3D12_DRED_ALLOCATION_NODE1(i32);
pub struct D3D12_DRED_ALLOCATION_TYPE(i32);
pub struct D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT(i32);
pub struct D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1(i32);
pub struct D3D12_DRED_BREADCRUMB_CONTEXT(i32);
pub struct D3D12_DRED_DEVICE_STATE(i32);
pub struct D3D12_DRED_ENABLEMENT(i32);
pub struct D3D12_DRED_FLAGS(i32);
pub struct D3D12_DRED_PAGE_FAULT_FLAGS(i32);
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT(i32);
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT1(i32);
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT2(i32);
pub struct D3D12_DRED_VERSION(i32);
pub struct D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967287u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967280u32;
pub struct D3D12_DSV_DIMENSION(i32);
pub struct D3D12_DSV_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_DS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub struct D3D12_DXIL_LIBRARY_DESC(i32);
pub struct D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION(i32);
pub struct D3D12_ELEMENTS_LAYOUT(i32);
pub struct D3D12_EXISTING_COLLECTION_DESC(i32);
pub struct D3D12_EXPORT_DESC(i32);
pub struct D3D12_EXPORT_FLAGS(i32);
pub struct D3D12_FEATURE(i32);
pub struct D3D12_FEATURE_DATA_ARCHITECTURE(i32);
pub struct D3D12_FEATURE_DATA_ARCHITECTURE1(i32);
pub struct D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY(i32);
pub struct D3D12_FEATURE_DATA_CROSS_NODE(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS1(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS10(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS11(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS2(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS3(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS4(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS5(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS6(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS7(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS8(i32);
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS9(i32);
pub struct D3D12_FEATURE_DATA_DISPLAYABLE(i32);
pub struct D3D12_FEATURE_DATA_EXISTING_HEAPS(i32);
pub struct D3D12_FEATURE_DATA_FEATURE_LEVELS(i32);
pub struct D3D12_FEATURE_DATA_FORMAT_INFO(i32);
pub struct D3D12_FEATURE_DATA_FORMAT_SUPPORT(i32);
pub struct D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT(i32);
pub struct D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS(i32);
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT(i32);
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES(i32);
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT(i32);
pub struct D3D12_FEATURE_DATA_QUERY_META_COMMAND(i32);
pub struct D3D12_FEATURE_DATA_ROOT_SIGNATURE(i32);
pub struct D3D12_FEATURE_DATA_SERIALIZATION(i32);
pub struct D3D12_FEATURE_DATA_SHADER_CACHE(i32);
pub struct D3D12_FEATURE_DATA_SHADER_MODEL(i32);
pub struct D3D12_FENCE_FLAGS(i32);
pub struct D3D12_FILL_MODE(i32);
pub struct D3D12_FILTER(i32);
pub struct D3D12_FILTER_REDUCTION_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FILTER_REDUCTION_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FILTER_REDUCTION_TYPE_SHIFT: u32 = 7u32;
pub struct D3D12_FILTER_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FILTER_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
pub struct D3D12_FORMAT_SUPPORT1(i32);
pub struct D3D12_FORMAT_SUPPORT2(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
pub struct D3D12_FUNCTION_DESC(i32);
pub struct D3D12_GLOBAL_ROOT_SIGNATURE(i32);
pub struct D3D12_GPU_BASED_VALIDATION_FLAGS(i32);
pub struct D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS(i32);
pub struct D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE(i32);
pub struct D3D12_GPU_DESCRIPTOR_HANDLE(i32);
pub struct D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE(i32);
pub struct D3D12_GPU_VIRTUAL_ADDRESS_RANGE(i32);
pub struct D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE(i32);
pub struct D3D12_GRAPHICS_PIPELINE_STATE_DESC(i32);
pub struct D3D12_GRAPHICS_STATES(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_INPUT_REGISTER_VERTICES: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_MAX_INSTANCE_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_OUTPUT_ELEMENTS: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub struct D3D12_HEAP_DESC(i32);
pub struct D3D12_HEAP_FLAGS(i32);
pub struct D3D12_HEAP_PROPERTIES(i32);
pub struct D3D12_HEAP_SERIALIZATION_TIER(i32);
pub struct D3D12_HEAP_TYPE(i32);
pub struct D3D12_HIT_GROUP_DESC(i32);
pub struct D3D12_HIT_GROUP_TYPE(i32);
pub struct D3D12_HIT_KIND(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_MAXTESSFACTOR_LOWER_BOUND: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_MAXTESSFACTOR_UPPER_BOUND: f32 = 64f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_PATCH_MAX_CONTROL_POINT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32u32;
pub struct D3D12_INDEX_BUFFER_STRIP_CUT_VALUE(i32);
pub struct D3D12_INDEX_BUFFER_VIEW(i32);
pub struct D3D12_INDIRECT_ARGUMENT_DESC(i32);
pub struct D3D12_INDIRECT_ARGUMENT_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
pub struct D3D12_INFO_QUEUE_FILTER(i32);
pub struct D3D12_INFO_QUEUE_FILTER_DESC(i32);
pub struct D3D12_INPUT_CLASSIFICATION(i32);
pub struct D3D12_INPUT_ELEMENT_DESC(i32);
pub struct D3D12_INPUT_LAYOUT_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295u32;
pub struct D3D12_LIBRARY_DESC(i32);
pub struct D3D12_LIFETIME_STATE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_LINEAR_GAMMA: f32 = 1f32;
pub struct D3D12_LOCAL_ROOT_SIGNATURE(i32);
pub struct D3D12_LOGIC_OP(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAG_FILTER_SHIFT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAJOR_VERSION: u32 = 12u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_DEPTH: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_LIVE_STATIC_SAMPLERS: u32 = 2032u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_MAXANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_ROOT_COST: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_1: u32 = 1000000u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_2: u32 = 1000000u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MAX_VIEW_INSTANCE_COUNT: u32 = 4u32;
pub struct D3D12_MEASUREMENTS_ACTION(i32);
pub struct D3D12_MEMCPY_DEST(i32);
pub struct D3D12_MEMORY_POOL(i32);
pub struct D3D12_MESH_SHADER_TIER(i32);
pub struct D3D12_MESSAGE(i32);
pub struct D3D12_MESSAGE_CALLBACK_FLAGS(i32);
pub struct D3D12_MESSAGE_CATEGORY(i32);
pub struct D3D12_MESSAGE_ID(i32);
pub struct D3D12_MESSAGE_SEVERITY(i32);
pub struct D3D12_META_COMMAND_DESC(i32);
pub struct D3D12_META_COMMAND_PARAMETER_DESC(i32);
pub struct D3D12_META_COMMAND_PARAMETER_FLAGS(i32);
pub struct D3D12_META_COMMAND_PARAMETER_STAGE(i32);
pub struct D3D12_META_COMMAND_PARAMETER_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIN_DEPTH: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIN_FILTER_SHIFT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIN_MAXANISOTROPY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIP_FILTER_SHIFT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIP_LOD_BIAS_MIN: f32 = -16f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
pub struct D3D12_MIP_REGION(i32);
pub struct D3D12_MULTIPLE_FENCE_WAIT_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
pub struct D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS(i32);
pub struct D3D12_NODE_MASK(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967288u32;
pub struct D3D12_PACKED_MIP_INFO(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PACKED_TILE: u32 = 4294967295u32;
pub struct D3D12_PARAMETER_DESC(i32);
pub struct D3D12_PIPELINE_STATE_FLAGS(i32);
pub struct D3D12_PIPELINE_STATE_STREAM_DESC(i32);
pub struct D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
pub struct D3D12_PLACED_SUBRESOURCE_FOOTPRINT(i32);
pub struct D3D12_PREDICATION_OP(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub struct D3D12_PRIMITIVE_TOPOLOGY_TYPE(i32);
pub struct D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER(i32);
pub const D3D12_PROTECTED_RESOURCES_SESSION_HARDWARE_PROTECTED: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1655703630, data2: 50958, data3: 19882, data4: [161, 9, 48, 255, 141, 90, 4, 130] };
pub struct D3D12_PROTECTED_RESOURCE_SESSION_DESC(i32);
pub struct D3D12_PROTECTED_RESOURCE_SESSION_DESC1(i32);
pub struct D3D12_PROTECTED_RESOURCE_SESSION_FLAGS(i32);
pub struct D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS(i32);
pub struct D3D12_PROTECTED_SESSION_STATUS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_CS_UAV_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_CS_UAV_REGISTER_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_CS_UAV_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_CS_UAV_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS(i32);
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS1(i32);
pub struct D3D12_QUERY_DATA_SO_STATISTICS(i32);
pub struct D3D12_QUERY_HEAP_DESC(i32);
pub struct D3D12_QUERY_HEAP_TYPE(i32);
pub struct D3D12_QUERY_TYPE(i32);
pub struct D3D12_RANGE(i32);
pub struct D3D12_RANGE_UINT64(i32);
pub struct D3D12_RASTERIZER_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16u32;
pub struct D3D12_RAYTRACING_AABB(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_AABB_BYTE_ALIGNMENT: u32 = 8u32;
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BYTE_ALIGNMENT: u32 = 256u32;
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV(i32);
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE(i32);
pub struct D3D12_RAYTRACING_GEOMETRY_AABBS_DESC(i32);
pub struct D3D12_RAYTRACING_GEOMETRY_DESC(i32);
pub struct D3D12_RAYTRACING_GEOMETRY_FLAGS(i32);
pub struct D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC(i32);
pub struct D3D12_RAYTRACING_GEOMETRY_TYPE(i32);
pub struct D3D12_RAYTRACING_INSTANCE_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_INSTANCE_DESCS_BYTE_ALIGNMENT: u32 = 16u32;
pub struct D3D12_RAYTRACING_INSTANCE_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_MAX_ATTRIBUTE_SIZE_IN_BYTES: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_MAX_DECLARABLE_TRACE_RECURSION_DEPTH: u32 = 31u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_MAX_GEOMETRIES_PER_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_MAX_INSTANCES_PER_TOP_LEVEL_ACCELERATION_STRUCTURE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_MAX_PRIMITIVES_PER_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_MAX_RAY_GENERATION_SHADER_THREADS: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_MAX_SHADER_RECORD_STRIDE: u32 = 4096u32;
pub struct D3D12_RAYTRACING_PIPELINE_CONFIG(i32);
pub struct D3D12_RAYTRACING_PIPELINE_CONFIG1(i32);
pub struct D3D12_RAYTRACING_PIPELINE_FLAGS(i32);
pub struct D3D12_RAYTRACING_SHADER_CONFIG(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_SHADER_TABLE_BYTE_ALIGNMENT: u32 = 64u32;
pub struct D3D12_RAYTRACING_TIER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RAYTRACING_TRANSFORM3X4_BYTE_ALIGNMENT: u32 = 16u32;
pub struct D3D12_RAY_FLAGS(i32);
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS(i32);
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS(i32);
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE(i32);
pub struct D3D12_RENDER_PASS_DEPTH_STENCIL_DESC(i32);
pub struct D3D12_RENDER_PASS_ENDING_ACCESS(i32);
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS(i32);
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS(i32);
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_TYPE(i32);
pub struct D3D12_RENDER_PASS_FLAGS(i32);
pub struct D3D12_RENDER_PASS_RENDER_TARGET_DESC(i32);
pub struct D3D12_RENDER_PASS_TIER(i32);
pub struct D3D12_RENDER_TARGET_BLEND_DESC(i32);
pub struct D3D12_RENDER_TARGET_VIEW_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_MAXANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_MIP_LEVELS: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: f32 = 0.25f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: u32 = 20u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_SUBRESOURCES: u32 = 30720u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_TEXTURE1D_U_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_REQ_TEXTURECUBE_DIMENSION: u32 = 16384u32;
pub struct D3D12_RESIDENCY_FLAGS(i32);
pub struct D3D12_RESIDENCY_PRIORITY(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
pub struct D3D12_RESOLVE_MODE(i32);
pub struct D3D12_RESOURCE_ALIASING_BARRIER(i32);
pub struct D3D12_RESOURCE_ALLOCATION_INFO(i32);
pub struct D3D12_RESOURCE_ALLOCATION_INFO1(i32);
pub struct D3D12_RESOURCE_BARRIER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: u32 = 4294967295u32;
pub struct D3D12_RESOURCE_BARRIER_FLAGS(i32);
pub struct D3D12_RESOURCE_BARRIER_TYPE(i32);
pub struct D3D12_RESOURCE_BINDING_TIER(i32);
pub struct D3D12_RESOURCE_DESC(i32);
pub struct D3D12_RESOURCE_DESC1(i32);
pub struct D3D12_RESOURCE_DIMENSION(i32);
pub struct D3D12_RESOURCE_FLAGS(i32);
pub struct D3D12_RESOURCE_HEAP_TIER(i32);
pub struct D3D12_RESOURCE_STATES(i32);
pub struct D3D12_RESOURCE_TRANSITION_BARRIER(i32);
pub struct D3D12_RESOURCE_UAV_BARRIER(i32);
pub struct D3D12_RLDO_FLAGS(i32);
pub struct D3D12_ROOT_CONSTANTS(i32);
pub struct D3D12_ROOT_DESCRIPTOR(i32);
pub struct D3D12_ROOT_DESCRIPTOR1(i32);
pub struct D3D12_ROOT_DESCRIPTOR_FLAGS(i32);
pub struct D3D12_ROOT_DESCRIPTOR_TABLE(i32);
pub struct D3D12_ROOT_DESCRIPTOR_TABLE1(i32);
pub struct D3D12_ROOT_PARAMETER(i32);
pub struct D3D12_ROOT_PARAMETER1(i32);
pub struct D3D12_ROOT_PARAMETER_TYPE(i32);
pub struct D3D12_ROOT_SIGNATURE_DESC(i32);
pub struct D3D12_ROOT_SIGNATURE_DESC1(i32);
pub struct D3D12_ROOT_SIGNATURE_FLAGS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_RS_SET_SHADING_RATE_COMBINER_COUNT: u32 = 2u32;
pub struct D3D12_RTV_DIMENSION(i32);
pub struct D3D12_RT_FORMAT_ARRAY(i32);
pub struct D3D12_SAMPLER_DESC(i32);
pub struct D3D12_SAMPLER_FEEDBACK_TIER(i32);
pub struct D3D12_SAMPLE_POSITION(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SDK_VERSION: u32 = 5u32;
pub struct D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER(i32);
pub struct D3D12_SERIALIZED_DATA_TYPE(i32);
pub struct D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER(i32);
pub struct D3D12_SHADER_BUFFER_DESC(i32);
pub struct D3D12_SHADER_BYTECODE(i32);
pub struct D3D12_SHADER_CACHE_CONTROL_FLAGS(i32);
pub struct D3D12_SHADER_CACHE_FLAGS(i32);
pub struct D3D12_SHADER_CACHE_KIND_FLAGS(i32);
pub struct D3D12_SHADER_CACHE_MODE(i32);
pub struct D3D12_SHADER_CACHE_SESSION_DESC(i32);
pub struct D3D12_SHADER_CACHE_SUPPORT_FLAGS(i32);
pub struct D3D12_SHADER_COMPONENT_MAPPING(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_ALWAYS_SET_BIT_AVOIDING_ZEROMEM_MISTAKES: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_MASK: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_COMPONENT_MAPPING_SHIFT: u32 = 3u32;
pub struct D3D12_SHADER_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES: u32 = 32u32;
pub struct D3D12_SHADER_INPUT_BIND_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_MAJOR_VERSION: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_MAX_INSTANCES: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_MAX_INTERFACES: u32 = 253u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_MAX_INTERFACE_CALL_SITES: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_MAX_TYPES: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADER_MINOR_VERSION: u32 = 1u32;
pub struct D3D12_SHADER_MIN_PRECISION_SUPPORT(i32);
pub struct D3D12_SHADER_RESOURCE_VIEW_DESC(i32);
pub struct D3D12_SHADER_TYPE_DESC(i32);
pub struct D3D12_SHADER_VARIABLE_DESC(i32);
pub struct D3D12_SHADER_VERSION_TYPE(i32);
pub struct D3D12_SHADER_VISIBILITY(i32);
pub struct D3D12_SHADING_RATE(i32);
pub struct D3D12_SHADING_RATE_COMBINER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADING_RATE_VALID_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHADING_RATE_X_AXIS_SHIFT: u32 = 2u32;
pub struct D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
pub struct D3D12_SIGNATURE_PARAMETER_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SMALL_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SMALL_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
pub struct D3D12_SO_DECLARATION_ENTRY(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SO_NO_RASTERIZED_STREAM: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SO_OUTPUT_COMPONENT_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SO_STREAM_COUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SPEC_DATE_DAY: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SPEC_DATE_MONTH: u32 = 11u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SPEC_DATE_YEAR: u32 = 2014u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SPEC_VERSION: f64 = 1.16f64;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SRGB_GAMMA: f32 = 2.2f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
pub struct D3D12_SRV_DIMENSION(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_STANDARD_VECTOR_SIZE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
pub struct D3D12_STATE_OBJECT_CONFIG(i32);
pub struct D3D12_STATE_OBJECT_DESC(i32);
pub struct D3D12_STATE_OBJECT_FLAGS(i32);
pub struct D3D12_STATE_OBJECT_TYPE(i32);
pub struct D3D12_STATE_SUBOBJECT(i32);
pub struct D3D12_STATE_SUBOBJECT_TYPE(i32);
pub struct D3D12_STATIC_BORDER_COLOR(i32);
pub struct D3D12_STATIC_SAMPLER_DESC(i32);
pub struct D3D12_STENCIL_OP(i32);
pub struct D3D12_STREAM_OUTPUT_BUFFER_VIEW(i32);
pub struct D3D12_STREAM_OUTPUT_DESC(i32);
pub struct D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub struct D3D12_SUBRESOURCE_DATA(i32);
pub struct D3D12_SUBRESOURCE_FOOTPRINT(i32);
pub struct D3D12_SUBRESOURCE_INFO(i32);
pub struct D3D12_SUBRESOURCE_RANGE_UINT64(i32);
pub struct D3D12_SUBRESOURCE_TILING(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967280u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1u32;
pub struct D3D12_TEX1D_ARRAY_DSV(i32);
pub struct D3D12_TEX1D_ARRAY_RTV(i32);
pub struct D3D12_TEX1D_ARRAY_SRV(i32);
pub struct D3D12_TEX1D_ARRAY_UAV(i32);
pub struct D3D12_TEX1D_DSV(i32);
pub struct D3D12_TEX1D_RTV(i32);
pub struct D3D12_TEX1D_SRV(i32);
pub struct D3D12_TEX1D_UAV(i32);
pub struct D3D12_TEX2DMS_ARRAY_DSV(i32);
pub struct D3D12_TEX2DMS_ARRAY_RTV(i32);
pub struct D3D12_TEX2DMS_ARRAY_SRV(i32);
pub struct D3D12_TEX2DMS_DSV(i32);
pub struct D3D12_TEX2DMS_RTV(i32);
pub struct D3D12_TEX2DMS_SRV(i32);
pub struct D3D12_TEX2D_ARRAY_DSV(i32);
pub struct D3D12_TEX2D_ARRAY_RTV(i32);
pub struct D3D12_TEX2D_ARRAY_SRV(i32);
pub struct D3D12_TEX2D_ARRAY_UAV(i32);
pub struct D3D12_TEX2D_DSV(i32);
pub struct D3D12_TEX2D_RTV(i32);
pub struct D3D12_TEX2D_SRV(i32);
pub struct D3D12_TEX2D_UAV(i32);
pub struct D3D12_TEX3D_RTV(i32);
pub struct D3D12_TEX3D_SRV(i32);
pub struct D3D12_TEX3D_UAV(i32);
pub struct D3D12_TEXCUBE_ARRAY_SRV(i32);
pub struct D3D12_TEXCUBE_SRV(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub struct D3D12_TEXTURE_ADDRESS_MODE(i32);
pub struct D3D12_TEXTURE_COPY_LOCATION(i32);
pub struct D3D12_TEXTURE_COPY_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TEXTURE_DATA_PITCH_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT: u32 = 512u32;
pub struct D3D12_TEXTURE_LAYOUT(i32);
pub struct D3D12_TILED_RESOURCES_TIER(i32);
pub struct D3D12_TILED_RESOURCE_COORDINATE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TILED_RESOURCE_TILE_SIZE_IN_BYTES: u32 = 65536u32;
pub struct D3D12_TILE_COPY_FLAGS(i32);
pub struct D3D12_TILE_MAPPING_FLAGS(i32);
pub struct D3D12_TILE_RANGE_FLAGS(i32);
pub struct D3D12_TILE_REGION_SIZE(i32);
pub struct D3D12_TILE_SHAPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_TRACKED_WORKLOAD_MAX_INSTANCES: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_UAV_COUNTER_PLACEMENT_ALIGNMENT: u32 = 4096u32;
pub struct D3D12_UAV_DIMENSION(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_UAV_SLOT_COUNT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
pub struct D3D12_UNORDERED_ACCESS_VIEW_DESC(i32);
pub struct D3D12_VARIABLE_SHADING_RATE_TIER(i32);
pub struct D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA(i32);
pub struct D3D12_VERSIONED_ROOT_SIGNATURE_DESC(i32);
pub struct D3D12_VERTEX_BUFFER_VIEW(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIDEO_DECODE_MAX_ARGUMENTS: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIDEO_DECODE_MAX_HISTOGRAM_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIDEO_DECODE_MIN_BITSTREAM_OFFSET_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIDEO_DECODE_MIN_HISTOGRAM_OFFSET_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIDEO_DECODE_STATUS_MACROBLOCKS_AFFECTED_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIDEO_PROCESS_MAX_FILTERS: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIDEO_PROCESS_STEREO_VIEWS: u32 = 2u32;
pub struct D3D12_VIEWPORT(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIEWPORT_BOUNDS_MAX: u32 = 32767u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VIEWPORT_BOUNDS_MIN: i32 = -32768i32;
pub struct D3D12_VIEW_INSTANCE_LOCATION(i32);
pub struct D3D12_VIEW_INSTANCING_DESC(i32);
pub struct D3D12_VIEW_INSTANCING_FLAGS(i32);
pub struct D3D12_VIEW_INSTANCING_TIER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub struct D3D12_WAVE_MMA_TIER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D12_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
pub struct D3D12_WRITEBUFFERIMMEDIATE_MODE(i32);
pub struct D3D12_WRITEBUFFERIMMEDIATE_PARAMETER(i32);
pub struct D3D_ROOT_SIGNATURE_VERSION(i32);
pub struct D3D_SHADER_MODEL(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_DESCRIPTOR_HEAP_RESOURCE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_GROUP_SHARED: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_TYPED_RESOURCE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_BARYCENTRICS: u32 = 131072u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_DERIVATIVES_IN_MESH_AND_AMPLIFICATION_SHADERS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_INNER_COVERAGE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_INT64_OPS: u32 = 32768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_NATIVE_16BIT_OPS: u32 = 262144u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_RAYTRACING_TIER_1_1: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_RESOURCE_DESCRIPTOR_HEAP_INDEXING: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_ROVS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_SAMPLER_DESCRIPTOR_HEAP_INDEXING: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_SAMPLER_FEEDBACK: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_SHADING_RATE: u32 = 524288u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_STENCIL_REF: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_TYPED_UAV_LOAD_ADDITIONAL_FORMATS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_VIEWPORT_AND_RT_ARRAY_INDEX_FROM_ANY_SHADER_FEEDING_RASTERIZER: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_VIEW_ID: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_WAVE_MMA: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const D3D_SHADER_REQUIRES_WAVE_OPS: u32 = 16384u32;
pub const DXGI_DEBUG_D3D12: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3478759820,
    data2: 43344,
    data3: 17190,
    data4: [145, 239, 155, 186, 161, 123, 253, 149],
};
pub struct ID3D12CommandAllocator(i32);
pub struct ID3D12CommandList(i32);
pub struct ID3D12CommandQueue(i32);
pub struct ID3D12CommandSignature(i32);
pub struct ID3D12Debug(i32);
pub struct ID3D12Debug1(i32);
pub struct ID3D12Debug2(i32);
pub struct ID3D12Debug3(i32);
pub struct ID3D12Debug4(i32);
pub struct ID3D12Debug5(i32);
pub struct ID3D12DebugCommandList(i32);
pub struct ID3D12DebugCommandList1(i32);
pub struct ID3D12DebugCommandList2(i32);
pub struct ID3D12DebugCommandQueue(i32);
pub struct ID3D12DebugDevice(i32);
pub struct ID3D12DebugDevice1(i32);
pub struct ID3D12DebugDevice2(i32);
pub struct ID3D12DescriptorHeap(i32);
pub struct ID3D12Device(i32);
pub struct ID3D12Device1(i32);
pub struct ID3D12Device2(i32);
pub struct ID3D12Device3(i32);
pub struct ID3D12Device4(i32);
pub struct ID3D12Device5(i32);
pub struct ID3D12Device6(i32);
pub struct ID3D12Device7(i32);
pub struct ID3D12Device8(i32);
pub struct ID3D12Device9(i32);
pub struct ID3D12DeviceChild(i32);
pub struct ID3D12DeviceRemovedExtendedData(i32);
pub struct ID3D12DeviceRemovedExtendedData1(i32);
pub struct ID3D12DeviceRemovedExtendedData2(i32);
pub struct ID3D12DeviceRemovedExtendedDataSettings(i32);
pub struct ID3D12DeviceRemovedExtendedDataSettings1(i32);
pub struct ID3D12Fence(i32);
pub struct ID3D12Fence1(i32);
pub struct ID3D12FunctionParameterReflection(i32);
pub struct ID3D12FunctionReflection(i32);
pub struct ID3D12GraphicsCommandList(i32);
pub struct ID3D12GraphicsCommandList1(i32);
pub struct ID3D12GraphicsCommandList2(i32);
pub struct ID3D12GraphicsCommandList3(i32);
pub struct ID3D12GraphicsCommandList4(i32);
pub struct ID3D12GraphicsCommandList5(i32);
pub struct ID3D12GraphicsCommandList6(i32);
pub struct ID3D12Heap(i32);
pub struct ID3D12Heap1(i32);
pub struct ID3D12InfoQueue(i32);
pub struct ID3D12InfoQueue1(i32);
pub struct ID3D12LibraryReflection(i32);
pub struct ID3D12LifetimeOwner(i32);
pub struct ID3D12LifetimeTracker(i32);
pub struct ID3D12MetaCommand(i32);
pub struct ID3D12Object(i32);
pub struct ID3D12Pageable(i32);
pub struct ID3D12PipelineLibrary(i32);
pub struct ID3D12PipelineLibrary1(i32);
pub struct ID3D12PipelineState(i32);
pub struct ID3D12ProtectedResourceSession(i32);
pub struct ID3D12ProtectedResourceSession1(i32);
pub struct ID3D12ProtectedSession(i32);
pub struct ID3D12QueryHeap(i32);
pub struct ID3D12Resource(i32);
pub struct ID3D12Resource1(i32);
pub struct ID3D12Resource2(i32);
pub struct ID3D12RootSignature(i32);
pub struct ID3D12RootSignatureDeserializer(i32);
pub struct ID3D12SDKConfiguration(i32);
pub struct ID3D12ShaderCacheSession(i32);
pub struct ID3D12ShaderReflection(i32);
pub struct ID3D12ShaderReflectionConstantBuffer(i32);
pub struct ID3D12ShaderReflectionType(i32);
pub struct ID3D12ShaderReflectionVariable(i32);
pub struct ID3D12SharingContract(i32);
pub struct ID3D12StateObject(i32);
pub struct ID3D12StateObjectProperties(i32);
pub struct ID3D12SwapChainAssistant(i32);
pub struct ID3D12Tools(i32);
pub struct ID3D12VersionedRootSignatureDeserializer(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
pub const LUID_DEFINED: u32 = 1u32;
pub struct PFN_D3D12_CREATE_DEVICE(i32);
pub struct PFN_D3D12_CREATE_ROOT_SIGNATURE_DESERIALIZER(i32);
pub struct PFN_D3D12_CREATE_VERSIONED_ROOT_SIGNATURE_DESERIALIZER(i32);
pub struct PFN_D3D12_GET_DEBUG_INTERFACE(i32);
pub struct PFN_D3D12_GET_INTERFACE(i32);
pub struct PFN_D3D12_SERIALIZE_ROOT_SIGNATURE(i32);
pub struct PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE(i32);
pub const WKPDID_D3DAutoDebugObjectNameW: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3566218806,
    data2: 30074,
    data3: 18754,
    data4: [149, 148, 182, 118, 154, 250, 67, 205],
};
