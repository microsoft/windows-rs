pub const ALLOW_PARTIAL_READS: u32 = 5;
pub const AUTO_CLEAR_STALL: u32 = 2;
pub const AUTO_FLUSH: u32 = 6;
pub const AUTO_SUSPEND: u32 = 129;
pub const DEVICE_SPEED: u32 = 1;
pub const FullSpeed: u32 = 2;
pub const HighSpeed: u32 = 3;
pub const IGNORE_SHORT_PACKETS: u32 = 4;
pub const LowSpeed: u32 = 1;
pub const MAXIMUM_TRANSFER_SIZE: u32 = 8;
pub const PIPE_TRANSFER_TIMEOUT: u32 = 3;
#[cfg(feature = "Win32_usb")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWINUSB_PIPE_INFORMATION(pub *mut WINUSB_PIPE_INFORMATION);
#[cfg(feature = "Win32_usb")]
impl PWINUSB_PIPE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usb")]
impl Default for PWINUSB_PIPE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usb")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWINUSB_PIPE_INFORMATION_EX(pub *mut WINUSB_PIPE_INFORMATION_EX);
#[cfg(feature = "Win32_usb")]
impl PWINUSB_PIPE_INFORMATION_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usb")]
impl Default for PWINUSB_PIPE_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RAW_IO: u32 = 7;
pub const RESET_PIPE_ON_RESUME: u32 = 9;
pub const SHORT_PACKET_TERMINATE: u32 = 1;
pub const SUSPEND_DELAY: u32 = 131;
#[repr(C)]
#[cfg(feature = "Win32_usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINUSB_PIPE_INFORMATION {
    pub PipeType: super::usb::USBD_PIPE_TYPE,
    pub PipeId: u8,
    pub MaximumPacketSize: u16,
    pub Interval: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINUSB_PIPE_INFORMATION_EX {
    pub PipeType: super::usb::USBD_PIPE_TYPE,
    pub PipeId: u8,
    pub MaximumPacketSize: u16,
    pub Interval: u8,
    pub MaximumBytesPerInterval: u32,
}
pub const WinUSB_TestGuid: windows_core::GUID = windows_core::GUID::from_u128(0xda812bff_12c3_46a2_8e2b_dbd3b7834c43);
