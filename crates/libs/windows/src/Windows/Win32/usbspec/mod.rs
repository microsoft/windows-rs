pub const BMREQUEST_CLASS: u32 = 1;
pub const BMREQUEST_DEVICE_TO_HOST: u32 = 1;
pub const BMREQUEST_HOST_TO_DEVICE: u32 = 0;
pub const BMREQUEST_STANDARD: u32 = 0;
pub const BMREQUEST_TO_DEVICE: u32 = 0;
pub const BMREQUEST_TO_ENDPOINT: u32 = 2;
pub const BMREQUEST_TO_INTERFACE: u32 = 1;
pub const BMREQUEST_TO_OTHER: u32 = 3;
pub const BMREQUEST_VENDOR: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union BM_REQUEST_TYPE {
    pub s: BM_REQUEST_TYPE_0,
    pub B: u8,
}
impl Default for BM_REQUEST_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BM_REQUEST_TYPE_0 {
    pub _bitfield: u8,
}
impl BM_REQUEST_TYPE_0 {
    pub fn Recipient(&self) -> u8 {
        (self._bitfield << 6) >> 6
    }
    pub fn set_Recipient(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Reserved(&self) -> u8 {
        (self._bitfield << 3) >> 5
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 2)) | ((value & 7) << 2);
    }
    pub fn Type(&self) -> u8 {
        (self._bitfield << 1) >> 6
    }
    pub fn set_Type(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 5)) | ((value & 3) << 5);
    }
    pub fn Dir(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Dir(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wMaxPacketSize: u16,
    pub dwBytesPerInterval: u32,
}
pub const EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: u32 = 18;
pub const GUID_USB_MSOS20_PLATFORM_CAPABILITY_ID: windows_core::GUID = windows_core::GUID::from_u128(0xd8dd60df_4589_4cc7_9cd2_659d9e648a9f);
pub const MAXIMUM_USB_STRING_LENGTH: u32 = 255;
pub type PBM_REQUEST_TYPE = *mut BM_REQUEST_TYPE;
pub type PEUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR = *mut EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR;
pub const PORT_LINK_STATE_COMPLIANCE_MODE: u32 = 10;
pub const PORT_LINK_STATE_DISABLED: u32 = 4;
pub const PORT_LINK_STATE_HOT_RESET: u32 = 9;
pub const PORT_LINK_STATE_INACTIVE: u32 = 6;
pub const PORT_LINK_STATE_LOOPBACK: u32 = 11;
pub const PORT_LINK_STATE_POLLING: u32 = 7;
pub const PORT_LINK_STATE_RECOVERY: u32 = 8;
pub const PORT_LINK_STATE_RX_DETECT: u32 = 5;
pub const PORT_LINK_STATE_TEST_MODE: u32 = 11;
pub const PORT_LINK_STATE_U0: u32 = 0;
pub const PORT_LINK_STATE_U1: u32 = 1;
pub const PORT_LINK_STATE_U2: u32 = 2;
pub const PORT_LINK_STATE_U3: u32 = 3;
pub type PUSB_20_PORT_CHANGE = *mut USB_20_PORT_CHANGE;
pub type PUSB_20_PORT_STATUS = *mut USB_20_PORT_STATUS;
pub type PUSB_30_HUB_DESCRIPTOR = *mut USB_30_HUB_DESCRIPTOR;
pub type PUSB_30_PORT_CHANGE = *mut USB_30_PORT_CHANGE;
pub type PUSB_30_PORT_STATUS = *mut USB_30_PORT_STATUS;
pub type PUSB_BOS_DESCRIPTOR = *mut USB_BOS_DESCRIPTOR;
pub type PUSB_COMMON_DESCRIPTOR = *mut USB_COMMON_DESCRIPTOR;
pub type PUSB_CONFIGURATION_DESCRIPTOR = *mut USB_CONFIGURATION_DESCRIPTOR;
pub type PUSB_CONFIGURATION_POWER_DESCRIPTOR = *mut USB_CONFIGURATION_POWER_DESCRIPTOR;
pub type PUSB_DEFAULT_PIPE_SETUP_PACKET = *mut USB_DEFAULT_PIPE_SETUP_PACKET;
pub type PUSB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED = *mut USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED;
pub type PUSB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR;
pub type PUSB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR = *mut USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR;
pub type PUSB_DEVICE_DESCRIPTOR = *mut USB_DEVICE_DESCRIPTOR;
pub type PUSB_DEVICE_QUALIFIER_DESCRIPTOR = *mut USB_DEVICE_QUALIFIER_DESCRIPTOR;
pub type PUSB_DEVICE_STATUS = *mut USB_DEVICE_STATUS;
pub type PUSB_ENDPOINT_DESCRIPTOR = *mut USB_ENDPOINT_DESCRIPTOR;
pub type PUSB_ENDPOINT_STATUS = *mut USB_ENDPOINT_STATUS;
pub type PUSB_FUNCTION_SUSPEND_OPTIONS = *mut USB_FUNCTION_SUSPEND_OPTIONS;
pub type PUSB_HIGH_SPEED_MAXPACKET = *mut USB_HIGH_SPEED_MAXPACKET;
pub type PUSB_HUB_30_PORT_REMOTE_WAKE_MASK = *mut USB_HUB_30_PORT_REMOTE_WAKE_MASK;
pub type PUSB_HUB_CHANGE = *mut USB_HUB_CHANGE;
pub type PUSB_HUB_DESCRIPTOR = *mut USB_HUB_DESCRIPTOR;
pub type PUSB_HUB_STATUS = *mut USB_HUB_STATUS;
pub type PUSB_HUB_STATUS_AND_CHANGE = *mut USB_HUB_STATUS_AND_CHANGE;
pub type PUSB_INTERFACE_ASSOCIATION_DESCRIPTOR = *mut USB_INTERFACE_ASSOCIATION_DESCRIPTOR;
pub type PUSB_INTERFACE_DESCRIPTOR = *mut USB_INTERFACE_DESCRIPTOR;
pub type PUSB_INTERFACE_POWER_DESCRIPTOR = *mut USB_INTERFACE_POWER_DESCRIPTOR;
pub type PUSB_INTERFACE_STATUS = *mut USB_INTERFACE_STATUS;
pub type PUSB_PORT_CHANGE = *mut USB_PORT_CHANGE;
pub type PUSB_PORT_EXT_STATUS = *mut USB_PORT_EXT_STATUS;
pub type PUSB_PORT_EXT_STATUS_AND_CHANGE = *mut USB_PORT_EXT_STATUS_AND_CHANGE;
pub type PUSB_PORT_STATUS = *mut USB_PORT_STATUS;
pub type PUSB_PORT_STATUS_AND_CHANGE = *mut USB_PORT_STATUS_AND_CHANGE;
pub type PUSB_STRING_DESCRIPTOR = *mut USB_STRING_DESCRIPTOR;
pub type PUSB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR = *mut USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR;
pub type PUSB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR = *mut USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR;
pub const USB_20_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: u32 = 252;
pub const USB_20_HUB_DESCRIPTOR_TYPE: u32 = 41;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_20_PORT_CHANGE {
    pub AsUshort16: u16,
    pub Anonymous: USB_20_PORT_CHANGE_0,
}
impl Default for USB_20_PORT_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_20_PORT_CHANGE_0 {
    pub _bitfield: u16,
}
impl USB_20_PORT_CHANGE_0 {
    pub fn ConnectStatusChange(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ConnectStatusChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn PortEnableDisableChange(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_PortEnableDisableChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn SuspendChange(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_SuspendChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn OverCurrentIndicatorChange(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_OverCurrentIndicatorChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn ResetChange(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_ResetChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Reserved2(&self) -> u16 {
        self._bitfield >> 5
    }
    pub fn set_Reserved2(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(2047 << 5)) | ((value & 2047) << 5);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_20_PORT_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_20_PORT_STATUS_0,
}
impl Default for USB_20_PORT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_20_PORT_STATUS_0 {
    pub _bitfield: u16,
}
impl USB_20_PORT_STATUS_0 {
    pub fn CurrentConnectStatus(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CurrentConnectStatus(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn PortEnabledDisabled(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_PortEnabledDisabled(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Suspend(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Suspend(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn OverCurrent(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_OverCurrent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn Reset(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Reset(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn L1(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_L1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn Reserved0(&self) -> u16 {
        (self._bitfield << 8) >> 14
    }
    pub fn set_Reserved0(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
    pub fn PortPower(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_PortPower(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u16) << 8);
    }
    pub fn LowSpeedDeviceAttached(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_LowSpeedDeviceAttached(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u16) << 9);
    }
    pub fn HighSpeedDeviceAttached(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_HighSpeedDeviceAttached(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u16) << 10);
    }
    pub fn PortTestMode(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_PortTestMode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u16) << 11);
    }
    pub fn PortIndicatorControl(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_PortIndicatorControl(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u16) << 12);
    }
    pub fn Reserved1(&self) -> u16 {
        self._bitfield >> 13
    }
    pub fn set_Reserved1(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(7 << 13)) | ((value & 7) << 13);
    }
}
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: u32 = 204;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_MASK: u32 = 48;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_NOTIFICATION: u32 = 16;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_PERIODIC: u32 = 0;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED10: u32 = 32;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED11: u32 = 48;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_30_HUB_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bNumberOfPorts: u8,
    pub wHubCharacteristics: u16,
    pub bPowerOnToPowerGood: u8,
    pub bHubControlCurrent: u8,
    pub bHubHdrDecLat: u8,
    pub wHubDelay: u16,
    pub DeviceRemovable: u16,
}
pub const USB_30_HUB_DESCRIPTOR_TYPE: u32 = 42;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_30_PORT_CHANGE {
    pub AsUshort16: u16,
    pub Anonymous: USB_30_PORT_CHANGE_0,
}
impl Default for USB_30_PORT_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_30_PORT_CHANGE_0 {
    pub _bitfield: u16,
}
impl USB_30_PORT_CHANGE_0 {
    pub fn ConnectStatusChange(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ConnectStatusChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved2(&self) -> u16 {
        (self._bitfield << 13) >> 14
    }
    pub fn set_Reserved2(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(3 << 1)) | ((value & 3) << 1);
    }
    pub fn OverCurrentIndicatorChange(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_OverCurrentIndicatorChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn ResetChange(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_ResetChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn BHResetChange(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_BHResetChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn PortLinkStateChange(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_PortLinkStateChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn PortConfigErrorChange(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_PortConfigErrorChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn Reserved3(&self) -> u16 {
        self._bitfield >> 8
    }
    pub fn set_Reserved3(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_30_PORT_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_30_PORT_STATUS_0,
}
impl Default for USB_30_PORT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_30_PORT_STATUS_0 {
    pub _bitfield: u16,
}
impl USB_30_PORT_STATUS_0 {
    pub fn CurrentConnectStatus(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CurrentConnectStatus(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn PortEnabledDisabled(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_PortEnabledDisabled(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Reserved0(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Reserved0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn OverCurrent(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_OverCurrent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn Reset(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Reset(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn PortLinkState(&self) -> u16 {
        (self._bitfield << 7) >> 12
    }
    pub fn set_PortLinkState(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 5)) | ((value & 15) << 5);
    }
    pub fn PortPower(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_PortPower(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u16) << 9);
    }
    pub fn NegotiatedDeviceSpeed(&self) -> u16 {
        (self._bitfield << 3) >> 13
    }
    pub fn set_NegotiatedDeviceSpeed(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(7 << 10)) | ((value & 7) << 10);
    }
    pub fn Reserved1(&self) -> u16 {
        self._bitfield >> 13
    }
    pub fn set_Reserved1(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(7 << 13)) | ((value & 7) << 13);
    }
}
pub const USB_ALLOW_FIRMWARE_UPDATE: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_BOS_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumDeviceCaps: u8,
}
pub const USB_BOS_DESCRIPTOR_TYPE: u32 = 15;
pub const USB_CHARGING_POLICY_DEFAULT: u32 = 0;
pub const USB_CHARGING_POLICY_ICCHPF: u32 = 1;
pub const USB_CHARGING_POLICY_ICCLPF: u32 = 2;
pub const USB_CHARGING_POLICY_NO_POWER: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_COMMON_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_CONFIGURATION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u8,
}
pub const USB_CONFIGURATION_DESCRIPTOR_TYPE: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_CONFIGURATION_POWER_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub SelfPowerConsumedD0: [u8; 3],
    pub bPowerSummaryId: u8,
    pub bBusPowerSavingD1: u8,
    pub bSelfPowerSavingD1: u8,
    pub bBusPowerSavingD2: u8,
    pub bSelfPowerSavingD2: u8,
    pub bBusPowerSavingD3: u8,
    pub bSelfPowerSavingD3: u8,
    pub TransitionTimeFromD1: u16,
    pub TransitionTimeFromD2: u16,
    pub TransitionTimeFromD3: u16,
}
impl Default for USB_CONFIGURATION_POWER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_CONFIG_BUS_POWERED: u32 = 128;
pub const USB_CONFIG_POWERED_MASK: u32 = 192;
pub const USB_CONFIG_POWER_DESCRIPTOR_TYPE: u32 = 7;
pub const USB_CONFIG_REMOTE_WAKEUP: u32 = 32;
pub const USB_CONFIG_RESERVED: u32 = 31;
pub const USB_CONFIG_SELF_POWERED: u32 = 64;
pub const USB_DEBUG_DESCRIPTOR_TYPE: u32 = 10;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DEFAULT_PIPE_SETUP_PACKET {
    pub bmRequestType: BM_REQUEST_TYPE,
    pub bRequest: u8,
    pub wValue: USB_DEFAULT_PIPE_SETUP_PACKET_0,
    pub wIndex: USB_DEFAULT_PIPE_SETUP_PACKET_1,
    pub wLength: u16,
}
impl Default for USB_DEFAULT_PIPE_SETUP_PACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEFAULT_PIPE_SETUP_PACKET_0 {
    pub Anonymous: USB_DEFAULT_PIPE_SETUP_PACKET_0_0,
    pub W: u16,
}
impl Default for USB_DEFAULT_PIPE_SETUP_PACKET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_DEFAULT_PIPE_SETUP_PACKET_0_0 {
    pub LowByte: u8,
    pub HiByte: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEFAULT_PIPE_SETUP_PACKET_1 {
    pub Anonymous: USB_DEFAULT_PIPE_SETUP_PACKET_1_0,
    pub W: u16,
}
impl Default for USB_DEFAULT_PIPE_SETUP_PACKET_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_DEFAULT_PIPE_SETUP_PACKET_1_0 {
    pub LowByte: u8,
    pub HiByte: u8,
}
pub const USB_DEVICE_CAPABILITY_BATTERY_INFO: u32 = 7;
pub const USB_DEVICE_CAPABILITY_BILLBOARD: u32 = 13;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub iAddtionalInfoURL: u8,
    pub bNumberOfAlternateModes: u8,
    pub bPreferredAlternateMode: u8,
    pub VconnPower: USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0,
    pub bmConfigured: [u8; 32],
    pub bReserved: u32,
    pub AlternateMode: [USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1; 1],
}
impl Default for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0 {
    pub AsUshort: u16,
    pub Anonymous: USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0_0,
}
impl Default for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0_0 {
    pub _bitfield: u16,
}
impl USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0_0 {
    pub fn VConnPowerNeededForFullFunctionality(&self) -> u16 {
        (self._bitfield << 13) >> 13
    }
    pub fn set_VConnPowerNeededForFullFunctionality(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 1) >> 4
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(4095 << 3)) | ((value & 4095) << 3);
    }
    pub fn NoVconnPowerRequired(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_NoVconnPowerRequired(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1 {
    pub wSVID: u16,
    pub bAlternateMode: u8,
    pub iAlternateModeSetting: u8,
}
pub const USB_DEVICE_CAPABILITY_CONTAINER_ID: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub ContainerID: [u8; 16],
}
impl Default for USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_DEVICE_CAPABILITY_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
}
pub const USB_DEVICE_CAPABILITY_DESCRIPTOR_TYPE: u32 = 16;
pub const USB_DEVICE_CAPABILITY_FIRMWARE_STATUS: u32 = 17;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bcdDescriptorVersion: u8,
    pub bmAttributes: USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0,
}
impl Default for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0 {
    pub AsUlong: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0_0,
}
impl Default for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0_0 {
    pub fn GetFirmwareImageHashSupport(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_GetFirmwareImageHashSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn DisallowFirmwareUpdateSupport(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_DisallowFirmwareUpdateSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
pub const USB_DEVICE_CAPABILITY_MAX_U1_LATENCY: u32 = 10;
pub const USB_DEVICE_CAPABILITY_MAX_U2_LATENCY: u32 = 2047;
pub const USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT: u32 = 8;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub bmCapabilities: USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0,
    pub wMinVoltage: u16,
    pub wMaxVoltage: u16,
    pub wReserved: u16,
    pub dwMaxOperatingPower: u32,
    pub dwMaxPeakPower: u32,
    pub dwMaxPeakPowerTime: u32,
}
impl Default for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0 {
    pub AsUshort: u16,
    pub Anonymous: USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0_0,
}
impl Default for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0_0 {
    pub _bitfield: u16,
}
impl USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0_0 {
    pub fn BatteryCharging(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_BatteryCharging(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn USBPowerDelivery(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_USBPowerDelivery(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn USBTypeCCurrent(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_USBTypeCCurrent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
}
pub const USB_DEVICE_CAPABILITY_PD_PROVIDER_PORT: u32 = 9;
pub const USB_DEVICE_CAPABILITY_PLATFORM: u32 = 5;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub PlatformCapabilityUuid: windows_core::GUID,
    pub CapabililityData: [u8; 1],
}
impl Default for USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_DEVICE_CAPABILITY_POWER_DELIVERY: u32 = 6;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub bmAttributes: USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0,
    pub bmProviderPorts: u16,
    pub bmConsumerPorts: u16,
    pub bcdBCVersion: u16,
    pub bcdPDVersion: u16,
    pub bcdUSBTypeCVersion: u16,
}
impl Default for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0 {
    pub AsUlong: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0_0,
}
impl Default for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0_0 {
    pub fn Reserved1(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Reserved1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn BatteryCharging(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_BatteryCharging(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn USBPowerDelivery(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_USBPowerDelivery(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn Provider(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Provider(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Consumer(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Consumer(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn ChargingPolicy(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_ChargingPolicy(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u32) << 5);
    }
    pub fn TypeCCurrent(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_TypeCCurrent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u32) << 6);
    }
    pub fn Reserved2(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Reserved2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u32) << 7);
    }
    pub fn ACSupply(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_ACSupply(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn Battery(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_Battery(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u32) << 9);
    }
    pub fn Other(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_Other(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u32) << 10);
    }
    pub fn NumBatteries(&self) -> u32 {
        (self._bitfield << 18) >> 29
    }
    pub fn set_NumBatteries(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 11)) | ((value & 7) << 11);
    }
    pub fn UsesVbus(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_UsesVbus(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u32) << 14);
    }
    pub fn Reserved3(&self) -> u32 {
        self._bitfield >> 15
    }
    pub fn set_Reserved3(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(131071 << 15)) | ((value & 131071) << 15);
    }
}
pub const USB_DEVICE_CAPABILITY_PRECISION_TIME_MEASUREMENT: u32 = 11;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED {
    pub AsUlong32: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_0,
}
impl Default for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_0 {
    pub _bitfield: u32,
}
impl USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_0 {
    pub fn SublinkSpeedAttrID(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_SublinkSpeedAttrID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn LaneSpeedExponent(&self) -> u32 {
        (self._bitfield << 26) >> 30
    }
    pub fn set_LaneSpeedExponent(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn SublinkTypeMode(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_SublinkTypeMode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u32) << 6);
    }
    pub fn SublinkTypeDir(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_SublinkTypeDir(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u32) << 7);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 18) >> 26
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(63 << 8)) | ((value & 63) << 8);
    }
    pub fn LinkProtocol(&self) -> u32 {
        (self._bitfield << 16) >> 30
    }
    pub fn set_LinkProtocol(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 14)) | ((value & 3) << 14);
    }
    pub fn LaneSpeedMantissa(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_LaneSpeedMantissa(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_RX: u32 = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_TX: u32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_BPS: u32 = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_GBPS: u32 = 3;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_KBPS: u32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_MBPS: u32 = 2;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_ASYMMETRIC: u32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_SYMMETRIC: u32 = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SS: u32 = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SSP: u32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB: u32 = 10;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub bmAttributes: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0,
    pub wFunctionalitySupport: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1,
    pub wReserved: u16,
    pub bmSublinkSpeedAttr: [USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED; 1],
}
impl Default for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0 {
    pub AsUlong: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0_0,
}
impl Default for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0_0 {
    pub fn SublinkSpeedAttrCount(&self) -> u32 {
        (self._bitfield << 27) >> 27
    }
    pub fn set_SublinkSpeedAttrCount(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !31) | (value & 31);
    }
    pub fn SublinkSpeedIDCount(&self) -> u32 {
        (self._bitfield << 23) >> 28
    }
    pub fn set_SublinkSpeedIDCount(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 5)) | ((value & 15) << 5);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 9
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(8388607 << 9)) | ((value & 8388607) << 9);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1 {
    pub AsUshort: u16,
    pub Anonymous: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1_0,
}
impl Default for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1_0 {
    pub _bitfield: u16,
}
impl USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1_0 {
    pub fn SublinkSpeedAttrID(&self) -> u16 {
        (self._bitfield << 12) >> 12
    }
    pub fn set_SublinkSpeedAttrID(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 8) >> 12
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn MinRxLaneCount(&self) -> u16 {
        (self._bitfield << 4) >> 12
    }
    pub fn set_MinRxLaneCount(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn MinTxLaneCount(&self) -> u16 {
        self._bitfield >> 12
    }
    pub fn set_MinTxLaneCount(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
}
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_LTM_CAPABLE: u32 = 2;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_RESERVED_MASK: u32 = 253;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_FULL: u32 = 2;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_HIGH: u32 = 4;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_LOW: u32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_RESERVED_MASK: u32 = 65520;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_SUPER: u32 = 8;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U1_DEVICE_EXIT_MAX_VALUE: u32 = 10;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U2_DEVICE_EXIT_MAX_VALUE: u32 = 2047;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_USB: u32 = 3;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bmAttributes: u8,
    pub wSpeedsSupported: u16,
    pub bFunctionalitySupport: u8,
    pub bU1DevExitLat: u8,
    pub wU2DevExitLat: u16,
}
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION: u32 = 2;
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION_BMATTRIBUTES_RESERVED_MASK: u32 = 4294901985;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bmAttributes: USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0,
}
impl Default for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0 {
    pub AsUlong: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0_0,
}
impl Default for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0_0 {
    pub fn Reserved(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Reserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn LPMCapable(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_LPMCapable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn BESLAndAlternateHIRDSupported(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_BESLAndAlternateHIRDSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn BaselineBESLValid(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_BaselineBESLValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn DeepBESLValid(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_DeepBESLValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn Reserved1(&self) -> u32 {
        (self._bitfield << 24) >> 29
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn BaselineBESL(&self) -> u32 {
        (self._bitfield << 20) >> 28
    }
    pub fn set_BaselineBESL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn DeepBESL(&self) -> u32 {
        (self._bitfield << 16) >> 28
    }
    pub fn set_DeepBESL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
    pub fn Reserved2(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved2(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
pub const USB_DEVICE_CAPABILITY_WIRELESS_USB: u32 = 1;
pub const USB_DEVICE_CLASS_APPLICATION_SPECIFIC: u32 = 254;
pub const USB_DEVICE_CLASS_AUDIO: u32 = 1;
pub const USB_DEVICE_CLASS_AUDIO_VIDEO: u32 = 16;
pub const USB_DEVICE_CLASS_BILLBOARD: u32 = 17;
pub const USB_DEVICE_CLASS_CDC_DATA: u32 = 10;
pub const USB_DEVICE_CLASS_COMMUNICATIONS: u32 = 2;
pub const USB_DEVICE_CLASS_CONTENT_SECURITY: u32 = 13;
pub const USB_DEVICE_CLASS_DIAGNOSTIC_DEVICE: u32 = 220;
pub const USB_DEVICE_CLASS_HUB: u32 = 9;
pub const USB_DEVICE_CLASS_HUMAN_INTERFACE: u32 = 3;
pub const USB_DEVICE_CLASS_IMAGE: u32 = 6;
pub const USB_DEVICE_CLASS_MISCELLANEOUS: u32 = 239;
pub const USB_DEVICE_CLASS_MONITOR: u32 = 4;
pub const USB_DEVICE_CLASS_PERSONAL_HEALTHCARE: u32 = 15;
pub const USB_DEVICE_CLASS_PHYSICAL_INTERFACE: u32 = 5;
pub const USB_DEVICE_CLASS_POWER: u32 = 6;
pub const USB_DEVICE_CLASS_PRINTER: u32 = 7;
pub const USB_DEVICE_CLASS_RESERVED: u32 = 0;
pub const USB_DEVICE_CLASS_SMART_CARD: u32 = 11;
pub const USB_DEVICE_CLASS_STORAGE: u32 = 8;
pub const USB_DEVICE_CLASS_VENDOR_SPECIFIC: u32 = 255;
pub const USB_DEVICE_CLASS_VIDEO: u32 = 14;
pub const USB_DEVICE_CLASS_WIRELESS_CONTROLLER: u32 = 224;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}
pub const USB_DEVICE_DESCRIPTOR_TYPE: u32 = 1;
pub const USB_DEVICE_FIRMWARE_HASH_LENGTH: u32 = 32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_QUALIFIER_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub bNumConfigurations: u8,
    pub bReserved: u8,
}
pub const USB_DEVICE_QUALIFIER_DESCRIPTOR_TYPE: u32 = 6;
pub type USB_DEVICE_SPEED = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_DEVICE_STATUS_0,
}
impl Default for USB_DEVICE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_STATUS_0 {
    pub _bitfield: u16,
}
impl USB_DEVICE_STATUS_0 {
    pub fn SelfPowered(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_SelfPowered(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn RemoteWakeup(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_RemoteWakeup(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn U1Enable(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_U1Enable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn U2Enable(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_U2Enable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn LtmEnable(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_LtmEnable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(2047 << 5)) | ((value & 2047) << 5);
    }
}
pub type USB_DEVICE_TYPE = i32;
pub const USB_DISALLOW_FIRMWARE_UPDATE: u32 = 0;
pub const USB_ENDPOINT_ADDRESS_MASK: u32 = 15;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_ENDPOINT_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}
pub const USB_ENDPOINT_DESCRIPTOR_TYPE: u32 = 5;
pub const USB_ENDPOINT_DIRECTION_MASK: u32 = 128;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_ENDPOINT_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_ENDPOINT_STATUS_0,
}
impl Default for USB_ENDPOINT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_ENDPOINT_STATUS_0 {
    pub _bitfield: u16,
}
impl USB_ENDPOINT_STATUS_0 {
    pub fn Halt(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Halt(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
pub const USB_ENDPOINT_SUPERSPEED_BULK_MAX_PACKET_SIZE: u32 = 1024;
pub const USB_ENDPOINT_SUPERSPEED_CONTROL_MAX_PACKET_SIZE: u32 = 512;
pub const USB_ENDPOINT_SUPERSPEED_INTERRUPT_MAX_PACKET_SIZE: u32 = 1024;
pub const USB_ENDPOINT_SUPERSPEED_ISO_MAX_PACKET_SIZE: u32 = 1024;
pub const USB_ENDPOINT_TYPE_BULK: u32 = 2;
pub const USB_ENDPOINT_TYPE_BULK_RESERVED_MASK: u32 = 252;
pub const USB_ENDPOINT_TYPE_CONTROL: u32 = 0;
pub const USB_ENDPOINT_TYPE_CONTROL_RESERVED_MASK: u32 = 252;
pub const USB_ENDPOINT_TYPE_INTERRUPT: u32 = 3;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS: u32 = 1;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_RESERVED_MASK: u32 = 192;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ADAPTIVE: u32 = 8;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ASYNCHRONOUS: u32 = 4;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_MASK: u32 = 12;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_NO_SYNCHRONIZATION: u32 = 0;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_SYNCHRONOUS: u32 = 12;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_DATA_ENDOINT: u32 = 0;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_FEEDBACK_ENDPOINT: u32 = 16;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_IMPLICIT_FEEDBACK_DATA_ENDPOINT: u32 = 32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_MASK: u32 = 48;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_RESERVED: u32 = 48;
pub const USB_ENDPOINT_TYPE_MASK: u32 = 3;
pub const USB_FEATURE_BATTERY_WAKE_MASK: u32 = 40;
pub const USB_FEATURE_CHARGING_POLICY: u32 = 54;
pub const USB_FEATURE_ENDPOINT_STALL: u32 = 0;
pub const USB_FEATURE_FUNCTION_SUSPEND: u32 = 0;
pub const USB_FEATURE_INTERFACE_POWER_D0: u32 = 2;
pub const USB_FEATURE_INTERFACE_POWER_D1: u32 = 3;
pub const USB_FEATURE_INTERFACE_POWER_D2: u32 = 4;
pub const USB_FEATURE_INTERFACE_POWER_D3: u32 = 5;
pub const USB_FEATURE_LDM_ENABLE: u32 = 53;
pub const USB_FEATURE_LTM_ENABLE: u32 = 50;
pub const USB_FEATURE_OS_IS_PD_AWARE: u32 = 41;
pub const USB_FEATURE_POLICY_MODE: u32 = 42;
pub const USB_FEATURE_REMOTE_WAKEUP: u32 = 1;
pub const USB_FEATURE_TEST_MODE: u32 = 2;
pub const USB_FEATURE_U1_ENABLE: u32 = 48;
pub const USB_FEATURE_U2_ENABLE: u32 = 49;
#[repr(C)]
#[derive(Clone, Copy)]
pub union USB_FUNCTION_SUSPEND_OPTIONS {
    pub AsUchar: u8,
    pub Anonymous: USB_FUNCTION_SUSPEND_OPTIONS_0,
}
impl Default for USB_FUNCTION_SUSPEND_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_FUNCTION_SUSPEND_OPTIONS_0 {
    pub _bitfield: u8,
}
impl USB_FUNCTION_SUSPEND_OPTIONS_0 {
    pub fn PowerState(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PowerState(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn RemoteWakeEnabled(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_RemoteWakeEnabled(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
pub const USB_GETSTATUS_LTM_ENABLE: u32 = 16;
pub const USB_GETSTATUS_REMOTE_WAKEUP_ENABLED: u32 = 2;
pub const USB_GETSTATUS_SELF_POWERED: u32 = 1;
pub const USB_GETSTATUS_U1_ENABLE: u32 = 4;
pub const USB_GETSTATUS_U2_ENABLE: u32 = 8;
pub const USB_GET_FIRMWARE_ALLOWED_OR_DISALLOWED_STATE: u32 = 0;
pub const USB_GET_FIRMWARE_HASH: u32 = 1;
pub const USB_HIGHSPEED_EUSB2_ISOCHRONOUS_MAX_BYTESPERINTERVAL: u32 = 6144;
pub const USB_HIGHSPEED_EUSB2_ISOCHRONOUS_MIN_BYTESPERINTERVAL: u32 = 3073;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_HIGH_SPEED_MAXPACKET {
    pub us: u16,
}
impl Default for USB_HIGH_SPEED_MAXPACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_HIGH_SPEED_MAXPACKET_0 {
    pub _bitfield: u16,
}
impl USB_HIGH_SPEED_MAXPACKET_0 {
    pub fn MaxPacket(&self) -> u16 {
        (self._bitfield << 5) >> 5
    }
    pub fn set_MaxPacket(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !2047) | (value & 2047);
    }
    pub fn HSmux(&self) -> u16 {
        (self._bitfield << 3) >> 14
    }
    pub fn set_HSmux(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(3 << 11)) | ((value & 3) << 11);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 13
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(7 << 13)) | ((value & 7) << 13);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union USB_HUB_30_PORT_REMOTE_WAKE_MASK {
    pub AsUchar8: u8,
    pub Anonymous: USB_HUB_30_PORT_REMOTE_WAKE_MASK_0,
}
impl Default for USB_HUB_30_PORT_REMOTE_WAKE_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_HUB_30_PORT_REMOTE_WAKE_MASK_0 {
    pub _bitfield: u8,
}
impl USB_HUB_30_PORT_REMOTE_WAKE_MASK_0 {
    pub fn ConnectRemoteWakeEnable(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ConnectRemoteWakeEnable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn DisconnectRemoteWakeEnable(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_DisconnectRemoteWakeEnable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn OverCurrentRemoteWakeEnable(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_OverCurrentRemoteWakeEnable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Reserved0(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved0(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_HUB_CHANGE {
    pub AsUshort16: u16,
    pub Anonymous: USB_HUB_CHANGE_0,
}
impl Default for USB_HUB_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_CHANGE_0 {
    pub _bitfield: u16,
}
impl USB_HUB_CHANGE_0 {
    pub fn LocalPowerChange(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_LocalPowerChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn OverCurrentChange(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_OverCurrentChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_HUB_DESCRIPTOR {
    pub bDescriptorLength: u8,
    pub bDescriptorType: u8,
    pub bNumberOfPorts: u8,
    pub wHubCharacteristics: u16,
    pub bPowerOnToPowerGood: u8,
    pub bHubControlCurrent: u8,
    pub bRemoveAndPowerMask: [u8; 64],
}
impl Default for USB_HUB_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_HUB_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_HUB_STATUS_0,
}
impl Default for USB_HUB_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_STATUS_0 {
    pub _bitfield: u16,
}
impl USB_HUB_STATUS_0 {
    pub fn LocalPowerLost(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_LocalPowerLost(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn OverCurrent(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_OverCurrent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_HUB_STATUS_AND_CHANGE {
    pub AsUlong32: u32,
    pub Anonymous: USB_HUB_STATUS_AND_CHANGE_0,
}
impl Default for USB_HUB_STATUS_AND_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USB_HUB_STATUS_AND_CHANGE_0 {
    pub HubStatus: USB_HUB_STATUS,
    pub HubChange: USB_HUB_CHANGE,
}
impl Default for USB_HUB_STATUS_AND_CHANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_INTERFACE_ASSOCIATION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bFirstInterface: u8,
    pub bInterfaceCount: u8,
    pub bFunctionClass: u8,
    pub bFunctionSubClass: u8,
    pub bFunctionProtocol: u8,
    pub iFunction: u8,
}
pub const USB_INTERFACE_ASSOCIATION_DESCRIPTOR_TYPE: u32 = 11;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_INTERFACE_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}
pub const USB_INTERFACE_DESCRIPTOR_TYPE: u32 = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_INTERFACE_POWER_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bmCapabilitiesFlags: u8,
    pub bBusPowerSavingD1: u8,
    pub bSelfPowerSavingD1: u8,
    pub bBusPowerSavingD2: u8,
    pub bSelfPowerSavingD2: u8,
    pub bBusPowerSavingD3: u8,
    pub bSelfPowerSavingD3: u8,
    pub TransitionTimeFromD1: u16,
    pub TransitionTimeFromD2: u16,
    pub TransitionTimeFromD3: u16,
}
pub const USB_INTERFACE_POWER_DESCRIPTOR_TYPE: u32 = 8;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_INTERFACE_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_INTERFACE_STATUS_0,
}
impl Default for USB_INTERFACE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_INTERFACE_STATUS_0 {
    pub _bitfield: u16,
}
impl USB_INTERFACE_STATUS_0 {
    pub fn RemoteWakeupCapable(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_RemoteWakeupCapable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn RemoteWakeupEnabled(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_RemoteWakeupEnabled(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
}
pub const USB_OTG_DESCRIPTOR_TYPE: u32 = 9;
pub const USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_TYPE: u32 = 7;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_PORT_CHANGE {
    pub AsUshort16: u16,
    pub Usb20PortChange: USB_20_PORT_CHANGE,
    pub Usb30PortChange: USB_30_PORT_CHANGE,
}
impl Default for USB_PORT_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_PORT_EXT_STATUS {
    pub AsUlong32: u32,
    pub Anonymous: USB_PORT_EXT_STATUS_0,
}
impl Default for USB_PORT_EXT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_PORT_EXT_STATUS_0 {
    pub _bitfield: u32,
}
impl USB_PORT_EXT_STATUS_0 {
    pub fn RxSublinkSpeedID(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_RxSublinkSpeedID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn TxSublinkSpeedID(&self) -> u32 {
        (self._bitfield << 24) >> 28
    }
    pub fn set_TxSublinkSpeedID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn RxLaneCount(&self) -> u32 {
        (self._bitfield << 20) >> 28
    }
    pub fn set_RxLaneCount(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn TxLaneCount(&self) -> u32 {
        (self._bitfield << 16) >> 28
    }
    pub fn set_TxLaneCount(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_PORT_EXT_STATUS_AND_CHANGE {
    pub AsUlong64: u64,
    pub Anonymous: USB_PORT_EXT_STATUS_AND_CHANGE_0,
}
impl Default for USB_PORT_EXT_STATUS_AND_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USB_PORT_EXT_STATUS_AND_CHANGE_0 {
    pub PortStatusChange: USB_PORT_STATUS_AND_CHANGE,
    pub PortExtStatus: USB_PORT_EXT_STATUS,
}
impl Default for USB_PORT_EXT_STATUS_AND_CHANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_PORT_STATUS {
    pub AsUshort16: u16,
    pub Usb20PortStatus: USB_20_PORT_STATUS,
    pub Usb30PortStatus: USB_30_PORT_STATUS,
}
impl Default for USB_PORT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_PORT_STATUS_AND_CHANGE {
    pub AsUlong32: u32,
    pub Anonymous: USB_PORT_STATUS_AND_CHANGE_0,
}
impl Default for USB_PORT_STATUS_AND_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USB_PORT_STATUS_AND_CHANGE_0 {
    pub PortStatus: USB_PORT_STATUS,
    pub PortChange: USB_PORT_CHANGE,
}
impl Default for USB_PORT_STATUS_AND_CHANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_PORT_STATUS_CONNECT: u32 = 1;
pub const USB_PORT_STATUS_ENABLE: u32 = 2;
pub const USB_PORT_STATUS_HIGH_SPEED: u32 = 1024;
pub const USB_PORT_STATUS_LOW_SPEED: u32 = 512;
pub const USB_PORT_STATUS_OVER_CURRENT: u32 = 8;
pub const USB_PORT_STATUS_POWER: u32 = 256;
pub const USB_PORT_STATUS_RESET: u32 = 16;
pub const USB_PORT_STATUS_SUSPEND: u32 = 4;
pub const USB_REQUEST_CLEAR_FEATURE: u32 = 1;
pub const USB_REQUEST_CLEAR_TT_BUFFER: u32 = 8;
pub const USB_REQUEST_GET_CONFIGURATION: u32 = 8;
pub const USB_REQUEST_GET_DESCRIPTOR: u32 = 6;
pub const USB_REQUEST_GET_FIRMWARE_STATUS: u32 = 26;
pub const USB_REQUEST_GET_INTERFACE: u32 = 10;
pub const USB_REQUEST_GET_PORT_ERR_COUNT: u32 = 13;
pub const USB_REQUEST_GET_STATE: u32 = 2;
pub const USB_REQUEST_GET_STATUS: u32 = 0;
pub const USB_REQUEST_GET_TT_STATE: u32 = 10;
pub const USB_REQUEST_ISOCH_DELAY: u32 = 49;
pub const USB_REQUEST_RESET_TT: u32 = 9;
pub const USB_REQUEST_SET_ADDRESS: u32 = 5;
pub const USB_REQUEST_SET_CONFIGURATION: u32 = 9;
pub const USB_REQUEST_SET_DESCRIPTOR: u32 = 7;
pub const USB_REQUEST_SET_FEATURE: u32 = 3;
pub const USB_REQUEST_SET_FIRMWARE_STATUS: u32 = 27;
pub const USB_REQUEST_SET_HUB_DEPTH: u32 = 12;
pub const USB_REQUEST_SET_INTERFACE: u32 = 11;
pub const USB_REQUEST_SET_SEL: u32 = 48;
pub const USB_REQUEST_STOP_TT: u32 = 11;
pub const USB_REQUEST_SYNC_FRAME: u32 = 12;
pub const USB_RESERVED_DESCRIPTOR_TYPE: u32 = 6;
pub const USB_STATUS_EXT_PORT_STATUS: u32 = 2;
pub const USB_STATUS_PD_STATUS: u32 = 1;
pub const USB_STATUS_PORT_STATUS: u32 = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_STRING_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bString: [u16; 1],
}
impl Default for USB_STRING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_STRING_DESCRIPTOR_TYPE: u32 = 3;
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MAX_BYTESPERINTERVAL: u32 = 16777215;
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MIN_BYTESPERINTERVAL: u32 = 49153;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wReserved: u16,
    pub dwBytesPerInterval: u32,
}
pub const USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: u32 = 49;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bMaxBurst: u8,
    pub bmAttributes: USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0,
    pub wBytesPerInterval: u16,
}
impl Default for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0 {
    pub AsUchar: u8,
    pub Bulk: USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_0,
    pub Isochronous: USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_1,
}
impl Default for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
impl USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_0 {
    pub fn MaxStreams(&self) -> u8 {
        (self._bitfield << 3) >> 3
    }
    pub fn set_MaxStreams(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !31) | (value & 31);
    }
    pub fn Reserved1(&self) -> u8 {
        self._bitfield >> 5
    }
    pub fn set_Reserved1(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_1 {
    pub _bitfield: u8,
}
impl USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_1 {
    pub fn Mult(&self) -> u8 {
        (self._bitfield << 6) >> 6
    }
    pub fn set_Mult(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Reserved2(&self) -> u8 {
        (self._bitfield << 1) >> 3
    }
    pub fn set_Reserved2(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 2)) | ((value & 31) << 2);
    }
    pub fn SspCompanion(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_SspCompanion(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
pub const USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: u32 = 48;
pub const USB_SUPERSPEED_ISOCHRONOUS_MAX_MULTIPLIER: u32 = 2;
pub const USB_SUPPORT_D0_COMMAND: u32 = 1;
pub const USB_SUPPORT_D1_COMMAND: u32 = 2;
pub const USB_SUPPORT_D1_WAKEUP: u32 = 16;
pub const USB_SUPPORT_D2_COMMAND: u32 = 4;
pub const USB_SUPPORT_D2_WAKEUP: u32 = 32;
pub const USB_SUPPORT_D3_COMMAND: u32 = 8;
pub const Usb11Device: USB_DEVICE_TYPE = 0;
pub const Usb20Device: USB_DEVICE_TYPE = 1;
pub const UsbFullSpeed: USB_DEVICE_SPEED = 1;
pub const UsbHighSpeed: USB_DEVICE_SPEED = 2;
pub const UsbLowSpeed: USB_DEVICE_SPEED = 0;
pub const UsbSuperSpeed: USB_DEVICE_SPEED = 3;
