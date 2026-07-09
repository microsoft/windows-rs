#[cfg(feature = "Win32_dxgitype")]
pub type DWRITE_COLOR_F = super::dxgitype::D3DCOLORVALUE;
#[repr(C)]
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy)]
pub struct DWRITE_COLOR_GLYPH_RUN {
    pub glyphRun: super::dwrite::DWRITE_GLYPH_RUN,
    pub glyphRunDescription: *mut super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION,
    pub baselineOriginX: f32,
    pub baselineOriginY: f32,
    pub runColor: DWRITE_COLOR_F,
    pub paletteIndex: u16,
}
#[cfg(all(feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl Default for DWRITE_COLOR_GLYPH_RUN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DWRITE_GRID_FIT_MODE = i32;
pub const DWRITE_GRID_FIT_MODE_DEFAULT: DWRITE_GRID_FIT_MODE = 0;
pub const DWRITE_GRID_FIT_MODE_DISABLED: DWRITE_GRID_FIT_MODE = 1;
pub const DWRITE_GRID_FIT_MODE_ENABLED: DWRITE_GRID_FIT_MODE = 2;
pub const DWRITE_NO_PALETTE_INDEX: u32 = 65535;
pub type DWRITE_OPTICAL_ALIGNMENT = i32;
pub const DWRITE_OPTICAL_ALIGNMENT_NONE: DWRITE_OPTICAL_ALIGNMENT = 0;
pub const DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS: DWRITE_OPTICAL_ALIGNMENT = 1;
#[repr(C)]
#[cfg(feature = "Win32_dwrite")]
#[derive(Clone, Copy, Default)]
pub struct DWRITE_TEXT_METRICS1 {
    pub Base: super::dwrite::DWRITE_TEXT_METRICS,
    pub heightIncludingTrailingWhitespace: f32,
}
