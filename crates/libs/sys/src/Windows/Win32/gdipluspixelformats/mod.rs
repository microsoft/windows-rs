pub const ALPHA_SHIFT: i32 = 24;
pub type ARGB = u32;
pub const BLUE_SHIFT: i32 = 0;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ColorPalette {
    pub Flags: super::UINT,
    pub Count: super::UINT,
    pub Entries: [ARGB; 1],
}
#[cfg(feature = "shared")]
impl Default for ColorPalette {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DitherType = i32;
pub const DitherTypeDualSpiral4x4: DitherType = 7;
pub const DitherTypeDualSpiral8x8: DitherType = 8;
pub const DitherTypeErrorDiffusion: DitherType = 9;
pub const DitherTypeMax: DitherType = 10;
pub const DitherTypeNone: DitherType = 0;
pub const DitherTypeOrdered16x16: DitherType = 4;
pub const DitherTypeOrdered4x4: DitherType = 2;
pub const DitherTypeOrdered8x8: DitherType = 3;
pub const DitherTypeSolid: DitherType = 1;
pub const DitherTypeSpiral4x4: DitherType = 5;
pub const DitherTypeSpiral8x8: DitherType = 6;
pub const GREEN_SHIFT: i32 = 8;
pub type PaletteType = i32;
pub const PaletteTypeCustom: PaletteType = 0;
pub const PaletteTypeFixedBW: PaletteType = 2;
pub const PaletteTypeFixedHalftone125: PaletteType = 6;
pub const PaletteTypeFixedHalftone216: PaletteType = 7;
pub const PaletteTypeFixedHalftone252: PaletteType = 8;
pub const PaletteTypeFixedHalftone256: PaletteType = 9;
pub const PaletteTypeFixedHalftone27: PaletteType = 4;
pub const PaletteTypeFixedHalftone64: PaletteType = 5;
pub const PaletteTypeFixedHalftone8: PaletteType = 3;
pub const PaletteTypeOptimal: PaletteType = 1;
pub type PixelFormat = i32;
pub const PixelFormat16bppARGB1555: i32 = 397319;
pub const PixelFormat16bppGrayScale: i32 = 1052676;
pub const PixelFormat16bppRGB555: i32 = 135173;
pub const PixelFormat16bppRGB565: i32 = 135174;
pub const PixelFormat1bppIndexed: i32 = 196865;
pub const PixelFormat24bppRGB: i32 = 137224;
pub const PixelFormat32bppARGB: i32 = 2498570;
pub const PixelFormat32bppCMYK: i32 = 8207;
pub const PixelFormat32bppPARGB: i32 = 925707;
pub const PixelFormat32bppRGB: i32 = 139273;
pub const PixelFormat48bppRGB: i32 = 1060876;
pub const PixelFormat4bppIndexed: i32 = 197634;
pub const PixelFormat64bppARGB: i32 = 3424269;
pub const PixelFormat64bppPARGB: i32 = 1851406;
pub const PixelFormat8bppIndexed: i32 = 198659;
pub const PixelFormatAlpha: i32 = 262144;
pub const PixelFormatCanonical: i32 = 2097152;
pub const PixelFormatDontCare: i32 = 0;
pub const PixelFormatExtended: i32 = 1048576;
pub const PixelFormatGDI: i32 = 131072;
pub const PixelFormatIndexed: i32 = 65536;
pub const PixelFormatMax: i32 = 16;
pub const PixelFormatPAlpha: i32 = 524288;
pub const PixelFormatUndefined: i32 = 0;
pub const RED_SHIFT: i32 = 16;
