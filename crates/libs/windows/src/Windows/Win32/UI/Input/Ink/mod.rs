pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = 2i32;
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = 1i32;
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
impl windows_core::TypeKind for INK_HIGH_CONTRAST_ADJUSTMENT {
    type TypeKind = windows_core::CopyType;
}
pub const InkD2DRenderer: windows_core::GUID = windows_core::GUID::from_u128(0x4044e60c_7b01_4671_a97c_04e0210a07a5);
pub const InkDesktopHost: windows_core::GUID = windows_core::GUID::from_u128(0x062584a6_f830_4bdc_a4d2_0a10ab062b1d);
