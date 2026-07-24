pub const BMREQUEST_CLASS: i32 = 1;
pub const BMREQUEST_DEVICE_TO_HOST: i32 = 1;
pub const BMREQUEST_HOST_TO_DEVICE: i32 = 0;
pub const BMREQUEST_STANDARD: i32 = 0;
pub const BMREQUEST_TO_DEVICE: i32 = 0;
pub const BMREQUEST_TO_ENDPOINT: i32 = 2;
pub const BMREQUEST_TO_INTERFACE: i32 = 1;
pub const BMREQUEST_TO_OTHER: i32 = 3;
pub const BMREQUEST_VENDOR: i32 = 2;
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
#[derive(Clone, Copy, Default)]
pub struct BM_REQUEST_TYPE_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wMaxPacketSize: u16,
    pub dwBytesPerInterval: u32,
}
pub const EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: i32 = 18;
pub const GUID_USB_MSOS20_PLATFORM_CAPABILITY_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd8dd60df_4589_4cc7_9cd2_659d9e648a9f);
pub const MAXIMUM_USB_STRING_LENGTH: i32 = 255;
pub type PBM_REQUEST_TYPE = *mut BM_REQUEST_TYPE;
pub type PEUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR = *mut EUSB2_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR;
pub const PORT_LINK_STATE_COMPLIANCE_MODE: i32 = 10;
pub const PORT_LINK_STATE_DISABLED: i32 = 4;
pub const PORT_LINK_STATE_HOT_RESET: i32 = 9;
pub const PORT_LINK_STATE_INACTIVE: i32 = 6;
pub const PORT_LINK_STATE_LOOPBACK: i32 = 11;
pub const PORT_LINK_STATE_POLLING: i32 = 7;
pub const PORT_LINK_STATE_RECOVERY: i32 = 8;
pub const PORT_LINK_STATE_RX_DETECT: i32 = 5;
pub const PORT_LINK_STATE_TEST_MODE: i32 = 11;
pub const PORT_LINK_STATE_U0: i32 = 0;
pub const PORT_LINK_STATE_U1: i32 = 1;
pub const PORT_LINK_STATE_U2: i32 = 2;
pub const PORT_LINK_STATE_U3: i32 = 3;
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
pub const USB_20_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: i32 = 252;
pub const USB_20_HUB_DESCRIPTOR_TYPE: i32 = 41;
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
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: i32 = 204;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_MASK: i32 = 48;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_NOTIFICATION: i32 = 16;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_PERIODIC: i32 = 0;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED10: i32 = 32;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED11: i32 = 48;
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
pub const USB_30_HUB_DESCRIPTOR_TYPE: i32 = 42;
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
pub const USB_ALLOW_FIRMWARE_UPDATE: i32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_BOS_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumDeviceCaps: u8,
}
pub const USB_BOS_DESCRIPTOR_TYPE: i32 = 15;
pub const USB_CHARGING_POLICY_DEFAULT: i32 = 0;
pub const USB_CHARGING_POLICY_ICCHPF: i32 = 1;
pub const USB_CHARGING_POLICY_ICCLPF: i32 = 2;
pub const USB_CHARGING_POLICY_NO_POWER: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const USB_CONFIGURATION_DESCRIPTOR_TYPE: i32 = 2;
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
pub const USB_CONFIG_BUS_POWERED: i32 = 128;
pub const USB_CONFIG_POWERED_MASK: i32 = 192;
pub const USB_CONFIG_POWER_DESCRIPTOR_TYPE: i32 = 7;
pub const USB_CONFIG_REMOTE_WAKEUP: i32 = 32;
pub const USB_CONFIG_RESERVED: i32 = 31;
pub const USB_CONFIG_SELF_POWERED: i32 = 64;
pub const USB_DEBUG_DESCRIPTOR_TYPE: i32 = 10;
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct USB_DEFAULT_PIPE_SETUP_PACKET_1_0 {
    pub LowByte: u8,
    pub HiByte: u8,
}
pub const USB_DEVICE_CAPABILITY_BATTERY_INFO: i32 = 7;
pub const USB_DEVICE_CAPABILITY_BILLBOARD: i32 = 13;
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
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1 {
    pub wSVID: u16,
    pub bAlternateMode: u8,
    pub iAlternateModeSetting: u8,
}
pub const USB_DEVICE_CAPABILITY_CONTAINER_ID: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_CAPABILITY_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
}
pub const USB_DEVICE_CAPABILITY_DESCRIPTOR_TYPE: i32 = 16;
pub const USB_DEVICE_CAPABILITY_FIRMWARE_STATUS: i32 = 17;
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
pub const USB_DEVICE_CAPABILITY_MAX_U1_LATENCY: i32 = 10;
pub const USB_DEVICE_CAPABILITY_MAX_U2_LATENCY: i32 = 2047;
pub const USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT: i32 = 8;
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
pub const USB_DEVICE_CAPABILITY_PD_PROVIDER_PORT: i32 = 9;
pub const USB_DEVICE_CAPABILITY_PLATFORM: i32 = 5;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub PlatformCapabilityUuid: windows_sys::core::GUID,
    pub CapabililityData: [u8; 1],
}
impl Default for USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_DEVICE_CAPABILITY_POWER_DELIVERY: i32 = 6;
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
pub const USB_DEVICE_CAPABILITY_PRECISION_TIME_MEASUREMENT: i32 = 11;
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
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_RX: i32 = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_TX: i32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_BPS: i32 = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_GBPS: i32 = 3;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_KBPS: i32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_MBPS: i32 = 2;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_ASYMMETRIC: i32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_SYMMETRIC: i32 = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SS: i32 = 0;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SSP: i32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB: i32 = 10;
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
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_LTM_CAPABLE: i32 = 2;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_RESERVED_MASK: i32 = 253;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_FULL: i32 = 2;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_HIGH: i32 = 4;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_LOW: i32 = 1;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_RESERVED_MASK: i32 = 65520;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_SUPER: i32 = 8;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U1_DEVICE_EXIT_MAX_VALUE: i32 = 10;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U2_DEVICE_EXIT_MAX_VALUE: i32 = 2047;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_USB: i32 = 3;
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
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION: i32 = 2;
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
pub const USB_DEVICE_CAPABILITY_WIRELESS_USB: i32 = 1;
pub const USB_DEVICE_CLASS_APPLICATION_SPECIFIC: i32 = 254;
pub const USB_DEVICE_CLASS_AUDIO: i32 = 1;
pub const USB_DEVICE_CLASS_AUDIO_VIDEO: i32 = 16;
pub const USB_DEVICE_CLASS_BILLBOARD: i32 = 17;
pub const USB_DEVICE_CLASS_CDC_DATA: i32 = 10;
pub const USB_DEVICE_CLASS_COMMUNICATIONS: i32 = 2;
pub const USB_DEVICE_CLASS_CONTENT_SECURITY: i32 = 13;
pub const USB_DEVICE_CLASS_DIAGNOSTIC_DEVICE: i32 = 220;
pub const USB_DEVICE_CLASS_HUB: i32 = 9;
pub const USB_DEVICE_CLASS_HUMAN_INTERFACE: i32 = 3;
pub const USB_DEVICE_CLASS_IMAGE: i32 = 6;
pub const USB_DEVICE_CLASS_MISCELLANEOUS: i32 = 239;
pub const USB_DEVICE_CLASS_MONITOR: i32 = 4;
pub const USB_DEVICE_CLASS_PERSONAL_HEALTHCARE: i32 = 15;
pub const USB_DEVICE_CLASS_PHYSICAL_INTERFACE: i32 = 5;
pub const USB_DEVICE_CLASS_POWER: i32 = 6;
pub const USB_DEVICE_CLASS_PRINTER: i32 = 7;
pub const USB_DEVICE_CLASS_RESERVED: i32 = 0;
pub const USB_DEVICE_CLASS_SMART_CARD: i32 = 11;
pub const USB_DEVICE_CLASS_STORAGE: i32 = 8;
pub const USB_DEVICE_CLASS_VENDOR_SPECIFIC: i32 = 255;
pub const USB_DEVICE_CLASS_VIDEO: i32 = 14;
pub const USB_DEVICE_CLASS_WIRELESS_CONTROLLER: i32 = 224;
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
pub const USB_DEVICE_DESCRIPTOR_TYPE: i32 = 1;
pub const USB_DEVICE_FIRMWARE_HASH_LENGTH: i32 = 32;
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
pub const USB_DEVICE_QUALIFIER_DESCRIPTOR_TYPE: i32 = 6;
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
pub type USB_DEVICE_TYPE = i32;
pub const USB_DISALLOW_FIRMWARE_UPDATE: i32 = 0;
pub const USB_ENDPOINT_ADDRESS_MASK: i32 = 15;
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
pub const USB_ENDPOINT_DESCRIPTOR_TYPE: i32 = 5;
pub const USB_ENDPOINT_DIRECTION_MASK: i32 = 128;
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
pub const USB_ENDPOINT_SUPERSPEED_BULK_MAX_PACKET_SIZE: i32 = 1024;
pub const USB_ENDPOINT_SUPERSPEED_CONTROL_MAX_PACKET_SIZE: i32 = 512;
pub const USB_ENDPOINT_SUPERSPEED_INTERRUPT_MAX_PACKET_SIZE: i32 = 1024;
pub const USB_ENDPOINT_SUPERSPEED_ISO_MAX_PACKET_SIZE: i32 = 1024;
pub const USB_ENDPOINT_TYPE_BULK: i32 = 2;
pub const USB_ENDPOINT_TYPE_BULK_RESERVED_MASK: i32 = 252;
pub const USB_ENDPOINT_TYPE_CONTROL: i32 = 0;
pub const USB_ENDPOINT_TYPE_CONTROL_RESERVED_MASK: i32 = 252;
pub const USB_ENDPOINT_TYPE_INTERRUPT: i32 = 3;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS: i32 = 1;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_RESERVED_MASK: i32 = 192;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ADAPTIVE: i32 = 8;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ASYNCHRONOUS: i32 = 4;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_MASK: i32 = 12;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_NO_SYNCHRONIZATION: i32 = 0;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_SYNCHRONOUS: i32 = 12;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_DATA_ENDOINT: i32 = 0;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_FEEDBACK_ENDPOINT: i32 = 16;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_IMPLICIT_FEEDBACK_DATA_ENDPOINT: i32 = 32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_MASK: i32 = 48;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_RESERVED: i32 = 48;
pub const USB_ENDPOINT_TYPE_MASK: i32 = 3;
pub const USB_FEATURE_BATTERY_WAKE_MASK: i32 = 40;
pub const USB_FEATURE_CHARGING_POLICY: i32 = 54;
pub const USB_FEATURE_ENDPOINT_STALL: i32 = 0;
pub const USB_FEATURE_FUNCTION_SUSPEND: i32 = 0;
pub const USB_FEATURE_INTERFACE_POWER_D0: i32 = 2;
pub const USB_FEATURE_INTERFACE_POWER_D1: i32 = 3;
pub const USB_FEATURE_INTERFACE_POWER_D2: i32 = 4;
pub const USB_FEATURE_INTERFACE_POWER_D3: i32 = 5;
pub const USB_FEATURE_LDM_ENABLE: i32 = 53;
pub const USB_FEATURE_LTM_ENABLE: i32 = 50;
pub const USB_FEATURE_OS_IS_PD_AWARE: i32 = 41;
pub const USB_FEATURE_POLICY_MODE: i32 = 42;
pub const USB_FEATURE_REMOTE_WAKEUP: i32 = 1;
pub const USB_FEATURE_TEST_MODE: i32 = 2;
pub const USB_FEATURE_U1_ENABLE: i32 = 48;
pub const USB_FEATURE_U2_ENABLE: i32 = 49;
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
#[derive(Clone, Copy, Default)]
pub struct USB_FUNCTION_SUSPEND_OPTIONS_0 {
    pub _bitfield: u8,
}
pub const USB_GETSTATUS_LTM_ENABLE: i32 = 16;
pub const USB_GETSTATUS_REMOTE_WAKEUP_ENABLED: i32 = 2;
pub const USB_GETSTATUS_SELF_POWERED: i32 = 1;
pub const USB_GETSTATUS_U1_ENABLE: i32 = 4;
pub const USB_GETSTATUS_U2_ENABLE: i32 = 8;
pub const USB_GET_FIRMWARE_ALLOWED_OR_DISALLOWED_STATE: i32 = 0;
pub const USB_GET_FIRMWARE_HASH: i32 = 1;
pub const USB_HIGHSPEED_EUSB2_ISOCHRONOUS_MAX_BYTESPERINTERVAL: i32 = 6144;
pub const USB_HIGHSPEED_EUSB2_ISOCHRONOUS_MIN_BYTESPERINTERVAL: i32 = 3073;
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
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_30_PORT_REMOTE_WAKE_MASK_0 {
    pub _bitfield: u8,
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
#[derive(Clone, Copy, Default)]
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
pub const USB_INTERFACE_ASSOCIATION_DESCRIPTOR_TYPE: i32 = 11;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const USB_INTERFACE_DESCRIPTOR_TYPE: i32 = 4;
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
pub const USB_INTERFACE_POWER_DESCRIPTOR_TYPE: i32 = 8;
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
pub const USB_OTG_DESCRIPTOR_TYPE: i32 = 9;
pub const USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_TYPE: i32 = 7;
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
pub const USB_PORT_STATUS_CONNECT: i32 = 1;
pub const USB_PORT_STATUS_ENABLE: i32 = 2;
pub const USB_PORT_STATUS_HIGH_SPEED: i32 = 1024;
pub const USB_PORT_STATUS_LOW_SPEED: i32 = 512;
pub const USB_PORT_STATUS_OVER_CURRENT: i32 = 8;
pub const USB_PORT_STATUS_POWER: i32 = 256;
pub const USB_PORT_STATUS_RESET: i32 = 16;
pub const USB_PORT_STATUS_SUSPEND: i32 = 4;
pub const USB_REQUEST_CLEAR_FEATURE: i32 = 1;
pub const USB_REQUEST_CLEAR_TT_BUFFER: i32 = 8;
pub const USB_REQUEST_GET_CONFIGURATION: i32 = 8;
pub const USB_REQUEST_GET_DESCRIPTOR: i32 = 6;
pub const USB_REQUEST_GET_FIRMWARE_STATUS: i32 = 26;
pub const USB_REQUEST_GET_INTERFACE: i32 = 10;
pub const USB_REQUEST_GET_PORT_ERR_COUNT: i32 = 13;
pub const USB_REQUEST_GET_STATE: i32 = 2;
pub const USB_REQUEST_GET_STATUS: i32 = 0;
pub const USB_REQUEST_GET_TT_STATE: i32 = 10;
pub const USB_REQUEST_ISOCH_DELAY: i32 = 49;
pub const USB_REQUEST_RESET_TT: i32 = 9;
pub const USB_REQUEST_SET_ADDRESS: i32 = 5;
pub const USB_REQUEST_SET_CONFIGURATION: i32 = 9;
pub const USB_REQUEST_SET_DESCRIPTOR: i32 = 7;
pub const USB_REQUEST_SET_FEATURE: i32 = 3;
pub const USB_REQUEST_SET_FIRMWARE_STATUS: i32 = 27;
pub const USB_REQUEST_SET_HUB_DEPTH: i32 = 12;
pub const USB_REQUEST_SET_INTERFACE: i32 = 11;
pub const USB_REQUEST_SET_SEL: i32 = 48;
pub const USB_REQUEST_STOP_TT: i32 = 11;
pub const USB_REQUEST_SYNC_FRAME: i32 = 12;
pub const USB_RESERVED_DESCRIPTOR_TYPE: i32 = 6;
pub const USB_STATUS_EXT_PORT_STATUS: i32 = 2;
pub const USB_STATUS_PD_STATUS: i32 = 1;
pub const USB_STATUS_PORT_STATUS: i32 = 0;
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
pub const USB_STRING_DESCRIPTOR_TYPE: i32 = 3;
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MAX_BYTESPERINTERVAL: i32 = 16777215;
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MIN_BYTESPERINTERVAL: i32 = 49153;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wReserved: u16,
    pub dwBytesPerInterval: u32,
}
pub const USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: i32 = 49;
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
#[derive(Clone, Copy, Default)]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_1 {
    pub _bitfield: u8,
}
pub const USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: i32 = 48;
pub const USB_SUPERSPEED_ISOCHRONOUS_MAX_MULTIPLIER: i32 = 2;
pub const USB_SUPPORT_D0_COMMAND: i32 = 1;
pub const USB_SUPPORT_D1_COMMAND: i32 = 2;
pub const USB_SUPPORT_D1_WAKEUP: i32 = 16;
pub const USB_SUPPORT_D2_COMMAND: i32 = 4;
pub const USB_SUPPORT_D2_WAKEUP: i32 = 32;
pub const USB_SUPPORT_D3_COMMAND: i32 = 8;
pub const Usb11Device: USB_DEVICE_TYPE = 0;
pub const Usb20Device: USB_DEVICE_TYPE = 1;
pub const UsbFullSpeed: USB_DEVICE_SPEED = 1;
pub const UsbHighSpeed: USB_DEVICE_SPEED = 2;
pub const UsbLowSpeed: USB_DEVICE_SPEED = 0;
pub const UsbSuperSpeed: USB_DEVICE_SPEED = 3;
