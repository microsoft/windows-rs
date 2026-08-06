#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_HID_BackgroundAccess: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: super::DEVPROPID(8) };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_HID_IsReadOnly: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: super::DEVPROPID(4) };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_HID_ProductId: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: super::DEVPROPID(6) };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_HID_UsageId: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: super::DEVPROPID(3) };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_HID_UsagePage: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: super::DEVPROPID(2) };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_HID_VendorId: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: super::DEVPROPID(5) };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_HID_VersionNumber: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: super::DEVPROPID(7) };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_HID_WakeScreenOnInputCapable: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: super::DEVPROPID(9) };
pub const GUID_DEVINTERFACE_HID: windows_core::GUID = windows_core::GUID::from_u128(0x4d1e55b2_f16f_11cf_88cb_001111000030);
pub const GUID_HID_INTERFACE_HIDPARSE: windows_core::GUID = windows_core::GUID::from_u128(0xf5c315a5_69ac_4bc2_9279_d0b64576f44b);
pub const GUID_HID_INTERFACE_NOTIFY: windows_core::GUID = windows_core::GUID::from_u128(0x2c4e2e88_25e6_4c33_882f_3d82e6073681);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HID_COLLECTION_INFORMATION {
    pub DescriptorSize: u32,
    pub Polled: bool,
    pub Reserved1: [u8; 1],
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
impl Default for HID_COLLECTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HID_DRIVER_CONFIG {
    pub Size: u32,
    pub RingBufferSize: u32,
}
pub const HID_REVISION: i32 = 1;
pub const IOCTL_GET_NUM_DEVICE_INPUT_BUFFERS: i32 = 721312;
pub const IOCTL_GET_PHYSICAL_DESCRIPTOR: i32 = 721306;
pub const IOCTL_HID_DEVICERESET_NOTIFICATION: i32 = 721459;
pub const IOCTL_HID_DISABLE_SECURE_READ: i32 = 721423;
pub const IOCTL_HID_ENABLE_SECURE_READ: i32 = 721419;
pub const IOCTL_HID_ENABLE_WAKE_ON_SX: i32 = 721324;
pub const IOCTL_HID_FLUSH_QUEUE: i32 = 721303;
pub const IOCTL_HID_GET_COLLECTION_DESCRIPTOR: i32 = 721299;
pub const IOCTL_HID_GET_COLLECTION_INFORMATION: i32 = 721320;
pub const IOCTL_HID_GET_DRIVER_CONFIG: i32 = 721296;
pub const IOCTL_HID_GET_FEATURE: i32 = 721298;
pub const IOCTL_HID_GET_HARDWARE_ID: i32 = 721310;
pub const IOCTL_HID_GET_INDEXED_STRING: i32 = 721378;
pub const IOCTL_HID_GET_INPUT_REPORT: i32 = 721314;
pub const IOCTL_HID_GET_MANUFACTURER_STRING: i32 = 721338;
pub const IOCTL_HID_GET_MS_GENRE_DESCRIPTOR: i32 = 721382;
pub const IOCTL_HID_GET_OUTPUT_REPORT: i32 = 721318;
pub const IOCTL_HID_GET_POLL_FREQUENCY_MSEC: i32 = 721304;
pub const IOCTL_HID_GET_PRODUCT_STRING: i32 = 721342;
pub const IOCTL_HID_GET_SERIALNUMBER_STRING: i32 = 721346;
pub const IOCTL_HID_SET_DRIVER_CONFIG: i32 = 721300;
pub const IOCTL_HID_SET_FEATURE: i32 = 721297;
pub const IOCTL_HID_SET_OUTPUT_REPORT: i32 = 721301;
pub const IOCTL_HID_SET_POLL_FREQUENCY_MSEC: i32 = 721308;
pub const IOCTL_HID_SET_S0_IDLE_TIMEOUT: i32 = 721328;
pub const IOCTL_SET_NUM_DEVICE_INPUT_BUFFERS: i32 = 721316;
pub type PHID_COLLECTION_INFORMATION = *mut HID_COLLECTION_INFORMATION;
pub type PHID_DRIVER_CONFIG = *mut HID_DRIVER_CONFIG;
