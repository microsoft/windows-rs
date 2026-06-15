pub type GAMING_DEVICE_DEVICE_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
pub type GAMING_DEVICE_VENDOR_ID = i32;
