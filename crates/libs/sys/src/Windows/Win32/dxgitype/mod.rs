#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
pub const DXGI_CPU_ACCESS_DYNAMIC: u32 = 1;
pub const DXGI_CPU_ACCESS_FIELD: u32 = 15;
pub const DXGI_CPU_ACCESS_NONE: u32 = 0;
pub const DXGI_CPU_ACCESS_READ_WRITE: u32 = 2;
pub const DXGI_CPU_ACCESS_SCRATCH: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_GAMMA_CONTROL {
    pub Scale: DXGI_RGB,
    pub Offset: DXGI_RGB,
    pub GammaCurve: [DXGI_RGB; 1025],
}
impl Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    pub ScaleAndOffsetSupported: windows_sys::core::BOOL,
    pub MaxConvertedValue: f32,
    pub MinConvertedValue: f32,
    pub NumGammaControlPoints: u32,
    pub ControlPointPositions: [f32; 1025],
}
impl Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_AC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 16],
    pub CodeValues: [u8; 162],
}
impl Default for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_DC_HUFFMAN_TABLE {
    pub CodeCounts: [u8; 12],
    pub CodeValues: [u8; 12],
}
impl Default for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGI_JPEG_QUANTIZATION_TABLE {
    pub Elements: [u8; 64],
}
impl Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: super::dxgicommon::DXGI_RATIONAL,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
pub type DXGI_MODE_ROTATION = i32;
pub const DXGI_MODE_ROTATION_IDENTITY: DXGI_MODE_ROTATION = 1;
pub const DXGI_MODE_ROTATION_ROTATE180: DXGI_MODE_ROTATION = 3;
pub const DXGI_MODE_ROTATION_ROTATE270: DXGI_MODE_ROTATION = 4;
pub const DXGI_MODE_ROTATION_ROTATE90: DXGI_MODE_ROTATION = 2;
pub const DXGI_MODE_ROTATION_UNSPECIFIED: DXGI_MODE_ROTATION = 0;
pub type DXGI_MODE_SCALING = i32;
pub const DXGI_MODE_SCALING_CENTERED: DXGI_MODE_SCALING = 1;
pub const DXGI_MODE_SCALING_STRETCHED: DXGI_MODE_SCALING = 2;
pub const DXGI_MODE_SCALING_UNSPECIFIED: DXGI_MODE_SCALING = 0;
pub type DXGI_MODE_SCANLINE_ORDER = i32;
pub const DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER = 3;
pub const DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE: DXGI_MODE_SCANLINE_ORDER = 1;
pub const DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED: DXGI_MODE_SCANLINE_ORDER = 0;
pub const DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST: DXGI_MODE_SCANLINE_ORDER = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
pub type DXGI_RGBA = D3DCOLORVALUE;
