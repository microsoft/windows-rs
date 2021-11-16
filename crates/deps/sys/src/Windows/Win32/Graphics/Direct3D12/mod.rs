#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12CreateDevice(padapter: ::windows_sys::core::IUnknown, minimumfeaturelevel: super::Direct3D::D3D_FEATURE_LEVEL, riid: *const ::windows_sys::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn D3D12CreateRootSignatureDeserializer(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows_sys::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn D3D12CreateVersionedRootSignatureDeserializer(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows_sys::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn D3D12EnableExperimentalFeatures(numfeatures: u32, piids: *const ::windows_sys::core::GUID, pconfigurationstructs: *const ::core::ffi::c_void, pconfigurationstructsizes: *const u32) -> ::windows_sys::core::HRESULT;
    pub fn D3D12GetDebugInterface(riid: *const ::windows_sys::core::GUID, ppvdebug: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn D3D12GetInterface(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppvdebug: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12SerializeRootSignature(prootsignature: *const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, ppblob: *mut super::Direct3D::ID3DBlob, pperrorblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12SerializeVersionedRootSignature(prootsignature: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob: *mut super::Direct3D::ID3DBlob, pperrorblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_D3D12Debug: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4063570667,
    data2: 56708,
    data3: 18942,
    data4: [185, 123, 169, 220, 253, 204, 27, 79],
};
pub const CLSID_D3D12DeviceRemovedExtendedData: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1249229764,
    data2: 40948,
    data3: 19160,
    data4: [159, 24, 171, 174, 132, 220, 95, 242],
};
pub const CLSID_D3D12SDKConfiguration: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2094688970, data2: 41022, data3: 18888, data4: [148, 88, 3, 52, 210, 14, 7, 206] };
pub const CLSID_D3D12Tools: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3816953521, data2: 15500, data3: 18483, data4: [170, 9, 10, 6, 182, 93, 150, 200] };
pub const D3D12ExperimentalShaderModels: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1995790142,
    data2: 61754,
    data3: 16629,
    data4: [178, 151, 129, 206, 158, 24, 147, 63],
};
#[cfg(feature = "Win32_Foundation")]
pub type D3D12MessageFunc = unsafe extern "system" fn(category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: super::super::Foundation::PSTR, pcontext: *mut ::core::ffi::c_void);
pub const D3D12MetaCommand: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3342125438,
    data2: 32887,
    data3: 18632,
    data4: [159, 220, 217, 209, 221, 49, 221, 119],
};
pub const D3D12TiledResourceTier4: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3385094751,
    data2: 43034,
    data3: 20310,
    data4: [140, 91, 197, 16, 57, 214, 148, 251],
};
pub const D3D12_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
pub const D3D12_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
pub const D3D12_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
pub const D3D12_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
pub const D3D12_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
pub const D3D12_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_AUTO_BREADCRUMB_NODE {
    pub pCommandListDebugNameA: *mut u8,
    pub pCommandListDebugNameW: super::super::Foundation::PWSTR,
    pub pCommandQueueDebugNameA: *mut u8,
    pub pCommandQueueDebugNameW: super::super::Foundation::PWSTR,
    pub pCommandList: ID3D12GraphicsCommandList,
    pub pCommandQueue: ID3D12CommandQueue,
    pub BreadcrumbCount: u32,
    pub pLastBreadcrumbValue: *mut u32,
    pub pCommandHistory: *mut D3D12_AUTO_BREADCRUMB_OP,
    pub pNext: *mut D3D12_AUTO_BREADCRUMB_NODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_AUTO_BREADCRUMB_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_AUTO_BREADCRUMB_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_AUTO_BREADCRUMB_NODE1 {
    pub pCommandListDebugNameA: *mut u8,
    pub pCommandListDebugNameW: super::super::Foundation::PWSTR,
    pub pCommandQueueDebugNameA: *mut u8,
    pub pCommandQueueDebugNameW: super::super::Foundation::PWSTR,
    pub pCommandList: ID3D12GraphicsCommandList,
    pub pCommandQueue: ID3D12CommandQueue,
    pub BreadcrumbCount: u32,
    pub pLastBreadcrumbValue: *mut u32,
    pub pCommandHistory: *mut D3D12_AUTO_BREADCRUMB_OP,
    pub pNext: *mut D3D12_AUTO_BREADCRUMB_NODE1,
    pub BreadcrumbContextsCount: u32,
    pub pBreadcrumbContexts: *mut D3D12_DRED_BREADCRUMB_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_AUTO_BREADCRUMB_NODE1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_AUTO_BREADCRUMB_NODE1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_AUTO_BREADCRUMB_OP_SETMARKER: i32 = 0i32;
pub const D3D12_AUTO_BREADCRUMB_OP_BEGINEVENT: i32 = 1i32;
pub const D3D12_AUTO_BREADCRUMB_OP_ENDEVENT: i32 = 2i32;
pub const D3D12_AUTO_BREADCRUMB_OP_DRAWINSTANCED: i32 = 3i32;
pub const D3D12_AUTO_BREADCRUMB_OP_DRAWINDEXEDINSTANCED: i32 = 4i32;
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEINDIRECT: i32 = 5i32;
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCH: i32 = 6i32;
pub const D3D12_AUTO_BREADCRUMB_OP_COPYBUFFERREGION: i32 = 7i32;
pub const D3D12_AUTO_BREADCRUMB_OP_COPYTEXTUREREGION: i32 = 8i32;
pub const D3D12_AUTO_BREADCRUMB_OP_COPYRESOURCE: i32 = 9i32;
pub const D3D12_AUTO_BREADCRUMB_OP_COPYTILES: i32 = 10i32;
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCE: i32 = 11i32;
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARRENDERTARGETVIEW: i32 = 12i32;
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARUNORDEREDACCESSVIEW: i32 = 13i32;
pub const D3D12_AUTO_BREADCRUMB_OP_CLEARDEPTHSTENCILVIEW: i32 = 14i32;
pub const D3D12_AUTO_BREADCRUMB_OP_RESOURCEBARRIER: i32 = 15i32;
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEBUNDLE: i32 = 16i32;
pub const D3D12_AUTO_BREADCRUMB_OP_PRESENT: i32 = 17i32;
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEQUERYDATA: i32 = 18i32;
pub const D3D12_AUTO_BREADCRUMB_OP_BEGINSUBMISSION: i32 = 19i32;
pub const D3D12_AUTO_BREADCRUMB_OP_ENDSUBMISSION: i32 = 20i32;
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME: i32 = 21i32;
pub const D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES: i32 = 22i32;
pub const D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT: i32 = 23i32;
pub const D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT64: i32 = 24i32;
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCEREGION: i32 = 25i32;
pub const D3D12_AUTO_BREADCRUMB_OP_WRITEBUFFERIMMEDIATE: i32 = 26i32;
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME1: i32 = 27i32;
pub const D3D12_AUTO_BREADCRUMB_OP_SETPROTECTEDRESOURCESESSION: i32 = 28i32;
pub const D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME2: i32 = 29i32;
pub const D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES1: i32 = 30i32;
pub const D3D12_AUTO_BREADCRUMB_OP_BUILDRAYTRACINGACCELERATIONSTRUCTURE: i32 = 31i32;
pub const D3D12_AUTO_BREADCRUMB_OP_EMITRAYTRACINGACCELERATIONSTRUCTUREPOSTBUILDINFO: i32 = 32i32;
pub const D3D12_AUTO_BREADCRUMB_OP_COPYRAYTRACINGACCELERATIONSTRUCTURE: i32 = 33i32;
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCHRAYS: i32 = 34i32;
pub const D3D12_AUTO_BREADCRUMB_OP_INITIALIZEMETACOMMAND: i32 = 35i32;
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEMETACOMMAND: i32 = 36i32;
pub const D3D12_AUTO_BREADCRUMB_OP_ESTIMATEMOTION: i32 = 37i32;
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEMOTIONVECTORHEAP: i32 = 38i32;
pub const D3D12_AUTO_BREADCRUMB_OP_SETPIPELINESTATE1: i32 = 39i32;
pub const D3D12_AUTO_BREADCRUMB_OP_INITIALIZEEXTENSIONCOMMAND: i32 = 40i32;
pub const D3D12_AUTO_BREADCRUMB_OP_EXECUTEEXTENSIONCOMMAND: i32 = 41i32;
pub const D3D12_AUTO_BREADCRUMB_OP_DISPATCHMESH: i32 = 42i32;
pub const D3D12_AUTO_BREADCRUMB_OP_ENCODEFRAME: i32 = 43i32;
pub const D3D12_AUTO_BREADCRUMB_OP_RESOLVEENCODEROUTPUTMETADATA: i32 = 44i32;
pub const D3D12_AXIS_SHADING_RATE_1X: i32 = 0i32;
pub const D3D12_AXIS_SHADING_RATE_2X: i32 = 1i32;
pub const D3D12_AXIS_SHADING_RATE_4X: i32 = 2i32;
pub const D3D12_BACKGROUND_PROCESSING_MODE_ALLOWED: i32 = 0i32;
pub const D3D12_BACKGROUND_PROCESSING_MODE_ALLOW_INTRUSIVE_MEASUREMENTS: i32 = 1i32;
pub const D3D12_BACKGROUND_PROCESSING_MODE_DISABLE_BACKGROUND_WORK: i32 = 2i32;
pub const D3D12_BACKGROUND_PROCESSING_MODE_DISABLE_PROFILING_BY_SYSTEM: i32 = 3i32;
pub const D3D12_BLEND_ZERO: i32 = 1i32;
pub const D3D12_BLEND_ONE: i32 = 2i32;
pub const D3D12_BLEND_SRC_COLOR: i32 = 3i32;
pub const D3D12_BLEND_INV_SRC_COLOR: i32 = 4i32;
pub const D3D12_BLEND_SRC_ALPHA: i32 = 5i32;
pub const D3D12_BLEND_INV_SRC_ALPHA: i32 = 6i32;
pub const D3D12_BLEND_DEST_ALPHA: i32 = 7i32;
pub const D3D12_BLEND_INV_DEST_ALPHA: i32 = 8i32;
pub const D3D12_BLEND_DEST_COLOR: i32 = 9i32;
pub const D3D12_BLEND_INV_DEST_COLOR: i32 = 10i32;
pub const D3D12_BLEND_SRC_ALPHA_SAT: i32 = 11i32;
pub const D3D12_BLEND_BLEND_FACTOR: i32 = 14i32;
pub const D3D12_BLEND_INV_BLEND_FACTOR: i32 = 15i32;
pub const D3D12_BLEND_SRC1_COLOR: i32 = 16i32;
pub const D3D12_BLEND_INV_SRC1_COLOR: i32 = 17i32;
pub const D3D12_BLEND_SRC1_ALPHA: i32 = 18i32;
pub const D3D12_BLEND_INV_SRC1_ALPHA: i32 = 19i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_BLEND_DESC {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D12_RENDER_TARGET_BLEND_DESC; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_BLEND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_BLEND_OP_ADD: i32 = 1i32;
pub const D3D12_BLEND_OP_SUBTRACT: i32 = 2i32;
pub const D3D12_BLEND_OP_REV_SUBTRACT: i32 = 3i32;
pub const D3D12_BLEND_OP_MIN: i32 = 4i32;
pub const D3D12_BLEND_OP_MAX: i32 = 5i32;
#[repr(C)]
pub struct D3D12_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl ::core::marker::Copy for D3D12_BOX {}
impl ::core::clone::Clone for D3D12_BOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_BUFFER_RTV {
    pub FirstElement: u64,
    pub NumElements: u32,
}
impl ::core::marker::Copy for D3D12_BUFFER_RTV {}
impl ::core::clone::Clone for D3D12_BUFFER_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_BUFFER_SRV {
    pub FirstElement: u64,
    pub NumElements: u32,
    pub StructureByteStride: u32,
    pub Flags: D3D12_BUFFER_SRV_FLAGS,
}
impl ::core::marker::Copy for D3D12_BUFFER_SRV {}
impl ::core::clone::Clone for D3D12_BUFFER_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_BUFFER_SRV_FLAG_NONE: u32 = 0u32;
pub const D3D12_BUFFER_SRV_FLAG_RAW: u32 = 1u32;
#[repr(C)]
pub struct D3D12_BUFFER_UAV {
    pub FirstElement: u64,
    pub NumElements: u32,
    pub StructureByteStride: u32,
    pub CounterOffsetInBytes: u64,
    pub Flags: D3D12_BUFFER_UAV_FLAGS,
}
impl ::core::marker::Copy for D3D12_BUFFER_UAV {}
impl ::core::clone::Clone for D3D12_BUFFER_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_BUFFER_UAV_FLAG_NONE: u32 = 0u32;
pub const D3D12_BUFFER_UAV_FLAG_RAW: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    pub DestAccelerationStructureData: u64,
    pub Inputs: D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS,
    pub SourceAccelerationStructureData: u64,
    pub ScratchAccelerationStructureData: u64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    pub Type: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE,
    pub Flags: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS,
    pub NumDescs: u32,
    pub DescsLayout: D3D12_ELEMENTS_LAYOUT,
    pub Anonymous: D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
    pub InstanceDescs: u64,
    pub pGeometryDescs: *mut D3D12_RAYTRACING_GEOMETRY_DESC,
    pub ppGeometryDescs: *mut *mut D3D12_RAYTRACING_GEOMETRY_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    pub Type: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE,
    pub NumDescs: u32,
}
impl ::core::marker::Copy for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {}
impl ::core::clone::Clone for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_CACHED_PIPELINE_STATE {
    pub pCachedBlob: *mut ::core::ffi::c_void,
    pub CachedBlobSizeInBytes: usize,
}
impl ::core::marker::Copy for D3D12_CACHED_PIPELINE_STATE {}
impl ::core::clone::Clone for D3D12_CACHED_PIPELINE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_CLEAR_FLAG_DEPTH: u32 = 1u32;
pub const D3D12_CLEAR_FLAG_STENCIL: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_CLEAR_VALUE {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Anonymous: D3D12_CLEAR_VALUE_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_CLEAR_VALUE {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_CLEAR_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_CLEAR_VALUE_0 {
    pub Color: [f32; 4],
    pub DepthStencil: D3D12_DEPTH_STENCIL_VALUE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_CLEAR_VALUE_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_CLEAR_VALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D12_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
pub const D3D12_COLOR_WRITE_ENABLE_RED: i32 = 1i32;
pub const D3D12_COLOR_WRITE_ENABLE_GREEN: i32 = 2i32;
pub const D3D12_COLOR_WRITE_ENABLE_BLUE: i32 = 4i32;
pub const D3D12_COLOR_WRITE_ENABLE_ALPHA: i32 = 8i32;
pub const D3D12_COLOR_WRITE_ENABLE_ALL: i32 = 15i32;
pub const D3D12_COMMAND_LIST_FLAG_NONE: u32 = 0u32;
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_NONE: u32 = 0u32;
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_DIRECT: u32 = 1u32;
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_BUNDLE: u32 = 2u32;
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_COMPUTE: u32 = 4u32;
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_COPY: u32 = 8u32;
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_DECODE: u32 = 16u32;
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_PROCESS: u32 = 32u32;
pub const D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_ENCODE: u32 = 64u32;
pub const D3D12_COMMAND_LIST_TYPE_DIRECT: i32 = 0i32;
pub const D3D12_COMMAND_LIST_TYPE_BUNDLE: i32 = 1i32;
pub const D3D12_COMMAND_LIST_TYPE_COMPUTE: i32 = 2i32;
pub const D3D12_COMMAND_LIST_TYPE_COPY: i32 = 3i32;
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE: i32 = 4i32;
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS: i32 = 5i32;
pub const D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE: i32 = 6i32;
pub const D3D12_COMMAND_POOL_FLAG_NONE: u32 = 0u32;
#[repr(C)]
pub struct D3D12_COMMAND_QUEUE_DESC {
    pub Type: D3D12_COMMAND_LIST_TYPE,
    pub Priority: i32,
    pub Flags: D3D12_COMMAND_QUEUE_FLAGS,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_COMMAND_QUEUE_DESC {}
impl ::core::clone::Clone for D3D12_COMMAND_QUEUE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_COMMAND_QUEUE_FLAG_NONE: u32 = 0u32;
pub const D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT: u32 = 1u32;
pub const D3D12_COMMAND_QUEUE_PRIORITY_NORMAL: i32 = 0i32;
pub const D3D12_COMMAND_QUEUE_PRIORITY_HIGH: i32 = 100i32;
pub const D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME: i32 = 10000i32;
pub const D3D12_COMMAND_RECORDER_FLAG_NONE: u32 = 0u32;
#[repr(C)]
pub struct D3D12_COMMAND_SIGNATURE_DESC {
    pub ByteStride: u32,
    pub NumArgumentDescs: u32,
    pub pArgumentDescs: *mut D3D12_INDIRECT_ARGUMENT_DESC,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_COMMAND_SIGNATURE_DESC {}
impl ::core::clone::Clone for D3D12_COMMAND_SIGNATURE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
pub const D3D12_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
pub const D3D12_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
pub const D3D12_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
pub const D3D12_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
pub const D3D12_COMPARISON_FUNC_NEVER: i32 = 1i32;
pub const D3D12_COMPARISON_FUNC_LESS: i32 = 2i32;
pub const D3D12_COMPARISON_FUNC_EQUAL: i32 = 3i32;
pub const D3D12_COMPARISON_FUNC_LESS_EQUAL: i32 = 4i32;
pub const D3D12_COMPARISON_FUNC_GREATER: i32 = 5i32;
pub const D3D12_COMPARISON_FUNC_NOT_EQUAL: i32 = 6i32;
pub const D3D12_COMPARISON_FUNC_GREATER_EQUAL: i32 = 7i32;
pub const D3D12_COMPARISON_FUNC_ALWAYS: i32 = 8i32;
#[repr(C)]
pub struct D3D12_COMPUTE_PIPELINE_STATE_DESC {
    pub pRootSignature: ID3D12RootSignature,
    pub CS: D3D12_SHADER_BYTECODE,
    pub NodeMask: u32,
    pub CachedPSO: D3D12_CACHED_PIPELINE_STATE,
    pub Flags: D3D12_PIPELINE_STATE_FLAGS,
}
impl ::core::marker::Copy for D3D12_COMPUTE_PIPELINE_STATE_DESC {}
impl ::core::clone::Clone for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF: i32 = 0i32;
pub const D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON: i32 = 1i32;
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_1: i32 = 1i32;
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_2: i32 = 2i32;
pub const D3D12_CONSERVATIVE_RASTERIZATION_TIER_3: i32 = 3i32;
pub const D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT: u32 = 256u32;
#[repr(C)]
pub struct D3D12_CONSTANT_BUFFER_VIEW_DESC {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
}
impl ::core::marker::Copy for D3D12_CONSTANT_BUFFER_VIEW_DESC {}
impl ::core::clone::Clone for D3D12_CONSTANT_BUFFER_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_CPU_DESCRIPTOR_HANDLE {
    pub ptr: usize,
}
impl ::core::marker::Copy for D3D12_CPU_DESCRIPTOR_HANDLE {}
impl ::core::clone::Clone for D3D12_CPU_DESCRIPTOR_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_CPU_PAGE_PROPERTY_UNKNOWN: i32 = 0i32;
pub const D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE: i32 = 1i32;
pub const D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE: i32 = 2i32;
pub const D3D12_CPU_PAGE_PROPERTY_WRITE_BACK: i32 = 3i32;
pub const D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_CROSS_NODE_SHARING_TIER_1_EMULATED: i32 = 1i32;
pub const D3D12_CROSS_NODE_SHARING_TIER_1: i32 = 2i32;
pub const D3D12_CROSS_NODE_SHARING_TIER_2: i32 = 3i32;
pub const D3D12_CROSS_NODE_SHARING_TIER_3: i32 = 4i32;
pub const D3D12_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 256u32;
pub const D3D12_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: u32 = 64u32;
pub const D3D12_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 240u32;
pub const D3D12_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: u32 = 68u32;
pub const D3D12_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 224u32;
pub const D3D12_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: u32 = 72u32;
pub const D3D12_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 208u32;
pub const D3D12_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: u32 = 76u32;
pub const D3D12_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 192u32;
pub const D3D12_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: u32 = 84u32;
pub const D3D12_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 176u32;
pub const D3D12_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: u32 = 92u32;
pub const D3D12_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 160u32;
pub const D3D12_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: u32 = 100u32;
pub const D3D12_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 144u32;
pub const D3D12_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: u32 = 112u32;
pub const D3D12_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 128u32;
pub const D3D12_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: u32 = 128u32;
pub const D3D12_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 112u32;
pub const D3D12_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: u32 = 144u32;
pub const D3D12_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 96u32;
pub const D3D12_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: u32 = 168u32;
pub const D3D12_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 80u32;
pub const D3D12_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: u32 = 204u32;
pub const D3D12_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 64u32;
pub const D3D12_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: u32 = 256u32;
pub const D3D12_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 48u32;
pub const D3D12_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: u32 = 340u32;
pub const D3D12_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 32u32;
pub const D3D12_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: u32 = 512u32;
pub const D3D12_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 16u32;
pub const D3D12_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: u32 = 768u32;
pub const D3D12_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: u32 = 1u32;
pub const D3D12_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: u32 = 256u32;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 768u32;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_X: u32 = 768u32;
pub const D3D12_CS_4_X_THREAD_GROUP_MAX_Y: u32 = 768u32;
pub const D3D12_CS_4_X_UAV_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: u32 = 65535u32;
pub const D3D12_CS_TGSM_REGISTER_COUNT: u32 = 8192u32;
pub const D3D12_CS_TGSM_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_CS_THREADGROUPID_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D12_CS_THREADGROUPID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D12_CS_THREADIDINGROUP_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_THREADID_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D12_CS_THREADID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 1024u32;
pub const D3D12_CS_THREAD_GROUP_MAX_X: u32 = 1024u32;
pub const D3D12_CS_THREAD_GROUP_MAX_Y: u32 = 1024u32;
pub const D3D12_CS_THREAD_GROUP_MAX_Z: u32 = 64u32;
pub const D3D12_CS_THREAD_GROUP_MIN_X: u32 = 1u32;
pub const D3D12_CS_THREAD_GROUP_MIN_Y: u32 = 1u32;
pub const D3D12_CS_THREAD_GROUP_MIN_Z: u32 = 1u32;
pub const D3D12_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: u32 = 16384u32;
pub const D3D12_CULL_MODE_NONE: i32 = 1i32;
pub const D3D12_CULL_MODE_FRONT: i32 = 2i32;
pub const D3D12_CULL_MODE_BACK: i32 = 3i32;
#[repr(C)]
pub struct D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    pub ShaderPatchMode: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE,
}
impl ::core::marker::Copy for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {}
impl ::core::clone::Clone for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_DEBUG_COMMAND_LIST_PARAMETER_GPU_BASED_VALIDATION_SETTINGS: i32 = 0i32;
#[repr(C)]
pub struct D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    pub MaxMessagesPerCommandList: u32,
    pub DefaultShaderPatchMode: D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE,
    pub PipelineStateCreateFlags: D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS,
}
impl ::core::marker::Copy for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {}
impl ::core::clone::Clone for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    pub SlowdownFactor: f32,
}
impl ::core::marker::Copy for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {}
impl ::core::clone::Clone for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_DEBUG_DEVICE_PARAMETER_FEATURE_FLAGS: i32 = 0i32;
pub const D3D12_DEBUG_DEVICE_PARAMETER_GPU_BASED_VALIDATION_SETTINGS: i32 = 1i32;
pub const D3D12_DEBUG_DEVICE_PARAMETER_GPU_SLOWDOWN_PERFORMANCE_FACTOR: i32 = 2i32;
pub const D3D12_DEBUG_FEATURE_NONE: i32 = 0i32;
pub const D3D12_DEBUG_FEATURE_ALLOW_BEHAVIOR_CHANGING_DEBUG_AIDS: i32 = 1i32;
pub const D3D12_DEBUG_FEATURE_CONSERVATIVE_RESOURCE_STATE_TRACKING: i32 = 2i32;
pub const D3D12_DEBUG_FEATURE_DISABLE_VIRTUALIZED_BUNDLES_VALIDATION: i32 = 4i32;
pub const D3D12_DEBUG_FEATURE_EMULATE_WINDOWS7: i32 = 8i32;
pub const D3D12_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
pub const D3D12_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
pub const D3D12_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
pub const D3D12_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
pub const D3D12_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D12_DEFAULT_DEPTH_BIAS: i32 = 0i32;
pub const D3D12_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
pub const D3D12_DEFAULT_MAX_ANISOTROPY: u32 = 16u32;
pub const D3D12_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
pub const D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 4194304u32;
pub const D3D12_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
pub const D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 65536u32;
pub const D3D12_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D12_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
pub const D3D12_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
pub const D3D12_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
pub const D3D12_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
pub const D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: u32 = 5768u32;
pub const D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
pub const D3D12_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
pub const D3D12_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
pub const D3D12_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
pub const D3D12_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
pub const D3D12_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
pub const D3D12_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
pub const D3D12_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
pub const D3D12_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
pub const D3D12_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
#[repr(C)]
pub struct D3D12_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D12_STENCIL_OP,
    pub StencilDepthFailOp: D3D12_STENCIL_OP,
    pub StencilPassOp: D3D12_STENCIL_OP,
    pub StencilFunc: D3D12_COMPARISON_FUNC,
}
impl ::core::marker::Copy for D3D12_DEPTH_STENCILOP_DESC {}
impl ::core::clone::Clone for D3D12_DEPTH_STENCILOP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DEPTH_STENCIL_DESC {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D12_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D12_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D12_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D12_DEPTH_STENCILOP_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DEPTH_STENCIL_DESC1 {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D12_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D12_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D12_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D12_DEPTH_STENCILOP_DESC,
    pub DepthBoundsTestEnable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_DEPTH_STENCIL_VALUE {
    pub Depth: f32,
    pub Stencil: u8,
}
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_VALUE {}
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_DSV_DIMENSION,
    pub Flags: D3D12_DSV_FLAGS,
    pub Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D12_TEX1D_DSV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D12_TEX2D_DSV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D12_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_DEPTH_STENCIL_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_DEPTH_WRITE_MASK_ZERO: i32 = 0i32;
pub const D3D12_DEPTH_WRITE_MASK_ALL: i32 = 1i32;
#[repr(C)]
pub struct D3D12_DESCRIPTOR_HEAP_DESC {
    pub Type: D3D12_DESCRIPTOR_HEAP_TYPE,
    pub NumDescriptors: u32,
    pub Flags: D3D12_DESCRIPTOR_HEAP_FLAGS,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_DESCRIPTOR_HEAP_DESC {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_DESCRIPTOR_HEAP_FLAG_NONE: u32 = 0u32;
pub const D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE: u32 = 1u32;
pub const D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV: i32 = 0i32;
pub const D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER: i32 = 1i32;
pub const D3D12_DESCRIPTOR_HEAP_TYPE_RTV: i32 = 2i32;
pub const D3D12_DESCRIPTOR_HEAP_TYPE_DSV: i32 = 3i32;
pub const D3D12_DESCRIPTOR_HEAP_TYPE_NUM_TYPES: i32 = 4i32;
#[repr(C)]
pub struct D3D12_DESCRIPTOR_RANGE {
    pub RangeType: D3D12_DESCRIPTOR_RANGE_TYPE,
    pub NumDescriptors: u32,
    pub BaseShaderRegister: u32,
    pub RegisterSpace: u32,
    pub OffsetInDescriptorsFromTableStart: u32,
}
impl ::core::marker::Copy for D3D12_DESCRIPTOR_RANGE {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_DESCRIPTOR_RANGE1 {
    pub RangeType: D3D12_DESCRIPTOR_RANGE_TYPE,
    pub NumDescriptors: u32,
    pub BaseShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Flags: D3D12_DESCRIPTOR_RANGE_FLAGS,
    pub OffsetInDescriptorsFromTableStart: u32,
}
impl ::core::marker::Copy for D3D12_DESCRIPTOR_RANGE1 {}
impl ::core::clone::Clone for D3D12_DESCRIPTOR_RANGE1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_DESCRIPTOR_RANGE_FLAG_NONE: u32 = 0u32;
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_VOLATILE: u32 = 1u32;
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_VOLATILE: u32 = 2u32;
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE: u32 = 4u32;
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC: u32 = 8u32;
pub const D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_STATIC_KEEPING_BUFFER_BOUNDS_CHECKS: u32 = 65536u32;
pub const D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = 4294967295u32;
pub const D3D12_DESCRIPTOR_RANGE_TYPE_SRV: i32 = 0i32;
pub const D3D12_DESCRIPTOR_RANGE_TYPE_UAV: i32 = 1i32;
pub const D3D12_DESCRIPTOR_RANGE_TYPE_CBV: i32 = 2i32;
pub const D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    pub Flags: D3D12_DRED_FLAGS,
    pub pHeadAutoBreadcrumbNode: *mut D3D12_AUTO_BREADCRUMB_NODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DEVICE_REMOVED_EXTENDED_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    pub DeviceRemovedReason: ::windows_sys::core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    pub DeviceRemovedReason: ::windows_sys::core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    pub DeviceRemovedReason: ::windows_sys::core::HRESULT,
    pub AutoBreadcrumbsOutput: D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1,
    pub PageFaultOutput: D3D12_DRED_PAGE_FAULT_OUTPUT2,
    pub DeviceState: D3D12_DRED_DEVICE_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DISCARD_REGION {
    pub NumRects: u32,
    pub pRects: *mut super::super::Foundation::RECT,
    pub FirstSubresource: u32,
    pub NumSubresources: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DISCARD_REGION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DISCARD_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_DISPATCH_ARGUMENTS {
    pub ThreadGroupCountX: u32,
    pub ThreadGroupCountY: u32,
    pub ThreadGroupCountZ: u32,
}
impl ::core::marker::Copy for D3D12_DISPATCH_ARGUMENTS {}
impl ::core::clone::Clone for D3D12_DISPATCH_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_DISPATCH_MESH_ARGUMENTS {
    pub ThreadGroupCountX: u32,
    pub ThreadGroupCountY: u32,
    pub ThreadGroupCountZ: u32,
}
impl ::core::marker::Copy for D3D12_DISPATCH_MESH_ARGUMENTS {}
impl ::core::clone::Clone for D3D12_DISPATCH_MESH_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_DISPATCH_RAYS_DESC {
    pub RayGenerationShaderRecord: D3D12_GPU_VIRTUAL_ADDRESS_RANGE,
    pub MissShaderTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub HitGroupTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub CallableShaderTable: D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
impl ::core::marker::Copy for D3D12_DISPATCH_RAYS_DESC {}
impl ::core::clone::Clone for D3D12_DISPATCH_RAYS_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_DRAW_ARGUMENTS {
    pub VertexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartVertexLocation: u32,
    pub StartInstanceLocation: u32,
}
impl ::core::marker::Copy for D3D12_DRAW_ARGUMENTS {}
impl ::core::clone::Clone for D3D12_DRAW_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_DRAW_INDEXED_ARGUMENTS {
    pub IndexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartIndexLocation: u32,
    pub BaseVertexLocation: i32,
    pub StartInstanceLocation: u32,
}
impl ::core::marker::Copy for D3D12_DRAW_INDEXED_ARGUMENTS {}
impl ::core::clone::Clone for D3D12_DRAW_INDEXED_ARGUMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DRED_ALLOCATION_NODE {
    pub ObjectNameA: *mut u8,
    pub ObjectNameW: super::super::Foundation::PWSTR,
    pub AllocationType: D3D12_DRED_ALLOCATION_TYPE,
    pub pNext: *mut D3D12_DRED_ALLOCATION_NODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DRED_ALLOCATION_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DRED_ALLOCATION_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DRED_ALLOCATION_NODE1 {
    pub ObjectNameA: *mut u8,
    pub ObjectNameW: super::super::Foundation::PWSTR,
    pub AllocationType: D3D12_DRED_ALLOCATION_TYPE,
    pub pNext: *mut D3D12_DRED_ALLOCATION_NODE1,
    pub pObject: ::windows_sys::core::IUnknown,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DRED_ALLOCATION_NODE1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DRED_ALLOCATION_NODE1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_QUEUE: i32 = 19i32;
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_ALLOCATOR: i32 = 20i32;
pub const D3D12_DRED_ALLOCATION_TYPE_PIPELINE_STATE: i32 = 21i32;
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_LIST: i32 = 22i32;
pub const D3D12_DRED_ALLOCATION_TYPE_FENCE: i32 = 23i32;
pub const D3D12_DRED_ALLOCATION_TYPE_DESCRIPTOR_HEAP: i32 = 24i32;
pub const D3D12_DRED_ALLOCATION_TYPE_HEAP: i32 = 25i32;
pub const D3D12_DRED_ALLOCATION_TYPE_QUERY_HEAP: i32 = 27i32;
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_SIGNATURE: i32 = 28i32;
pub const D3D12_DRED_ALLOCATION_TYPE_PIPELINE_LIBRARY: i32 = 29i32;
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_DECODER: i32 = 30i32;
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_PROCESSOR: i32 = 32i32;
pub const D3D12_DRED_ALLOCATION_TYPE_RESOURCE: i32 = 34i32;
pub const D3D12_DRED_ALLOCATION_TYPE_PASS: i32 = 35i32;
pub const D3D12_DRED_ALLOCATION_TYPE_CRYPTOSESSION: i32 = 36i32;
pub const D3D12_DRED_ALLOCATION_TYPE_CRYPTOSESSIONPOLICY: i32 = 37i32;
pub const D3D12_DRED_ALLOCATION_TYPE_PROTECTEDRESOURCESESSION: i32 = 38i32;
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_DECODER_HEAP: i32 = 39i32;
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_POOL: i32 = 40i32;
pub const D3D12_DRED_ALLOCATION_TYPE_COMMAND_RECORDER: i32 = 41i32;
pub const D3D12_DRED_ALLOCATION_TYPE_STATE_OBJECT: i32 = 42i32;
pub const D3D12_DRED_ALLOCATION_TYPE_METACOMMAND: i32 = 43i32;
pub const D3D12_DRED_ALLOCATION_TYPE_SCHEDULINGGROUP: i32 = 44i32;
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_MOTION_ESTIMATOR: i32 = 45i32;
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_MOTION_VECTOR_HEAP: i32 = 46i32;
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_EXTENSION_COMMAND: i32 = 47i32;
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_ENCODER: i32 = 48i32;
pub const D3D12_DRED_ALLOCATION_TYPE_VIDEO_ENCODER_HEAP: i32 = 49i32;
pub const D3D12_DRED_ALLOCATION_TYPE_INVALID: i32 = -1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    pub pHeadAutoBreadcrumbNode: *mut D3D12_AUTO_BREADCRUMB_NODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    pub pHeadAutoBreadcrumbNode: *mut D3D12_AUTO_BREADCRUMB_NODE1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DRED_BREADCRUMB_CONTEXT {
    pub BreadcrumbIndex: u32,
    pub pContextString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DRED_BREADCRUMB_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DRED_BREADCRUMB_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_DRED_DEVICE_STATE_UNKNOWN: i32 = 0i32;
pub const D3D12_DRED_DEVICE_STATE_HUNG: i32 = 3i32;
pub const D3D12_DRED_DEVICE_STATE_FAULT: i32 = 6i32;
pub const D3D12_DRED_DEVICE_STATE_PAGEFAULT: i32 = 7i32;
pub const D3D12_DRED_ENABLEMENT_SYSTEM_CONTROLLED: i32 = 0i32;
pub const D3D12_DRED_ENABLEMENT_FORCED_OFF: i32 = 1i32;
pub const D3D12_DRED_ENABLEMENT_FORCED_ON: i32 = 2i32;
pub const D3D12_DRED_FLAG_NONE: u32 = 0u32;
pub const D3D12_DRED_FLAG_FORCE_ENABLE: u32 = 1u32;
pub const D3D12_DRED_FLAG_DISABLE_AUTOBREADCRUMBS: u32 = 2u32;
pub const D3D12_DRED_PAGE_FAULT_FLAGS_NONE: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *mut D3D12_DRED_ALLOCATION_NODE,
    pub pHeadRecentFreedAllocationNode: *mut D3D12_DRED_ALLOCATION_NODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DRED_PAGE_FAULT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DRED_PAGE_FAULT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *mut D3D12_DRED_ALLOCATION_NODE1,
    pub pHeadRecentFreedAllocationNode: *mut D3D12_DRED_ALLOCATION_NODE1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DRED_PAGE_FAULT_OUTPUT1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    pub PageFaultVA: u64,
    pub pHeadExistingAllocationNode: *mut D3D12_DRED_ALLOCATION_NODE1,
    pub pHeadRecentFreedAllocationNode: *mut D3D12_DRED_ALLOCATION_NODE1,
    pub PageFaultFlags: D3D12_DRED_PAGE_FAULT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DRED_PAGE_FAULT_OUTPUT2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_DRED_VERSION_1_0: i32 = 1i32;
pub const D3D12_DRED_VERSION_1_1: i32 = 2i32;
pub const D3D12_DRED_VERSION_1_2: i32 = 3i32;
pub const D3D12_DRED_VERSION_1_3: i32 = 4i32;
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_COMPATIBLE_WITH_DEVICE: i32 = 0i32;
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_UNSUPPORTED_TYPE: i32 = 1i32;
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_UNRECOGNIZED: i32 = 2i32;
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_VERSION: i32 = 3i32;
pub const D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_TYPE: i32 = 4i32;
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967287u32;
pub const D3D12_DRIVER_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967280u32;
pub const D3D12_DSV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D12_DSV_DIMENSION_TEXTURE1D: i32 = 1i32;
pub const D3D12_DSV_DIMENSION_TEXTURE1DARRAY: i32 = 2i32;
pub const D3D12_DSV_DIMENSION_TEXTURE2D: i32 = 3i32;
pub const D3D12_DSV_DIMENSION_TEXTURE2DARRAY: i32 = 4i32;
pub const D3D12_DSV_DIMENSION_TEXTURE2DMS: i32 = 5i32;
pub const D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY: i32 = 6i32;
pub const D3D12_DSV_FLAG_NONE: u32 = 0u32;
pub const D3D12_DSV_FLAG_READ_ONLY_DEPTH: u32 = 1u32;
pub const D3D12_DSV_FLAG_READ_ONLY_STENCIL: u32 = 2u32;
pub const D3D12_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_DS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DXIL_LIBRARY_DESC {
    pub DXILLibrary: D3D12_SHADER_BYTECODE,
    pub NumExports: u32,
    pub pExports: *mut D3D12_EXPORT_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DXIL_LIBRARY_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DXIL_LIBRARY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    pub SubobjectToAssociate: super::super::Foundation::PWSTR,
    pub NumExports: u32,
    pub pExports: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_ELEMENTS_LAYOUT_ARRAY: i32 = 0i32;
pub const D3D12_ELEMENTS_LAYOUT_ARRAY_OF_POINTERS: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_EXISTING_COLLECTION_DESC {
    pub pExistingCollection: ID3D12StateObject,
    pub NumExports: u32,
    pub pExports: *mut D3D12_EXPORT_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_EXISTING_COLLECTION_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_EXISTING_COLLECTION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_EXPORT_DESC {
    pub Name: super::super::Foundation::PWSTR,
    pub ExportToRename: super::super::Foundation::PWSTR,
    pub Flags: D3D12_EXPORT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_EXPORT_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_EXPORT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_EXPORT_FLAG_NONE: u32 = 0u32;
pub const D3D12_FEATURE_D3D12_OPTIONS: i32 = 0i32;
pub const D3D12_FEATURE_ARCHITECTURE: i32 = 1i32;
pub const D3D12_FEATURE_FEATURE_LEVELS: i32 = 2i32;
pub const D3D12_FEATURE_FORMAT_SUPPORT: i32 = 3i32;
pub const D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS: i32 = 4i32;
pub const D3D12_FEATURE_FORMAT_INFO: i32 = 5i32;
pub const D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT: i32 = 6i32;
pub const D3D12_FEATURE_SHADER_MODEL: i32 = 7i32;
pub const D3D12_FEATURE_D3D12_OPTIONS1: i32 = 8i32;
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_SUPPORT: i32 = 10i32;
pub const D3D12_FEATURE_ROOT_SIGNATURE: i32 = 12i32;
pub const D3D12_FEATURE_ARCHITECTURE1: i32 = 16i32;
pub const D3D12_FEATURE_D3D12_OPTIONS2: i32 = 18i32;
pub const D3D12_FEATURE_SHADER_CACHE: i32 = 19i32;
pub const D3D12_FEATURE_COMMAND_QUEUE_PRIORITY: i32 = 20i32;
pub const D3D12_FEATURE_D3D12_OPTIONS3: i32 = 21i32;
pub const D3D12_FEATURE_EXISTING_HEAPS: i32 = 22i32;
pub const D3D12_FEATURE_D3D12_OPTIONS4: i32 = 23i32;
pub const D3D12_FEATURE_SERIALIZATION: i32 = 24i32;
pub const D3D12_FEATURE_CROSS_NODE: i32 = 25i32;
pub const D3D12_FEATURE_D3D12_OPTIONS5: i32 = 27i32;
pub const D3D12_FEATURE_DISPLAYABLE: i32 = 28i32;
pub const D3D12_FEATURE_D3D12_OPTIONS6: i32 = 30i32;
pub const D3D12_FEATURE_QUERY_META_COMMAND: i32 = 31i32;
pub const D3D12_FEATURE_D3D12_OPTIONS7: i32 = 32i32;
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPE_COUNT: i32 = 33i32;
pub const D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPES: i32 = 34i32;
pub const D3D12_FEATURE_D3D12_OPTIONS8: i32 = 36i32;
pub const D3D12_FEATURE_D3D12_OPTIONS9: i32 = 37i32;
pub const D3D12_FEATURE_D3D12_OPTIONS10: i32 = 39i32;
pub const D3D12_FEATURE_D3D12_OPTIONS11: i32 = 40i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_ARCHITECTURE {
    pub NodeIndex: u32,
    pub TileBasedRenderer: super::super::Foundation::BOOL,
    pub UMA: super::super::Foundation::BOOL,
    pub CacheCoherentUMA: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_ARCHITECTURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_ARCHITECTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_ARCHITECTURE1 {
    pub NodeIndex: u32,
    pub TileBasedRenderer: super::super::Foundation::BOOL,
    pub UMA: super::super::Foundation::BOOL,
    pub CacheCoherentUMA: super::super::Foundation::BOOL,
    pub IsolatedMMU: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_ARCHITECTURE1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_ARCHITECTURE1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    pub CommandListType: D3D12_COMMAND_LIST_TYPE,
    pub Priority: u32,
    pub PriorityForTypeIsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_CROSS_NODE {
    pub SharingTier: D3D12_CROSS_NODE_SHARING_TIER,
    pub AtomicShaderInstructions: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_CROSS_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_CROSS_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS {
    pub DoublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
    pub OutputMergerLogicOp: super::super::Foundation::BOOL,
    pub MinPrecisionSupport: D3D12_SHADER_MIN_PRECISION_SUPPORT,
    pub TiledResourcesTier: D3D12_TILED_RESOURCES_TIER,
    pub ResourceBindingTier: D3D12_RESOURCE_BINDING_TIER,
    pub PSSpecifiedStencilRefSupported: super::super::Foundation::BOOL,
    pub TypedUAVLoadAdditionalFormats: super::super::Foundation::BOOL,
    pub ROVsSupported: super::super::Foundation::BOOL,
    pub ConservativeRasterizationTier: D3D12_CONSERVATIVE_RASTERIZATION_TIER,
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub StandardSwizzle64KBSupported: super::super::Foundation::BOOL,
    pub CrossNodeSharingTier: D3D12_CROSS_NODE_SHARING_TIER,
    pub CrossAdapterRowMajorTextureSupported: super::super::Foundation::BOOL,
    pub VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation: super::super::Foundation::BOOL,
    pub ResourceHeapTier: D3D12_RESOURCE_HEAP_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    pub WaveOps: super::super::Foundation::BOOL,
    pub WaveLaneCountMin: u32,
    pub WaveLaneCountMax: u32,
    pub TotalLaneCount: u32,
    pub ExpandedComputeResourceStates: super::super::Foundation::BOOL,
    pub Int64ShaderOps: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    pub VariableRateShadingSumCombinerSupported: super::super::Foundation::BOOL,
    pub MeshShaderPerPrimitiveShadingRateSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    pub AtomicInt64OnDescriptorHeapResourceSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    pub DepthBoundsTestSupported: super::super::Foundation::BOOL,
    pub ProgrammableSamplePositionsTier: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    pub CopyQueueTimestampQueriesSupported: super::super::Foundation::BOOL,
    pub CastingFullyTypedFormatSupported: super::super::Foundation::BOOL,
    pub WriteBufferImmediateSupportFlags: D3D12_COMMAND_LIST_SUPPORT_FLAGS,
    pub ViewInstancingTier: D3D12_VIEW_INSTANCING_TIER,
    pub BarycentricsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    pub MSAA64KBAlignedTextureSupported: super::super::Foundation::BOOL,
    pub SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
    pub Native16BitShaderOpsSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    pub SRVOnlyTiledResourceTier3: super::super::Foundation::BOOL,
    pub RenderPassesTier: D3D12_RENDER_PASS_TIER,
    pub RaytracingTier: D3D12_RAYTRACING_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    pub AdditionalShadingRatesSupported: super::super::Foundation::BOOL,
    pub PerPrimitiveShadingRateSupportedWithViewportIndexing: super::super::Foundation::BOOL,
    pub VariableShadingRateTier: D3D12_VARIABLE_SHADING_RATE_TIER,
    pub ShadingRateImageTileSize: u32,
    pub BackgroundProcessingSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    pub MeshShaderTier: D3D12_MESH_SHADER_TIER,
    pub SamplerFeedbackTier: D3D12_SAMPLER_FEEDBACK_TIER,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    pub UnalignedBlockTexturesSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    pub MeshShaderPipelineStatsSupported: super::super::Foundation::BOOL,
    pub MeshShaderSupportsFullRangeRenderTargetArrayIndex: super::super::Foundation::BOOL,
    pub AtomicInt64OnTypedResourceSupported: super::super::Foundation::BOOL,
    pub AtomicInt64OnGroupSharedSupported: super::super::Foundation::BOOL,
    pub DerivativesInMeshAndAmplificationShadersSupported: super::super::Foundation::BOOL,
    pub WaveMMATier: D3D12_WAVE_MMA_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_DISPLAYABLE {
    pub DisplayableTexture: super::super::Foundation::BOOL,
    pub SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_DISPLAYABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_DISPLAYABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_FEATURE_DATA_EXISTING_HEAPS {
    pub Supported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_EXISTING_HEAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_EXISTING_HEAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3D12_FEATURE_DATA_FEATURE_LEVELS {
    pub NumFeatureLevels: u32,
    pub pFeatureLevelsRequested: *mut super::Direct3D::D3D_FEATURE_LEVEL,
    pub MaxSupportedFeatureLevel: super::Direct3D::D3D_FEATURE_LEVEL,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_FEATURE_LEVELS {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_FEATURE_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_FORMAT_INFO {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub PlaneCount: u8,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_FORMAT_INFO {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_FORMAT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Support1: D3D12_FORMAT_SUPPORT1,
    pub Support2: D3D12_FORMAT_SUPPORT2,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_FORMAT_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub MaxGPUVirtualAddressBitsPerProcess: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleCount: u32,
    pub Flags: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS,
    pub NumQualityLevels: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    pub NodeIndex: u32,
    pub Support: D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    pub NodeIndex: u32,
    pub Count: u32,
    pub pTypes: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    pub NodeIndex: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    pub CommandId: ::windows_sys::core::GUID,
    pub NodeMask: u32,
    pub pQueryInputData: *mut ::core::ffi::c_void,
    pub QueryInputDataSizeInBytes: usize,
    pub pQueryOutputData: *mut ::core::ffi::c_void,
    pub QueryOutputDataSizeInBytes: usize,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_QUERY_META_COMMAND {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    pub HighestVersion: D3D_ROOT_SIGNATURE_VERSION,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_ROOT_SIGNATURE {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_SERIALIZATION {
    pub NodeIndex: u32,
    pub HeapSerializationTier: D3D12_HEAP_SERIALIZATION_TIER,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_SERIALIZATION {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_SERIALIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_SHADER_CACHE {
    pub SupportFlags: D3D12_SHADER_CACHE_SUPPORT_FLAGS,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_SHADER_CACHE {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_SHADER_CACHE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_FEATURE_DATA_SHADER_MODEL {
    pub HighestShaderModel: D3D_SHADER_MODEL,
}
impl ::core::marker::Copy for D3D12_FEATURE_DATA_SHADER_MODEL {}
impl ::core::clone::Clone for D3D12_FEATURE_DATA_SHADER_MODEL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_FENCE_FLAG_NONE: u32 = 0u32;
pub const D3D12_FENCE_FLAG_SHARED: u32 = 1u32;
pub const D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER: u32 = 2u32;
pub const D3D12_FENCE_FLAG_NON_MONITORED: u32 = 4u32;
pub const D3D12_FILL_MODE_WIREFRAME: i32 = 2i32;
pub const D3D12_FILL_MODE_SOLID: i32 = 3i32;
pub const D3D12_FILTER_MIN_MAG_MIP_POINT: i32 = 0i32;
pub const D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR: i32 = 1i32;
pub const D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 4i32;
pub const D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR: i32 = 5i32;
pub const D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT: i32 = 16i32;
pub const D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 17i32;
pub const D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT: i32 = 20i32;
pub const D3D12_FILTER_MIN_MAG_MIP_LINEAR: i32 = 21i32;
pub const D3D12_FILTER_ANISOTROPIC: i32 = 85i32;
pub const D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT: i32 = 128i32;
pub const D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: i32 = 129i32;
pub const D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 132i32;
pub const D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: i32 = 133i32;
pub const D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: i32 = 144i32;
pub const D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 145i32;
pub const D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: i32 = 148i32;
pub const D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: i32 = 149i32;
pub const D3D12_FILTER_COMPARISON_ANISOTROPIC: i32 = 213i32;
pub const D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT: i32 = 256i32;
pub const D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR: i32 = 257i32;
pub const D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 260i32;
pub const D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR: i32 = 261i32;
pub const D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT: i32 = 272i32;
pub const D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 273i32;
pub const D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT: i32 = 276i32;
pub const D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR: i32 = 277i32;
pub const D3D12_FILTER_MINIMUM_ANISOTROPIC: i32 = 341i32;
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT: i32 = 384i32;
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR: i32 = 385i32;
pub const D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 388i32;
pub const D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR: i32 = 389i32;
pub const D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT: i32 = 400i32;
pub const D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 401i32;
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT: i32 = 404i32;
pub const D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR: i32 = 405i32;
pub const D3D12_FILTER_MAXIMUM_ANISOTROPIC: i32 = 469i32;
pub const D3D12_FILTER_REDUCTION_TYPE_STANDARD: i32 = 0i32;
pub const D3D12_FILTER_REDUCTION_TYPE_COMPARISON: i32 = 1i32;
pub const D3D12_FILTER_REDUCTION_TYPE_MINIMUM: i32 = 2i32;
pub const D3D12_FILTER_REDUCTION_TYPE_MAXIMUM: i32 = 3i32;
pub const D3D12_FILTER_REDUCTION_TYPE_MASK: u32 = 3u32;
pub const D3D12_FILTER_REDUCTION_TYPE_SHIFT: u32 = 7u32;
pub const D3D12_FILTER_TYPE_POINT: i32 = 0i32;
pub const D3D12_FILTER_TYPE_LINEAR: i32 = 1i32;
pub const D3D12_FILTER_TYPE_MASK: u32 = 3u32;
pub const D3D12_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D12_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
pub const D3D12_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
pub const D3D12_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
pub const D3D12_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
pub const D3D12_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
pub const D3D12_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
pub const D3D12_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
pub const D3D12_FORMAT_SUPPORT1_NONE: u32 = 0u32;
pub const D3D12_FORMAT_SUPPORT1_BUFFER: u32 = 1u32;
pub const D3D12_FORMAT_SUPPORT1_IA_VERTEX_BUFFER: u32 = 2u32;
pub const D3D12_FORMAT_SUPPORT1_IA_INDEX_BUFFER: u32 = 4u32;
pub const D3D12_FORMAT_SUPPORT1_SO_BUFFER: u32 = 8u32;
pub const D3D12_FORMAT_SUPPORT1_TEXTURE1D: u32 = 16u32;
pub const D3D12_FORMAT_SUPPORT1_TEXTURE2D: u32 = 32u32;
pub const D3D12_FORMAT_SUPPORT1_TEXTURE3D: u32 = 64u32;
pub const D3D12_FORMAT_SUPPORT1_TEXTURECUBE: u32 = 128u32;
pub const D3D12_FORMAT_SUPPORT1_SHADER_LOAD: u32 = 256u32;
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE: u32 = 512u32;
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_COMPARISON: u32 = 1024u32;
pub const D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_MONO_TEXT: u32 = 2048u32;
pub const D3D12_FORMAT_SUPPORT1_MIP: u32 = 4096u32;
pub const D3D12_FORMAT_SUPPORT1_RENDER_TARGET: u32 = 16384u32;
pub const D3D12_FORMAT_SUPPORT1_BLENDABLE: u32 = 32768u32;
pub const D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL: u32 = 65536u32;
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RESOLVE: u32 = 262144u32;
pub const D3D12_FORMAT_SUPPORT1_DISPLAY: u32 = 524288u32;
pub const D3D12_FORMAT_SUPPORT1_CAST_WITHIN_BIT_LAYOUT: u32 = 1048576u32;
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RENDERTARGET: u32 = 2097152u32;
pub const D3D12_FORMAT_SUPPORT1_MULTISAMPLE_LOAD: u32 = 4194304u32;
pub const D3D12_FORMAT_SUPPORT1_SHADER_GATHER: u32 = 8388608u32;
pub const D3D12_FORMAT_SUPPORT1_BACK_BUFFER_CAST: u32 = 16777216u32;
pub const D3D12_FORMAT_SUPPORT1_TYPED_UNORDERED_ACCESS_VIEW: u32 = 33554432u32;
pub const D3D12_FORMAT_SUPPORT1_SHADER_GATHER_COMPARISON: u32 = 67108864u32;
pub const D3D12_FORMAT_SUPPORT1_DECODER_OUTPUT: u32 = 134217728u32;
pub const D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_OUTPUT: u32 = 268435456u32;
pub const D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_INPUT: u32 = 536870912u32;
pub const D3D12_FORMAT_SUPPORT1_VIDEO_ENCODER: u32 = 1073741824u32;
pub const D3D12_FORMAT_SUPPORT2_NONE: u32 = 0u32;
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_ADD: u32 = 1u32;
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS: u32 = 2u32;
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE: u32 = 4u32;
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE: u32 = 8u32;
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX: u32 = 16u32;
pub const D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX: u32 = 32u32;
pub const D3D12_FORMAT_SUPPORT2_UAV_TYPED_LOAD: u32 = 64u32;
pub const D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE: u32 = 128u32;
pub const D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP: u32 = 256u32;
pub const D3D12_FORMAT_SUPPORT2_TILED: u32 = 512u32;
pub const D3D12_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY: u32 = 16384u32;
pub const D3D12_FORMAT_SUPPORT2_SAMPLER_FEEDBACK: u32 = 32768u32;
pub const D3D12_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
pub const D3D12_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
pub const D3D12_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
pub const D3D12_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D12_FUNCTION_DESC {
    pub Version: u32,
    pub Creator: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub MovInstructionCount: u32,
    pub MovcInstructionCount: u32,
    pub ConversionInstructionCount: u32,
    pub BitwiseInstructionCount: u32,
    pub MinFeatureLevel: super::Direct3D::D3D_FEATURE_LEVEL,
    pub RequiredFeatureFlags: u64,
    pub Name: super::super::Foundation::PSTR,
    pub FunctionParameterCount: i32,
    pub HasReturn: super::super::Foundation::BOOL,
    pub Has10Level9VertexShader: super::super::Foundation::BOOL,
    pub Has10Level9PixelShader: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D12_FUNCTION_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D12_FUNCTION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_GLOBAL_ROOT_SIGNATURE {
    pub pGlobalRootSignature: ID3D12RootSignature,
}
impl ::core::marker::Copy for D3D12_GLOBAL_ROOT_SIGNATURE {}
impl ::core::clone::Clone for D3D12_GLOBAL_ROOT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_GPU_BASED_VALIDATION_FLAGS_NONE: i32 = 0i32;
pub const D3D12_GPU_BASED_VALIDATION_FLAGS_DISABLE_STATE_TRACKING: i32 = 1i32;
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_NONE: i32 = 0i32;
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_TRACKING_ONLY_SHADERS: i32 = 1i32;
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_UNGUARDED_VALIDATION_SHADERS: i32 = 2i32;
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_GUARDED_VALIDATION_SHADERS: i32 = 4i32;
pub const D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS_VALID_MASK: i32 = 7i32;
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_NONE: i32 = 0i32;
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_STATE_TRACKING_ONLY: i32 = 1i32;
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_UNGUARDED_VALIDATION: i32 = 2i32;
pub const D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_GUARDED_VALIDATION: i32 = 3i32;
pub const NUM_D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODES: i32 = 4i32;
#[repr(C)]
pub struct D3D12_GPU_DESCRIPTOR_HANDLE {
    pub ptr: u64,
}
impl ::core::marker::Copy for D3D12_GPU_DESCRIPTOR_HANDLE {}
impl ::core::clone::Clone for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    pub StartAddress: u64,
    pub StrideInBytes: u64,
}
impl ::core::marker::Copy for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {}
impl ::core::clone::Clone for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {}
impl ::core::clone::Clone for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
    pub StrideInBytes: u64,
}
impl ::core::marker::Copy for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {}
impl ::core::clone::Clone for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    pub pRootSignature: ID3D12RootSignature,
    pub VS: D3D12_SHADER_BYTECODE,
    pub PS: D3D12_SHADER_BYTECODE,
    pub DS: D3D12_SHADER_BYTECODE,
    pub HS: D3D12_SHADER_BYTECODE,
    pub GS: D3D12_SHADER_BYTECODE,
    pub StreamOutput: D3D12_STREAM_OUTPUT_DESC,
    pub BlendState: D3D12_BLEND_DESC,
    pub SampleMask: u32,
    pub RasterizerState: D3D12_RASTERIZER_DESC,
    pub DepthStencilState: D3D12_DEPTH_STENCIL_DESC,
    pub InputLayout: D3D12_INPUT_LAYOUT_DESC,
    pub IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE,
    pub PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    pub NumRenderTargets: u32,
    pub RTVFormats: [super::Dxgi::Common::DXGI_FORMAT; 8],
    pub DSVFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub NodeMask: u32,
    pub CachedPSO: D3D12_CACHED_PIPELINE_STATE,
    pub Flags: D3D12_PIPELINE_STATE_FLAGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_GRAPHICS_PIPELINE_STATE_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_GRAPHICS_STATE_NONE: u32 = 0u32;
pub const D3D12_GRAPHICS_STATE_IA_VERTEX_BUFFERS: u32 = 1u32;
pub const D3D12_GRAPHICS_STATE_IA_INDEX_BUFFER: u32 = 2u32;
pub const D3D12_GRAPHICS_STATE_IA_PRIMITIVE_TOPOLOGY: u32 = 4u32;
pub const D3D12_GRAPHICS_STATE_DESCRIPTOR_HEAP: u32 = 8u32;
pub const D3D12_GRAPHICS_STATE_GRAPHICS_ROOT_SIGNATURE: u32 = 16u32;
pub const D3D12_GRAPHICS_STATE_COMPUTE_ROOT_SIGNATURE: u32 = 32u32;
pub const D3D12_GRAPHICS_STATE_RS_VIEWPORTS: u32 = 64u32;
pub const D3D12_GRAPHICS_STATE_RS_SCISSOR_RECTS: u32 = 128u32;
pub const D3D12_GRAPHICS_STATE_PREDICATION: u32 = 256u32;
pub const D3D12_GRAPHICS_STATE_OM_RENDER_TARGETS: u32 = 512u32;
pub const D3D12_GRAPHICS_STATE_OM_STENCIL_REF: u32 = 1024u32;
pub const D3D12_GRAPHICS_STATE_OM_BLEND_FACTOR: u32 = 2048u32;
pub const D3D12_GRAPHICS_STATE_PIPELINE_STATE: u32 = 4096u32;
pub const D3D12_GRAPHICS_STATE_SO_TARGETS: u32 = 8192u32;
pub const D3D12_GRAPHICS_STATE_OM_DEPTH_BOUNDS: u32 = 16384u32;
pub const D3D12_GRAPHICS_STATE_SAMPLE_POSITIONS: u32 = 32768u32;
pub const D3D12_GRAPHICS_STATE_VIEW_INSTANCE_MASK: u32 = 65536u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_READS_PER_INST: u32 = 2u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_READ_PORTS: u32 = 1u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_GS_INPUT_REGISTER_VERTICES: u32 = 32u32;
pub const D3D12_GS_MAX_INSTANCE_COUNT: u32 = 32u32;
pub const D3D12_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: u32 = 1024u32;
pub const D3D12_GS_OUTPUT_ELEMENTS: u32 = 32u32;
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[repr(C)]
pub struct D3D12_HEAP_DESC {
    pub SizeInBytes: u64,
    pub Properties: D3D12_HEAP_PROPERTIES,
    pub Alignment: u64,
    pub Flags: D3D12_HEAP_FLAGS,
}
impl ::core::marker::Copy for D3D12_HEAP_DESC {}
impl ::core::clone::Clone for D3D12_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_HEAP_FLAG_NONE: u32 = 0u32;
pub const D3D12_HEAP_FLAG_SHARED: u32 = 1u32;
pub const D3D12_HEAP_FLAG_DENY_BUFFERS: u32 = 4u32;
pub const D3D12_HEAP_FLAG_ALLOW_DISPLAY: u32 = 8u32;
pub const D3D12_HEAP_FLAG_SHARED_CROSS_ADAPTER: u32 = 32u32;
pub const D3D12_HEAP_FLAG_DENY_RT_DS_TEXTURES: u32 = 64u32;
pub const D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES: u32 = 128u32;
pub const D3D12_HEAP_FLAG_HARDWARE_PROTECTED: u32 = 256u32;
pub const D3D12_HEAP_FLAG_ALLOW_WRITE_WATCH: u32 = 512u32;
pub const D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS: u32 = 1024u32;
pub const D3D12_HEAP_FLAG_CREATE_NOT_RESIDENT: u32 = 2048u32;
pub const D3D12_HEAP_FLAG_CREATE_NOT_ZEROED: u32 = 4096u32;
pub const D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES: u32 = 0u32;
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS: u32 = 192u32;
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES: u32 = 68u32;
pub const D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES: u32 = 132u32;
#[repr(C)]
pub struct D3D12_HEAP_PROPERTIES {
    pub Type: D3D12_HEAP_TYPE,
    pub CPUPageProperty: D3D12_CPU_PAGE_PROPERTY,
    pub MemoryPoolPreference: D3D12_MEMORY_POOL,
    pub CreationNodeMask: u32,
    pub VisibleNodeMask: u32,
}
impl ::core::marker::Copy for D3D12_HEAP_PROPERTIES {}
impl ::core::clone::Clone for D3D12_HEAP_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_HEAP_SERIALIZATION_TIER_0: i32 = 0i32;
pub const D3D12_HEAP_SERIALIZATION_TIER_10: i32 = 10i32;
pub const D3D12_HEAP_TYPE_DEFAULT: i32 = 1i32;
pub const D3D12_HEAP_TYPE_UPLOAD: i32 = 2i32;
pub const D3D12_HEAP_TYPE_READBACK: i32 = 3i32;
pub const D3D12_HEAP_TYPE_CUSTOM: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_HIT_GROUP_DESC {
    pub HitGroupExport: super::super::Foundation::PWSTR,
    pub Type: D3D12_HIT_GROUP_TYPE,
    pub AnyHitShaderImport: super::super::Foundation::PWSTR,
    pub ClosestHitShaderImport: super::super::Foundation::PWSTR,
    pub IntersectionShaderImport: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_HIT_GROUP_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_HIT_GROUP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_HIT_GROUP_TYPE_TRIANGLES: i32 = 0i32;
pub const D3D12_HIT_GROUP_TYPE_PROCEDURAL_PRIMITIVE: i32 = 1i32;
pub const D3D12_HIT_KIND_TRIANGLE_FRONT_FACE: i32 = 254i32;
pub const D3D12_HIT_KIND_TRIANGLE_BACK_FACE: i32 = 255i32;
pub const D3D12_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
pub const D3D12_HS_MAXTESSFACTOR_LOWER_BOUND: f32 = 1f32;
pub const D3D12_HS_MAXTESSFACTOR_UPPER_BOUND: f32 = 64f32;
pub const D3D12_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: u32 = 128u32;
pub const D3D12_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D12_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
pub const D3D12_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D12_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
pub const D3D12_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D12_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
pub const D3D12_IA_PATCH_MAX_CONTROL_POINT_COUNT: u32 = 32u32;
pub const D3D12_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D12_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
pub const D3D12_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32u32;
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
pub const D3D12_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32u32;
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED: i32 = 0i32;
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF: i32 = 1i32;
pub const D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_INDEX_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_INDEX_BUFFER_VIEW {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_INDEX_BUFFER_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC {
    pub Type: D3D12_INDIRECT_ARGUMENT_TYPE,
    pub Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D12_INDIRECT_ARGUMENT_DESC_0 {
    pub VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4,
    pub Constant: D3D12_INDIRECT_ARGUMENT_DESC_0_1,
    pub ConstantBufferView: D3D12_INDIRECT_ARGUMENT_DESC_0_0,
    pub ShaderResourceView: D3D12_INDIRECT_ARGUMENT_DESC_0_2,
    pub UnorderedAccessView: D3D12_INDIRECT_ARGUMENT_DESC_0_3,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
    pub RootParameterIndex: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_0 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
    pub RootParameterIndex: u32,
    pub DestOffsetIn32BitValues: u32,
    pub Num32BitValuesToSet: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_1 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
    pub RootParameterIndex: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_2 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
    pub RootParameterIndex: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_3 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_INDIRECT_ARGUMENT_DESC_0_4 {
    pub Slot: u32,
}
impl ::core::marker::Copy for D3D12_INDIRECT_ARGUMENT_DESC_0_4 {}
impl ::core::clone::Clone for D3D12_INDIRECT_ARGUMENT_DESC_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DRAW: i32 = 0i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED: i32 = 1i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH: i32 = 2i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW: i32 = 3i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_INDEX_BUFFER_VIEW: i32 = 4i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT: i32 = 5i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW: i32 = 6i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW: i32 = 7i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW: i32 = 8i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_RAYS: i32 = 9i32;
pub const D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_MESH: i32 = 10i32;
pub const D3D12_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
pub struct D3D12_INFO_QUEUE_FILTER {
    pub AllowList: D3D12_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D12_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for D3D12_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for D3D12_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D12_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D12_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D12_MESSAGE_ID,
}
impl ::core::marker::Copy for D3D12_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for D3D12_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA: i32 = 0i32;
pub const D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA: i32 = 1i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_INPUT_ELEMENT_DESC {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D12_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_INPUT_ELEMENT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_INPUT_LAYOUT_DESC {
    pub pInputElementDescs: *mut D3D12_INPUT_ELEMENT_DESC,
    pub NumElements: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_INPUT_LAYOUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_INPUT_LAYOUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D12_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D12_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295u32;
pub const D3D12_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_LIBRARY_DESC {
    pub Creator: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_LIBRARY_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_LIBRARY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_LIFETIME_STATE_IN_USE: i32 = 0i32;
pub const D3D12_LIFETIME_STATE_NOT_IN_USE: i32 = 1i32;
pub const D3D12_LINEAR_GAMMA: f32 = 1f32;
#[repr(C)]
pub struct D3D12_LOCAL_ROOT_SIGNATURE {
    pub pLocalRootSignature: ID3D12RootSignature,
}
impl ::core::marker::Copy for D3D12_LOCAL_ROOT_SIGNATURE {}
impl ::core::clone::Clone for D3D12_LOCAL_ROOT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_LOGIC_OP_CLEAR: i32 = 0i32;
pub const D3D12_LOGIC_OP_SET: i32 = 1i32;
pub const D3D12_LOGIC_OP_COPY: i32 = 2i32;
pub const D3D12_LOGIC_OP_COPY_INVERTED: i32 = 3i32;
pub const D3D12_LOGIC_OP_NOOP: i32 = 4i32;
pub const D3D12_LOGIC_OP_INVERT: i32 = 5i32;
pub const D3D12_LOGIC_OP_AND: i32 = 6i32;
pub const D3D12_LOGIC_OP_NAND: i32 = 7i32;
pub const D3D12_LOGIC_OP_OR: i32 = 8i32;
pub const D3D12_LOGIC_OP_NOR: i32 = 9i32;
pub const D3D12_LOGIC_OP_XOR: i32 = 10i32;
pub const D3D12_LOGIC_OP_EQUIV: i32 = 11i32;
pub const D3D12_LOGIC_OP_AND_REVERSE: i32 = 12i32;
pub const D3D12_LOGIC_OP_AND_INVERTED: i32 = 13i32;
pub const D3D12_LOGIC_OP_OR_REVERSE: i32 = 14i32;
pub const D3D12_LOGIC_OP_OR_INVERTED: i32 = 15i32;
pub const D3D12_MAG_FILTER_SHIFT: u32 = 2u32;
pub const D3D12_MAJOR_VERSION: u32 = 12u32;
pub const D3D12_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
pub const D3D12_MAX_DEPTH: f32 = 1f32;
pub const D3D12_MAX_LIVE_STATIC_SAMPLERS: u32 = 2032u32;
pub const D3D12_MAX_MAXANISOTROPY: u32 = 16u32;
pub const D3D12_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
pub const D3D12_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
pub const D3D12_MAX_ROOT_COST: u32 = 64u32;
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_1: u32 = 1000000u32;
pub const D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_2: u32 = 1000000u32;
pub const D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE: u32 = 2048u32;
pub const D3D12_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
pub const D3D12_MAX_VIEW_INSTANCE_COUNT: u32 = 4u32;
pub const D3D12_MEASUREMENTS_ACTION_KEEP_ALL: i32 = 0i32;
pub const D3D12_MEASUREMENTS_ACTION_COMMIT_RESULTS: i32 = 1i32;
pub const D3D12_MEASUREMENTS_ACTION_COMMIT_RESULTS_HIGH_PRIORITY: i32 = 2i32;
pub const D3D12_MEASUREMENTS_ACTION_DISCARD_PREVIOUS: i32 = 3i32;
#[repr(C)]
pub struct D3D12_MEMCPY_DEST {
    pub pData: *mut ::core::ffi::c_void,
    pub RowPitch: usize,
    pub SlicePitch: usize,
}
impl ::core::marker::Copy for D3D12_MEMCPY_DEST {}
impl ::core::clone::Clone for D3D12_MEMCPY_DEST {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_MEMORY_POOL_UNKNOWN: i32 = 0i32;
pub const D3D12_MEMORY_POOL_L0: i32 = 1i32;
pub const D3D12_MEMORY_POOL_L1: i32 = 2i32;
pub const D3D12_MESH_SHADER_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_MESH_SHADER_TIER_1: i32 = 10i32;
#[repr(C)]
pub struct D3D12_MESSAGE {
    pub Category: D3D12_MESSAGE_CATEGORY,
    pub Severity: D3D12_MESSAGE_SEVERITY,
    pub ID: D3D12_MESSAGE_ID,
    pub pDescription: *mut u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for D3D12_MESSAGE {}
impl ::core::clone::Clone for D3D12_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_MESSAGE_CALLBACK_FLAG_NONE: i32 = 0i32;
pub const D3D12_MESSAGE_CALLBACK_IGNORE_FILTERS: i32 = 1i32;
pub const D3D12_MESSAGE_CATEGORY_APPLICATION_DEFINED: i32 = 0i32;
pub const D3D12_MESSAGE_CATEGORY_MISCELLANEOUS: i32 = 1i32;
pub const D3D12_MESSAGE_CATEGORY_INITIALIZATION: i32 = 2i32;
pub const D3D12_MESSAGE_CATEGORY_CLEANUP: i32 = 3i32;
pub const D3D12_MESSAGE_CATEGORY_COMPILATION: i32 = 4i32;
pub const D3D12_MESSAGE_CATEGORY_STATE_CREATION: i32 = 5i32;
pub const D3D12_MESSAGE_CATEGORY_STATE_SETTING: i32 = 6i32;
pub const D3D12_MESSAGE_CATEGORY_STATE_GETTING: i32 = 7i32;
pub const D3D12_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: i32 = 8i32;
pub const D3D12_MESSAGE_CATEGORY_EXECUTION: i32 = 9i32;
pub const D3D12_MESSAGE_CATEGORY_SHADER: i32 = 10i32;
pub const D3D12_MESSAGE_ID_UNKNOWN: i32 = 0i32;
pub const D3D12_MESSAGE_ID_STRING_FROM_APPLICATION: i32 = 1i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_THIS: i32 = 2i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER1: i32 = 3i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER2: i32 = 4i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER3: i32 = 5i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER4: i32 = 6i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER5: i32 = 7i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER6: i32 = 8i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER7: i32 = 9i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER8: i32 = 10i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER9: i32 = 11i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER10: i32 = 12i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER11: i32 = 13i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER12: i32 = 14i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER13: i32 = 15i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER14: i32 = 16i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_PARAMETER15: i32 = 17i32;
pub const D3D12_MESSAGE_ID_CORRUPTED_MULTITHREADING: i32 = 18i32;
pub const D3D12_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: i32 = 19i32;
pub const D3D12_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: i32 = 20i32;
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: i32 = 21i32;
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: i32 = 24i32;
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: i32 = 25i32;
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: i32 = 26i32;
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: i32 = 27i32;
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: i32 = 28i32;
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANESLICE: i32 = 29i32;
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANESLICE: i32 = 30i32;
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: i32 = 31i32;
pub const D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: i32 = 32i32;
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: i32 = 35i32;
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: i32 = 36i32;
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: i32 = 37i32;
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: i32 = 38i32;
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDVIDEOPLANESLICE: i32 = 39i32;
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDPLANESLICE: i32 = 40i32;
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: i32 = 41i32;
pub const D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: i32 = 42i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: i32 = 45i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: i32 = 46i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: i32 = 47i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: i32 = 48i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: i32 = 49i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: i32 = 52i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: i32 = 53i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: i32 = 54i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: i32 = 55i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: i32 = 56i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: i32 = 57i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: i32 = 58i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: i32 = 59i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: i32 = 60i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: i32 = 61i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: i32 = 62i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: i32 = 63i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: i32 = 64i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: i32 = 65i32;
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: i32 = 66i32;
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: i32 = 67i32;
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: i32 = 68i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: i32 = 69i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: i32 = 70i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: i32 = 71i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: i32 = 72i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: i32 = 73i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: i32 = 74i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: i32 = 75i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: i32 = 76i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: i32 = 79i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: i32 = 80i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: i32 = 81i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: i32 = 82i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: i32 = 83i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: i32 = 84i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: i32 = 85i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: i32 = 86i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: i32 = 87i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: i32 = 88i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: i32 = 89i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: i32 = 90i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: i32 = 91i32;
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: i32 = 92i32;
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: i32 = 93i32;
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: i32 = 94i32;
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: i32 = 95i32;
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: i32 = 96i32;
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: i32 = 97i32;
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: i32 = 98i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: i32 = 100i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: i32 = 101i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: i32 = 102i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: i32 = 103i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: i32 = 104i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: i32 = 105i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: i32 = 106i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: i32 = 107i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: i32 = 108i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: i32 = 109i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: i32 = 111i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: i32 = 112i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: i32 = 113i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: i32 = 114i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: i32 = 115i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: i32 = 116i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: i32 = 117i32;
pub const D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: i32 = 135i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_NOT_SET: i32 = 200i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_MISMATCH: i32 = 201i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_NOT_SET: i32 = 202i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: i32 = 209i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_TOO_SMALL: i32 = 210i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_NOT_SET: i32 = 211i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_FORMAT_INVALID: i32 = 212i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_TOO_SMALL: i32 = 213i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INVALID_PRIMITIVETOPOLOGY: i32 = 219i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_STRIDE_UNALIGNED: i32 = 221i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_OFFSET_UNALIGNED: i32 = 222i32;
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: i32 = 232i32;
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: i32 = 233i32;
pub const D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: i32 = 234i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: i32 = 239i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: i32 = 240i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: i32 = 245i32;
pub const D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: i32 = 253i32;
pub const D3D12_MESSAGE_ID_LIVE_OBJECT_SUMMARY: i32 = 255i32;
pub const D3D12_MESSAGE_ID_LIVE_DEVICE: i32 = 274i32;
pub const D3D12_MESSAGE_ID_LIVE_SWAPCHAIN: i32 = 275i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFLAGS: i32 = 276i32;
pub const D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDCLASSLINKAGE: i32 = 277i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDCLASSLINKAGE: i32 = 278i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAMTORASTERIZER: i32 = 280i32;
pub const D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDCLASSLINKAGE: i32 = 283i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAM: i32 = 284i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDENTRIES: i32 = 285i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTRIDES: i32 = 286i32;
pub const D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTRIDES: i32 = 287i32;
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_OUTOFMEMORY: i32 = 289i32;
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERBYTECODE: i32 = 290i32;
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERTYPE: i32 = 291i32;
pub const D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDCLASSLINKAGE: i32 = 292i32;
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_OUTOFMEMORY: i32 = 294i32;
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERBYTECODE: i32 = 295i32;
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERTYPE: i32 = 296i32;
pub const D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCLASSLINKAGE: i32 = 297i32;
pub const D3D12_MESSAGE_ID_RESOURCE_UNMAP_NOTMAPPED: i32 = 310i32;
pub const D3D12_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_MISMATCHED_DATA_SIZE: i32 = 318i32;
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_OUTOFMEMORY: i32 = 321i32;
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERBYTECODE: i32 = 322i32;
pub const D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCLASSLINKAGE: i32 = 323i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 331i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 332i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 333i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 334i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 335i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 336i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 337i32;
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE: i32 = 340i32;
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC: i32 = 341i32;
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT: i32 = 342i32;
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANESLICE: i32 = 343i32;
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANESLICE: i32 = 344i32;
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS: i32 = 345i32;
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT: i32 = 346i32;
pub const D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS: i32 = 354i32;
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFORCEDSAMPLECOUNT: i32 = 401i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDLOGICOPS: i32 = 403i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 410i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 412i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 414i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 416i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 418i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 420i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 422i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_UAVSNOTSUPPORTED: i32 = 425i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_UAVSNOTSUPPORTED: i32 = 426i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_UAVSNOTSUPPORTED: i32 = 427i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_UAVSNOTSUPPORTED: i32 = 428i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UAVSNOTSUPPORTED: i32 = 429i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_UAVSNOTSUPPORTED: i32 = 430i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_UAVSNOTSUPPORTED: i32 = 431i32;
pub const D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDSOURCERECT: i32 = 447i32;
pub const D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_EMPTYRECT: i32 = 448i32;
pub const D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_INVALID_PARAMETER: i32 = 493i32;
pub const D3D12_MESSAGE_ID_COPYTILEMAPPINGS_INVALID_PARAMETER: i32 = 494i32;
pub const D3D12_MESSAGE_ID_CREATEDEVICE_INVALIDARGS: i32 = 506i32;
pub const D3D12_MESSAGE_ID_CREATEDEVICE_WARNING: i32 = 507i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_TYPE: i32 = 519i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_NULL_POINTER: i32 = 520i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SUBRESOURCE: i32 = 521i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_RESERVED_BITS: i32 = 522i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISSING_BIND_FLAGS: i32 = 523i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_MISC_FLAGS: i32 = 524i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MATCHING_STATES: i32 = 525i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINATION: i32 = 526i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_BEFORE_AFTER_MISMATCH: i32 = 527i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_RESOURCE: i32 = 528i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_SAMPLE_COUNT: i32 = 529i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS: i32 = 530i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINED_FLAGS: i32 = 531i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS_FOR_FORMAT: i32 = 532i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SPLIT_BARRIER: i32 = 533i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_END: i32 = 534i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_BEGIN: i32 = 535i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAG: i32 = 536i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMMAND_LIST_TYPE: i32 = 537i32;
pub const D3D12_MESSAGE_ID_INVALID_SUBRESOURCE_STATE: i32 = 538i32;
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CONTENTION: i32 = 540i32;
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET: i32 = 541i32;
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET_BUNDLE: i32 = 542i32;
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CANNOT_RESET: i32 = 543i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_OPEN: i32 = 544i32;
pub const D3D12_MESSAGE_ID_INVALID_BUNDLE_API: i32 = 546i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_CLOSED: i32 = 547i32;
pub const D3D12_MESSAGE_ID_WRONG_COMMAND_ALLOCATOR_TYPE: i32 = 549i32;
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_SYNC: i32 = 552i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_SYNC: i32 = 553i32;
pub const D3D12_MESSAGE_ID_SET_DESCRIPTOR_HEAP_INVALID: i32 = 554i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMANDQUEUE: i32 = 557i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMANDALLOCATOR: i32 = 558i32;
pub const D3D12_MESSAGE_ID_CREATE_PIPELINESTATE: i32 = 559i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMANDLIST12: i32 = 560i32;
pub const D3D12_MESSAGE_ID_CREATE_RESOURCE: i32 = 562i32;
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTORHEAP: i32 = 563i32;
pub const D3D12_MESSAGE_ID_CREATE_ROOTSIGNATURE: i32 = 564i32;
pub const D3D12_MESSAGE_ID_CREATE_LIBRARY: i32 = 565i32;
pub const D3D12_MESSAGE_ID_CREATE_HEAP: i32 = 566i32;
pub const D3D12_MESSAGE_ID_CREATE_MONITOREDFENCE: i32 = 567i32;
pub const D3D12_MESSAGE_ID_CREATE_QUERYHEAP: i32 = 568i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMANDSIGNATURE: i32 = 569i32;
pub const D3D12_MESSAGE_ID_LIVE_COMMANDQUEUE: i32 = 570i32;
pub const D3D12_MESSAGE_ID_LIVE_COMMANDALLOCATOR: i32 = 571i32;
pub const D3D12_MESSAGE_ID_LIVE_PIPELINESTATE: i32 = 572i32;
pub const D3D12_MESSAGE_ID_LIVE_COMMANDLIST12: i32 = 573i32;
pub const D3D12_MESSAGE_ID_LIVE_RESOURCE: i32 = 575i32;
pub const D3D12_MESSAGE_ID_LIVE_DESCRIPTORHEAP: i32 = 576i32;
pub const D3D12_MESSAGE_ID_LIVE_ROOTSIGNATURE: i32 = 577i32;
pub const D3D12_MESSAGE_ID_LIVE_LIBRARY: i32 = 578i32;
pub const D3D12_MESSAGE_ID_LIVE_HEAP: i32 = 579i32;
pub const D3D12_MESSAGE_ID_LIVE_MONITOREDFENCE: i32 = 580i32;
pub const D3D12_MESSAGE_ID_LIVE_QUERYHEAP: i32 = 581i32;
pub const D3D12_MESSAGE_ID_LIVE_COMMANDSIGNATURE: i32 = 582i32;
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDQUEUE: i32 = 583i32;
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDALLOCATOR: i32 = 584i32;
pub const D3D12_MESSAGE_ID_DESTROY_PIPELINESTATE: i32 = 585i32;
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDLIST12: i32 = 586i32;
pub const D3D12_MESSAGE_ID_DESTROY_RESOURCE: i32 = 588i32;
pub const D3D12_MESSAGE_ID_DESTROY_DESCRIPTORHEAP: i32 = 589i32;
pub const D3D12_MESSAGE_ID_DESTROY_ROOTSIGNATURE: i32 = 590i32;
pub const D3D12_MESSAGE_ID_DESTROY_LIBRARY: i32 = 591i32;
pub const D3D12_MESSAGE_ID_DESTROY_HEAP: i32 = 592i32;
pub const D3D12_MESSAGE_ID_DESTROY_MONITOREDFENCE: i32 = 593i32;
pub const D3D12_MESSAGE_ID_DESTROY_QUERYHEAP: i32 = 594i32;
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDSIGNATURE: i32 = 595i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONS: i32 = 597i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMISCFLAGS: i32 = 599i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDARG_RETURN: i32 = 602i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_OUTOFMEMORY_RETURN: i32 = 603i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDESC: i32 = 604i32;
pub const D3D12_MESSAGE_ID_POSSIBLY_INVALID_SUBRESOURCE_STATE: i32 = 607i32;
pub const D3D12_MESSAGE_ID_INVALID_USE_OF_NON_RESIDENT_RESOURCE: i32 = 608i32;
pub const D3D12_MESSAGE_ID_POSSIBLE_INVALID_USE_OF_NON_RESIDENT_RESOURCE: i32 = 609i32;
pub const D3D12_MESSAGE_ID_BUNDLE_PIPELINE_STATE_MISMATCH: i32 = 610i32;
pub const D3D12_MESSAGE_ID_PRIMITIVE_TOPOLOGY_MISMATCH_PIPELINE_STATE: i32 = 611i32;
pub const D3D12_MESSAGE_ID_RENDER_TARGET_FORMAT_MISMATCH_PIPELINE_STATE: i32 = 613i32;
pub const D3D12_MESSAGE_ID_RENDER_TARGET_SAMPLE_DESC_MISMATCH_PIPELINE_STATE: i32 = 614i32;
pub const D3D12_MESSAGE_ID_DEPTH_STENCIL_FORMAT_MISMATCH_PIPELINE_STATE: i32 = 615i32;
pub const D3D12_MESSAGE_ID_DEPTH_STENCIL_SAMPLE_DESC_MISMATCH_PIPELINE_STATE: i32 = 616i32;
pub const D3D12_MESSAGE_ID_CREATESHADER_INVALIDBYTECODE: i32 = 622i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_NULLDESC: i32 = 623i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDSIZE: i32 = 624i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDHEAPTYPE: i32 = 625i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES: i32 = 626i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMEMORYPOOL: i32 = 627i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDPROPERTIES: i32 = 628i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDALIGNMENT: i32 = 629i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMISCFLAGS: i32 = 630i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDMISCFLAGS: i32 = 631i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_INVALIDARG_RETURN: i32 = 632i32;
pub const D3D12_MESSAGE_ID_CREATEHEAP_OUTOFMEMORY_RETURN: i32 = 633i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAPPROPERTIES: i32 = 634i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPTYPE: i32 = 635i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES: i32 = 636i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDMEMORYPOOL: i32 = 637i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPPROPERTIES: i32 = 638i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPMISCFLAGS: i32 = 639i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPMISCFLAGS: i32 = 640i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDARG_RETURN: i32 = 641i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_OUTOFMEMORY_RETURN: i32 = 642i32;
pub const D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_UNRECOGNIZEDHEAPTYPE: i32 = 643i32;
pub const D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_INVALIDHEAPTYPE: i32 = 644i32;
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_INVALID_DESC: i32 = 645i32;
pub const D3D12_MESSAGE_ID_INVALID_DESCRIPTOR_HANDLE: i32 = 646i32;
pub const D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_CONSERVATIVERASTERMODE: i32 = 647i32;
pub const D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_RESOURCE: i32 = 649i32;
pub const D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_DESC: i32 = 650i32;
pub const D3D12_MESSAGE_ID_CREATE_UNORDEREDACCESS_VIEW_INVALID_COUNTER_USAGE: i32 = 652i32;
pub const D3D12_MESSAGE_ID_COPY_DESCRIPTORS_INVALID_RANGES: i32 = 653i32;
pub const D3D12_MESSAGE_ID_COPY_DESCRIPTORS_WRITE_ONLY_DESCRIPTOR: i32 = 654i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RTV_FORMAT_NOT_UNKNOWN: i32 = 655i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_RENDER_TARGET_COUNT: i32 = 656i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VERTEX_SHADER_NOT_SET: i32 = 657i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_NOT_SET: i32 = 658i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_HS_DS_SIGNATURE_MISMATCH: i32 = 659i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERINDEX: i32 = 660i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_COMPONENTTYPE: i32 = 661i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERMASK: i32 = 662i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SYSTEMVALUE: i32 = 663i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: i32 = 664i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_MINPRECISION: i32 = 665i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: i32 = 666i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_XOR_DS_MISMATCH: i32 = 667i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HULL_SHADER_INPUT_TOPOLOGY_MISMATCH: i32 = 668i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_CONTROL_POINT_COUNT_MISMATCH: i32 = 669i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_TESSELLATOR_DOMAIN_MISMATCH: i32 = 670i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: i32 = 671i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_FORCED_SAMPLE_COUNT: i32 = 672i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_PRIMITIVETOPOLOGY: i32 = 673i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SYSTEMVALUE: i32 = 674i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: i32 = 675i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: i32 = 676i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_TYPE_MISMATCH: i32 = 677i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_LOGIC_OPS: i32 = 678i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RENDERTARGETVIEW_NOT_SET: i32 = 679i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DEPTHSTENCILVIEW_NOT_SET: i32 = 680i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_INPUT_PRIMITIVE_MISMATCH: i32 = 681i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_POSITION_NOT_PRESENT: i32 = 682i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE_FLAGS: i32 = 683i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_INDEX_BUFFER_PROPERTIES: i32 = 684i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SAMPLE_DESC: i32 = 685i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_ROOT_SIGNATURE_MISMATCH: i32 = 686i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DS_ROOT_SIGNATURE_MISMATCH: i32 = 687i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VS_ROOT_SIGNATURE_MISMATCH: i32 = 688i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_ROOT_SIGNATURE_MISMATCH: i32 = 689i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_ROOT_SIGNATURE_MISMATCH: i32 = 690i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE: i32 = 691i32;
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_OPEN_BUNDLE: i32 = 692i32;
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_DESCRIPTOR_HEAP_MISMATCH: i32 = 693i32;
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_TYPE: i32 = 694i32;
pub const D3D12_MESSAGE_ID_DRAW_EMPTY_SCISSOR_RECTANGLE: i32 = 695i32;
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_BLOB_NOT_FOUND: i32 = 696i32;
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_DESERIALIZE_FAILED: i32 = 697i32;
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_INVALID_CONFIGURATION: i32 = 698i32;
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_SUPPORTED_ON_DEVICE: i32 = 699i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLRESOURCEPROPERTIES: i32 = 700i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAP: i32 = 701i32;
pub const D3D12_MESSAGE_ID_GETRESOURCEALLOCATIONINFO_INVALIDRDESCS: i32 = 702i32;
pub const D3D12_MESSAGE_ID_MAKERESIDENT_NULLOBJECTARRAY: i32 = 703i32;
pub const D3D12_MESSAGE_ID_EVICT_NULLOBJECTARRAY: i32 = 705i32;
pub const D3D12_MESSAGE_ID_SET_DESCRIPTOR_TABLE_INVALID: i32 = 708i32;
pub const D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_INVALID: i32 = 709i32;
pub const D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_BUFFER_VIEW_INVALID: i32 = 710i32;
pub const D3D12_MESSAGE_ID_SET_ROOT_SHADER_RESOURCE_VIEW_INVALID: i32 = 711i32;
pub const D3D12_MESSAGE_ID_SET_ROOT_UNORDERED_ACCESS_VIEW_INVALID: i32 = 712i32;
pub const D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID_DESC: i32 = 713i32;
pub const D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID_DESC: i32 = 715i32;
pub const D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID_DESC: i32 = 717i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDDIMENSIONALITY: i32 = 718i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDLAYOUT: i32 = 719i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONALITY: i32 = 720i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDALIGNMENT: i32 = 721i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMIPLEVELS: i32 = 722i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDSAMPLEDESC: i32 = 723i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDLAYOUT: i32 = 724i32;
pub const D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID: i32 = 725i32;
pub const D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID: i32 = 726i32;
pub const D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID: i32 = 727i32;
pub const D3D12_MESSAGE_ID_SET_RENDER_TARGETS_INVALID: i32 = 728i32;
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_INVALID_PARAMETERS: i32 = 729i32;
pub const D3D12_MESSAGE_ID_BEGIN_END_QUERY_INVALID_PARAMETERS: i32 = 731i32;
pub const D3D12_MESSAGE_ID_CLOSE_COMMAND_LIST_OPEN_QUERY: i32 = 732i32;
pub const D3D12_MESSAGE_ID_RESOLVE_QUERY_DATA_INVALID_PARAMETERS: i32 = 733i32;
pub const D3D12_MESSAGE_ID_SET_PREDICATION_INVALID_PARAMETERS: i32 = 734i32;
pub const D3D12_MESSAGE_ID_TIMESTAMPS_NOT_SUPPORTED: i32 = 735i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDFORMAT: i32 = 737i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDFORMAT: i32 = 738i32;
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDSUBRESOURCERANGE: i32 = 739i32;
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDBASEOFFSET: i32 = 740i32;
pub const D3D12_MESSAGE_ID_GETCOPYABLELAYOUT_INVALIDSUBRESOURCERANGE: i32 = 739i32;
pub const D3D12_MESSAGE_ID_GETCOPYABLELAYOUT_INVALIDBASEOFFSET: i32 = 740i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_HEAP: i32 = 741i32;
pub const D3D12_MESSAGE_ID_CREATE_SAMPLER_INVALID: i32 = 742i32;
pub const D3D12_MESSAGE_ID_CREATECOMMANDSIGNATURE_INVALID: i32 = 743i32;
pub const D3D12_MESSAGE_ID_EXECUTE_INDIRECT_INVALID_PARAMETERS: i32 = 744i32;
pub const D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_RESOURCE_DIMENSION: i32 = 745i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUE: i32 = 815i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDCLEARVALUEFORMAT: i32 = 816i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUEFORMAT: i32 = 817i32;
pub const D3D12_MESSAGE_ID_CREATERESOURCE_CLEARVALUEDENORMFLUSH: i32 = 818i32;
pub const D3D12_MESSAGE_ID_CLEARRENDERTARGETVIEW_MISMATCHINGCLEARVALUE: i32 = 820i32;
pub const D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_MISMATCHINGCLEARVALUE: i32 = 821i32;
pub const D3D12_MESSAGE_ID_MAP_INVALIDHEAP: i32 = 822i32;
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDHEAP: i32 = 823i32;
pub const D3D12_MESSAGE_ID_MAP_INVALIDRESOURCE: i32 = 824i32;
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDRESOURCE: i32 = 825i32;
pub const D3D12_MESSAGE_ID_MAP_INVALIDSUBRESOURCE: i32 = 826i32;
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDSUBRESOURCE: i32 = 827i32;
pub const D3D12_MESSAGE_ID_MAP_INVALIDRANGE: i32 = 828i32;
pub const D3D12_MESSAGE_ID_UNMAP_INVALIDRANGE: i32 = 829i32;
pub const D3D12_MESSAGE_ID_MAP_INVALIDDATAPOINTER: i32 = 832i32;
pub const D3D12_MESSAGE_ID_MAP_INVALIDARG_RETURN: i32 = 833i32;
pub const D3D12_MESSAGE_ID_MAP_OUTOFMEMORY_RETURN: i32 = 834i32;
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_BUNDLENOTSUPPORTED: i32 = 835i32;
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_COMMANDLISTMISMATCH: i32 = 836i32;
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_OPENCOMMANDLIST: i32 = 837i32;
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_FAILEDCOMMANDLIST: i32 = 838i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLDST: i32 = 839i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDDSTRESOURCEDIMENSION: i32 = 840i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_DSTRANGEOUTOFBOUNDS: i32 = 841i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLSRC: i32 = 842i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDSRCRESOURCEDIMENSION: i32 = 843i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_SRCRANGEOUTOFBOUNDS: i32 = 844i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDCOPYFLAGS: i32 = 845i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLDST: i32 = 846i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTTYPE: i32 = 847i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCEDIMENSION: i32 = 848i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCE: i32 = 849i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTSUBRESOURCE: i32 = 850i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTOFFSET: i32 = 851i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTFORMAT: i32 = 852i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTFORMAT: i32 = 853i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDIMENSIONS: i32 = 854i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTROWPITCH: i32 = 855i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTPLACEMENT: i32 = 856i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDSPLACEDFOOTPRINTFORMAT: i32 = 857i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_DSTREGIONOUTOFBOUNDS: i32 = 858i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLSRC: i32 = 859i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCTYPE: i32 = 860i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCEDIMENSION: i32 = 861i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCE: i32 = 862i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCSUBRESOURCE: i32 = 863i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCOFFSET: i32 = 864i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCFORMAT: i32 = 865i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCFORMAT: i32 = 866i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDIMENSIONS: i32 = 867i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCROWPITCH: i32 = 868i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCPLACEMENT: i32 = 869i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDSPLACEDFOOTPRINTFORMAT: i32 = 870i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_SRCREGIONOUTOFBOUNDS: i32 = 871i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTCOORDINATES: i32 = 872i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCBOX: i32 = 873i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_FORMATMISMATCH: i32 = 874i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_EMPTYBOX: i32 = 875i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDCOPYFLAGS: i32 = 876i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SUBRESOURCE_INDEX: i32 = 877i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_FORMAT: i32 = 878i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_MISMATCH: i32 = 879i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SAMPLE_COUNT: i32 = 880i32;
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_INVALID_SHADER: i32 = 881i32;
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_CS_ROOT_SIGNATURE_MISMATCH: i32 = 882i32;
pub const D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_MISSING_ROOT_SIGNATURE: i32 = 883i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALIDCACHEDBLOB: i32 = 884i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBADAPTERMISMATCH: i32 = 885i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDRIVERVERSIONMISMATCH: i32 = 886i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDESCMISMATCH: i32 = 887i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBIGNORED: i32 = 888i32;
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDHEAP: i32 = 889i32;
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDRESOURCE: i32 = 890i32;
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDBOX: i32 = 891i32;
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDSUBRESOURCE: i32 = 892i32;
pub const D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_EMPTYBOX: i32 = 893i32;
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDHEAP: i32 = 894i32;
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDRESOURCE: i32 = 895i32;
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDBOX: i32 = 896i32;
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDSUBRESOURCE: i32 = 897i32;
pub const D3D12_MESSAGE_ID_READFROMSUBRESOURCE_EMPTYBOX: i32 = 898i32;
pub const D3D12_MESSAGE_ID_TOO_MANY_NODES_SPECIFIED: i32 = 899i32;
pub const D3D12_MESSAGE_ID_INVALID_NODE_INDEX: i32 = 900i32;
pub const D3D12_MESSAGE_ID_GETHEAPPROPERTIES_INVALIDRESOURCE: i32 = 901i32;
pub const D3D12_MESSAGE_ID_NODE_MASK_MISMATCH: i32 = 902i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_OUTOFMEMORY: i32 = 903i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_MULTIPLE_SWAPCHAIN_BUFFER_REFERENCES: i32 = 904i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_TOO_MANY_SWAPCHAIN_REFERENCES: i32 = 905i32;
pub const D3D12_MESSAGE_ID_COMMAND_QUEUE_TOO_MANY_SWAPCHAIN_REFERENCES: i32 = 906i32;
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_WRONGSWAPCHAINBUFFERREFERENCE: i32 = 907i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_SETRENDERTARGETS_INVALIDNUMRENDERTARGETS: i32 = 908i32;
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_TYPE: i32 = 909i32;
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_FLAGS: i32 = 910i32;
pub const D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFLAGS: i32 = 911i32;
pub const D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFORMAT: i32 = 912i32;
pub const D3D12_MESSAGE_ID_CREATESHAREDHEAP_INVALIDFLAGS: i32 = 913i32;
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_UNRECOGNIZEDPROPERTIES: i32 = 914i32;
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDSIZE: i32 = 915i32;
pub const D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDOBJECT: i32 = 916i32;
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDOBJECT: i32 = 917i32;
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDKEY: i32 = 918i32;
pub const D3D12_MESSAGE_ID_KEYEDMUTEX_WRONGSTATE: i32 = 919i32;
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_PRIORITY: i32 = 920i32;
pub const D3D12_MESSAGE_ID_OBJECT_DELETED_WHILE_STILL_IN_USE: i32 = 921i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_FLAGS: i32 = 922i32;
pub const D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_HAS_NO_RESOURCE: i32 = 923i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_RENDER_TARGET_DELETED: i32 = 924i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_ALL_RENDER_TARGETS_HAVE_UNKNOWN_FORMAT: i32 = 925i32;
pub const D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_INTERSECTS_MULTIPLE_BUFFERS: i32 = 926i32;
pub const D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_GPU_WRITTEN_READBACK_RESOURCE_MAPPED: i32 = 927i32;
pub const D3D12_MESSAGE_ID_UNMAP_RANGE_NOT_EMPTY: i32 = 929i32;
pub const D3D12_MESSAGE_ID_MAP_INVALID_NULLRANGE: i32 = 930i32;
pub const D3D12_MESSAGE_ID_UNMAP_INVALID_NULLRANGE: i32 = 931i32;
pub const D3D12_MESSAGE_ID_NO_GRAPHICS_API_SUPPORT: i32 = 932i32;
pub const D3D12_MESSAGE_ID_NO_COMPUTE_API_SUPPORT: i32 = 933i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_FLAGS_NOT_SUPPORTED: i32 = 934i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_ARGUMENT_UNINITIALIZED: i32 = 935i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_HEAP_INDEX_OUT_OF_BOUNDS: i32 = 936i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TABLE_REGISTER_INDEX_OUT_OF_BOUNDS: i32 = 937i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_UNINITIALIZED: i32 = 938i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TYPE_MISMATCH: i32 = 939i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SRV_RESOURCE_DIMENSION_MISMATCH: i32 = 940i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UAV_RESOURCE_DIMENSION_MISMATCH: i32 = 941i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INCOMPATIBLE_RESOURCE_STATE: i32 = 942i32;
pub const D3D12_MESSAGE_ID_COPYRESOURCE_NULLDST: i32 = 943i32;
pub const D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDDSTRESOURCE: i32 = 944i32;
pub const D3D12_MESSAGE_ID_COPYRESOURCE_NULLSRC: i32 = 945i32;
pub const D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDSRCRESOURCE: i32 = 946i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLDST: i32 = 947i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDDSTRESOURCE: i32 = 948i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLSRC: i32 = 949i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDSRCRESOURCE: i32 = 950i32;
pub const D3D12_MESSAGE_ID_PIPELINE_STATE_TYPE_MISMATCH: i32 = 951i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_NOT_SET: i32 = 952i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_MISMATCH: i32 = 953i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_ZERO_BARRIERS: i32 = 954i32;
pub const D3D12_MESSAGE_ID_BEGIN_END_EVENT_MISMATCH: i32 = 955i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_POSSIBLE_BEFORE_AFTER_MISMATCH: i32 = 956i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_BEGIN_END: i32 = 957i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INVALID_RESOURCE: i32 = 958i32;
pub const D3D12_MESSAGE_ID_USE_OF_ZERO_REFCOUNT_OBJECT: i32 = 959i32;
pub const D3D12_MESSAGE_ID_OBJECT_EVICTED_WHILE_STILL_IN_USE: i32 = 960i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_DESCRIPTOR_ACCESS_OUT_OF_BOUNDS: i32 = 961i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_INVALIDLIBRARYBLOB: i32 = 962i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_DRIVERVERSIONMISMATCH: i32 = 963i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_ADAPTERVERSIONMISMATCH: i32 = 964i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_UNSUPPORTED: i32 = 965i32;
pub const D3D12_MESSAGE_ID_CREATE_PIPELINELIBRARY: i32 = 966i32;
pub const D3D12_MESSAGE_ID_LIVE_PIPELINELIBRARY: i32 = 967i32;
pub const D3D12_MESSAGE_ID_DESTROY_PIPELINELIBRARY: i32 = 968i32;
pub const D3D12_MESSAGE_ID_STOREPIPELINE_NONAME: i32 = 969i32;
pub const D3D12_MESSAGE_ID_STOREPIPELINE_DUPLICATENAME: i32 = 970i32;
pub const D3D12_MESSAGE_ID_LOADPIPELINE_NAMENOTFOUND: i32 = 971i32;
pub const D3D12_MESSAGE_ID_LOADPIPELINE_INVALIDDESC: i32 = 972i32;
pub const D3D12_MESSAGE_ID_PIPELINELIBRARY_SERIALIZE_NOTENOUGHMEMORY: i32 = 973i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_RT_OUTPUT_MISMATCH: i32 = 974i32;
pub const D3D12_MESSAGE_ID_SETEVENTONMULTIPLEFENCECOMPLETION_INVALIDFLAGS: i32 = 975i32;
pub const D3D12_MESSAGE_ID_CREATE_QUEUE_VIDEO_NOT_SUPPORTED: i32 = 976i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_ALLOCATOR_VIDEO_NOT_SUPPORTED: i32 = 977i32;
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_VIDEO_DECODE_STATISTICS_NOT_SUPPORTED: i32 = 978i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDLIST: i32 = 979i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODER: i32 = 980i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODESTREAM: i32 = 981i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDLIST: i32 = 982i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODER: i32 = 983i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODESTREAM: i32 = 984i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDLIST: i32 = 985i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODER: i32 = 986i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODESTREAM: i32 = 987i32;
pub const D3D12_MESSAGE_ID_DECODE_FRAME_INVALID_PARAMETERS: i32 = 988i32;
pub const D3D12_MESSAGE_ID_DEPRECATED_API: i32 = 989i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_COMMAND_LIST_TYPE: i32 = 990i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_DESCRIPTOR_TABLE_NOT_SET: i32 = 991i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_CONSTANT_BUFFER_VIEW_NOT_SET: i32 = 992i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_SHADER_RESOURCE_VIEW_NOT_SET: i32 = 993i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_UNORDERED_ACCESS_VIEW_NOT_SET: i32 = 994i32;
pub const D3D12_MESSAGE_ID_DISCARD_INVALID_SUBRESOURCE_RANGE: i32 = 995i32;
pub const D3D12_MESSAGE_ID_DISCARD_ONE_SUBRESOURCE_FOR_MIPS_WITH_RECTS: i32 = 996i32;
pub const D3D12_MESSAGE_ID_DISCARD_NO_RECTS_FOR_NON_TEXTURE2D: i32 = 997i32;
pub const D3D12_MESSAGE_ID_COPY_ON_SAME_SUBRESOURCE: i32 = 998i32;
pub const D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PAGEABLE: i32 = 999i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UNSUPPORTED: i32 = 1000i32;
pub const D3D12_MESSAGE_ID_STATIC_DESCRIPTOR_INVALID_DESCRIPTOR_CHANGE: i32 = 1001i32;
pub const D3D12_MESSAGE_ID_DATA_STATIC_DESCRIPTOR_INVALID_DATA_CHANGE: i32 = 1002i32;
pub const D3D12_MESSAGE_ID_DATA_STATIC_WHILE_SET_AT_EXECUTE_DESCRIPTOR_INVALID_DATA_CHANGE: i32 = 1003i32;
pub const D3D12_MESSAGE_ID_EXECUTE_BUNDLE_STATIC_DESCRIPTOR_DATA_STATIC_NOT_SET: i32 = 1004i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_ACCESS_OUT_OF_BOUNDS: i32 = 1005i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SAMPLER_MODE_MISMATCH: i32 = 1006i32;
pub const D3D12_MESSAGE_ID_CREATE_FENCE_INVALID_FLAGS: i32 = 1007i32;
pub const D3D12_MESSAGE_ID_RESOURCE_BARRIER_DUPLICATE_SUBRESOURCE_TRANSITIONS: i32 = 1008i32;
pub const D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PRIORITY: i32 = 1009i32;
pub const D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_LARGE_NUM_DESCRIPTORS: i32 = 1013i32;
pub const D3D12_MESSAGE_ID_BEGIN_EVENT: i32 = 1014i32;
pub const D3D12_MESSAGE_ID_END_EVENT: i32 = 1015i32;
pub const D3D12_MESSAGE_ID_CREATEDEVICE_DEBUG_LAYER_STARTUP_OPTIONS: i32 = 1016i32;
pub const D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_DEPTHBOUNDSTEST_UNSUPPORTED: i32 = 1017i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_DUPLICATE_SUBOBJECT: i32 = 1018i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_UNKNOWN_SUBOBJECT: i32 = 1019i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_ZERO_SIZE_STREAM: i32 = 1020i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_STREAM: i32 = 1021i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CANNOT_DEDUCE_TYPE: i32 = 1022i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_RESOURCE_DIMENSION_MISMATCH: i32 = 1023i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_PRIVILEGE_FOR_GLOBAL_REALTIME: i32 = 1024i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_HARDWARE_SUPPORT_FOR_GLOBAL_REALTIME: i32 = 1025i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_ARCHITECTURE: i32 = 1026i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DST: i32 = 1027i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE_DIMENSION: i32 = 1028i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DST_RANGE_OUT_OF_BOUNDS: i32 = 1029i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_SRC: i32 = 1030i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE_DIMENSION: i32 = 1031i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_SRC_RANGE_OUT_OF_BOUNDS: i32 = 1032i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_OFFSET_ALIGNMENT: i32 = 1033i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_RESOURCES: i32 = 1034i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_SUBRESOURCE_RANGES: i32 = 1035i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_RESOURCE: i32 = 1036i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_SUBRESOURCE_RANGE: i32 = 1037i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_SUBRESOURCE_OUT_OF_BOUNDS: i32 = 1038i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_RANGE_OUT_OF_BOUNDS: i32 = 1039i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_ZERO_DEPENDENCIES: i32 = 1040i32;
pub const D3D12_MESSAGE_ID_DEVICE_CREATE_SHARED_HANDLE_INVALIDARG: i32 = 1041i32;
pub const D3D12_MESSAGE_ID_DESCRIPTOR_HANDLE_WITH_INVALID_RESOURCE: i32 = 1042i32;
pub const D3D12_MESSAGE_ID_SETDEPTHBOUNDS_INVALIDARGS: i32 = 1043i32;
pub const D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_STATE_IMPRECISE: i32 = 1044i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_PIPELINE_STATE_NOT_SET: i32 = 1045i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_MODEL_MISMATCH: i32 = 1046i32;
pub const D3D12_MESSAGE_ID_OBJECT_ACCESSED_WHILE_STILL_IN_USE: i32 = 1047i32;
pub const D3D12_MESSAGE_ID_PROGRAMMABLE_MSAA_UNSUPPORTED: i32 = 1048i32;
pub const D3D12_MESSAGE_ID_SETSAMPLEPOSITIONS_INVALIDARGS: i32 = 1049i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCEREGION_INVALID_RECT: i32 = 1050i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDQUEUE: i32 = 1051i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDLIST: i32 = 1052i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDQUEUE: i32 = 1053i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDQUEUE: i32 = 1054i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDLIST: i32 = 1055i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDQUEUE: i32 = 1056i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDQUEUE: i32 = 1057i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDLIST: i32 = 1058i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDQUEUE: i32 = 1059i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSOR: i32 = 1060i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSSTREAM: i32 = 1061i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSOR: i32 = 1062i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSSTREAM: i32 = 1063i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSOR: i32 = 1064i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSSTREAM: i32 = 1065i32;
pub const D3D12_MESSAGE_ID_PROCESS_FRAME_INVALID_PARAMETERS: i32 = 1066i32;
pub const D3D12_MESSAGE_ID_COPY_INVALIDLAYOUT: i32 = 1067i32;
pub const D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION: i32 = 1068i32;
pub const D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION_POLICY: i32 = 1069i32;
pub const D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION: i32 = 1070i32;
pub const D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION: i32 = 1071i32;
pub const D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION_POLICY: i32 = 1072i32;
pub const D3D12_MESSAGE_ID_LIVE_PROTECTED_RESOURCE_SESSION: i32 = 1073i32;
pub const D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION: i32 = 1074i32;
pub const D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION_POLICY: i32 = 1075i32;
pub const D3D12_MESSAGE_ID_DESTROY_PROTECTED_RESOURCE_SESSION: i32 = 1076i32;
pub const D3D12_MESSAGE_ID_PROTECTED_RESOURCE_SESSION_UNSUPPORTED: i32 = 1077i32;
pub const D3D12_MESSAGE_ID_FENCE_INVALIDOPERATION: i32 = 1078i32;
pub const D3D12_MESSAGE_ID_CREATEQUERY_HEAP_COPY_QUEUE_TIMESTAMPS_NOT_SUPPORTED: i32 = 1079i32;
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_DEFERRED: i32 = 1080i32;
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMFIRSTUSE: i32 = 1081i32;
pub const D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMCLEAR: i32 = 1082i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEODECODERHEAP: i32 = 1083i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEODECODERHEAP: i32 = 1084i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEODECODERHEAP: i32 = 1085i32;
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDARG_RETURN: i32 = 1086i32;
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_OUTOFMEMORY_RETURN: i32 = 1087i32;
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDADDRESS: i32 = 1088i32;
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDHANDLE: i32 = 1089i32;
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_DEST: i32 = 1090i32;
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_MODE: i32 = 1091i32;
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_ALIGNMENT: i32 = 1092i32;
pub const D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_NOT_SUPPORTED: i32 = 1093i32;
pub const D3D12_MESSAGE_ID_SETVIEWINSTANCEMASK_INVALIDARGS: i32 = 1094i32;
pub const D3D12_MESSAGE_ID_VIEW_INSTANCING_UNSUPPORTED: i32 = 1095i32;
pub const D3D12_MESSAGE_ID_VIEW_INSTANCING_INVALIDARGS: i32 = 1096i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_DECODE_REFERENCE_ONLY_FLAG: i32 = 1097i32;
pub const D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_DECODE_REFERENCE_ONLY_FLAG: i32 = 1098i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_FAILURE: i32 = 1099i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_UNSUPPORTED: i32 = 1100i32;
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_INVALID_INPUT: i32 = 1101i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_DECODER_UNSUPPORTED: i32 = 1102i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_METADATA_ERROR: i32 = 1103i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VIEW_INSTANCING_VERTEX_SIZE_EXCEEDED: i32 = 1104i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RUNTIME_INTERNAL_ERROR: i32 = 1105i32;
pub const D3D12_MESSAGE_ID_NO_VIDEO_API_SUPPORT: i32 = 1106i32;
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_INVALID_INPUT: i32 = 1107i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_PROCESSOR_CAPS_FAILURE: i32 = 1108i32;
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_UNSUPPORTED_FORMAT: i32 = 1109i32;
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_FRAME_INVALID_ARGUMENT: i32 = 1110i32;
pub const D3D12_MESSAGE_ID_ENQUEUE_MAKE_RESIDENT_INVALID_FLAGS: i32 = 1111i32;
pub const D3D12_MESSAGE_ID_OPENEXISTINGHEAP_UNSUPPORTED: i32 = 1112i32;
pub const D3D12_MESSAGE_ID_VIDEO_PROCESS_FRAMES_INVALID_ARGUMENT: i32 = 1113i32;
pub const D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_UNSUPPORTED: i32 = 1114i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMANDRECORDER: i32 = 1115i32;
pub const D3D12_MESSAGE_ID_LIVE_COMMANDRECORDER: i32 = 1116i32;
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDRECORDER: i32 = 1117i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_VIDEO_NOT_SUPPORTED: i32 = 1118i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_SUPPORT_FLAGS: i32 = 1119i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_FLAGS: i32 = 1120i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_MORE_RECORDERS_THAN_LOGICAL_PROCESSORS: i32 = 1121i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMANDPOOL: i32 = 1122i32;
pub const D3D12_MESSAGE_ID_LIVE_COMMANDPOOL: i32 = 1123i32;
pub const D3D12_MESSAGE_ID_DESTROY_COMMANDPOOL: i32 = 1124i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_POOL_INVALID_FLAGS: i32 = 1125i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_VIDEO_NOT_SUPPORTED: i32 = 1126i32;
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_SUPPORT_FLAGS_MISMATCH: i32 = 1127i32;
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_CONTENTION: i32 = 1128i32;
pub const D3D12_MESSAGE_ID_COMMAND_RECORDER_USAGE_WITH_CREATECOMMANDLIST_COMMAND_LIST: i32 = 1129i32;
pub const D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_USAGE_WITH_CREATECOMMANDLIST1_COMMAND_LIST: i32 = 1130i32;
pub const D3D12_MESSAGE_ID_CANNOT_EXECUTE_EMPTY_COMMAND_LIST: i32 = 1131i32;
pub const D3D12_MESSAGE_ID_CANNOT_RESET_COMMAND_POOL_WITH_OPEN_COMMAND_LISTS: i32 = 1132i32;
pub const D3D12_MESSAGE_ID_CANNOT_USE_COMMAND_RECORDER_WITHOUT_CURRENT_TARGET: i32 = 1133i32;
pub const D3D12_MESSAGE_ID_CANNOT_CHANGE_COMMAND_RECORDER_TARGET_WHILE_RECORDING: i32 = 1134i32;
pub const D3D12_MESSAGE_ID_COMMAND_POOL_SYNC: i32 = 1135i32;
pub const D3D12_MESSAGE_ID_EVICT_UNDERFLOW: i32 = 1136i32;
pub const D3D12_MESSAGE_ID_CREATE_META_COMMAND: i32 = 1137i32;
pub const D3D12_MESSAGE_ID_LIVE_META_COMMAND: i32 = 1138i32;
pub const D3D12_MESSAGE_ID_DESTROY_META_COMMAND: i32 = 1139i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_DST_RESOURCE: i32 = 1140i32;
pub const D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_SRC_RESOURCE: i32 = 1141i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE: i32 = 1142i32;
pub const D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE: i32 = 1143i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_BUFFER: i32 = 1144i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_RESOURCE_DESC: i32 = 1145i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_UNSUPPORTED: i32 = 1146i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_DIMENSION: i32 = 1147i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_FLAGS: i32 = 1148i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_OFFSET: i32 = 1149i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_DIMENSION: i32 = 1150i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_FLAGS: i32 = 1151i32;
pub const D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_OUTOFMEMORY_RETURN: i32 = 1152i32;
pub const D3D12_MESSAGE_ID_CANNOT_CREATE_GRAPHICS_AND_VIDEO_COMMAND_RECORDER: i32 = 1153i32;
pub const D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_POSSIBLY_MISMATCHING_PROPERTIES: i32 = 1154i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE: i32 = 1155i32;
pub const D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INCOMPATIBLE_WITH_STRUCTURED_BUFFERS: i32 = 1156i32;
pub const D3D12_MESSAGE_ID_COMPUTE_ONLY_DEVICE_OPERATION_UNSUPPORTED: i32 = 1157i32;
pub const D3D12_MESSAGE_ID_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INVALID: i32 = 1158i32;
pub const D3D12_MESSAGE_ID_EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_INVALID: i32 = 1159i32;
pub const D3D12_MESSAGE_ID_COPY_RAYTRACING_ACCELERATION_STRUCTURE_INVALID: i32 = 1160i32;
pub const D3D12_MESSAGE_ID_DISPATCH_RAYS_INVALID: i32 = 1161i32;
pub const D3D12_MESSAGE_ID_GET_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO_INVALID: i32 = 1162i32;
pub const D3D12_MESSAGE_ID_CREATE_LIFETIMETRACKER: i32 = 1163i32;
pub const D3D12_MESSAGE_ID_LIVE_LIFETIMETRACKER: i32 = 1164i32;
pub const D3D12_MESSAGE_ID_DESTROY_LIFETIMETRACKER: i32 = 1165i32;
pub const D3D12_MESSAGE_ID_DESTROYOWNEDOBJECT_OBJECTNOTOWNED: i32 = 1166i32;
pub const D3D12_MESSAGE_ID_CREATE_TRACKEDWORKLOAD: i32 = 1167i32;
pub const D3D12_MESSAGE_ID_LIVE_TRACKEDWORKLOAD: i32 = 1168i32;
pub const D3D12_MESSAGE_ID_DESTROY_TRACKEDWORKLOAD: i32 = 1169i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_ERROR: i32 = 1170i32;
pub const D3D12_MESSAGE_ID_META_COMMAND_ID_INVALID: i32 = 1171i32;
pub const D3D12_MESSAGE_ID_META_COMMAND_UNSUPPORTED_PARAMS: i32 = 1172i32;
pub const D3D12_MESSAGE_ID_META_COMMAND_FAILED_ENUMERATION: i32 = 1173i32;
pub const D3D12_MESSAGE_ID_META_COMMAND_PARAMETER_SIZE_MISMATCH: i32 = 1174i32;
pub const D3D12_MESSAGE_ID_UNINITIALIZED_META_COMMAND: i32 = 1175i32;
pub const D3D12_MESSAGE_ID_META_COMMAND_INVALID_GPU_VIRTUAL_ADDRESS: i32 = 1176i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDLIST: i32 = 1177i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDLIST: i32 = 1178i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDLIST: i32 = 1179i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDQUEUE: i32 = 1180i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDQUEUE: i32 = 1181i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDQUEUE: i32 = 1182i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONESTIMATOR: i32 = 1183i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONESTIMATOR: i32 = 1184i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONESTIMATOR: i32 = 1185i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONVECTORHEAP: i32 = 1186i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONVECTORHEAP: i32 = 1187i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONVECTORHEAP: i32 = 1188i32;
pub const D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOADS: i32 = 1189i32;
pub const D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOAD_PAIRS: i32 = 1190i32;
pub const D3D12_MESSAGE_ID_OUT_OF_ORDER_TRACKED_WORKLOAD_PAIR: i32 = 1191i32;
pub const D3D12_MESSAGE_ID_CANNOT_ADD_TRACKED_WORKLOAD: i32 = 1192i32;
pub const D3D12_MESSAGE_ID_INCOMPLETE_TRACKED_WORKLOAD_PAIR: i32 = 1193i32;
pub const D3D12_MESSAGE_ID_CREATE_STATE_OBJECT_ERROR: i32 = 1194i32;
pub const D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_ERROR: i32 = 1195i32;
pub const D3D12_MESSAGE_ID_GET_SHADER_STACK_SIZE_ERROR: i32 = 1196i32;
pub const D3D12_MESSAGE_ID_GET_PIPELINE_STACK_SIZE_ERROR: i32 = 1197i32;
pub const D3D12_MESSAGE_ID_SET_PIPELINE_STACK_SIZE_ERROR: i32 = 1198i32;
pub const D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_SIZE_INVALID: i32 = 1199i32;
pub const D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_INVALID: i32 = 1200i32;
pub const D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_DRIVER_REPORTED_ISSUE: i32 = 1201i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_INVALID_RESOURCE_BARRIER: i32 = 1202i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_DISALLOWED_API_CALLED: i32 = 1203i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_NEST_RENDER_PASSES: i32 = 1204i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_END_WITHOUT_BEGIN: i32 = 1205i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_CLOSE_COMMAND_LIST: i32 = 1206i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_GPU_WORK_WHILE_SUSPENDED: i32 = 1207i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_SUSPEND_RESUME: i32 = 1208i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_NO_PRIOR_SUSPEND_WITHIN_EXECUTECOMMANDLISTS: i32 = 1209i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_NO_SUBSEQUENT_RESUME_WITHIN_EXECUTECOMMANDLISTS: i32 = 1210i32;
pub const D3D12_MESSAGE_ID_TRACKED_WORKLOAD_COMMAND_QUEUE_MISMATCH: i32 = 1211i32;
pub const D3D12_MESSAGE_ID_TRACKED_WORKLOAD_NOT_SUPPORTED: i32 = 1212i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_NO_ACCESS: i32 = 1213i32;
pub const D3D12_MESSAGE_ID_RENDER_PASS_UNSUPPORTED_RESOLVE: i32 = 1214i32;
pub const D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INVALID_RESOURCE_PTR: i32 = 1215i32;
pub const D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_SIGNAL: i32 = 1216i32;
pub const D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_WAIT: i32 = 1217i32;
pub const D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_ESTIMATOR_INVALID_ARGUMENT: i32 = 1218i32;
pub const D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT: i32 = 1219i32;
pub const D3D12_MESSAGE_ID_ESTIMATE_MOTION_INVALID_ARGUMENT: i32 = 1220i32;
pub const D3D12_MESSAGE_ID_RESOLVE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT: i32 = 1221i32;
pub const D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_HEAP_TYPE: i32 = 1222i32;
pub const D3D12_MESSAGE_ID_SET_BACKGROUND_PROCESSING_MODE_INVALID_ARGUMENT: i32 = 1223i32;
pub const D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE_FOR_FEATURE_LEVEL: i32 = 1224i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOEXTENSIONCOMMAND: i32 = 1225i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOEXTENSIONCOMMAND: i32 = 1226i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOEXTENSIONCOMMAND: i32 = 1227i32;
pub const D3D12_MESSAGE_ID_INVALID_VIDEO_EXTENSION_COMMAND_ID: i32 = 1228i32;
pub const D3D12_MESSAGE_ID_VIDEO_EXTENSION_COMMAND_INVALID_ARGUMENT: i32 = 1229i32;
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_UNIQUE_IN_DXIL_LIBRARY: i32 = 1230i32;
pub const D3D12_MESSAGE_ID_VARIABLE_SHADING_RATE_NOT_ALLOWED_WITH_TIR: i32 = 1231i32;
pub const D3D12_MESSAGE_ID_GEOMETRY_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: i32 = 1232i32;
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_SHADING_RATE: i32 = 1233i32;
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_SHADING_RATE_NOT_PERMITTED_BY_CAP: i32 = 1234i32;
pub const D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_COMBINER: i32 = 1235i32;
pub const D3D12_MESSAGE_ID_RSSETSHADINGRATEIMAGE_REQUIRES_TIER_2: i32 = 1236i32;
pub const D3D12_MESSAGE_ID_RSSETSHADINGRATE_REQUIRES_TIER_1: i32 = 1237i32;
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_FORMAT: i32 = 1238i32;
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_ARRAY_SIZE: i32 = 1239i32;
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_MIP_LEVEL: i32 = 1240i32;
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_COUNT: i32 = 1241i32;
pub const D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_QUALITY: i32 = 1242i32;
pub const D3D12_MESSAGE_ID_NON_RETAIL_SHADER_MODEL_WONT_VALIDATE: i32 = 1243i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_ROOT_SIGNATURE_MISMATCH: i32 = 1244i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_ROOT_SIGNATURE_MISMATCH: i32 = 1245i32;
pub const D3D12_MESSAGE_ID_ADD_TO_STATE_OBJECT_ERROR: i32 = 1246i32;
pub const D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION_INVALID_ARGUMENT: i32 = 1247i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_PSO_DESC_MISMATCH: i32 = 1248i32;
pub const D3D12_MESSAGE_ID_CREATEPIPELINESTATE_MS_INCOMPLETE_TYPE: i32 = 1249i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_NOT_MS_MISMATCH: i32 = 1250i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_NOT_PS_MISMATCH: i32 = 1251i32;
pub const D3D12_MESSAGE_ID_NONZERO_SAMPLER_FEEDBACK_MIP_REGION_WITH_INCOMPATIBLE_FORMAT: i32 = 1252i32;
pub const D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_SHADER_MISMATCH: i32 = 1253i32;
pub const D3D12_MESSAGE_ID_EMPTY_DISPATCH: i32 = 1254i32;
pub const D3D12_MESSAGE_ID_RESOURCE_FORMAT_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY: i32 = 1255i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_MIP_REGION: i32 = 1256i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_DIMENSION: i32 = 1257i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_COUNT: i32 = 1258i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_QUALITY: i32 = 1259i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_LAYOUT: i32 = 1260i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_REQUIRES_UNORDERED_ACCESS_FLAG: i32 = 1261i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_NULL_ARGUMENTS: i32 = 1262i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_UAV_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY: i32 = 1263i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_REQUIRES_FEEDBACK_MAP_FORMAT: i32 = 1264i32;
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_INVALIDSHADERBYTECODE: i32 = 1265i32;
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTOFMEMORY: i32 = 1266i32;
pub const D3D12_MESSAGE_ID_CREATEMESHSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: i32 = 1267i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_INVALID_FORMAT: i32 = 1268i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_INVALID_MIP_LEVEL_COUNT: i32 = 1269i32;
pub const D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_ARRAY_SIZE_MISMATCH: i32 = 1270i32;
pub const D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_MISMATCHING_TARGETED_RESOURCE: i32 = 1271i32;
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTPUTEXCEEDSMAXSIZE: i32 = 1272i32;
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_GROUPSHAREDEXCEEDSMAXSIZE: i32 = 1273i32;
pub const D3D12_MESSAGE_ID_VERTEX_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: i32 = 1274i32;
pub const D3D12_MESSAGE_ID_MESH_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE: i32 = 1275i32;
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_MISMATCHEDASMSPAYLOADSIZE: i32 = 1276i32;
pub const D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_UNBOUNDED_STATIC_DESCRIPTORS: i32 = 1277i32;
pub const D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_INVALIDSHADERBYTECODE: i32 = 1278i32;
pub const D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_OUTOFMEMORY: i32 = 1279i32;
pub const D3D12_MESSAGE_ID_CREATE_SHADERCACHESESSION: i32 = 1280i32;
pub const D3D12_MESSAGE_ID_LIVE_SHADERCACHESESSION: i32 = 1281i32;
pub const D3D12_MESSAGE_ID_DESTROY_SHADERCACHESESSION: i32 = 1282i32;
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_INVALIDARGS: i32 = 1283i32;
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_DISABLED: i32 = 1284i32;
pub const D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_ALREADYOPEN: i32 = 1285i32;
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_DEVELOPERMODE: i32 = 1286i32;
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_INVALIDFLAGS: i32 = 1287i32;
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_STATEALREADYSET: i32 = 1288i32;
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_IGNOREDFLAG: i32 = 1289i32;
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_ALREADYPRESENT: i32 = 1290i32;
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_HASHCOLLISION: i32 = 1291i32;
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_CACHEFULL: i32 = 1292i32;
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_FINDVALUE_NOTFOUND: i32 = 1293i32;
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_CORRUPT: i32 = 1294i32;
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_DISABLED: i32 = 1295i32;
pub const D3D12_MESSAGE_ID_OVERSIZED_DISPATCH: i32 = 1296i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODER: i32 = 1297i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODER: i32 = 1298i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODER: i32 = 1299i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEOENCODERHEAP: i32 = 1300i32;
pub const D3D12_MESSAGE_ID_LIVE_VIDEOENCODERHEAP: i32 = 1301i32;
pub const D3D12_MESSAGE_ID_DESTROY_VIDEOENCODERHEAP: i32 = 1302i32;
pub const D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG: i32 = 1303i32;
pub const D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG: i32 = 1304i32;
pub const D3D12_MESSAGE_ID_ENCODE_FRAME_INVALID_PARAMETERS: i32 = 1305i32;
pub const D3D12_MESSAGE_ID_ENCODE_FRAME_UNSUPPORTED_PARAMETERS: i32 = 1306i32;
pub const D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_INVALID_PARAMETERS: i32 = 1307i32;
pub const D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_UNSUPPORTED_PARAMETERS: i32 = 1308i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_INVALID_PARAMETERS: i32 = 1309i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_UNSUPPORTED_PARAMETERS: i32 = 1310i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_INVALID_PARAMETERS: i32 = 1311i32;
pub const D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_UNSUPPORTED_PARAMETERS: i32 = 1312i32;
pub const D3D12_MESSAGE_ID_CREATECOMMANDLIST_NULL_COMMANDALLOCATOR: i32 = 1313i32;
pub const D3D12_MESSAGE_ID_CLEAR_UNORDERED_ACCESS_VIEW_INVALID_DESCRIPTOR_HANDLE: i32 = 1314i32;
pub const D3D12_MESSAGE_ID_DESCRIPTOR_HEAP_NOT_SHADER_VISIBLE: i32 = 1315i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOP_WARNING: i32 = 1316i32;
pub const D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOPALPHA_WARNING: i32 = 1317i32;
pub const D3D12_MESSAGE_ID_WRITE_COMBINE_PERFORMANCE_WARNING: i32 = 1318i32;
pub const D3D12_MESSAGE_ID_RESOLVE_QUERY_INVALID_QUERY_STATE: i32 = 1319i32;
pub const D3D12_MESSAGE_ID_SETPRIVATEDATA_NO_ACCESS: i32 = 1320i32;
pub const D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_SAMPLER_MODE_MISMATCH: i32 = 1321i32;
pub const D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_UNSUPPORTED_BUFFER_WIDTH: i32 = 1322i32;
pub const D3D12_MESSAGE_ID_CREATEMESHSHADER_TOPOLOGY_MISMATCH: i32 = 1323i32;
pub const D3D12_MESSAGE_ID_VRS_SUM_COMBINER_REQUIRES_CAPABILITY: i32 = 1324i32;
pub const D3D12_MESSAGE_ID_SETTING_SHADING_RATE_FROM_MS_REQUIRES_CAPABILITY: i32 = 1325i32;
pub const D3D12_MESSAGE_ID_SHADERCACHESESSION_SHADERCACHEDELETE_NOTSUPPORTED: i32 = 1326i32;
pub const D3D12_MESSAGE_ID_SHADERCACHECONTROL_SHADERCACHECLEAR_NOTSUPPORTED: i32 = 1327i32;
pub const D3D12_MESSAGE_ID_D3D12_MESSAGES_END: i32 = 1328i32;
pub const D3D12_MESSAGE_SEVERITY_CORRUPTION: i32 = 0i32;
pub const D3D12_MESSAGE_SEVERITY_ERROR: i32 = 1i32;
pub const D3D12_MESSAGE_SEVERITY_WARNING: i32 = 2i32;
pub const D3D12_MESSAGE_SEVERITY_INFO: i32 = 3i32;
pub const D3D12_MESSAGE_SEVERITY_MESSAGE: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_META_COMMAND_DESC {
    pub Id: ::windows_sys::core::GUID,
    pub Name: super::super::Foundation::PWSTR,
    pub InitializationDirtyState: D3D12_GRAPHICS_STATES,
    pub ExecutionDirtyState: D3D12_GRAPHICS_STATES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_META_COMMAND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_META_COMMAND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_META_COMMAND_PARAMETER_DESC {
    pub Name: super::super::Foundation::PWSTR,
    pub Type: D3D12_META_COMMAND_PARAMETER_TYPE,
    pub Flags: D3D12_META_COMMAND_PARAMETER_FLAGS,
    pub RequiredResourceState: D3D12_RESOURCE_STATES,
    pub StructureOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_META_COMMAND_PARAMETER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_META_COMMAND_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_META_COMMAND_PARAMETER_FLAG_INPUT: u32 = 1u32;
pub const D3D12_META_COMMAND_PARAMETER_FLAG_OUTPUT: u32 = 2u32;
pub const D3D12_META_COMMAND_PARAMETER_STAGE_CREATION: i32 = 0i32;
pub const D3D12_META_COMMAND_PARAMETER_STAGE_INITIALIZATION: i32 = 1i32;
pub const D3D12_META_COMMAND_PARAMETER_STAGE_EXECUTION: i32 = 2i32;
pub const D3D12_META_COMMAND_PARAMETER_TYPE_FLOAT: i32 = 0i32;
pub const D3D12_META_COMMAND_PARAMETER_TYPE_UINT64: i32 = 1i32;
pub const D3D12_META_COMMAND_PARAMETER_TYPE_GPU_VIRTUAL_ADDRESS: i32 = 2i32;
pub const D3D12_META_COMMAND_PARAMETER_TYPE_CPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV: i32 = 3i32;
pub const D3D12_META_COMMAND_PARAMETER_TYPE_GPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV: i32 = 4i32;
pub const D3D12_MINOR_VERSION: u32 = 0u32;
pub const D3D12_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D12_MIN_DEPTH: f32 = 0f32;
pub const D3D12_MIN_FILTER_SHIFT: u32 = 4u32;
pub const D3D12_MIN_MAXANISOTROPY: u32 = 0u32;
pub const D3D12_MIP_FILTER_SHIFT: u32 = 0u32;
pub const D3D12_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
pub const D3D12_MIP_LOD_BIAS_MIN: f32 = -16f32;
pub const D3D12_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D12_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
#[repr(C)]
pub struct D3D12_MIP_REGION {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
impl ::core::marker::Copy for D3D12_MIP_REGION {}
impl ::core::clone::Clone for D3D12_MIP_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_NONE: u32 = 0u32;
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_ANY: u32 = 1u32;
pub const D3D12_MULTIPLE_FENCE_WAIT_FLAG_ALL: u32 = 0u32;
pub const D3D12_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
pub const D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_NONE: u32 = 0u32;
pub const D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_TILED_RESOURCE: u32 = 1u32;
#[repr(C)]
pub struct D3D12_NODE_MASK {
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_NODE_MASK {}
impl ::core::clone::Clone for D3D12_NODE_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967295u32;
pub const D3D12_OS_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967288u32;
#[repr(C)]
pub struct D3D12_PACKED_MIP_INFO {
    pub NumStandardMips: u8,
    pub NumPackedMips: u8,
    pub NumTilesForPackedMips: u32,
    pub StartTileIndexInOverallResource: u32,
}
impl ::core::marker::Copy for D3D12_PACKED_MIP_INFO {}
impl ::core::clone::Clone for D3D12_PACKED_MIP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_PACKED_TILE: u32 = 4294967295u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D12_PARAMETER_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub SemanticName: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub InterpolationMode: super::Direct3D::D3D_INTERPOLATION_MODE,
    pub Flags: super::Direct3D::D3D_PARAMETER_FLAGS,
    pub FirstInRegister: u32,
    pub FirstInComponent: u32,
    pub FirstOutRegister: u32,
    pub FirstOutComponent: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D12_PARAMETER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D12_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_PIPELINE_STATE_FLAG_NONE: u32 = 0u32;
pub const D3D12_PIPELINE_STATE_FLAG_TOOL_DEBUG: u32 = 1u32;
#[repr(C)]
pub struct D3D12_PIPELINE_STATE_STREAM_DESC {
    pub SizeInBytes: usize,
    pub pPipelineStateSubobjectStream: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3D12_PIPELINE_STATE_STREAM_DESC {}
impl ::core::clone::Clone for D3D12_PIPELINE_STATE_STREAM_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE: i32 = 0i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS: i32 = 1i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS: i32 = 2i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS: i32 = 3i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS: i32 = 4i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS: i32 = 5i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CS: i32 = 6i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT: i32 = 7i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND: i32 = 8i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK: i32 = 9i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER: i32 = 10i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL: i32 = 11i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT: i32 = 12i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE: i32 = 13i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY: i32 = 14i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS: i32 = 15i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT: i32 = 16i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC: i32 = 17i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK: i32 = 18i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO: i32 = 19i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS: i32 = 20i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1: i32 = 21i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VIEW_INSTANCING: i32 = 22i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_AS: i32 = 24i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MS: i32 = 25i32;
pub const D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MAX_VALID: i32 = 26i32;
pub const D3D12_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    pub Offset: u64,
    pub Footprint: D3D12_SUBRESOURCE_FOOTPRINT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_PREDICATION_OP_EQUAL_ZERO: i32 = 0i32;
pub const D3D12_PREDICATION_OP_NOT_EQUAL_ZERO: i32 = 1i32;
pub const D3D12_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_UNDEFINED: i32 = 0i32;
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT: i32 = 1i32;
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE: i32 = 2i32;
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE: i32 = 3i32;
pub const D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH: i32 = 4i32;
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_1: i32 = 1i32;
pub const D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_2: i32 = 2i32;
pub const D3D12_PROTECTED_RESOURCES_SESSION_HARDWARE_PROTECTED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1655703630, data2: 50958, data3: 19882, data4: [161, 9, 48, 255, 141, 90, 4, 130] };
#[repr(C)]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    pub NodeMask: u32,
    pub Flags: D3D12_PROTECTED_RESOURCE_SESSION_FLAGS,
}
impl ::core::marker::Copy for D3D12_PROTECTED_RESOURCE_SESSION_DESC {}
impl ::core::clone::Clone for D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    pub NodeMask: u32,
    pub Flags: D3D12_PROTECTED_RESOURCE_SESSION_FLAGS,
    pub ProtectionType: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {}
impl ::core::clone::Clone for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_PROTECTED_RESOURCE_SESSION_FLAG_NONE: u32 = 0u32;
pub const D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_NONE: u32 = 0u32;
pub const D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_SUPPORTED: u32 = 1u32;
pub const D3D12_PROTECTED_SESSION_STATUS_OK: i32 = 0i32;
pub const D3D12_PROTECTED_SESSION_STATUS_INVALID: i32 = 1i32;
pub const D3D12_PS_CS_UAV_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_PS_CS_UAV_REGISTER_COUNT: u32 = 8u32;
pub const D3D12_PS_CS_UAV_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D12_PS_CS_UAV_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
pub const D3D12_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
pub const D3D12_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
pub const D3D12_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
pub const D3D12_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
#[repr(C)]
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
    pub HSInvocations: u64,
    pub DSInvocations: u64,
    pub CSInvocations: u64,
}
impl ::core::marker::Copy for D3D12_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::clone::Clone for D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
    pub HSInvocations: u64,
    pub DSInvocations: u64,
    pub CSInvocations: u64,
    pub ASInvocations: u64,
    pub MSInvocations: u64,
    pub MSPrimitives: u64,
}
impl ::core::marker::Copy for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {}
impl ::core::clone::Clone for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
impl ::core::marker::Copy for D3D12_QUERY_DATA_SO_STATISTICS {}
impl ::core::clone::Clone for D3D12_QUERY_DATA_SO_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_QUERY_HEAP_DESC {
    pub Type: D3D12_QUERY_HEAP_TYPE,
    pub Count: u32,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D12_QUERY_HEAP_DESC {}
impl ::core::clone::Clone for D3D12_QUERY_HEAP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_QUERY_HEAP_TYPE_OCCLUSION: i32 = 0i32;
pub const D3D12_QUERY_HEAP_TYPE_TIMESTAMP: i32 = 1i32;
pub const D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS: i32 = 2i32;
pub const D3D12_QUERY_HEAP_TYPE_SO_STATISTICS: i32 = 3i32;
pub const D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS: i32 = 4i32;
pub const D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP: i32 = 5i32;
pub const D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS1: i32 = 7i32;
pub const D3D12_QUERY_TYPE_OCCLUSION: i32 = 0i32;
pub const D3D12_QUERY_TYPE_BINARY_OCCLUSION: i32 = 1i32;
pub const D3D12_QUERY_TYPE_TIMESTAMP: i32 = 2i32;
pub const D3D12_QUERY_TYPE_PIPELINE_STATISTICS: i32 = 3i32;
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM0: i32 = 4i32;
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM1: i32 = 5i32;
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM2: i32 = 6i32;
pub const D3D12_QUERY_TYPE_SO_STATISTICS_STREAM3: i32 = 7i32;
pub const D3D12_QUERY_TYPE_VIDEO_DECODE_STATISTICS: i32 = 8i32;
pub const D3D12_QUERY_TYPE_PIPELINE_STATISTICS1: i32 = 10i32;
#[repr(C)]
pub struct D3D12_RANGE {
    pub Begin: usize,
    pub End: usize,
}
impl ::core::marker::Copy for D3D12_RANGE {}
impl ::core::clone::Clone for D3D12_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RANGE_UINT64 {
    pub Begin: u64,
    pub End: u64,
}
impl ::core::marker::Copy for D3D12_RANGE_UINT64 {}
impl ::core::clone::Clone for D3D12_RANGE_UINT64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_RASTERIZER_DESC {
    pub FillMode: D3D12_FILL_MODE,
    pub CullMode: D3D12_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_RASTERIZER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16u32;
#[repr(C)]
pub struct D3D12_RAYTRACING_AABB {
    pub MinX: f32,
    pub MinY: f32,
    pub MinZ: f32,
    pub MaxX: f32,
    pub MaxY: f32,
    pub MaxZ: f32,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_AABB {}
impl ::core::clone::Clone for D3D12_RAYTRACING_AABB {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAYTRACING_AABB_BYTE_ALIGNMENT: u32 = 8u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_NONE: u32 = 0u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_UPDATE: u32 = 1u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_COMPACTION: u32 = 2u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_TRACE: u32 = 4u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_BUILD: u32 = 8u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_MINIMIZE_MEMORY: u32 = 16u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PERFORM_UPDATE: u32 = 32u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BYTE_ALIGNMENT: u32 = 256u32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_CLONE: i32 = 0i32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_COMPACT: i32 = 1i32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_VISUALIZATION_DECODE_FOR_TOOLS: i32 = 2i32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_SERIALIZE: i32 = 3i32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_DESERIALIZE: i32 = 4i32;
#[repr(C)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    pub CompactedSizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    pub CurrentSizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    pub DestBuffer: u64,
    pub InfoType: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    pub SerializedSizeInBytes: u64,
    pub NumBottomLevelAccelerationStructurePointers: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    pub DecodedSizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE: i32 = 0i32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION: i32 = 1i32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION: i32 = 2i32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE: i32 = 3i32;
#[repr(C)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    pub ResultDataMaxSizeInBytes: u64,
    pub ScratchDataSizeInBytes: u64,
    pub UpdateScratchDataSizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    pub Location: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {}
impl ::core::clone::Clone for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL: i32 = 0i32;
pub const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL: i32 = 1i32;
#[repr(C)]
pub struct D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    pub AABBCount: u64,
    pub AABBs: D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RAYTRACING_GEOMETRY_DESC {
    pub Type: D3D12_RAYTRACING_GEOMETRY_TYPE,
    pub Flags: D3D12_RAYTRACING_GEOMETRY_FLAGS,
    pub Anonymous: D3D12_RAYTRACING_GEOMETRY_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_RAYTRACING_GEOMETRY_DESC_0 {
    pub Triangles: D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC,
    pub AABBs: D3D12_RAYTRACING_GEOMETRY_AABBS_DESC,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_NONE: u32 = 0u32;
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_OPAQUE: u32 = 1u32;
pub const D3D12_RAYTRACING_GEOMETRY_FLAG_NO_DUPLICATE_ANYHIT_INVOCATION: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    pub Transform3x4: u64,
    pub IndexFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub VertexFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub IndexCount: u32,
    pub VertexCount: u32,
    pub IndexBuffer: u64,
    pub VertexBuffer: D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAYTRACING_GEOMETRY_TYPE_TRIANGLES: i32 = 0i32;
pub const D3D12_RAYTRACING_GEOMETRY_TYPE_PROCEDURAL_PRIMITIVE_AABBS: i32 = 1i32;
#[repr(C)]
pub struct D3D12_RAYTRACING_INSTANCE_DESC {
    pub Transform: [f32; 12],
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub AccelerationStructure: u64,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_INSTANCE_DESC {}
impl ::core::clone::Clone for D3D12_RAYTRACING_INSTANCE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAYTRACING_INSTANCE_DESCS_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D12_RAYTRACING_INSTANCE_FLAG_NONE: u32 = 0u32;
pub const D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_CULL_DISABLE: u32 = 1u32;
pub const D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_FRONT_COUNTERCLOCKWISE: u32 = 2u32;
pub const D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_OPAQUE: u32 = 4u32;
pub const D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_NON_OPAQUE: u32 = 8u32;
pub const D3D12_RAYTRACING_MAX_ATTRIBUTE_SIZE_IN_BYTES: u32 = 32u32;
pub const D3D12_RAYTRACING_MAX_DECLARABLE_TRACE_RECURSION_DEPTH: u32 = 31u32;
pub const D3D12_RAYTRACING_MAX_GEOMETRIES_PER_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: u32 = 16777216u32;
pub const D3D12_RAYTRACING_MAX_INSTANCES_PER_TOP_LEVEL_ACCELERATION_STRUCTURE: u32 = 16777216u32;
pub const D3D12_RAYTRACING_MAX_PRIMITIVES_PER_BOTTOM_LEVEL_ACCELERATION_STRUCTURE: u32 = 536870912u32;
pub const D3D12_RAYTRACING_MAX_RAY_GENERATION_SHADER_THREADS: u32 = 1073741824u32;
pub const D3D12_RAYTRACING_MAX_SHADER_RECORD_STRIDE: u32 = 4096u32;
#[repr(C)]
pub struct D3D12_RAYTRACING_PIPELINE_CONFIG {
    pub MaxTraceRecursionDepth: u32,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_PIPELINE_CONFIG {}
impl ::core::clone::Clone for D3D12_RAYTRACING_PIPELINE_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    pub MaxTraceRecursionDepth: u32,
    pub Flags: D3D12_RAYTRACING_PIPELINE_FLAGS,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_PIPELINE_CONFIG1 {}
impl ::core::clone::Clone for D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAYTRACING_PIPELINE_FLAG_NONE: u32 = 0u32;
pub const D3D12_RAYTRACING_PIPELINE_FLAG_SKIP_TRIANGLES: u32 = 256u32;
pub const D3D12_RAYTRACING_PIPELINE_FLAG_SKIP_PROCEDURAL_PRIMITIVES: u32 = 512u32;
#[repr(C)]
pub struct D3D12_RAYTRACING_SHADER_CONFIG {
    pub MaxPayloadSizeInBytes: u32,
    pub MaxAttributeSizeInBytes: u32,
}
impl ::core::marker::Copy for D3D12_RAYTRACING_SHADER_CONFIG {}
impl ::core::clone::Clone for D3D12_RAYTRACING_SHADER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT: u32 = 32u32;
pub const D3D12_RAYTRACING_SHADER_TABLE_BYTE_ALIGNMENT: u32 = 64u32;
pub const D3D12_RAYTRACING_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_RAYTRACING_TIER_1_0: i32 = 10i32;
pub const D3D12_RAYTRACING_TIER_1_1: i32 = 11i32;
pub const D3D12_RAYTRACING_TRANSFORM3X4_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D12_RAY_FLAG_NONE: u32 = 0u32;
pub const D3D12_RAY_FLAG_FORCE_OPAQUE: u32 = 1u32;
pub const D3D12_RAY_FLAG_FORCE_NON_OPAQUE: u32 = 2u32;
pub const D3D12_RAY_FLAG_ACCEPT_FIRST_HIT_AND_END_SEARCH: u32 = 4u32;
pub const D3D12_RAY_FLAG_SKIP_CLOSEST_HIT_SHADER: u32 = 8u32;
pub const D3D12_RAY_FLAG_CULL_BACK_FACING_TRIANGLES: u32 = 16u32;
pub const D3D12_RAY_FLAG_CULL_FRONT_FACING_TRIANGLES: u32 = 32u32;
pub const D3D12_RAY_FLAG_CULL_OPAQUE: u32 = 64u32;
pub const D3D12_RAY_FLAG_CULL_NON_OPAQUE: u32 = 128u32;
pub const D3D12_RAY_FLAG_SKIP_TRIANGLES: u32 = 256u32;
pub const D3D12_RAY_FLAG_SKIP_PROCEDURAL_PRIMITIVES: u32 = 512u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS {
    pub Type: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE,
    pub Anonymous: D3D12_RENDER_PASS_BEGINNING_ACCESS_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_PASS_BEGINNING_ACCESS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_PASS_BEGINNING_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
    pub Clear: D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_PASS_BEGINNING_ACCESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    pub ClearValue: D3D12_CLEAR_VALUE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_DISCARD: i32 = 0i32;
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE: i32 = 1i32;
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_CLEAR: i32 = 2i32;
pub const D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_NO_ACCESS: i32 = 3i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    pub cpuDescriptor: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub DepthBeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub StencilBeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub DepthEndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
    pub StencilEndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS {
    pub Type: D3D12_RENDER_PASS_ENDING_ACCESS_TYPE,
    pub Anonymous: D3D12_RENDER_PASS_ENDING_ACCESS_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_RENDER_PASS_ENDING_ACCESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub union D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    pub Resolve: D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_RENDER_PASS_ENDING_ACCESS_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    pub pSrcResource: ID3D12Resource,
    pub pDstResource: ID3D12Resource,
    pub SubresourceCount: u32,
    pub pSubresourceParameters: *mut D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ResolveMode: D3D12_RESOLVE_MODE,
    pub PreserveResolveSource: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    pub SrcSubresource: u32,
    pub DstSubresource: u32,
    pub DstX: u32,
    pub DstY: u32,
    pub SrcRect: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_DISCARD: i32 = 0i32;
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE: i32 = 1i32;
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_RESOLVE: i32 = 2i32;
pub const D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_NO_ACCESS: i32 = 3i32;
pub const D3D12_RENDER_PASS_FLAG_NONE: u32 = 0u32;
pub const D3D12_RENDER_PASS_FLAG_ALLOW_UAV_WRITES: u32 = 1u32;
pub const D3D12_RENDER_PASS_FLAG_SUSPENDING_PASS: u32 = 2u32;
pub const D3D12_RENDER_PASS_FLAG_RESUMING_PASS: u32 = 4u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    pub cpuDescriptor: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub BeginningAccess: D3D12_RENDER_PASS_BEGINNING_ACCESS,
    pub EndingAccess: D3D12_RENDER_PASS_ENDING_ACCESS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D12_RENDER_PASS_RENDER_TARGET_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RENDER_PASS_TIER_0: i32 = 0i32;
pub const D3D12_RENDER_PASS_TIER_1: i32 = 1i32;
pub const D3D12_RENDER_PASS_TIER_2: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub LogicOpEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D12_BLEND,
    pub DestBlend: D3D12_BLEND,
    pub BlendOp: D3D12_BLEND_OP,
    pub SrcBlendAlpha: D3D12_BLEND,
    pub DestBlendAlpha: D3D12_BLEND,
    pub BlendOpAlpha: D3D12_BLEND_OP,
    pub LogicOp: D3D12_LOGIC_OP,
    pub RenderTargetWriteMask: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_RENDER_TARGET_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_RENDER_TARGET_BLEND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RENDER_TARGET_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_RTV_DIMENSION,
    pub Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_TARGET_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_TARGET_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_RTV,
    pub Texture1D: D3D12_TEX1D_RTV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D12_TEX2D_RTV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D12_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D12_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RENDER_TARGET_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RENDER_TARGET_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D12_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
pub const D3D12_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D12_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D12_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D12_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D12_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 16384u32;
pub const D3D12_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
pub const D3D12_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D12_REQ_MAXANISOTROPY: u32 = 16u32;
pub const D3D12_REQ_MIP_LEVELS: u32 = 15u32;
pub const D3D12_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
pub const D3D12_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D12_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 16384u32;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: u32 = 128u32;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: f32 = 0.25f32;
pub const D3D12_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: u32 = 2048u32;
pub const D3D12_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: u32 = 20u32;
pub const D3D12_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D12_REQ_SUBRESOURCES: u32 = 30720u32;
pub const D3D12_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
pub const D3D12_REQ_TEXTURE1D_U_DIMENSION: u32 = 16384u32;
pub const D3D12_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
pub const D3D12_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 16384u32;
pub const D3D12_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
pub const D3D12_REQ_TEXTURECUBE_DIMENSION: u32 = 16384u32;
pub const D3D12_RESIDENCY_FLAG_NONE: u32 = 0u32;
pub const D3D12_RESIDENCY_FLAG_DENY_OVERBUDGET: u32 = 1u32;
pub const D3D12_RESIDENCY_PRIORITY_MINIMUM: i32 = 671088640i32;
pub const D3D12_RESIDENCY_PRIORITY_LOW: i32 = 1342177280i32;
pub const D3D12_RESIDENCY_PRIORITY_NORMAL: i32 = 2013265920i32;
pub const D3D12_RESIDENCY_PRIORITY_HIGH: i32 = -1610547200i32;
pub const D3D12_RESIDENCY_PRIORITY_MAXIMUM: i32 = -939524096i32;
pub const D3D12_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
pub const D3D12_RESOLVE_MODE_DECOMPRESS: i32 = 0i32;
pub const D3D12_RESOLVE_MODE_MIN: i32 = 1i32;
pub const D3D12_RESOLVE_MODE_MAX: i32 = 2i32;
pub const D3D12_RESOLVE_MODE_AVERAGE: i32 = 3i32;
pub const D3D12_RESOLVE_MODE_ENCODE_SAMPLER_FEEDBACK: i32 = 4i32;
pub const D3D12_RESOLVE_MODE_DECODE_SAMPLER_FEEDBACK: i32 = 5i32;
#[repr(C)]
pub struct D3D12_RESOURCE_ALIASING_BARRIER {
    pub pResourceBefore: ID3D12Resource,
    pub pResourceAfter: ID3D12Resource,
}
impl ::core::marker::Copy for D3D12_RESOURCE_ALIASING_BARRIER {}
impl ::core::clone::Clone for D3D12_RESOURCE_ALIASING_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RESOURCE_ALLOCATION_INFO {
    pub SizeInBytes: u64,
    pub Alignment: u64,
}
impl ::core::marker::Copy for D3D12_RESOURCE_ALLOCATION_INFO {}
impl ::core::clone::Clone for D3D12_RESOURCE_ALLOCATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RESOURCE_ALLOCATION_INFO1 {
    pub Offset: u64,
    pub Alignment: u64,
    pub SizeInBytes: u64,
}
impl ::core::marker::Copy for D3D12_RESOURCE_ALLOCATION_INFO1 {}
impl ::core::clone::Clone for D3D12_RESOURCE_ALLOCATION_INFO1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RESOURCE_BARRIER {
    pub Type: D3D12_RESOURCE_BARRIER_TYPE,
    pub Flags: D3D12_RESOURCE_BARRIER_FLAGS,
    pub Anonymous: D3D12_RESOURCE_BARRIER_0,
}
impl ::core::marker::Copy for D3D12_RESOURCE_BARRIER {}
impl ::core::clone::Clone for D3D12_RESOURCE_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D12_RESOURCE_BARRIER_0 {
    pub Transition: D3D12_RESOURCE_TRANSITION_BARRIER,
    pub Aliasing: D3D12_RESOURCE_ALIASING_BARRIER,
    pub UAV: D3D12_RESOURCE_UAV_BARRIER,
}
impl ::core::marker::Copy for D3D12_RESOURCE_BARRIER_0 {}
impl ::core::clone::Clone for D3D12_RESOURCE_BARRIER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: u32 = 4294967295u32;
pub const D3D12_RESOURCE_BARRIER_FLAG_NONE: u32 = 0u32;
pub const D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY: u32 = 1u32;
pub const D3D12_RESOURCE_BARRIER_FLAG_END_ONLY: u32 = 2u32;
pub const D3D12_RESOURCE_BARRIER_TYPE_TRANSITION: i32 = 0i32;
pub const D3D12_RESOURCE_BARRIER_TYPE_ALIASING: i32 = 1i32;
pub const D3D12_RESOURCE_BARRIER_TYPE_UAV: i32 = 2i32;
pub const D3D12_RESOURCE_BINDING_TIER_1: i32 = 1i32;
pub const D3D12_RESOURCE_BINDING_TIER_2: i32 = 2i32;
pub const D3D12_RESOURCE_BINDING_TIER_3: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RESOURCE_DESC {
    pub Dimension: D3D12_RESOURCE_DIMENSION,
    pub Alignment: u64,
    pub Width: u64,
    pub Height: u32,
    pub DepthOrArraySize: u16,
    pub MipLevels: u16,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Layout: D3D12_TEXTURE_LAYOUT,
    pub Flags: D3D12_RESOURCE_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RESOURCE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RESOURCE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RESOURCE_DESC1 {
    pub Dimension: D3D12_RESOURCE_DIMENSION,
    pub Alignment: u64,
    pub Width: u64,
    pub Height: u32,
    pub DepthOrArraySize: u16,
    pub MipLevels: u16,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Layout: D3D12_TEXTURE_LAYOUT,
    pub Flags: D3D12_RESOURCE_FLAGS,
    pub SamplerFeedbackMipRegion: D3D12_MIP_REGION,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RESOURCE_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RESOURCE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RESOURCE_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D12_RESOURCE_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D12_RESOURCE_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D12_RESOURCE_DIMENSION_TEXTURE2D: i32 = 3i32;
pub const D3D12_RESOURCE_DIMENSION_TEXTURE3D: i32 = 4i32;
pub const D3D12_RESOURCE_FLAG_NONE: u32 = 0u32;
pub const D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET: u32 = 1u32;
pub const D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL: u32 = 2u32;
pub const D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS: u32 = 4u32;
pub const D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE: u32 = 8u32;
pub const D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER: u32 = 16u32;
pub const D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS: u32 = 32u32;
pub const D3D12_RESOURCE_FLAG_VIDEO_DECODE_REFERENCE_ONLY: u32 = 64u32;
pub const D3D12_RESOURCE_FLAG_VIDEO_ENCODE_REFERENCE_ONLY: u32 = 128u32;
pub const D3D12_RESOURCE_HEAP_TIER_1: i32 = 1i32;
pub const D3D12_RESOURCE_HEAP_TIER_2: i32 = 2i32;
pub const D3D12_RESOURCE_STATE_COMMON: u32 = 0u32;
pub const D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER: u32 = 1u32;
pub const D3D12_RESOURCE_STATE_INDEX_BUFFER: u32 = 2u32;
pub const D3D12_RESOURCE_STATE_RENDER_TARGET: u32 = 4u32;
pub const D3D12_RESOURCE_STATE_UNORDERED_ACCESS: u32 = 8u32;
pub const D3D12_RESOURCE_STATE_DEPTH_WRITE: u32 = 16u32;
pub const D3D12_RESOURCE_STATE_DEPTH_READ: u32 = 32u32;
pub const D3D12_RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE: u32 = 64u32;
pub const D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE: u32 = 128u32;
pub const D3D12_RESOURCE_STATE_STREAM_OUT: u32 = 256u32;
pub const D3D12_RESOURCE_STATE_INDIRECT_ARGUMENT: u32 = 512u32;
pub const D3D12_RESOURCE_STATE_COPY_DEST: u32 = 1024u32;
pub const D3D12_RESOURCE_STATE_COPY_SOURCE: u32 = 2048u32;
pub const D3D12_RESOURCE_STATE_RESOLVE_DEST: u32 = 4096u32;
pub const D3D12_RESOURCE_STATE_RESOLVE_SOURCE: u32 = 8192u32;
pub const D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE: u32 = 4194304u32;
pub const D3D12_RESOURCE_STATE_SHADING_RATE_SOURCE: u32 = 16777216u32;
pub const D3D12_RESOURCE_STATE_GENERIC_READ: u32 = 2755u32;
pub const D3D12_RESOURCE_STATE_ALL_SHADER_RESOURCE: u32 = 192u32;
pub const D3D12_RESOURCE_STATE_PRESENT: u32 = 0u32;
pub const D3D12_RESOURCE_STATE_PREDICATION: u32 = 512u32;
pub const D3D12_RESOURCE_STATE_VIDEO_DECODE_READ: u32 = 65536u32;
pub const D3D12_RESOURCE_STATE_VIDEO_DECODE_WRITE: u32 = 131072u32;
pub const D3D12_RESOURCE_STATE_VIDEO_PROCESS_READ: u32 = 262144u32;
pub const D3D12_RESOURCE_STATE_VIDEO_PROCESS_WRITE: u32 = 524288u32;
pub const D3D12_RESOURCE_STATE_VIDEO_ENCODE_READ: u32 = 2097152u32;
pub const D3D12_RESOURCE_STATE_VIDEO_ENCODE_WRITE: u32 = 8388608u32;
#[repr(C)]
pub struct D3D12_RESOURCE_TRANSITION_BARRIER {
    pub pResource: ID3D12Resource,
    pub Subresource: u32,
    pub StateBefore: D3D12_RESOURCE_STATES,
    pub StateAfter: D3D12_RESOURCE_STATES,
}
impl ::core::marker::Copy for D3D12_RESOURCE_TRANSITION_BARRIER {}
impl ::core::clone::Clone for D3D12_RESOURCE_TRANSITION_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_RESOURCE_UAV_BARRIER {
    pub pResource: ID3D12Resource,
}
impl ::core::marker::Copy for D3D12_RESOURCE_UAV_BARRIER {}
impl ::core::clone::Clone for D3D12_RESOURCE_UAV_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_RLDO_NONE: i32 = 0i32;
pub const D3D12_RLDO_SUMMARY: i32 = 1i32;
pub const D3D12_RLDO_DETAIL: i32 = 2i32;
pub const D3D12_RLDO_IGNORE_INTERNAL: i32 = 4i32;
#[repr(C)]
pub struct D3D12_ROOT_CONSTANTS {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Num32BitValues: u32,
}
impl ::core::marker::Copy for D3D12_ROOT_CONSTANTS {}
impl ::core::clone::Clone for D3D12_ROOT_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_ROOT_DESCRIPTOR {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
}
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_ROOT_DESCRIPTOR1 {
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub Flags: D3D12_ROOT_DESCRIPTOR_FLAGS,
}
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR1 {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_ROOT_DESCRIPTOR_FLAG_NONE: u32 = 0u32;
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_VOLATILE: u32 = 2u32;
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE: u32 = 4u32;
pub const D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC: u32 = 8u32;
#[repr(C)]
pub struct D3D12_ROOT_DESCRIPTOR_TABLE {
    pub NumDescriptorRanges: u32,
    pub pDescriptorRanges: *mut D3D12_DESCRIPTOR_RANGE,
}
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR_TABLE {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_ROOT_DESCRIPTOR_TABLE1 {
    pub NumDescriptorRanges: u32,
    pub pDescriptorRanges: *mut D3D12_DESCRIPTOR_RANGE1,
}
impl ::core::marker::Copy for D3D12_ROOT_DESCRIPTOR_TABLE1 {}
impl ::core::clone::Clone for D3D12_ROOT_DESCRIPTOR_TABLE1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_ROOT_PARAMETER {
    pub ParameterType: D3D12_ROOT_PARAMETER_TYPE,
    pub Anonymous: D3D12_ROOT_PARAMETER_0,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D12_ROOT_PARAMETER_0 {
    pub DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE,
    pub Constants: D3D12_ROOT_CONSTANTS,
    pub Descriptor: D3D12_ROOT_DESCRIPTOR,
}
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER_0 {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_ROOT_PARAMETER1 {
    pub ParameterType: D3D12_ROOT_PARAMETER_TYPE,
    pub Anonymous: D3D12_ROOT_PARAMETER1_0,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER1 {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D12_ROOT_PARAMETER1_0 {
    pub DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE1,
    pub Constants: D3D12_ROOT_CONSTANTS,
    pub Descriptor: D3D12_ROOT_DESCRIPTOR1,
}
impl ::core::marker::Copy for D3D12_ROOT_PARAMETER1_0 {}
impl ::core::clone::Clone for D3D12_ROOT_PARAMETER1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE: i32 = 0i32;
pub const D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS: i32 = 1i32;
pub const D3D12_ROOT_PARAMETER_TYPE_CBV: i32 = 2i32;
pub const D3D12_ROOT_PARAMETER_TYPE_SRV: i32 = 3i32;
pub const D3D12_ROOT_PARAMETER_TYPE_UAV: i32 = 4i32;
#[repr(C)]
pub struct D3D12_ROOT_SIGNATURE_DESC {
    pub NumParameters: u32,
    pub pParameters: *mut D3D12_ROOT_PARAMETER,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *mut D3D12_STATIC_SAMPLER_DESC,
    pub Flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
impl ::core::marker::Copy for D3D12_ROOT_SIGNATURE_DESC {}
impl ::core::clone::Clone for D3D12_ROOT_SIGNATURE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_ROOT_SIGNATURE_DESC1 {
    pub NumParameters: u32,
    pub pParameters: *mut D3D12_ROOT_PARAMETER1,
    pub NumStaticSamplers: u32,
    pub pStaticSamplers: *mut D3D12_STATIC_SAMPLER_DESC,
    pub Flags: D3D12_ROOT_SIGNATURE_FLAGS,
}
impl ::core::marker::Copy for D3D12_ROOT_SIGNATURE_DESC1 {}
impl ::core::clone::Clone for D3D12_ROOT_SIGNATURE_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_ROOT_SIGNATURE_FLAG_NONE: u32 = 0u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT: u32 = 1u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS: u32 = 2u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS: u32 = 4u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS: u32 = 8u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS: u32 = 16u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS: u32 = 32u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT: u32 = 64u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE: u32 = 128u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS: u32 = 256u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS: u32 = 512u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED: u32 = 1024u32;
pub const D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED: u32 = 2048u32;
pub const D3D12_RS_SET_SHADING_RATE_COMBINER_COUNT: u32 = 2u32;
pub const D3D12_RTV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D12_RTV_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D12_RTV_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D12_RTV_DIMENSION_TEXTURE1DARRAY: i32 = 3i32;
pub const D3D12_RTV_DIMENSION_TEXTURE2D: i32 = 4i32;
pub const D3D12_RTV_DIMENSION_TEXTURE2DARRAY: i32 = 5i32;
pub const D3D12_RTV_DIMENSION_TEXTURE2DMS: i32 = 6i32;
pub const D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY: i32 = 7i32;
pub const D3D12_RTV_DIMENSION_TEXTURE3D: i32 = 8i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_RT_FORMAT_ARRAY {
    pub RTFormats: [super::Dxgi::Common::DXGI_FORMAT; 8],
    pub NumRenderTargets: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_RT_FORMAT_ARRAY {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_RT_FORMAT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_SAMPLER_DESC {
    pub Filter: D3D12_FILTER,
    pub AddressU: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D12_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl ::core::marker::Copy for D3D12_SAMPLER_DESC {}
impl ::core::clone::Clone for D3D12_SAMPLER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SAMPLER_FEEDBACK_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_SAMPLER_FEEDBACK_TIER_0_9: i32 = 90i32;
pub const D3D12_SAMPLER_FEEDBACK_TIER_1_0: i32 = 100i32;
#[repr(C)]
pub struct D3D12_SAMPLE_POSITION {
    pub X: i8,
    pub Y: i8,
}
impl ::core::marker::Copy for D3D12_SAMPLE_POSITION {}
impl ::core::clone::Clone for D3D12_SAMPLE_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SDK_VERSION: u32 = 5u32;
#[repr(C)]
pub struct D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    pub DriverOpaqueGUID: ::windows_sys::core::GUID,
    pub DriverOpaqueVersioningData: [u8; 16],
}
impl ::core::marker::Copy for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {}
impl ::core::clone::Clone for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SERIALIZED_DATA_RAYTRACING_ACCELERATION_STRUCTURE: i32 = 0i32;
#[repr(C)]
pub struct D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    pub DriverMatchingIdentifier: D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER,
    pub SerializedSizeInBytesIncludingHeader: u64,
    pub DeserializedSizeInBytes: u64,
    pub NumBottomLevelAccelerationStructurePointersAfterHeader: u64,
}
impl ::core::marker::Copy for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {}
impl ::core::clone::Clone for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D12_SHADER_BUFFER_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D12_SHADER_BUFFER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D12_SHADER_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_SHADER_BYTECODE {
    pub pShaderBytecode: *mut ::core::ffi::c_void,
    pub BytecodeLength: usize,
}
impl ::core::marker::Copy for D3D12_SHADER_BYTECODE {}
impl ::core::clone::Clone for D3D12_SHADER_BYTECODE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_DISABLE: u32 = 1u32;
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_ENABLE: u32 = 2u32;
pub const D3D12_SHADER_CACHE_CONTROL_FLAG_CLEAR: u32 = 4u32;
pub const D3D12_SHADER_CACHE_FLAG_NONE: u32 = 0u32;
pub const D3D12_SHADER_CACHE_FLAG_DRIVER_VERSIONED: u32 = 1u32;
pub const D3D12_SHADER_CACHE_FLAG_USE_WORKING_DIR: u32 = 2u32;
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_D3D_CACHE_FOR_DRIVER: u32 = 1u32;
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_D3D_CONVERSIONS: u32 = 2u32;
pub const D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_DRIVER_MANAGED: u32 = 4u32;
pub const D3D12_SHADER_CACHE_KIND_FLAG_APPLICATION_MANAGED: u32 = 8u32;
pub const D3D12_SHADER_CACHE_MODE_MEMORY: i32 = 0i32;
pub const D3D12_SHADER_CACHE_MODE_DISK: i32 = 1i32;
#[repr(C)]
pub struct D3D12_SHADER_CACHE_SESSION_DESC {
    pub Identifier: ::windows_sys::core::GUID,
    pub Mode: D3D12_SHADER_CACHE_MODE,
    pub Flags: D3D12_SHADER_CACHE_FLAGS,
    pub MaximumInMemoryCacheSizeBytes: u32,
    pub MaximumInMemoryCacheEntries: u32,
    pub MaximumValueFileSizeBytes: u32,
    pub Version: u64,
}
impl ::core::marker::Copy for D3D12_SHADER_CACHE_SESSION_DESC {}
impl ::core::clone::Clone for D3D12_SHADER_CACHE_SESSION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SHADER_CACHE_SUPPORT_NONE: u32 = 0u32;
pub const D3D12_SHADER_CACHE_SUPPORT_SINGLE_PSO: u32 = 1u32;
pub const D3D12_SHADER_CACHE_SUPPORT_LIBRARY: u32 = 2u32;
pub const D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE: u32 = 4u32;
pub const D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE: u32 = 8u32;
pub const D3D12_SHADER_CACHE_SUPPORT_DRIVER_MANAGED_CACHE: u32 = 16u32;
pub const D3D12_SHADER_CACHE_SUPPORT_SHADER_CONTROL_CLEAR: u32 = 32u32;
pub const D3D12_SHADER_CACHE_SUPPORT_SHADER_SESSION_DELETE: u32 = 64u32;
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_0: i32 = 0i32;
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_1: i32 = 1i32;
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_2: i32 = 2i32;
pub const D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_3: i32 = 3i32;
pub const D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_0: i32 = 4i32;
pub const D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_1: i32 = 5i32;
pub const D3D12_SHADER_COMPONENT_MAPPING_ALWAYS_SET_BIT_AVOIDING_ZEROMEM_MISTAKES: u32 = 4096u32;
pub const D3D12_SHADER_COMPONENT_MAPPING_MASK: u32 = 7u32;
pub const D3D12_SHADER_COMPONENT_MAPPING_SHIFT: u32 = 3u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D12_SHADER_DESC {
    pub Version: u32,
    pub Creator: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InputParameters: u32,
    pub OutputParameters: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub CutInstructionCount: u32,
    pub EmitInstructionCount: u32,
    pub GSOutputTopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
    pub InputPrimitive: super::Direct3D::D3D_PRIMITIVE,
    pub PatchConstantParameters: u32,
    pub cGSInstanceCount: u32,
    pub cControlPoints: u32,
    pub HSOutputPrimitive: super::Direct3D::D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    pub HSPartitioning: super::Direct3D::D3D_TESSELLATOR_PARTITIONING,
    pub TessellatorDomain: super::Direct3D::D3D_TESSELLATOR_DOMAIN,
    pub cBarrierInstructions: u32,
    pub cInterlockedInstructions: u32,
    pub cTextureStoreInstructions: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D12_SHADER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D12_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES: u32 = 32u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D12_SHADER_INPUT_BIND_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::Direct3D::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
    pub Space: u32,
    pub uID: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D12_SHADER_INPUT_BIND_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D12_SHADER_INPUT_BIND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SHADER_MAJOR_VERSION: u32 = 5u32;
pub const D3D12_SHADER_MAX_INSTANCES: u32 = 65535u32;
pub const D3D12_SHADER_MAX_INTERFACES: u32 = 253u32;
pub const D3D12_SHADER_MAX_INTERFACE_CALL_SITES: u32 = 4096u32;
pub const D3D12_SHADER_MAX_TYPES: u32 = 65535u32;
pub const D3D12_SHADER_MINOR_VERSION: u32 = 1u32;
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_NONE: u32 = 0u32;
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_10_BIT: u32 = 1u32;
pub const D3D12_SHADER_MIN_PRECISION_SUPPORT_16_BIT: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_SRV_DIMENSION,
    pub Shader4ComponentMapping: u32,
    pub Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_SHADER_RESOURCE_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_SHADER_RESOURCE_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_SRV,
    pub Texture1D: D3D12_TEX1D_SRV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D12_TEX2D_SRV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D12_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D12_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D12_TEX3D_SRV,
    pub TextureCube: D3D12_TEXCUBE_SRV,
    pub TextureCubeArray: D3D12_TEXCUBE_ARRAY_SRV,
    pub RaytracingAccelerationStructure: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_SHADER_RESOURCE_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D12_SHADER_TYPE_DESC {
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
    pub Name: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D12_SHADER_TYPE_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D12_SHADER_TYPE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_SHADER_VARIABLE_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut ::core::ffi::c_void,
    pub StartTexture: u32,
    pub TextureSize: u32,
    pub StartSampler: u32,
    pub SamplerSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_SHADER_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_SHADER_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SHVER_PIXEL_SHADER: i32 = 0i32;
pub const D3D12_SHVER_VERTEX_SHADER: i32 = 1i32;
pub const D3D12_SHVER_GEOMETRY_SHADER: i32 = 2i32;
pub const D3D12_SHVER_HULL_SHADER: i32 = 3i32;
pub const D3D12_SHVER_DOMAIN_SHADER: i32 = 4i32;
pub const D3D12_SHVER_COMPUTE_SHADER: i32 = 5i32;
pub const D3D12_SHVER_RESERVED0: i32 = 65520i32;
pub const D3D12_SHADER_VISIBILITY_ALL: i32 = 0i32;
pub const D3D12_SHADER_VISIBILITY_VERTEX: i32 = 1i32;
pub const D3D12_SHADER_VISIBILITY_HULL: i32 = 2i32;
pub const D3D12_SHADER_VISIBILITY_DOMAIN: i32 = 3i32;
pub const D3D12_SHADER_VISIBILITY_GEOMETRY: i32 = 4i32;
pub const D3D12_SHADER_VISIBILITY_PIXEL: i32 = 5i32;
pub const D3D12_SHADER_VISIBILITY_AMPLIFICATION: i32 = 6i32;
pub const D3D12_SHADER_VISIBILITY_MESH: i32 = 7i32;
pub const D3D12_SHADING_RATE_1X1: i32 = 0i32;
pub const D3D12_SHADING_RATE_1X2: i32 = 1i32;
pub const D3D12_SHADING_RATE_2X1: i32 = 4i32;
pub const D3D12_SHADING_RATE_2X2: i32 = 5i32;
pub const D3D12_SHADING_RATE_2X4: i32 = 6i32;
pub const D3D12_SHADING_RATE_4X2: i32 = 9i32;
pub const D3D12_SHADING_RATE_4X4: i32 = 10i32;
pub const D3D12_SHADING_RATE_COMBINER_PASSTHROUGH: i32 = 0i32;
pub const D3D12_SHADING_RATE_COMBINER_OVERRIDE: i32 = 1i32;
pub const D3D12_SHADING_RATE_COMBINER_MIN: i32 = 2i32;
pub const D3D12_SHADING_RATE_COMBINER_MAX: i32 = 3i32;
pub const D3D12_SHADING_RATE_COMBINER_SUM: i32 = 4i32;
pub const D3D12_SHADING_RATE_VALID_MASK: u32 = 3u32;
pub const D3D12_SHADING_RATE_X_AXIS_SHIFT: u32 = 2u32;
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0: i32 = 0i32;
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_1: i32 = 1i32;
pub const D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_2: i32 = 2i32;
pub const D3D12_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D12_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D12_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
    pub Stream: u32,
    pub MinPrecision: super::Direct3D::D3D_MIN_PRECISION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D12_SIGNATURE_PARAMETER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D12_SIGNATURE_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D12_SMALL_MSAA_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 65536u32;
pub const D3D12_SMALL_RESOURCE_PLACEMENT_ALIGNMENT: u32 = 4096u32;
pub const D3D12_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D12_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 512u32;
pub const D3D12_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D12_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_SO_DECLARATION_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_SO_DECLARATION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SO_NO_RASTERIZED_STREAM: u32 = 4294967295u32;
pub const D3D12_SO_OUTPUT_COMPONENT_COUNT: u32 = 128u32;
pub const D3D12_SO_STREAM_COUNT: u32 = 4u32;
pub const D3D12_SPEC_DATE_DAY: u32 = 14u32;
pub const D3D12_SPEC_DATE_MONTH: u32 = 11u32;
pub const D3D12_SPEC_DATE_YEAR: u32 = 2014u32;
pub const D3D12_SPEC_VERSION: f64 = 1.16f64;
pub const D3D12_SRGB_GAMMA: f32 = 2.2f32;
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
pub const D3D12_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
pub const D3D12_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
pub const D3D12_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
pub const D3D12_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
pub const D3D12_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
pub const D3D12_SRV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D12_SRV_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D12_SRV_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D12_SRV_DIMENSION_TEXTURE1DARRAY: i32 = 3i32;
pub const D3D12_SRV_DIMENSION_TEXTURE2D: i32 = 4i32;
pub const D3D12_SRV_DIMENSION_TEXTURE2DARRAY: i32 = 5i32;
pub const D3D12_SRV_DIMENSION_TEXTURE2DMS: i32 = 6i32;
pub const D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY: i32 = 7i32;
pub const D3D12_SRV_DIMENSION_TEXTURE3D: i32 = 8i32;
pub const D3D12_SRV_DIMENSION_TEXTURECUBE: i32 = 9i32;
pub const D3D12_SRV_DIMENSION_TEXTURECUBEARRAY: i32 = 10i32;
pub const D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE: i32 = 11i32;
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
pub const D3D12_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
pub const D3D12_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D12_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D12_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D12_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
pub const D3D12_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
#[repr(C)]
pub struct D3D12_STATE_OBJECT_CONFIG {
    pub Flags: D3D12_STATE_OBJECT_FLAGS,
}
impl ::core::marker::Copy for D3D12_STATE_OBJECT_CONFIG {}
impl ::core::clone::Clone for D3D12_STATE_OBJECT_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_STATE_OBJECT_DESC {
    pub Type: D3D12_STATE_OBJECT_TYPE,
    pub NumSubobjects: u32,
    pub pSubobjects: *mut D3D12_STATE_SUBOBJECT,
}
impl ::core::marker::Copy for D3D12_STATE_OBJECT_DESC {}
impl ::core::clone::Clone for D3D12_STATE_OBJECT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_STATE_OBJECT_FLAG_NONE: u32 = 0u32;
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_LOCAL_DEPENDENCIES_ON_EXTERNAL_DEFINITIONS: u32 = 1u32;
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_EXTERNAL_DEPENDENCIES_ON_LOCAL_DEFINITIONS: u32 = 2u32;
pub const D3D12_STATE_OBJECT_FLAG_ALLOW_STATE_OBJECT_ADDITIONS: u32 = 4u32;
pub const D3D12_STATE_OBJECT_TYPE_COLLECTION: i32 = 0i32;
pub const D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE: i32 = 3i32;
#[repr(C)]
pub struct D3D12_STATE_SUBOBJECT {
    pub Type: D3D12_STATE_SUBOBJECT_TYPE,
    pub pDesc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3D12_STATE_SUBOBJECT {}
impl ::core::clone::Clone for D3D12_STATE_SUBOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_STATE_SUBOBJECT_TYPE_STATE_OBJECT_CONFIG: i32 = 0i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_GLOBAL_ROOT_SIGNATURE: i32 = 1i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_LOCAL_ROOT_SIGNATURE: i32 = 2i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_NODE_MASK: i32 = 3i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_DXIL_LIBRARY: i32 = 5i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_EXISTING_COLLECTION: i32 = 6i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_SUBOBJECT_TO_EXPORTS_ASSOCIATION: i32 = 7i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION: i32 = 8i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_SHADER_CONFIG: i32 = 9i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG: i32 = 10i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_HIT_GROUP: i32 = 11i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG1: i32 = 12i32;
pub const D3D12_STATE_SUBOBJECT_TYPE_MAX_VALID: i32 = 13i32;
pub const D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK: i32 = 0i32;
pub const D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK: i32 = 1i32;
pub const D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE: i32 = 2i32;
#[repr(C)]
pub struct D3D12_STATIC_SAMPLER_DESC {
    pub Filter: D3D12_FILTER,
    pub AddressU: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D12_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D12_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D12_COMPARISON_FUNC,
    pub BorderColor: D3D12_STATIC_BORDER_COLOR,
    pub MinLOD: f32,
    pub MaxLOD: f32,
    pub ShaderRegister: u32,
    pub RegisterSpace: u32,
    pub ShaderVisibility: D3D12_SHADER_VISIBILITY,
}
impl ::core::marker::Copy for D3D12_STATIC_SAMPLER_DESC {}
impl ::core::clone::Clone for D3D12_STATIC_SAMPLER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_STENCIL_OP_KEEP: i32 = 1i32;
pub const D3D12_STENCIL_OP_ZERO: i32 = 2i32;
pub const D3D12_STENCIL_OP_REPLACE: i32 = 3i32;
pub const D3D12_STENCIL_OP_INCR_SAT: i32 = 4i32;
pub const D3D12_STENCIL_OP_DECR_SAT: i32 = 5i32;
pub const D3D12_STENCIL_OP_INVERT: i32 = 6i32;
pub const D3D12_STENCIL_OP_INCR: i32 = 7i32;
pub const D3D12_STENCIL_OP_DECR: i32 = 8i32;
#[repr(C)]
pub struct D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u64,
    pub BufferFilledSizeLocation: u64,
}
impl ::core::marker::Copy for D3D12_STREAM_OUTPUT_BUFFER_VIEW {}
impl ::core::clone::Clone for D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_STREAM_OUTPUT_DESC {
    pub pSODeclaration: *mut D3D12_SO_DECLARATION_ENTRY,
    pub NumEntries: u32,
    pub pBufferStrides: *mut u32,
    pub NumStrides: u32,
    pub RasterizedStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_STREAM_OUTPUT_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_STREAM_OUTPUT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    pub pSubobjectToAssociate: *mut D3D12_STATE_SUBOBJECT,
    pub NumExports: u32,
    pub pExports: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[repr(C)]
pub struct D3D12_SUBRESOURCE_DATA {
    pub pData: *mut ::core::ffi::c_void,
    pub RowPitch: isize,
    pub SlicePitch: isize,
}
impl ::core::marker::Copy for D3D12_SUBRESOURCE_DATA {}
impl ::core::clone::Clone for D3D12_SUBRESOURCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_SUBRESOURCE_FOOTPRINT {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub RowPitch: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_SUBRESOURCE_FOOTPRINT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_SUBRESOURCE_FOOTPRINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_SUBRESOURCE_INFO {
    pub Offset: u64,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl ::core::marker::Copy for D3D12_SUBRESOURCE_INFO {}
impl ::core::clone::Clone for D3D12_SUBRESOURCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_SUBRESOURCE_RANGE_UINT64 {
    pub Subresource: u32,
    pub Range: D3D12_RANGE_UINT64,
}
impl ::core::marker::Copy for D3D12_SUBRESOURCE_RANGE_UINT64 {}
impl ::core::clone::Clone for D3D12_SUBRESOURCE_RANGE_UINT64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_SUBRESOURCE_TILING {
    pub WidthInTiles: u32,
    pub HeightInTiles: u16,
    pub DepthInTiles: u16,
    pub StartTileIndexInOverallResource: u32,
}
impl ::core::marker::Copy for D3D12_SUBRESOURCE_TILING {}
impl ::core::clone::Clone for D3D12_SUBRESOURCE_TILING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_END: u32 = 4294967295u32;
pub const D3D12_SYSTEM_RESERVED_REGISTER_SPACE_VALUES_START: u32 = 4294967280u32;
pub const D3D12_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D12_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D12_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63u32;
pub const D3D12_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D12_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2u32;
pub const D3D12_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1u32;
pub const D3D12_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1u32;
#[repr(C)]
pub struct D3D12_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D12_TEX1D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D12_TEX1D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX1D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D12_TEX1D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_ARRAY_UAV {}
impl ::core::clone::Clone for D3D12_TEX1D_ARRAY_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX1D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_DSV {}
impl ::core::clone::Clone for D3D12_TEX1D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX1D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_RTV {}
impl ::core::clone::Clone for D3D12_TEX1D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX1D_SRV {}
impl ::core::clone::Clone for D3D12_TEX1D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX1D_UAV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX1D_UAV {}
impl ::core::clone::Clone for D3D12_TEX1D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_ARRAY_DSV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_ARRAY_RTV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_ARRAY_SRV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_DSV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_RTV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D12_TEX2DMS_SRV {}
impl ::core::clone::Clone for D3D12_TEX2DMS_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D12_TEX2D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D12_TEX2D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX2D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D12_TEX2D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_ARRAY_UAV {}
impl ::core::clone::Clone for D3D12_TEX2D_ARRAY_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_DSV {}
impl ::core::clone::Clone for D3D12_TEX2D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2D_RTV {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_RTV {}
impl ::core::clone::Clone for D3D12_TEX2D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub PlaneSlice: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX2D_SRV {}
impl ::core::clone::Clone for D3D12_TEX2D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX2D_UAV {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D12_TEX2D_UAV {}
impl ::core::clone::Clone for D3D12_TEX2D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl ::core::marker::Copy for D3D12_TEX3D_RTV {}
impl ::core::clone::Clone for D3D12_TEX3D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEX3D_SRV {}
impl ::core::clone::Clone for D3D12_TEX3D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl ::core::marker::Copy for D3D12_TEX3D_UAV {}
impl ::core::clone::Clone for D3D12_TEX3D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEXCUBE_ARRAY_SRV {}
impl ::core::clone::Clone for D3D12_TEXCUBE_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub ResourceMinLODClamp: f32,
}
impl ::core::marker::Copy for D3D12_TEXCUBE_SRV {}
impl ::core::clone::Clone for D3D12_TEXCUBE_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub const D3D12_TEXTURE_ADDRESS_MODE_WRAP: i32 = 1i32;
pub const D3D12_TEXTURE_ADDRESS_MODE_MIRROR: i32 = 2i32;
pub const D3D12_TEXTURE_ADDRESS_MODE_CLAMP: i32 = 3i32;
pub const D3D12_TEXTURE_ADDRESS_MODE_BORDER: i32 = 4i32;
pub const D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE: i32 = 5i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_TEXTURE_COPY_LOCATION {
    pub pResource: ID3D12Resource,
    pub Type: D3D12_TEXTURE_COPY_TYPE,
    pub Anonymous: D3D12_TEXTURE_COPY_LOCATION_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_TEXTURE_COPY_LOCATION {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_TEXTURE_COPY_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_TEXTURE_COPY_LOCATION_0 {
    pub PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT,
    pub SubresourceIndex: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_TEXTURE_COPY_LOCATION_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_TEXTURE_COPY_LOCATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX: i32 = 0i32;
pub const D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT: i32 = 1i32;
pub const D3D12_TEXTURE_DATA_PITCH_ALIGNMENT: u32 = 256u32;
pub const D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT: u32 = 512u32;
pub const D3D12_TEXTURE_LAYOUT_UNKNOWN: i32 = 0i32;
pub const D3D12_TEXTURE_LAYOUT_ROW_MAJOR: i32 = 1i32;
pub const D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE: i32 = 2i32;
pub const D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE: i32 = 3i32;
pub const D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_TILED_RESOURCES_TIER_1: i32 = 1i32;
pub const D3D12_TILED_RESOURCES_TIER_2: i32 = 2i32;
pub const D3D12_TILED_RESOURCES_TIER_3: i32 = 3i32;
pub const D3D12_TILED_RESOURCES_TIER_4: i32 = 4i32;
#[repr(C)]
pub struct D3D12_TILED_RESOURCE_COORDINATE {
    pub X: u32,
    pub Y: u32,
    pub Z: u32,
    pub Subresource: u32,
}
impl ::core::marker::Copy for D3D12_TILED_RESOURCE_COORDINATE {}
impl ::core::clone::Clone for D3D12_TILED_RESOURCE_COORDINATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_TILED_RESOURCE_TILE_SIZE_IN_BYTES: u32 = 65536u32;
pub const D3D12_TILE_COPY_FLAG_NONE: u32 = 0u32;
pub const D3D12_TILE_COPY_FLAG_NO_HAZARD: u32 = 1u32;
pub const D3D12_TILE_COPY_FLAG_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE: u32 = 2u32;
pub const D3D12_TILE_COPY_FLAG_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER: u32 = 4u32;
pub const D3D12_TILE_MAPPING_FLAG_NONE: u32 = 0u32;
pub const D3D12_TILE_MAPPING_FLAG_NO_HAZARD: u32 = 1u32;
pub const D3D12_TILE_RANGE_FLAG_NONE: i32 = 0i32;
pub const D3D12_TILE_RANGE_FLAG_NULL: i32 = 1i32;
pub const D3D12_TILE_RANGE_FLAG_SKIP: i32 = 2i32;
pub const D3D12_TILE_RANGE_FLAG_REUSE_SINGLE_TILE: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_TILE_REGION_SIZE {
    pub NumTiles: u32,
    pub UseBox: super::super::Foundation::BOOL,
    pub Width: u32,
    pub Height: u16,
    pub Depth: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_TILE_REGION_SIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_TILE_REGION_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_TILE_SHAPE {
    pub WidthInTexels: u32,
    pub HeightInTexels: u32,
    pub DepthInTexels: u32,
}
impl ::core::marker::Copy for D3D12_TILE_SHAPE {}
impl ::core::clone::Clone for D3D12_TILE_SHAPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_TRACKED_WORKLOAD_MAX_INSTANCES: u32 = 32u32;
pub const D3D12_UAV_COUNTER_PLACEMENT_ALIGNMENT: u32 = 4096u32;
pub const D3D12_UAV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D12_UAV_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D12_UAV_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D12_UAV_DIMENSION_TEXTURE1DARRAY: i32 = 3i32;
pub const D3D12_UAV_DIMENSION_TEXTURE2D: i32 = 4i32;
pub const D3D12_UAV_DIMENSION_TEXTURE2DARRAY: i32 = 5i32;
pub const D3D12_UAV_DIMENSION_TEXTURE3D: i32 = 8i32;
pub const D3D12_UAV_SLOT_COUNT: u32 = 64u32;
pub const D3D12_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D12_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D12_UAV_DIMENSION,
    pub Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_UNORDERED_ACCESS_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_UNORDERED_ACCESS_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D12_BUFFER_UAV,
    pub Texture1D: D3D12_TEX1D_UAV,
    pub Texture1DArray: D3D12_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D12_TEX2D_UAV,
    pub Texture2DArray: D3D12_TEX2D_ARRAY_UAV,
    pub Texture3D: D3D12_TEX3D_UAV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_VARIABLE_SHADING_RATE_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_VARIABLE_SHADING_RATE_TIER_1: i32 = 1i32;
pub const D3D12_VARIABLE_SHADING_RATE_TIER_2: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    pub Version: D3D12_DRED_VERSION,
    pub Anonymous: D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {
    pub Dred_1_0: D3D12_DEVICE_REMOVED_EXTENDED_DATA,
    pub Dred_1_1: D3D12_DEVICE_REMOVED_EXTENDED_DATA1,
    pub Dred_1_2: D3D12_DEVICE_REMOVED_EXTENDED_DATA2,
    pub Dred_1_3: D3D12_DEVICE_REMOVED_EXTENDED_DATA3,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    pub Version: D3D_ROOT_SIGNATURE_VERSION,
    pub Anonymous: D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0,
}
impl ::core::marker::Copy for D3D12_VERSIONED_ROOT_SIGNATURE_DESC {}
impl ::core::clone::Clone for D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
    pub Desc_1_0: D3D12_ROOT_SIGNATURE_DESC,
    pub Desc_1_1: D3D12_ROOT_SIGNATURE_DESC1,
}
impl ::core::marker::Copy for D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {}
impl ::core::clone::Clone for D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_VERTEX_BUFFER_VIEW {
    pub BufferLocation: u64,
    pub SizeInBytes: u32,
    pub StrideInBytes: u32,
}
impl ::core::marker::Copy for D3D12_VERTEX_BUFFER_VIEW {}
impl ::core::clone::Clone for D3D12_VERTEX_BUFFER_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_VIDEO_DECODE_MAX_ARGUMENTS: u32 = 10u32;
pub const D3D12_VIDEO_DECODE_MAX_HISTOGRAM_COMPONENTS: u32 = 4u32;
pub const D3D12_VIDEO_DECODE_MIN_BITSTREAM_OFFSET_ALIGNMENT: u32 = 256u32;
pub const D3D12_VIDEO_DECODE_MIN_HISTOGRAM_OFFSET_ALIGNMENT: u32 = 256u32;
pub const D3D12_VIDEO_DECODE_STATUS_MACROBLOCKS_AFFECTED_UNKNOWN: u32 = 4294967295u32;
pub const D3D12_VIDEO_PROCESS_MAX_FILTERS: u32 = 32u32;
pub const D3D12_VIDEO_PROCESS_STEREO_VIEWS: u32 = 2u32;
#[repr(C)]
pub struct D3D12_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
impl ::core::marker::Copy for D3D12_VIEWPORT {}
impl ::core::clone::Clone for D3D12_VIEWPORT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
pub const D3D12_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
pub const D3D12_VIEWPORT_BOUNDS_MAX: u32 = 32767u32;
pub const D3D12_VIEWPORT_BOUNDS_MIN: i32 = -32768i32;
#[repr(C)]
pub struct D3D12_VIEW_INSTANCE_LOCATION {
    pub ViewportArrayIndex: u32,
    pub RenderTargetArrayIndex: u32,
}
impl ::core::marker::Copy for D3D12_VIEW_INSTANCE_LOCATION {}
impl ::core::clone::Clone for D3D12_VIEW_INSTANCE_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D12_VIEW_INSTANCING_DESC {
    pub ViewInstanceCount: u32,
    pub pViewInstanceLocations: *mut D3D12_VIEW_INSTANCE_LOCATION,
    pub Flags: D3D12_VIEW_INSTANCING_FLAGS,
}
impl ::core::marker::Copy for D3D12_VIEW_INSTANCING_DESC {}
impl ::core::clone::Clone for D3D12_VIEW_INSTANCING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D12_VIEW_INSTANCING_FLAG_NONE: u32 = 0u32;
pub const D3D12_VIEW_INSTANCING_FLAG_ENABLE_VIEW_INSTANCE_MASKING: u32 = 1u32;
pub const D3D12_VIEW_INSTANCING_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_VIEW_INSTANCING_TIER_1: i32 = 1i32;
pub const D3D12_VIEW_INSTANCING_TIER_2: i32 = 2i32;
pub const D3D12_VIEW_INSTANCING_TIER_3: i32 = 3i32;
pub const D3D12_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D12_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D12_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D12_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D12_WAVE_MMA_TIER_NOT_SUPPORTED: i32 = 0i32;
pub const D3D12_WAVE_MMA_TIER_1_0: i32 = 10i32;
pub const D3D12_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
pub const D3D12_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D12_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_DEFAULT: i32 = 0i32;
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_IN: i32 = 1i32;
pub const D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_OUT: i32 = 2i32;
#[repr(C)]
pub struct D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    pub Dest: u64,
    pub Value: u32,
}
impl ::core::marker::Copy for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {}
impl ::core::clone::Clone for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D_ROOT_SIGNATURE_VERSION_1: i32 = 1i32;
pub const D3D_ROOT_SIGNATURE_VERSION_1_0: i32 = 1i32;
pub const D3D_ROOT_SIGNATURE_VERSION_1_1: i32 = 2i32;
pub const D3D_SHADER_MODEL_5_1: i32 = 81i32;
pub const D3D_SHADER_MODEL_6_0: i32 = 96i32;
pub const D3D_SHADER_MODEL_6_1: i32 = 97i32;
pub const D3D_SHADER_MODEL_6_2: i32 = 98i32;
pub const D3D_SHADER_MODEL_6_3: i32 = 99i32;
pub const D3D_SHADER_MODEL_6_4: i32 = 100i32;
pub const D3D_SHADER_MODEL_6_5: i32 = 101i32;
pub const D3D_SHADER_MODEL_6_6: i32 = 102i32;
pub const D3D_SHADER_MODEL_6_7: i32 = 103i32;
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_DESCRIPTOR_HEAP_RESOURCE: u32 = 268435456u32;
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_GROUP_SHARED: u32 = 8388608u32;
pub const D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_TYPED_RESOURCE: u32 = 4194304u32;
pub const D3D_SHADER_REQUIRES_BARYCENTRICS: u32 = 131072u32;
pub const D3D_SHADER_REQUIRES_DERIVATIVES_IN_MESH_AND_AMPLIFICATION_SHADERS: u32 = 16777216u32;
pub const D3D_SHADER_REQUIRES_INNER_COVERAGE: u32 = 1024u32;
pub const D3D_SHADER_REQUIRES_INT64_OPS: u32 = 32768u32;
pub const D3D_SHADER_REQUIRES_NATIVE_16BIT_OPS: u32 = 262144u32;
pub const D3D_SHADER_REQUIRES_RAYTRACING_TIER_1_1: u32 = 1048576u32;
pub const D3D_SHADER_REQUIRES_RESOURCE_DESCRIPTOR_HEAP_INDEXING: u32 = 33554432u32;
pub const D3D_SHADER_REQUIRES_ROVS: u32 = 4096u32;
pub const D3D_SHADER_REQUIRES_SAMPLER_DESCRIPTOR_HEAP_INDEXING: u32 = 67108864u32;
pub const D3D_SHADER_REQUIRES_SAMPLER_FEEDBACK: u32 = 2097152u32;
pub const D3D_SHADER_REQUIRES_SHADING_RATE: u32 = 524288u32;
pub const D3D_SHADER_REQUIRES_STENCIL_REF: u32 = 512u32;
pub const D3D_SHADER_REQUIRES_TYPED_UAV_LOAD_ADDITIONAL_FORMATS: u32 = 2048u32;
pub const D3D_SHADER_REQUIRES_VIEWPORT_AND_RT_ARRAY_INDEX_FROM_ANY_SHADER_FEEDING_RASTERIZER: u32 = 8192u32;
pub const D3D_SHADER_REQUIRES_VIEW_ID: u32 = 65536u32;
pub const D3D_SHADER_REQUIRES_WAVE_MMA: u32 = 134217728u32;
pub const D3D_SHADER_REQUIRES_WAVE_OPS: u32 = 16384u32;
pub const DXGI_DEBUG_D3D12: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3478759820,
    data2: 43344,
    data3: 17190,
    data4: [145, 239, 155, 186, 161, 123, 253, 149],
};
#[repr(transparent)]
pub struct ID3D12CommandAllocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12CommandAllocator {}
impl ::core::clone::Clone for ID3D12CommandAllocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12CommandList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12CommandList {}
impl ::core::clone::Clone for ID3D12CommandList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12CommandQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12CommandQueue {}
impl ::core::clone::Clone for ID3D12CommandQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12CommandSignature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12CommandSignature {}
impl ::core::clone::Clone for ID3D12CommandSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Debug(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Debug {}
impl ::core::clone::Clone for ID3D12Debug {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Debug1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Debug1 {}
impl ::core::clone::Clone for ID3D12Debug1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Debug2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Debug2 {}
impl ::core::clone::Clone for ID3D12Debug2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Debug3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Debug3 {}
impl ::core::clone::Clone for ID3D12Debug3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Debug4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Debug4 {}
impl ::core::clone::Clone for ID3D12Debug4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Debug5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Debug5 {}
impl ::core::clone::Clone for ID3D12Debug5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DebugCommandList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DebugCommandList {}
impl ::core::clone::Clone for ID3D12DebugCommandList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DebugCommandList1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DebugCommandList1 {}
impl ::core::clone::Clone for ID3D12DebugCommandList1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DebugCommandList2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DebugCommandList2 {}
impl ::core::clone::Clone for ID3D12DebugCommandList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DebugCommandQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DebugCommandQueue {}
impl ::core::clone::Clone for ID3D12DebugCommandQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DebugDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DebugDevice {}
impl ::core::clone::Clone for ID3D12DebugDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DebugDevice1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DebugDevice1 {}
impl ::core::clone::Clone for ID3D12DebugDevice1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DebugDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DebugDevice2 {}
impl ::core::clone::Clone for ID3D12DebugDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DescriptorHeap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DescriptorHeap {}
impl ::core::clone::Clone for ID3D12DescriptorHeap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device {}
impl ::core::clone::Clone for ID3D12Device {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device1 {}
impl ::core::clone::Clone for ID3D12Device1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device2 {}
impl ::core::clone::Clone for ID3D12Device2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device3 {}
impl ::core::clone::Clone for ID3D12Device3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device4 {}
impl ::core::clone::Clone for ID3D12Device4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device5 {}
impl ::core::clone::Clone for ID3D12Device5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device6 {}
impl ::core::clone::Clone for ID3D12Device6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device7 {}
impl ::core::clone::Clone for ID3D12Device7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device8 {}
impl ::core::clone::Clone for ID3D12Device8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Device9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Device9 {}
impl ::core::clone::Clone for ID3D12Device9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DeviceChild(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DeviceChild {}
impl ::core::clone::Clone for ID3D12DeviceChild {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DeviceRemovedExtendedData {}
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedData1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DeviceRemovedExtendedData1 {}
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedData1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedData2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DeviceRemovedExtendedData2 {}
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedData2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedDataSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DeviceRemovedExtendedDataSettings {}
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedDataSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12DeviceRemovedExtendedDataSettings1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12DeviceRemovedExtendedDataSettings1 {}
impl ::core::clone::Clone for ID3D12DeviceRemovedExtendedDataSettings1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Fence(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Fence {}
impl ::core::clone::Clone for ID3D12Fence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Fence1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Fence1 {}
impl ::core::clone::Clone for ID3D12Fence1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12FunctionParameterReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12FunctionParameterReflection {}
impl ::core::clone::Clone for ID3D12FunctionParameterReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12FunctionReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12FunctionReflection {}
impl ::core::clone::Clone for ID3D12FunctionReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12GraphicsCommandList {}
impl ::core::clone::Clone for ID3D12GraphicsCommandList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12GraphicsCommandList1 {}
impl ::core::clone::Clone for ID3D12GraphicsCommandList1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12GraphicsCommandList2 {}
impl ::core::clone::Clone for ID3D12GraphicsCommandList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12GraphicsCommandList3 {}
impl ::core::clone::Clone for ID3D12GraphicsCommandList3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12GraphicsCommandList4 {}
impl ::core::clone::Clone for ID3D12GraphicsCommandList4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12GraphicsCommandList5 {}
impl ::core::clone::Clone for ID3D12GraphicsCommandList5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12GraphicsCommandList6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12GraphicsCommandList6 {}
impl ::core::clone::Clone for ID3D12GraphicsCommandList6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Heap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Heap {}
impl ::core::clone::Clone for ID3D12Heap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Heap1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Heap1 {}
impl ::core::clone::Clone for ID3D12Heap1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12InfoQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12InfoQueue {}
impl ::core::clone::Clone for ID3D12InfoQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12InfoQueue1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12InfoQueue1 {}
impl ::core::clone::Clone for ID3D12InfoQueue1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12LibraryReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12LibraryReflection {}
impl ::core::clone::Clone for ID3D12LibraryReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12LifetimeOwner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12LifetimeOwner {}
impl ::core::clone::Clone for ID3D12LifetimeOwner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12LifetimeTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12LifetimeTracker {}
impl ::core::clone::Clone for ID3D12LifetimeTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12MetaCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12MetaCommand {}
impl ::core::clone::Clone for ID3D12MetaCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Object(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Object {}
impl ::core::clone::Clone for ID3D12Object {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Pageable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Pageable {}
impl ::core::clone::Clone for ID3D12Pageable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12PipelineLibrary(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12PipelineLibrary {}
impl ::core::clone::Clone for ID3D12PipelineLibrary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12PipelineLibrary1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12PipelineLibrary1 {}
impl ::core::clone::Clone for ID3D12PipelineLibrary1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12PipelineState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12PipelineState {}
impl ::core::clone::Clone for ID3D12PipelineState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12ProtectedResourceSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12ProtectedResourceSession {}
impl ::core::clone::Clone for ID3D12ProtectedResourceSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12ProtectedResourceSession1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12ProtectedResourceSession1 {}
impl ::core::clone::Clone for ID3D12ProtectedResourceSession1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12ProtectedSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12ProtectedSession {}
impl ::core::clone::Clone for ID3D12ProtectedSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12QueryHeap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12QueryHeap {}
impl ::core::clone::Clone for ID3D12QueryHeap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Resource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Resource {}
impl ::core::clone::Clone for ID3D12Resource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Resource1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Resource1 {}
impl ::core::clone::Clone for ID3D12Resource1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Resource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Resource2 {}
impl ::core::clone::Clone for ID3D12Resource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12RootSignature(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12RootSignature {}
impl ::core::clone::Clone for ID3D12RootSignature {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12RootSignatureDeserializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12RootSignatureDeserializer {}
impl ::core::clone::Clone for ID3D12RootSignatureDeserializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12SDKConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12SDKConfiguration {}
impl ::core::clone::Clone for ID3D12SDKConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12ShaderCacheSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12ShaderCacheSession {}
impl ::core::clone::Clone for ID3D12ShaderCacheSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12ShaderReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12ShaderReflection {}
impl ::core::clone::Clone for ID3D12ShaderReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12ShaderReflectionConstantBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12ShaderReflectionConstantBuffer {}
impl ::core::clone::Clone for ID3D12ShaderReflectionConstantBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12ShaderReflectionType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12ShaderReflectionType {}
impl ::core::clone::Clone for ID3D12ShaderReflectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12ShaderReflectionVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12ShaderReflectionVariable {}
impl ::core::clone::Clone for ID3D12ShaderReflectionVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12SharingContract(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12SharingContract {}
impl ::core::clone::Clone for ID3D12SharingContract {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12StateObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12StateObject {}
impl ::core::clone::Clone for ID3D12StateObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12StateObjectProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12StateObjectProperties {}
impl ::core::clone::Clone for ID3D12StateObjectProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12SwapChainAssistant(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12SwapChainAssistant {}
impl ::core::clone::Clone for ID3D12SwapChainAssistant {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12Tools(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12Tools {}
impl ::core::clone::Clone for ID3D12Tools {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D12VersionedRootSignatureDeserializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D12VersionedRootSignatureDeserializer {}
impl ::core::clone::Clone for ID3D12VersionedRootSignatureDeserializer {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LUID_DEFINED: u32 = 1u32;
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_CREATE_DEVICE = unsafe extern "system" fn(param0: ::windows_sys::core::IUnknown, param1: super::Direct3D::D3D_FEATURE_LEVEL, param2: *const ::windows_sys::core::GUID, param3: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PFN_D3D12_CREATE_ROOT_SIGNATURE_DESERIALIZER = unsafe extern "system" fn(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows_sys::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PFN_D3D12_CREATE_VERSIONED_ROOT_SIGNATURE_DESERIALIZER = unsafe extern "system" fn(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows_sys::core::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PFN_D3D12_GET_DEBUG_INTERFACE = unsafe extern "system" fn(param0: *const ::windows_sys::core::GUID, param1: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type PFN_D3D12_GET_INTERFACE = unsafe extern "system" fn(param0: *const ::windows_sys::core::GUID, param1: *const ::windows_sys::core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_SERIALIZE_ROOT_SIGNATURE = unsafe extern "system" fn(prootsignature: *const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, ppblob: *mut super::Direct3D::ID3DBlob, pperrorblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub type PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE = unsafe extern "system" fn(prootsignature: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob: *mut super::Direct3D::ID3DBlob, pperrorblob: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
pub const WKPDID_D3DAutoDebugObjectNameW: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3566218806,
    data2: 30074,
    data3: 18754,
    data4: [149, 148, 182, 118, 154, 250, 67, 205],
};
