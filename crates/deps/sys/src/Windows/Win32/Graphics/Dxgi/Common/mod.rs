#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DXGI_ALPHA_MODE(i32);
pub const DXGI_CENTER_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967294u32;
#[repr(C)]
pub struct DXGI_COLOR_SPACE_TYPE(i32);
pub const DXGI_CPU_ACCESS_DYNAMIC: u32 = 1u32;
pub const DXGI_CPU_ACCESS_FIELD: u32 = 15u32;
pub const DXGI_CPU_ACCESS_NONE: u32 = 0u32;
pub const DXGI_CPU_ACCESS_READ_WRITE: u32 = 2u32;
pub const DXGI_CPU_ACCESS_SCRATCH: u32 = 3u32;
#[repr(C)]
pub struct DXGI_FORMAT(i32);
pub const DXGI_FORMAT_DEFINED: u32 = 1u32;
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES(i32);
#[repr(C)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE(i32);
#[repr(C)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE(i32);
#[repr(C)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE(i32);
#[repr(C)]
pub struct DXGI_MODE_DESC(i32);
#[repr(C)]
pub struct DXGI_MODE_ROTATION(i32);
#[repr(C)]
pub struct DXGI_MODE_SCALING(i32);
#[repr(C)]
pub struct DXGI_MODE_SCANLINE_ORDER(i32);
#[repr(C)]
pub struct DXGI_RATIONAL(i32);
#[repr(C)]
pub struct DXGI_RGB(i32);
#[repr(C)]
pub struct DXGI_SAMPLE_DESC(i32);
pub const DXGI_STANDARD_MULTISAMPLE_QUALITY_PATTERN: u32 = 4294967295u32;
pub const _FACDXGI: u32 = 2170u32;
