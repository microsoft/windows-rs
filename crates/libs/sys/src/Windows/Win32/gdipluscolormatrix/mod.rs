pub type ColorAdjustType = i32;
pub const ColorAdjustTypeAny: ColorAdjustType = 6;
pub const ColorAdjustTypeBitmap: ColorAdjustType = 1;
pub const ColorAdjustTypeBrush: ColorAdjustType = 2;
pub const ColorAdjustTypeCount: ColorAdjustType = 5;
pub const ColorAdjustTypeDefault: ColorAdjustType = 0;
pub const ColorAdjustTypePen: ColorAdjustType = 3;
pub const ColorAdjustTypeText: ColorAdjustType = 4;
#[repr(C, packed(8))]
#[cfg(feature = "gdiplustypes")]
#[derive(Clone, Copy)]
pub struct ColorMatrix {
    pub m: [[super::REAL; 5]; 5],
}
#[cfg(feature = "gdiplustypes")]
impl Default for ColorMatrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ColorMatrixFlags = i32;
pub const ColorMatrixFlagsAltGray: ColorMatrixFlags = 2;
pub const ColorMatrixFlagsDefault: ColorMatrixFlags = 0;
pub const ColorMatrixFlagsSkipGrays: ColorMatrixFlags = 1;
pub type HistogramFormat = i32;
pub const HistogramFormatA: HistogramFormat = 7;
pub const HistogramFormatARGB: HistogramFormat = 0;
pub const HistogramFormatB: HistogramFormat = 4;
pub const HistogramFormatG: HistogramFormat = 5;
pub const HistogramFormatGray: HistogramFormat = 3;
pub const HistogramFormatPARGB: HistogramFormat = 1;
pub const HistogramFormatR: HistogramFormat = 6;
pub const HistogramFormatRGB: HistogramFormat = 2;
