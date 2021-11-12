#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Foundation`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Dxgi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D11CreateDevice(padapter: super::Dxgi::IDXGIAdapter, drivertype: super::Direct3D::D3D_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: D3D11_CREATE_DEVICE_FLAG, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, ppdevice: *mut ID3D11Device, pfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppimmediatecontext: *mut ID3D11DeviceContext) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Foundation`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Dxgi`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D11CreateDeviceAndSwapChain(
        padapter: super::Dxgi::IDXGIAdapter,
        drivertype: super::Direct3D::D3D_DRIVER_TYPE,
        software: super::super::Foundation::HINSTANCE,
        flags: D3D11_CREATE_DEVICE_FLAG,
        pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL,
        featurelevels: u32,
        sdkversion: u32,
        pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut super::Dxgi::IDXGISwapChain,
        ppdevice: *mut ID3D11Device,
        pfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL,
        ppimmediatecontext: *mut ID3D11DeviceContext,
    ) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3DDisassemble11Trace(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ptrace: ID3D11ShaderTrace, startstep: u32, numsteps: u32, flags: u32, ppdisassembly: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT(pdevicecontext: ID3D11DeviceContext, pdesc: *const D3DX11_FFT_DESC, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT1DComplex(pdevicecontext: ID3D11DeviceContext, x: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT1DReal(pdevicecontext: ID3D11DeviceContext, x: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT2DComplex(pdevicecontext: ID3D11DeviceContext, x: u32, y: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT2DReal(pdevicecontext: ID3D11DeviceContext, x: u32, y: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT3DComplex(pdevicecontext: ID3D11DeviceContext, x: u32, y: u32, z: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateFFT3DReal(pdevicecontext: ID3D11DeviceContext, x: u32, y: u32, z: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateScan(pdevicecontext: ID3D11DeviceContext, maxelementscansize: u32, maxscancount: u32, ppscan: *mut ID3DX11Scan) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
    pub fn D3DX11CreateSegmentedScan(pdevicecontext: ID3D11DeviceContext, maxelementscansize: u32, ppscan: *mut ID3DX11SegmentedScan) -> ::windows_sys::core::HRESULT;
}
pub struct CD3D11_VIDEO_DEFAULT(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
pub struct D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_1_UAV_SLOT_COUNT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_2_TILED_RESOURCE_TILE_SIZE_IN_BYTES: u32 = 65536u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_4_VIDEO_DECODER_HISTOGRAM_OFFSET_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_4_VIDEO_DECODER_MAX_HISTOGRAM_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
pub struct D3D11_AES_CTR_IV(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
pub struct D3D11_ASYNC_GETDATA_FLAG(i32);
pub struct D3D11_AUTHENTICATED_CHANNEL_TYPE(i32);
pub struct D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT(i32);
pub const D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1665584212, data2: 11516, data3: 19156, data4: [130, 36, 209, 88, 55, 222, 119, 0] };
pub struct D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT(i32);
pub const D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1107292806,
    data2: 27360,
    data3: 19779,
    data4: [157, 85, 164, 110, 158, 253, 21, 138],
};
pub const D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 101796827,
    data2: 13603,
    data3: 18186,
    data4: [141, 202, 251, 194, 132, 81, 84, 240],
};
pub struct D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT(i32);
pub struct D3D11_AUTHENTICATED_CONFIGURE_INPUT(i32);
pub struct D3D11_AUTHENTICATED_CONFIGURE_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_CONFIGURE_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1346721368,
    data2: 16199,
    data3: 17250,
    data4: [191, 153, 191, 223, 205, 233, 237, 41],
};
pub struct D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT(i32);
pub const D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 124964935, data2: 6976, data3: 18664, data4: [156, 166, 181, 245, 16, 222, 159, 1] };
pub struct D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT(i32);
pub struct D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE(i32);
pub struct D3D11_AUTHENTICATED_PROTECTION_FLAGS(i32);
pub const D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1645533650,
    data2: 17196,
    data3: 19131,
    data4: [159, 206, 33, 110, 234, 38, 158, 59],
};
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3155892389,
    data2: 45563,
    data3: 17067,
    data4: [189, 148, 181, 130, 139, 75, 247, 190],
};
pub struct D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 640960926, data2: 53272, data3: 19828, data4: [172, 23, 127, 114, 64, 89, 82, 141] };
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3960967623,
    data2: 56019,
    data3: 20245,
    data4: [158, 195, 250, 169, 61, 96, 212, 240],
};
pub const D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3961279389,
    data2: 36095,
    data3: 20010,
    data4: [188, 196, 245, 105, 47, 153, 244, 128],
};
pub struct D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4164573528, data2: 59782, data3: 19418, data4: [190, 176, 65, 31, 106, 122, 1, 183] };
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3004133478, data2: 8252, data3: 19207, data4: [147, 252, 206, 170, 253, 97, 36, 30] };
pub struct D3D11_AUTHENTICATED_QUERY_INPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2208160931,
    data2: 39758,
    data3: 16868,
    data4: [176, 83, 137, 43, 210, 161, 30, 231],
};
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 738470750,
    data2: 35847,
    data3: 18133,
    data4: [170, 190, 143, 117, 203, 173, 76, 49],
};
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_QUERY_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2823730564,
    data2: 50325,
    data3: 18602,
    data4: [185, 77, 139, 210, 214, 251, 206, 5],
};
pub struct D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1687927515, data2: 61684, data3: 17977, data4: [161, 91, 36, 57, 63, 195, 171, 172] };
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 229771187,
    data2: 37968,
    data3: 18086,
    data4: [130, 222, 27, 150, 212, 79, 156, 242],
};
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT(i32);
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT(i32);
pub const D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 19860438, data2: 58978, data3: 17524, data4: [190, 253, 170, 83, 229, 20, 60, 109] };
pub struct D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT(i32);
pub struct D3D11_BIND_FLAG(i32);
pub struct D3D11_BLEND(i32);
pub struct D3D11_BLEND_DESC(i32);
pub struct D3D11_BLEND_DESC1(i32);
pub struct D3D11_BLEND_OP(i32);
pub struct D3D11_BOX(i32);
pub struct D3D11_BUFFEREX_SRV(i32);
pub struct D3D11_BUFFEREX_SRV_FLAG(i32);
pub struct D3D11_BUFFER_DESC(i32);
pub struct D3D11_BUFFER_RTV(i32);
pub struct D3D11_BUFFER_SRV(i32);
pub struct D3D11_BUFFER_UAV(i32);
pub struct D3D11_BUFFER_UAV_FLAG(i32);
pub struct D3D11_BUS_TYPE(i32);
pub struct D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG(i32);
pub struct D3D11_CLASS_INSTANCE_DESC(i32);
pub struct D3D11_CLEAR_FLAG(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
pub struct D3D11_COLOR_WRITE_ENABLE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_COMPARISON_FILTERING_BIT: u32 = 128u32;
pub struct D3D11_COMPARISON_FUNC(i32);
pub struct D3D11_COMPUTE_SHADER_TRACE_DESC(i32);
pub struct D3D11_CONSERVATIVE_RASTERIZATION_MODE(i32);
pub struct D3D11_CONSERVATIVE_RASTERIZATION_TIER(i32);
pub struct D3D11_CONTENT_PROTECTION_CAPS(i32);
pub struct D3D11_CONTEXT_TYPE(i32);
pub struct D3D11_COPY_FLAGS(i32);
pub struct D3D11_COUNTER(i32);
pub struct D3D11_COUNTER_DESC(i32);
pub struct D3D11_COUNTER_INFO(i32);
pub struct D3D11_COUNTER_TYPE(i32);
pub struct D3D11_CPU_ACCESS_FLAG(i32);
pub struct D3D11_CREATE_DEVICE_FLAG(i32);
pub struct D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS(i32);
pub struct D3D11_CRYPTO_SESSION_STATUS(i32);
pub const D3D11_CRYPTO_TYPE_AES128_CTR: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2607535889,
    data2: 20340,
    data3: 16841,
    data4: [158, 123, 11, 226, 215, 217, 59, 79],
};
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 240u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: u32 = 68u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 224u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: u32 = 72u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 208u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: u32 = 76u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 192u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: u32 = 84u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 176u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: u32 = 92u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 160u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: u32 = 100u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 144u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: u32 = 112u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 112u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: u32 = 144u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 96u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: u32 = 168u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 80u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: u32 = 204u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 48u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: u32 = 340u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: u32 = 768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_X: u32 = 768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_Y: u32 = 768u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_4_X_UAV_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_TGSM_REGISTER_COUNT: u32 = 8192u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_TGSM_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREADGROUPID_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREADGROUPID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREADIDINGROUP_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREADIDINGROUP_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREADID_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREADID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREAD_GROUP_MAX_X: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREAD_GROUP_MAX_Y: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREAD_GROUP_MAX_Z: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREAD_GROUP_MIN_X: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREAD_GROUP_MIN_Y: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREAD_GROUP_MIN_Z: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: u32 = 16384u32;
pub struct D3D11_CULL_MODE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEBUG_FEATURE_ALWAYS_DISCARD_OFFERED_RESOURCE: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEBUG_FEATURE_AVOID_BEHAVIOR_CHANGING_DEBUG_AIDS: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEBUG_FEATURE_DISABLE_TILED_RESOURCE_MAPPING_TRACKING_AND_VALIDATION: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEBUG_FEATURE_NEVER_DISCARD_OFFERED_RESOURCE: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4u32;
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CBCS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1110283033, data2: 40225, data3: 19383, data4: [147, 113, 250, 245, 168, 44, 62, 4] };
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CENC: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2957005365,
    data2: 49469,
    data3: 17650,
    data4: [154, 229, 221, 72, 224, 142, 91, 103],
};
pub const D3D11_DECODER_ENCRYPTION_HW_CENC: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2312547407,
    data2: 2546,
    data3: 16937,
    data4: [178, 205, 55, 116, 10, 109, 253, 129],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 387084297,
    data2: 40975,
    data3: 19681,
    data4: [153, 78, 191, 64, 129, 246, 243, 240],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2_420: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 763412182,
    data2: 40108,
    data3: 18485,
    data4: [158, 145, 50, 123, 188, 79, 158, 232],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE0: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3099479243,
    data2: 53075,
    data3: 18106,
    data4: [141, 89, 214, 184, 166, 218, 93, 42],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE1: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1765211919,
    data2: 17841,
    data3: 16739,
    data4: [156, 193, 100, 110, 246, 148, 97, 8],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 207563425, data2: 58689, data3: 16521, data4: [187, 123, 152, 17, 10, 25, 215, 200] };
