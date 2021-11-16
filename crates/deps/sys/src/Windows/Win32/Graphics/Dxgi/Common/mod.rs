#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const DXGI_ALPHA_MODE_UNSPECIFIED: u32 = 0u32;
pub const DXGI_ALPHA_MODE_PREMULTIPLIED: u32 = 1u32;
pub const DXGI_ALPHA_MODE_STRAIGHT: u32 = 2u32;
pub const DXGI_ALPHA_MODE_IGNORE: u32 = 3u32;
pub const DXGI_ALPHA_MODE_FORCE_DWORD: u32 = 4294967295u32;
pub const DXGI_CENTER_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967294u32;
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: i32 = 0i32;
pub const DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709: i32 = 1i32;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709: i32 = 2i32;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020: i32 = 3i32;
pub const DXGI_COLOR_SPACE_RESERVED: i32 = 4i32;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601: i32 = 5i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601: i32 = 6i32;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601: i32 = 7i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709: i32 = 8i32;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709: i32 = 9i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020: i32 = 10i32;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020: i32 = 11i32;
pub const DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020: i32 = 12i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020: i32 = 13i32;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020: i32 = 14i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020: i32 = 15i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020: i32 = 16i32;
pub const DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020: i32 = 17i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020: i32 = 18i32;
pub const DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020: i32 = 19i32;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709: i32 = 20i32;
pub const DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020: i32 = 21i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709: i32 = 22i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020: i32 = 23i32;
pub const DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020: i32 = 24i32;
pub const DXGI_COLOR_SPACE_CUSTOM: i32 = -1i32;
pub const DXGI_CPU_ACCESS_DYNAMIC: u32 = 1u32;
pub const DXGI_CPU_ACCESS_FIELD: u32 = 15u32;
pub const DXGI_CPU_ACCESS_NONE: u32 = 0u32;
pub const DXGI_CPU_ACCESS_READ_WRITE: u32 = 2u32;
pub const DXGI_CPU_ACCESS_SCRATCH: u32 = 3u32;
pub const DXGI_FORMAT_UNKNOWN: u32 = 0u32;
pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: u32 = 1u32;
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: u32 = 2u32;
pub const DXGI_FORMAT_R32G32B32A32_UINT: u32 = 3u32;
pub const DXGI_FORMAT_R32G32B32A32_SINT: u32 = 4u32;
pub const DXGI_FORMAT_R32G32B32_TYPELESS: u32 = 5u32;
pub const DXGI_FORMAT_R32G32B32_FLOAT: u32 = 6u32;
pub const DXGI_FORMAT_R32G32B32_UINT: u32 = 7u32;
pub const DXGI_FORMAT_R32G32B32_SINT: u32 = 8u32;
pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: u32 = 9u32;
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: u32 = 10u32;
pub const DXGI_FORMAT_R16G16B16A16_UNORM: u32 = 11u32;
pub const DXGI_FORMAT_R16G16B16A16_UINT: u32 = 12u32;
pub const DXGI_FORMAT_R16G16B16A16_SNORM: u32 = 13u32;
pub const DXGI_FORMAT_R16G16B16A16_SINT: u32 = 14u32;
pub const DXGI_FORMAT_R32G32_TYPELESS: u32 = 15u32;
pub const DXGI_FORMAT_R32G32_FLOAT: u32 = 16u32;
pub const DXGI_FORMAT_R32G32_UINT: u32 = 17u32;
pub const DXGI_FORMAT_R32G32_SINT: u32 = 18u32;
pub const DXGI_FORMAT_R32G8X24_TYPELESS: u32 = 19u32;
pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: u32 = 20u32;
pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: u32 = 21u32;
pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: u32 = 22u32;
pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: u32 = 23u32;
pub const DXGI_FORMAT_R10G10B10A2_UNORM: u32 = 24u32;
pub const DXGI_FORMAT_R10G10B10A2_UINT: u32 = 25u32;
pub const DXGI_FORMAT_R11G11B10_FLOAT: u32 = 26u32;
pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: u32 = 27u32;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: u32 = 28u32;
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: u32 = 29u32;
pub const DXGI_FORMAT_R8G8B8A8_UINT: u32 = 30u32;
pub const DXGI_FORMAT_R8G8B8A8_SNORM: u32 = 31u32;
pub const DXGI_FORMAT_R8G8B8A8_SINT: u32 = 32u32;
pub const DXGI_FORMAT_R16G16_TYPELESS: u32 = 33u32;
pub const DXGI_FORMAT_R16G16_FLOAT: u32 = 34u32;
pub const DXGI_FORMAT_R16G16_UNORM: u32 = 35u32;
pub const DXGI_FORMAT_R16G16_UINT: u32 = 36u32;
pub const DXGI_FORMAT_R16G16_SNORM: u32 = 37u32;
pub const DXGI_FORMAT_R16G16_SINT: u32 = 38u32;
pub const DXGI_FORMAT_R32_TYPELESS: u32 = 39u32;
pub const DXGI_FORMAT_D32_FLOAT: u32 = 40u32;
pub const DXGI_FORMAT_R32_FLOAT: u32 = 41u32;
pub const DXGI_FORMAT_R32_UINT: u32 = 42u32;
pub const DXGI_FORMAT_R32_SINT: u32 = 43u32;
pub const DXGI_FORMAT_R24G8_TYPELESS: u32 = 44u32;
pub const DXGI_FORMAT_D24_UNORM_S8_UINT: u32 = 45u32;
pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: u32 = 46u32;
pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: u32 = 47u32;
pub const DXGI_FORMAT_R8G8_TYPELESS: u32 = 48u32;
pub const DXGI_FORMAT_R8G8_UNORM: u32 = 49u32;
pub const DXGI_FORMAT_R8G8_UINT: u32 = 50u32;
pub const DXGI_FORMAT_R8G8_SNORM: u32 = 51u32;
pub const DXGI_FORMAT_R8G8_SINT: u32 = 52u32;
pub const DXGI_FORMAT_R16_TYPELESS: u32 = 53u32;
pub const DXGI_FORMAT_R16_FLOAT: u32 = 54u32;
pub const DXGI_FORMAT_D16_UNORM: u32 = 55u32;
pub const DXGI_FORMAT_R16_UNORM: u32 = 56u32;
pub const DXGI_FORMAT_R16_UINT: u32 = 57u32;
pub const DXGI_FORMAT_R16_SNORM: u32 = 58u32;
pub const DXGI_FORMAT_R16_SINT: u32 = 59u32;
pub const DXGI_FORMAT_R8_TYPELESS: u32 = 60u32;
pub const DXGI_FORMAT_R8_UNORM: u32 = 61u32;
pub const DXGI_FORMAT_R8_UINT: u32 = 62u32;
pub const DXGI_FORMAT_R8_SNORM: u32 = 63u32;
pub const DXGI_FORMAT_R8_SINT: u32 = 64u32;
pub const DXGI_FORMAT_A8_UNORM: u32 = 65u32;
pub const DXGI_FORMAT_R1_UNORM: u32 = 66u32;
pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: u32 = 67u32;
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: u32 = 68u32;
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: u32 = 69u32;
pub const DXGI_FORMAT_BC1_TYPELESS: u32 = 70u32;
pub const DXGI_FORMAT_BC1_UNORM: u32 = 71u32;
pub const DXGI_FORMAT_BC1_UNORM_SRGB: u32 = 72u32;
pub const DXGI_FORMAT_BC2_TYPELESS: u32 = 73u32;
pub const DXGI_FORMAT_BC2_UNORM: u32 = 74u32;
pub const DXGI_FORMAT_BC2_UNORM_SRGB: u32 = 75u32;
pub const DXGI_FORMAT_BC3_TYPELESS: u32 = 76u32;
pub const DXGI_FORMAT_BC3_UNORM: u32 = 77u32;
pub const DXGI_FORMAT_BC3_UNORM_SRGB: u32 = 78u32;
pub const DXGI_FORMAT_BC4_TYPELESS: u32 = 79u32;
pub const DXGI_FORMAT_BC4_UNORM: u32 = 80u32;
pub const DXGI_FORMAT_BC4_SNORM: u32 = 81u32;
pub const DXGI_FORMAT_BC5_TYPELESS: u32 = 82u32;
pub const DXGI_FORMAT_BC5_UNORM: u32 = 83u32;
pub const DXGI_FORMAT_BC5_SNORM: u32 = 84u32;
pub const DXGI_FORMAT_B5G6R5_UNORM: u32 = 85u32;
pub const DXGI_FORMAT_B5G5R5A1_UNORM: u32 = 86u32;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: u32 = 87u32;
pub const DXGI_FORMAT_B8G8R8X8_UNORM: u32 = 88u32;
pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: u32 = 89u32;
pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: u32 = 90u32;
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: u32 = 91u32;
pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: u32 = 92u32;
pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: u32 = 93u32;
pub const DXGI_FORMAT_BC6H_TYPELESS: u32 = 94u32;
pub const DXGI_FORMAT_BC6H_UF16: u32 = 95u32;
pub const DXGI_FORMAT_BC6H_SF16: u32 = 96u32;
pub const DXGI_FORMAT_BC7_TYPELESS: u32 = 97u32;
pub const DXGI_FORMAT_BC7_UNORM: u32 = 98u32;
pub const DXGI_FORMAT_BC7_UNORM_SRGB: u32 = 99u32;
pub const DXGI_FORMAT_AYUV: u32 = 100u32;
pub const DXGI_FORMAT_Y410: u32 = 101u32;
pub const DXGI_FORMAT_Y416: u32 = 102u32;
pub const DXGI_FORMAT_NV12: u32 = 103u32;
pub const DXGI_FORMAT_P010: u32 = 104u32;
pub const DXGI_FORMAT_P016: u32 = 105u32;
pub const DXGI_FORMAT_420_OPAQUE: u32 = 106u32;
pub const DXGI_FORMAT_YUY2: u32 = 107u32;
pub const DXGI_FORMAT_Y210: u32 = 108u32;
pub const DXGI_FORMAT_Y216: u32 = 109u32;
pub const DXGI_FORMAT_NV11: u32 = 110u32;
pub const DXGI_FORMAT_AI44: u32 = 111u32;
pub const DXGI_FORMAT_IA44: u32 = 112u32;
pub const DXGI_FORMAT_P8: u32 = 113u32;
pub const DXGI_FORMAT_A8P8: u32 = 114u32;
pub const DXGI_FORMAT_B4G4R4A4_UNORM: u32 = 115u32;
pub const DXGI_FORMAT_P208: u32 = 130u32;
pub const DXGI_FORMAT_V208: u32 = 131u32;
pub const DXGI_FORMAT_V408: u32 = 132u32;
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE: u32 = 189u32;
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE: u32 = 190u32;
pub const DXGI_FORMAT_FORCE_UINT: u32 = 4294967295u32;
pub const DXGI_FORMAT_DEFINED: u32 = 1u32;
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}
impl ::core::marker::Copy for DXGI_GAMMA_CONTROL {}
impl ::core::clone::Clone for DXGI_GAMMA_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: super::super::super::Foundation::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: u32,
    pub ControlPointPositions: [f32; 1025],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXGI_GAMMA_CONTROL_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 16],
    pub CodeValues: [u8; 162],
}
impl ::core::marker::Copy for DXGI_JPEG_AC_HUFFMAN_TABLE {}
impl ::core::clone::Clone for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 12],
    pub CodeValues: [u8; 12],
}
impl ::core::marker::Copy for DXGI_JPEG_DC_HUFFMAN_TABLE {}
impl ::core::clone::Clone for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [u8; 64],
}
impl ::core::marker::Copy for DXGI_JPEG_QUANTIZATION_TABLE {}
impl ::core::clone::Clone for DXGI_JPEG_QUANTIZATION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
impl ::core::marker::Copy for DXGI_MODE_DESC {}
impl ::core::clone::Clone for DXGI_MODE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_MODE_ROTATION_UNSPECIFIED: i32 = 0i32;
pub const DXGI_MODE_ROTATION_IDENTITY: i32 = 1i32;
pub const DXGI_MODE_ROTATION_ROTATE90: i32 = 2i32;
pub const DXGI_MODE_ROTATION_ROTATE180: i32 = 3i32;
pub const DXGI_MODE_ROTATION_ROTATE270: i32 = 4i32;
pub const DXGI_MODE_SCALING_UNSPECIFIED: i32 = 0i32;
pub const DXGI_MODE_SCALING_CENTERED: i32 = 1i32;
pub const DXGI_MODE_SCALING_STRETCHED: i32 = 2i32;
pub const DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED: i32 = 0i32;
pub const DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE: i32 = 1i32;
pub const DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST: i32 = 2i32;
pub const DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST: i32 = 3i32;
#[repr(C)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl ::core::marker::Copy for DXGI_RATIONAL {}
impl ::core::clone::Clone for DXGI_RATIONAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
impl ::core::marker::Copy for DXGI_RGB {}
impl ::core::clone::Clone for DXGI_RGB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
impl ::core::marker::Copy for DXGI_SAMPLE_DESC {}
impl ::core::clone::Clone for DXGI_SAMPLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967295u32;
pub const _FACDXGI: u32 = 2170u32;
