pub type GAMING_DEVICE_DEVICE_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: windows_sys::Win32::Gaming::GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