pub const D3D11_DECODER_PROFILE_H264_IDCT_FGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487719, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_IDCT_NOFGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487718, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_FGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487717, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_NOFGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487716, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_VLD_FGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487721, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_VLD_MULTIVIEW_NOFGT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1885052290,
    data2: 30415,
    data3: 18902,
    data4: [183, 230, 172, 136, 114, 219, 1, 60],
};
pub const D3D11_DECODER_PROFILE_H264_VLD_NOFGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487720, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_NOFGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4188720315, data2: 49846, data3: 19708, data4: [135, 121, 87, 7, 177, 118, 5, 82] };
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_PROGRESSIVE_NOFGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3617319130, data2: 3313, data3: 19585, data4: [184, 42, 105, 164, 226, 54, 244, 61] };
pub const D3D11_DECODER_PROFILE_H264_VLD_WITHFMOASO_NOFGT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3589296121,
    data2: 13336,
    data3: 17880,
    data4: [149, 97, 50, 167, 106, 174, 45, 221],
};
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1527895323, data2: 12108, data3: 17490, data4: [188, 195, 9, 242, 161, 22, 12, 192] };
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 276492512, data2: 61210, data3: 19737, data4: [171, 168, 103, 161, 99, 7, 61, 19] };
pub const D3D11_DECODER_PROFILE_MPEG1_VLD: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1866385177,
    data2: 14133,
    data3: 17100,
    data4: [128, 99, 101, 204, 60, 179, 102, 22],
};
pub const D3D11_DECODER_PROFILE_MPEG2_IDCT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3206720768, data2: 1002, data3: 18064, data4: [128, 119, 71, 51, 70, 32, 155, 126] };
pub const D3D11_DECODER_PROFILE_MPEG2_MOCOMP: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3869897803,
    data2: 25008,
    data3: 17763,
    data4: [158, 164, 99, 210, 163, 198, 254, 102],
};
pub const D3D11_DECODER_PROFILE_MPEG2_VLD: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3995550079, data2: 24104, data3: 20069, data4: [190, 234, 29, 38, 181, 8, 173, 201] };
pub const D3D11_DECODER_PROFILE_MPEG2and1_VLD: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2255052562,
    data2: 13326,
    data3: 20228,
    data4: [159, 211, 146, 83, 221, 50, 116, 96],
};
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_GMC: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2878966619,
    data2: 16984,
    data3: 17577,
    data4: [159, 235, 148, 229, 151, 166, 186, 174],
};
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_NOGMC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3980495519, data2: 269, data3: 20186, data4: [154, 227, 154, 101, 53, 141, 141, 46] };
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_SIMPLE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4023799156,
    data2: 51688,
    data3: 16855,
    data4: [165, 233, 233, 176, 227, 159, 163, 25],
};
pub const D3D11_DECODER_PROFILE_VC1_D2010: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487780, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VC1_IDCT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487778, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VC1_MOCOMP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487777, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VC1_POSTPROC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487776, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VC1_VLD: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487779, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VP8_VLD: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2428017130,
    data2: 14946,
    data3: 18181,
    data4: [136, 179, 141, 240, 75, 39, 68, 231],
};
pub const D3D11_DECODER_PROFILE_VP9_VLD_10BIT_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2764524015, data2: 28367, data3: 18602, data4: [132, 72, 80, 167, 161, 22, 95, 247] };
pub const D3D11_DECODER_PROFILE_VP9_VLD_PROFILE0: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1178011640,
    data2: 41424,
    data3: 17797,
    data4: [135, 109, 131, 170, 109, 96, 184, 158],
};
pub const D3D11_DECODER_PROFILE_WMV8_MOCOMP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487745, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_WMV8_POSTPROC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487744, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_WMV9_IDCT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487764, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_WMV9_MOCOMP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487761, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_WMV9_POSTPROC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487760, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_DEPTH_BIAS: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_MAX_ANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
pub struct D3D11_DEPTH_STENCILOP_DESC(i32);
pub struct D3D11_DEPTH_STENCIL_DESC(i32);
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC(i32);
pub struct D3D11_DEPTH_WRITE_MASK(i32);
pub struct D3D11_DEVICE_CONTEXT_TYPE(i32);
pub struct D3D11_DOMAIN_SHADER_TRACE_DESC(i32);
pub struct D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS(i32);
pub struct D3D11_DRAW_INSTANCED_INDIRECT_ARGS(i32);
pub struct D3D11_DSV_DIMENSION(i32);
pub struct D3D11_DSV_FLAG(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_DS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub struct D3D11_ENCRYPTED_BLOCK_INFO(i32);
pub struct D3D11_FEATURE(i32);
pub struct D3D11_FEATURE_DATA_ARCHITECTURE_INFO(i32);
pub struct D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS(i32);
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS(i32);
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS1(i32);
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS2(i32);
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS3(i32);
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS4(i32);
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS5(i32);
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS(i32);
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS1(i32);
pub struct D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT(i32);
pub struct D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT(i32);
pub struct D3D11_FEATURE_DATA_DISPLAYABLE(i32);
pub struct D3D11_FEATURE_DATA_DOUBLES(i32);
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT(i32);
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT2(i32);
pub struct D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT(i32);
pub struct D3D11_FEATURE_DATA_MARKER_SUPPORT(i32);
pub struct D3D11_FEATURE_DATA_SHADER_CACHE(i32);
pub struct D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT(i32);
pub struct D3D11_FEATURE_DATA_THREADING(i32);
pub struct D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM(i32);
pub struct D3D11_FEATURE_VIDEO(i32);
pub struct D3D11_FENCE_FLAG(i32);
pub struct D3D11_FILL_MODE(i32);
pub struct D3D11_FILTER(i32);
pub struct D3D11_FILTER_REDUCTION_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FILTER_REDUCTION_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FILTER_REDUCTION_TYPE_SHIFT: u32 = 7u32;
pub struct D3D11_FILTER_TYPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FILTER_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
pub struct D3D11_FORMAT_SUPPORT(i32);
pub struct D3D11_FORMAT_SUPPORT2(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
pub struct D3D11_FUNCTION_DESC(i32);
pub struct D3D11_GEOMETRY_SHADER_TRACE_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_INSTANCE_ID_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_INSTANCE_ID_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_INPUT_REGISTER_VERTICES: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_MAX_INSTANCE_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_OUTPUT_ELEMENTS: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_MAXTESSFACTOR_LOWER_BOUND: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_MAXTESSFACTOR_UPPER_BOUND: f32 = 64f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: u32 = 128u32;
pub struct D3D11_HULL_SHADER_TRACE_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_PATCH_MAX_CONTROL_POINT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
pub struct D3D11_INFO_QUEUE_FILTER(i32);
pub struct D3D11_INFO_QUEUE_FILTER_DESC(i32);
pub struct D3D11_INPUT_CLASSIFICATION(i32);
pub struct D3D11_INPUT_ELEMENT_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295u32;
pub const D3D11_KEY_EXCHANGE_HW_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2971078026,
    data2: 25229,
    data3: 19875,
    data4: [173, 59, 130, 221, 176, 139, 73, 112],
};
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA(i32);
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA(i32);
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA(i32);
pub const D3D11_KEY_EXCHANGE_RSAES_OAEP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3247741077, data2: 55082, data3: 18973, data4: [142, 93, 237, 133, 125, 23, 21, 32] };
pub struct D3D11_LIBRARY_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_LINEAR_GAMMA: f32 = 1f32;
pub struct D3D11_LOGIC_OP(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MAG_FILTER_SHIFT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MAJOR_VERSION: u32 = 11u32;
pub struct D3D11_MAP(i32);
pub struct D3D11_MAPPED_SUBRESOURCE(i32);
pub struct D3D11_MAP_FLAG(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MAX_DEPTH: f32 = 1f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MAX_MAXANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
pub struct D3D11_MESSAGE(i32);
pub struct D3D11_MESSAGE_CATEGORY(i32);
pub struct D3D11_MESSAGE_ID(i32);
pub struct D3D11_MESSAGE_SEVERITY(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIN_DEPTH: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIN_FILTER_SHIFT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIN_MAXANISOTROPY: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIP_FILTER_SHIFT: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIP_LOD_BIAS_MIN: f32 = -16f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
pub struct D3D11_OMAC(i32);
pub struct D3D11_PACKED_MIP_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PACKED_TILE: u32 = 4294967295u32;
pub struct D3D11_PARAMETER_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
pub struct D3D11_PIXEL_SHADER_TRACE_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_CS_UAV_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_CS_UAV_REGISTER_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_CS_UAV_REGISTER_READS_PER_INST: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_CS_UAV_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
pub struct D3D11_QUERY(i32);
pub struct D3D11_QUERY_DATA_PIPELINE_STATISTICS(i32);
pub struct D3D11_QUERY_DATA_SO_STATISTICS(i32);
pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT(i32);
pub struct D3D11_QUERY_DESC(i32);
pub struct D3D11_QUERY_DESC1(i32);
pub struct D3D11_QUERY_MISC_FLAG(i32);
pub struct D3D11_RAISE_FLAG(i32);
pub struct D3D11_RASTERIZER_DESC(i32);
pub struct D3D11_RASTERIZER_DESC1(i32);
pub struct D3D11_RASTERIZER_DESC2(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16u32;
pub struct D3D11_RENDER_TARGET_BLEND_DESC(i32);
pub struct D3D11_RENDER_TARGET_BLEND_DESC1(i32);
pub struct D3D11_RENDER_TARGET_VIEW_DESC(i32);
pub struct D3D11_RENDER_TARGET_VIEW_DESC1(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_MAXANISOTROPY: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_MIP_LEVELS: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: f32 = 0.25f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: u32 = 20u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_TEXTURE1D_U_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_REQ_TEXTURECUBE_DIMENSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
pub struct D3D11_RESOURCE_DIMENSION(i32);
pub struct D3D11_RESOURCE_MISC_FLAG(i32);
pub struct D3D11_RLDO_FLAGS(i32);
pub struct D3D11_RTV_DIMENSION(i32);
pub struct D3D11_SAMPLER_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SDK_LAYERS_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SDK_VERSION: u32 = 7u32;
pub struct D3D11_SHADER_BUFFER_DESC(i32);
pub struct D3D11_SHADER_CACHE_SUPPORT_FLAGS(i32);
pub struct D3D11_SHADER_DESC(i32);
pub struct D3D11_SHADER_INPUT_BIND_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHADER_MAJOR_VERSION: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHADER_MAX_INSTANCES: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHADER_MAX_INTERFACES: u32 = 253u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHADER_MAX_INTERFACE_CALL_SITES: u32 = 4096u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHADER_MAX_TYPES: u32 = 65535u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHADER_MINOR_VERSION: u32 = 0u32;
pub struct D3D11_SHADER_MIN_PRECISION_SUPPORT(i32);
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC(i32);
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC1(i32);
pub struct D3D11_SHADER_TRACE_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHADER_TRACE_FLAG_RECORD_REGISTER_READS: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHADER_TRACE_FLAG_RECORD_REGISTER_WRITES: u32 = 1u32;
pub struct D3D11_SHADER_TRACKING_OPTIONS(i32);
pub struct D3D11_SHADER_TRACKING_RESOURCE_TYPE(i32);
pub struct D3D11_SHADER_TYPE(i32);
pub struct D3D11_SHADER_TYPE_DESC(i32);
pub struct D3D11_SHADER_VARIABLE_DESC(i32);
pub struct D3D11_SHADER_VERSION_TYPE(i32);
pub struct D3D11_SHARED_RESOURCE_TIER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
pub struct D3D11_SIGNATURE_PARAMETER_DESC(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 512u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
pub struct D3D11_SO_DECLARATION_ENTRY(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SO_NO_RASTERIZED_STREAM: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SO_OUTPUT_COMPONENT_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SO_STREAM_COUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SPEC_DATE_DAY: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SPEC_DATE_MONTH: u32 = 5u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SPEC_DATE_YEAR: u32 = 2011u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SPEC_VERSION: f64 = 1.07f64;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SRGB_GAMMA: f32 = 2.2f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
pub struct D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_STANDARD_VECTOR_SIZE: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
pub struct D3D11_STENCIL_OP(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub struct D3D11_SUBRESOURCE_DATA(i32);
pub struct D3D11_SUBRESOURCE_TILING(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1u32;
pub struct D3D11_TEX1D_ARRAY_DSV(i32);
pub struct D3D11_TEX1D_ARRAY_RTV(i32);
pub struct D3D11_TEX1D_ARRAY_SRV(i32);
pub struct D3D11_TEX1D_ARRAY_UAV(i32);
pub struct D3D11_TEX1D_DSV(i32);
pub struct D3D11_TEX1D_RTV(i32);
pub struct D3D11_TEX1D_SRV(i32);
pub struct D3D11_TEX1D_UAV(i32);
pub struct D3D11_TEX2DMS_ARRAY_DSV(i32);
pub struct D3D11_TEX2DMS_ARRAY_RTV(i32);
pub struct D3D11_TEX2DMS_ARRAY_SRV(i32);
pub struct D3D11_TEX2DMS_DSV(i32);
pub struct D3D11_TEX2DMS_RTV(i32);
pub struct D3D11_TEX2DMS_SRV(i32);
pub struct D3D11_TEX2D_ARRAY_DSV(i32);
pub struct D3D11_TEX2D_ARRAY_RTV(i32);
pub struct D3D11_TEX2D_ARRAY_RTV1(i32);
pub struct D3D11_TEX2D_ARRAY_SRV(i32);
pub struct D3D11_TEX2D_ARRAY_SRV1(i32);
pub struct D3D11_TEX2D_ARRAY_UAV(i32);
pub struct D3D11_TEX2D_ARRAY_UAV1(i32);
pub struct D3D11_TEX2D_ARRAY_VPOV(i32);
pub struct D3D11_TEX2D_DSV(i32);
pub struct D3D11_TEX2D_RTV(i32);
pub struct D3D11_TEX2D_RTV1(i32);
pub struct D3D11_TEX2D_SRV(i32);
pub struct D3D11_TEX2D_SRV1(i32);
pub struct D3D11_TEX2D_UAV(i32);
pub struct D3D11_TEX2D_UAV1(i32);
pub struct D3D11_TEX2D_VDOV(i32);
pub struct D3D11_TEX2D_VPIV(i32);
pub struct D3D11_TEX2D_VPOV(i32);
pub struct D3D11_TEX3D_RTV(i32);
pub struct D3D11_TEX3D_SRV(i32);
pub struct D3D11_TEX3D_UAV(i32);
pub struct D3D11_TEXCUBE_ARRAY_SRV(i32);
pub struct D3D11_TEXCUBE_SRV(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub struct D3D11_TEXTURE1D_DESC(i32);
pub struct D3D11_TEXTURE2D_DESC(i32);
pub struct D3D11_TEXTURE2D_DESC1(i32);
pub struct D3D11_TEXTURE3D_DESC(i32);
pub struct D3D11_TEXTURE3D_DESC1(i32);
pub struct D3D11_TEXTURECUBE_FACE(i32);
pub struct D3D11_TEXTURE_ADDRESS_MODE(i32);
pub struct D3D11_TEXTURE_LAYOUT(i32);
pub struct D3D11_TILED_RESOURCES_TIER(i32);
pub struct D3D11_TILED_RESOURCE_COORDINATE(i32);
pub struct D3D11_TILE_COPY_FLAG(i32);
pub struct D3D11_TILE_MAPPING_FLAG(i32);
pub struct D3D11_TILE_RANGE_FLAG(i32);
pub struct D3D11_TILE_REGION_SIZE(i32);
pub struct D3D11_TILE_SHAPE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_COMPONENT_W: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_COMPONENT_X: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_COMPONENT_Y: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_COMPONENT_Z: u32 = 4u32;
pub struct D3D11_TRACE_GS_INPUT_PRIMITIVE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_MISC_GS_CUT: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_MISC_GS_CUT_STREAM: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_MISC_GS_EMIT: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_MISC_GS_EMIT_STREAM: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_MISC_HALT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_MISC_MESSAGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_MISC_PS_DISCARD: u32 = 4u32;
pub struct D3D11_TRACE_REGISTER(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_TRACE_REGISTER_FLAGS_RELATIVE_INDEXING: u32 = 1u32;
pub struct D3D11_TRACE_REGISTER_TYPE(i32);
pub struct D3D11_TRACE_STATS(i32);
pub struct D3D11_TRACE_STEP(i32);
pub struct D3D11_TRACE_VALUE(i32);
pub struct D3D11_UAV_DIMENSION(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC(i32);
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC1(i32);
pub struct D3D11_USAGE(i32);
pub struct D3D11_VDOV_DIMENSION(i32);
pub struct D3D11_VERTEX_SHADER_TRACE_DESC(i32);
pub struct D3D11_VIDEO_COLOR(i32);
pub struct D3D11_VIDEO_COLOR_RGBA(i32);
pub struct D3D11_VIDEO_COLOR_YCbCrA(i32);
pub struct D3D11_VIDEO_CONTENT_PROTECTION_CAPS(i32);
pub struct D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION(i32);
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC(i32);
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC1(i32);
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC2(i32);
pub struct D3D11_VIDEO_DECODER_BUFFER_TYPE(i32);
pub struct D3D11_VIDEO_DECODER_CAPS(i32);
pub struct D3D11_VIDEO_DECODER_CONFIG(i32);
pub struct D3D11_VIDEO_DECODER_DESC(i32);
pub struct D3D11_VIDEO_DECODER_EXTENSION(i32);
pub struct D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT(i32);
pub struct D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS(i32);
pub struct D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC(i32);
pub struct D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK(i32);
pub struct D3D11_VIDEO_FRAME_FORMAT(i32);
pub struct D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE(i32);
pub struct D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS(i32);
pub struct D3D11_VIDEO_PROCESSOR_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_COLOR_SPACE(i32);
pub struct D3D11_VIDEO_PROCESSOR_CONTENT_DESC(i32);
pub struct D3D11_VIDEO_PROCESSOR_CUSTOM_RATE(i32);
pub struct D3D11_VIDEO_PROCESSOR_DEVICE_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_FEATURE_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_FILTER(i32);
pub struct D3D11_VIDEO_PROCESSOR_FILTER_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_FILTER_RANGE(i32);
pub struct D3D11_VIDEO_PROCESSOR_FORMAT_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT(i32);
pub struct D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC(i32);
pub struct D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE(i32);
pub struct D3D11_VIDEO_PROCESSOR_OUTPUT_RATE(i32);
pub struct D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC(i32);
pub struct D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_ROTATION(i32);
pub struct D3D11_VIDEO_PROCESSOR_STEREO_CAPS(i32);
pub struct D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE(i32);
pub struct D3D11_VIDEO_PROCESSOR_STEREO_FORMAT(i32);
pub struct D3D11_VIDEO_PROCESSOR_STREAM(i32);
pub struct D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT(i32);
pub struct D3D11_VIDEO_SAMPLE_DESC(i32);
pub struct D3D11_VIDEO_USAGE(i32);
pub struct D3D11_VIEWPORT(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VIEWPORT_BOUNDS_MAX: u32 = 32767u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VIEWPORT_BOUNDS_MIN: i32 = -32768i32;
pub struct D3D11_VPIV_DIMENSION(i32);
pub struct D3D11_VPOV_DIMENSION(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D11_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
pub struct D3DX11_FFT_BUFFER_INFO(i32);
pub struct D3DX11_FFT_CREATE_FLAG(i32);
pub struct D3DX11_FFT_DATA_TYPE(i32);
pub struct D3DX11_FFT_DESC(i32);
pub struct D3DX11_FFT_DIM_MASK(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3DX11_FFT_MAX_DIMENSIONS: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3DX11_FFT_MAX_PRECOMPUTE_BUFFERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3DX11_FFT_MAX_TEMP_BUFFERS: u32 = 4u32;
pub struct D3DX11_SCAN_DATA_TYPE(i32);
pub struct D3DX11_SCAN_DIRECTION(i32);
pub struct D3DX11_SCAN_OPCODE(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_RETURN_PARAMETER_INDEX: i32 = -1i32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: u32 = 64u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_64_UAVS: u32 = 8u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_DOUBLES: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING: u32 = 128u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: u32 = 16u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_TILED_RESOURCES: u32 = 256u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: u32 = 4u32;
pub const DXGI_DEBUG_D3D11: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1268330875,
    data2: 44089,
    data3: 19110,
    data4: [187, 11, 186, 160, 71, 132, 121, 143],
};
pub struct ID3D11Asynchronous(i32);
pub struct ID3D11AuthenticatedChannel(i32);
pub struct ID3D11BlendState(i32);
pub struct ID3D11BlendState1(i32);
pub struct ID3D11Buffer(i32);
pub struct ID3D11ClassInstance(i32);
pub struct ID3D11ClassLinkage(i32);
pub struct ID3D11CommandList(i32);
pub struct ID3D11ComputeShader(i32);
pub struct ID3D11Counter(i32);
pub struct ID3D11CryptoSession(i32);
pub struct ID3D11Debug(i32);
pub struct ID3D11DepthStencilState(i32);
pub struct ID3D11DepthStencilView(i32);
pub struct ID3D11Device(i32);
pub struct ID3D11Device1(i32);
pub struct ID3D11Device2(i32);
pub struct ID3D11Device3(i32);
pub struct ID3D11Device4(i32);
pub struct ID3D11Device5(i32);
pub struct ID3D11DeviceChild(i32);
pub struct ID3D11DeviceContext(i32);
pub struct ID3D11DeviceContext1(i32);
pub struct ID3D11DeviceContext2(i32);
pub struct ID3D11DeviceContext3(i32);
pub struct ID3D11DeviceContext4(i32);
pub struct ID3D11DomainShader(i32);
pub struct ID3D11Fence(i32);
pub struct ID3D11FunctionLinkingGraph(i32);
pub struct ID3D11FunctionParameterReflection(i32);
pub struct ID3D11FunctionReflection(i32);
pub struct ID3D11GeometryShader(i32);
pub struct ID3D11HullShader(i32);
pub struct ID3D11InfoQueue(i32);
pub struct ID3D11InputLayout(i32);
pub struct ID3D11LibraryReflection(i32);
pub struct ID3D11Linker(i32);
pub struct ID3D11LinkingNode(i32);
pub struct ID3D11Module(i32);
pub struct ID3D11ModuleInstance(i32);
pub struct ID3D11Multithread(i32);
pub struct ID3D11PixelShader(i32);
pub struct ID3D11Predicate(i32);
pub struct ID3D11Query(i32);
pub struct ID3D11Query1(i32);
pub struct ID3D11RasterizerState(i32);
pub struct ID3D11RasterizerState1(i32);
pub struct ID3D11RasterizerState2(i32);
pub struct ID3D11RefDefaultTrackingOptions(i32);
pub struct ID3D11RefTrackingOptions(i32);
pub struct ID3D11RenderTargetView(i32);
pub struct ID3D11RenderTargetView1(i32);
pub struct ID3D11Resource(i32);
pub struct ID3D11SamplerState(i32);
pub struct ID3D11ShaderReflection(i32);
pub struct ID3D11ShaderReflectionConstantBuffer(i32);
pub struct ID3D11ShaderReflectionType(i32);
pub struct ID3D11ShaderReflectionVariable(i32);
pub struct ID3D11ShaderResourceView(i32);
pub struct ID3D11ShaderResourceView1(i32);
pub struct ID3D11ShaderTrace(i32);
pub struct ID3D11ShaderTraceFactory(i32);
pub struct ID3D11SwitchToRef(i32);
pub struct ID3D11Texture1D(i32);
pub struct ID3D11Texture2D(i32);
pub struct ID3D11Texture2D1(i32);
pub struct ID3D11Texture3D(i32);
pub struct ID3D11Texture3D1(i32);
pub struct ID3D11TracingDevice(i32);
pub struct ID3D11UnorderedAccessView(i32);
pub struct ID3D11UnorderedAccessView1(i32);
pub struct ID3D11VertexShader(i32);
pub struct ID3D11VideoContext(i32);
pub struct ID3D11VideoContext1(i32);
pub struct ID3D11VideoContext2(i32);
pub struct ID3D11VideoContext3(i32);
pub struct ID3D11VideoDecoder(i32);
pub struct ID3D11VideoDecoderOutputView(i32);
pub struct ID3D11VideoDevice(i32);
pub struct ID3D11VideoDevice1(i32);
pub struct ID3D11VideoDevice2(i32);
pub struct ID3D11VideoProcessor(i32);
pub struct ID3D11VideoProcessorEnumerator(i32);
pub struct ID3D11VideoProcessorEnumerator1(i32);
pub struct ID3D11VideoProcessorInputView(i32);
pub struct ID3D11VideoProcessorOutputView(i32);
pub struct ID3D11View(i32);
pub struct ID3DDeviceContextState(i32);
pub struct ID3DUserDefinedAnnotation(i32);
pub struct ID3DX11FFT(i32);
pub struct ID3DX11Scan(i32);
pub struct ID3DX11SegmentedScan(i32);
pub struct PFN_D3D11_CREATE_DEVICE(i32);
pub struct PFN_D3D11_CREATE_DEVICE_AND_SWAP_CHAIN(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D11`*"]
pub const _FACD3D11: u32 = 2172u32;
