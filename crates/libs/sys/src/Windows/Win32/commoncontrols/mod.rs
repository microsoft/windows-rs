windows_link::link!("comctl32.dll" "system" fn ImageList_CoCreateInstance(rclsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const ILDI_PURGE: i32 = 1;
pub const ILDI_QUERYACCESS: i32 = 8;
pub const ILDI_RESETACCESS: i32 = 4;
pub const ILDI_STANDBY: i32 = 2;
pub const ILDRF_IMAGELOWQUALITY: i32 = 1;
pub const ILDRF_OVERLAYLOWQUALITY: i32 = 16;
pub const ILFIP_ALWAYS: i32 = 0;
pub const ILFIP_FROMSTANDBY: i32 = 1;
pub const ILGOS_ALWAYS: i32 = 0;
pub const ILGOS_FROMSTANDBY: i32 = 1;
pub const ILIF_ALPHA: i32 = 1;
pub const ILIF_LOWQUALITY: i32 = 2;
pub const ILR_DEFAULT: i32 = 0;
pub const ILR_HORIZONTAL_CENTER: i32 = 1;
pub const ILR_HORIZONTAL_LEFT: i32 = 0;
pub const ILR_HORIZONTAL_RIGHT: i32 = 2;
pub const ILR_SCALE_ASPECTRATIO: i32 = 256;
pub const ILR_SCALE_CLIP: i32 = 0;
pub const ILR_VERTICAL_BOTTOM: i32 = 32;
pub const ILR_VERTICAL_CENTER: i32 = 16;
pub const ILR_VERTICAL_TOP: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGELISTSTATS {
    pub cbSize: u32,
    pub cAlloc: i32,
    pub cUsed: i32,
    pub cStandby: i32,
}
pub const ImageList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7c476ba2_02b1_48f4_8048_b24619ddc058);
