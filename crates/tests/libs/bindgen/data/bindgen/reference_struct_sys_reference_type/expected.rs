#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type GAMING_DEVICE_DEVICE_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: windows_sys::Win32::Gaming::GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
