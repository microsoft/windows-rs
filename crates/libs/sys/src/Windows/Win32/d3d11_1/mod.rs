#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_BLEND_DESC1 {
    pub Base: D3D11_BLEND_DESC1,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Default)]
pub struct CD3D11_RASTERIZER_DESC1 {
    pub Base: D3D11_RASTERIZER_DESC1,
}
pub type D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG = i32;
pub const D3D11_1_CREATE_DEVICE_CONTEXT_STATE_SINGLETHREADED: D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG = 1;
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy)]
pub struct D3D11_BLEND_DESC1 {
    pub AlphaToCoverageEnable: windows_sys::core::BOOL,
    pub IndependentBlendEnable: windows_sys::core::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1; 8],
}
#[cfg(feature = "Win32_d3d11")]
impl Default for D3D11_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_COPY_DISCARD: D3D11_COPY_FLAGS = 2;
pub type D3D11_COPY_FLAGS = i32;
pub const D3D11_COPY_NO_OVERWRITE: D3D11_COPY_FLAGS = 1;
pub type D3D11_CRYPTO_SESSION_STATUS = i32;
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_AND_CONTENT_LOST: D3D11_CRYPTO_SESSION_STATUS = 2;
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_LOST: D3D11_CRYPTO_SESSION_STATUS = 1;
pub const D3D11_CRYPTO_SESSION_STATUS_OK: D3D11_CRYPTO_SESSION_STATUS = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    pub HWProtectionFunctionID: u32,
    pub pInputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA,
    pub pOutputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA,
    pub Status: windows_sys::core::HRESULT,
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    pub PrivateDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub pbInput: [u8; 4],
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    pub PrivateDataSize: u32,
    pub MaxHWProtectionDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub TransportTime: u64,
    pub ExecutionTime: u64,
    pub pbOutput: [u8; 4],
}
impl Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_LOGIC_OP = i32;
pub const D3D11_LOGIC_OP_AND: D3D11_LOGIC_OP = 6;
pub const D3D11_LOGIC_OP_AND_INVERTED: D3D11_LOGIC_OP = 13;
pub const D3D11_LOGIC_OP_AND_REVERSE: D3D11_LOGIC_OP = 12;
pub const D3D11_LOGIC_OP_CLEAR: D3D11_LOGIC_OP = 0;
pub const D3D11_LOGIC_OP_COPY: D3D11_LOGIC_OP = 2;
pub const D3D11_LOGIC_OP_COPY_INVERTED: D3D11_LOGIC_OP = 3;
pub const D3D11_LOGIC_OP_EQUIV: D3D11_LOGIC_OP = 11;
pub const D3D11_LOGIC_OP_INVERT: D3D11_LOGIC_OP = 5;
pub const D3D11_LOGIC_OP_NAND: D3D11_LOGIC_OP = 7;
pub const D3D11_LOGIC_OP_NOOP: D3D11_LOGIC_OP = 4;
pub const D3D11_LOGIC_OP_NOR: D3D11_LOGIC_OP = 9;
pub const D3D11_LOGIC_OP_OR: D3D11_LOGIC_OP = 8;
pub const D3D11_LOGIC_OP_OR_INVERTED: D3D11_LOGIC_OP = 15;
pub const D3D11_LOGIC_OP_OR_REVERSE: D3D11_LOGIC_OP = 14;
pub const D3D11_LOGIC_OP_SET: D3D11_LOGIC_OP = 1;
pub const D3D11_LOGIC_OP_XOR: D3D11_LOGIC_OP = 10;
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_RASTERIZER_DESC1 {
    pub FillMode: super::d3d11::D3D11_FILL_MODE,
    pub CullMode: super::d3d11::D3D11_CULL_MODE,
    pub FrontCounterClockwise: windows_sys::core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_sys::core::BOOL,
    pub ScissorEnable: windows_sys::core::BOOL,
    pub MultisampleEnable: windows_sys::core::BOOL,
    pub AntialiasedLineEnable: windows_sys::core::BOOL,
    pub ForcedSampleCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: windows_sys::core::BOOL,
    pub LogicOpEnable: windows_sys::core::BOOL,
    pub SrcBlend: super::d3d11::D3D11_BLEND,
    pub DestBlend: super::d3d11::D3D11_BLEND,
    pub BlendOp: super::d3d11::D3D11_BLEND_OP,
    pub SrcBlendAlpha: super::d3d11::D3D11_BLEND,
    pub DestBlendAlpha: super::d3d11::D3D11_BLEND,
    pub BlendOpAlpha: super::d3d11::D3D11_BLEND_OP,
    pub LogicOp: D3D11_LOGIC_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    pub pCryptoSession: *mut core::ffi::c_void,
    pub BlobSize: u32,
    pub pBlob: *mut core::ffi::c_void,
    pub pKeyInfoId: *mut windows_sys::core::GUID,
    pub PrivateDataSize: u32,
    pub pPrivateData: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d11")]
impl Default for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d11")]
#[derive(Clone, Copy)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    pub BufferType: super::d3d11::D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
}
#[cfg(feature = "Win32_d3d11")]
impl Default for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_VIDEO_DECODER_CAPS = i32;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE: D3D11_VIDEO_DECODER_CAPS = 1;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_DYNAMIC: D3D11_VIDEO_DECODER_CAPS = 4;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_REQUIRED: D3D11_VIDEO_DECODER_CAPS = 8;
pub const D3D11_VIDEO_DECODER_CAPS_NON_REAL_TIME: D3D11_VIDEO_DECODER_CAPS = 2;
pub const D3D11_VIDEO_DECODER_CAPS_UNSUPPORTED: D3D11_VIDEO_DECODER_CAPS = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    pub ClearSize: u32,
    pub EncryptedSize: u32,
}
pub type D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = i32;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_COLOR_SPACE_CONVERSION: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 4;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_RESIZE: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 2;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_ROTATION: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 1;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_TRIPLE_BUFFER_OUTPUT: D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS = 8;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    pub Enable: windows_sys::core::BOOL,
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D11_VIDEO_SAMPLE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
}
