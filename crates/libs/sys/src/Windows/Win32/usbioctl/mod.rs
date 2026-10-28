pub const AcquireBusInfo: USB_NOTIFICATION_TYPE = 5;
pub const AcquireControllerName: USB_NOTIFICATION_TYPE = 7;
pub const AcquireHubName: USB_NOTIFICATION_TYPE = 6;
pub const CompositeDevice: USB_WMI_DEVICE_NODE_TYPE = 2;
pub const DeviceCausedOvercurrent: USB_CONNECTION_STATUS = 4;
pub const DeviceConnected: USB_CONNECTION_STATUS = 1;
pub const DeviceEnumerating: USB_CONNECTION_STATUS = 9;
pub const DeviceFailedEnumeration: USB_CONNECTION_STATUS = 2;
pub const DeviceGeneralFailure: USB_CONNECTION_STATUS = 3;
pub const DeviceHubNestedTooDeeply: USB_CONNECTION_STATUS = 7;
pub const DeviceInLegacyHub: USB_CONNECTION_STATUS = 8;
pub const DeviceNotEnoughBandwidth: USB_CONNECTION_STATUS = 6;
pub const DeviceNotEnoughPower: USB_CONNECTION_STATUS = 5;
pub const DeviceReset: USB_CONNECTION_STATUS = 10;
pub const EnumerationFailure: USB_NOTIFICATION_TYPE = 0;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HCD_ISO_STAT_COUNTERS {
    pub LateUrbs: super::USHORT,
    pub DoubleBufferedPackets: super::USHORT,
    pub TransfersCF_5ms: super::USHORT,
    pub TransfersCF_2ms: super::USHORT,
    pub TransfersCF_1ms: super::USHORT,
    pub MaxInterruptLatency: super::USHORT,
    pub BadStartFrame: super::USHORT,
    pub StaleUrbs: super::USHORT,
    pub IsoPacketNotAccesed: super::USHORT,
    pub IsoPacketHWError: super::USHORT,
    pub SmallestUrbPacketCount: super::USHORT,
    pub LargestUrbPacketCount: super::USHORT,
    pub IsoCRC_Error: super::USHORT,
    pub IsoOVERRUN_Error: super::USHORT,
    pub IsoINTERNAL_Error: super::USHORT,
    pub IsoUNKNOWN_Error: super::USHORT,
    pub IsoBytesTransferred: super::ULONG,
    pub LateMissedCount: super::USHORT,
    pub HWIsoMissedCount: super::USHORT,
    pub Reserved7: [super::ULONG; 8],
}
#[cfg(feature = "shared")]
impl Default for HCD_ISO_STAT_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HCD_STAT_COUNTERS {
    pub BytesTransferred: super::ULONG,
    pub IsoMissedCount: super::USHORT,
    pub DataOverrunErrorCount: super::USHORT,
    pub CrcErrorCount: super::USHORT,
    pub ScheduleOverrunCount: super::USHORT,
    pub TimeoutErrorCount: super::USHORT,
    pub InternalHcErrorCount: super::USHORT,
    pub BufferOverrunErrorCount: super::USHORT,
    pub SWErrorCount: super::USHORT,
    pub StallPidCount: super::USHORT,
    pub PortDisableCount: super::USHORT,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HCD_STAT_INFORMATION_1 {
    pub Reserved1: super::ULONG,
    pub Reserved2: super::ULONG,
    pub ResetCounters: super::ULONG,
    pub TimeRead: super::LARGE_INTEGER,
    pub Counters: HCD_STAT_COUNTERS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HCD_STAT_INFORMATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HCD_STAT_INFORMATION_2 {
    pub Reserved1: super::ULONG,
    pub Reserved2: super::ULONG,
    pub ResetCounters: super::ULONG,
    pub TimeRead: super::LARGE_INTEGER,
    pub LockedMemoryUsed: super::LONG,
    pub Counters: HCD_STAT_COUNTERS,
    pub IsoCounters: HCD_ISO_STAT_COUNTERS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HCD_STAT_INFORMATION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HUB_DEVICE_CONFIG_INFO {
    pub Version: super::ULONG,
    pub Length: super::ULONG,
    pub HubFlags: USB_HUB_CAP_FLAGS,
    pub HardwareIds: USB_ID_STRING,
    pub CompatibleIds: USB_ID_STRING,
    pub DeviceDescription: USB_ID_STRING,
    pub Reserved: [super::ULONG; 19],
    pub UxdSettings: USB_HUB_DEVICE_UXD_SETTINGS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HUB_DEVICE_CONFIG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HubDevice: USB_WMI_DEVICE_NODE_TYPE = 1;
pub const HubNestedTooDeeply: USB_NOTIFICATION_TYPE = 10;
pub const HubOvercurrent: USB_NOTIFICATION_TYPE = 8;
pub const HubPowerChange: USB_NOTIFICATION_TYPE = 9;
pub const IOCTL_GET_HCD_DRIVERKEY_NAME: i32 = 2229284;
pub const IOCTL_INTERNAL_USB_CYCLE_PORT: i32 = 2228255;
pub const IOCTL_INTERNAL_USB_ENABLE_PORT: i32 = 2228247;
pub const IOCTL_INTERNAL_USB_FAIL_GET_STATUS_FROM_DEVICE: i32 = 2229347;
pub const IOCTL_INTERNAL_USB_GET_BUSGUID_INFO: i32 = 2229288;
pub const IOCTL_INTERNAL_USB_GET_BUS_INFO: i32 = 2229280;
pub const IOCTL_INTERNAL_USB_GET_CONTROLLER_NAME: i32 = 2229284;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO: i32 = 2229327;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE: i32 = 2229299;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX: i32 = 2229303;
pub const IOCTL_INTERNAL_USB_GET_HUB_COUNT: i32 = 2228251;
pub const IOCTL_INTERNAL_USB_GET_HUB_NAME: i32 = 2228256;
pub const IOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO: i32 = 2229292;
pub const IOCTL_INTERNAL_USB_GET_PORT_STATUS: i32 = 2228243;
pub const IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO: i32 = 2228239;
pub const IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS: i32 = 2229311;
pub const IOCTL_INTERNAL_USB_GET_TT_DEVICE_HANDLE: i32 = 2229307;
pub const IOCTL_INTERNAL_USB_NOTIFY_IDLE_READY: i32 = 2229315;
pub const IOCTL_INTERNAL_USB_RECORD_FAILURE: i32 = 2228267;
pub const IOCTL_INTERNAL_USB_REGISTER_COMPOSITE_DEVICE: i32 = 4784131;
pub const IOCTL_INTERNAL_USB_REQUEST_REMOTE_WAKE_NOTIFICATION: i32 = 4784139;
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME: i32 = 2229323;
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND: i32 = 2229319;
pub const IOCTL_INTERNAL_USB_RESET_PORT: i32 = 2228231;
pub const IOCTL_INTERNAL_USB_SUBMIT_IDLE_NOTIFICATION: i32 = 2228263;
pub const IOCTL_INTERNAL_USB_SUBMIT_URB: i32 = 2228227;
pub const IOCTL_INTERNAL_USB_UNREGISTER_COMPOSITE_DEVICE: i32 = 4784135;
pub const IOCTL_USB_DIAGNOSTIC_MODE_OFF: i32 = 2229252;
pub const IOCTL_USB_DIAGNOSTIC_MODE_ON: i32 = 2229248;
pub const IOCTL_USB_DIAG_IGNORE_HUBS_OFF: i32 = 2229276;
pub const IOCTL_USB_DIAG_IGNORE_HUBS_ON: i32 = 2229272;
pub const IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION: i32 = 2229264;
pub const IOCTL_USB_GET_DEVICE_CHARACTERISTICS: i32 = 2229376;
pub const IOCTL_USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC: i32 = 2229368;
pub const IOCTL_USB_GET_HUB_CAPABILITIES: i32 = 2229308;
pub const IOCTL_USB_GET_HUB_CAPABILITIES_EX: i32 = 2229328;
pub const IOCTL_USB_GET_HUB_INFORMATION_EX: i32 = 2229332;
pub const IOCTL_USB_GET_NODE_CONNECTION_ATTRIBUTES: i32 = 2229312;
pub const IOCTL_USB_GET_NODE_CONNECTION_DRIVERKEY_NAME: i32 = 2229280;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION: i32 = 2229260;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX: i32 = 2229320;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2: i32 = 2229340;
pub const IOCTL_USB_GET_NODE_CONNECTION_NAME: i32 = 2229268;
pub const IOCTL_USB_GET_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION: i32 = 2229380;
pub const IOCTL_USB_GET_NODE_INFORMATION: i32 = 2229256;
pub const IOCTL_USB_GET_PORT_CONNECTOR_PROPERTIES: i32 = 2229336;
pub const IOCTL_USB_GET_ROOT_HUB_NAME: i32 = 2229256;
pub const IOCTL_USB_GET_TRANSPORT_CHARACTERISTICS: i32 = 2229348;
pub const IOCTL_USB_HCD_DISABLE_PORT: i32 = 2229296;
pub const IOCTL_USB_HCD_ENABLE_PORT: i32 = 2229300;
pub const IOCTL_USB_HCD_GET_STATS_1: i32 = 2229244;
pub const IOCTL_USB_HCD_GET_STATS_2: i32 = 2229288;
pub const IOCTL_USB_HUB_CYCLE_PORT: i32 = 2229316;
pub const IOCTL_USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE: i32 = 2229356;
pub const IOCTL_USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: i32 = 2229352;
pub const IOCTL_USB_RESET_HUB: i32 = 2229324;
pub const IOCTL_USB_START_TRACKING_FOR_TIME_SYNC: i32 = 2229364;
pub const IOCTL_USB_STOP_TRACKING_FOR_TIME_SYNC: i32 = 2229372;
pub const IOCTL_USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: i32 = 2229360;
pub const InsufficentBandwidth: USB_NOTIFICATION_TYPE = 1;
pub const InsufficentPower: USB_NOTIFICATION_TYPE = 2;
pub const ModernDeviceInLegacyHub: USB_NOTIFICATION_TYPE = 11;
pub const NoDeviceConnected: USB_CONNECTION_STATUS = 0;
pub const OverCurrent: USB_NOTIFICATION_TYPE = 3;
#[cfg(feature = "shared")]
pub type PHCD_ISO_STAT_COUNTERS = *mut HCD_ISO_STAT_COUNTERS;
#[cfg(feature = "shared")]
pub type PHCD_STAT_COUNTERS = *mut HCD_STAT_COUNTERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHCD_STAT_INFORMATION_1 = *mut HCD_STAT_INFORMATION_1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHCD_STAT_INFORMATION_2 = *mut HCD_STAT_INFORMATION_2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHUB_DEVICE_CONFIG_INFO = *mut HUB_DEVICE_CONFIG_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_ACQUIRE_INFO = *mut USB_ACQUIRE_INFO;
#[cfg(feature = "shared")]
pub type PUSB_BUS_NOTIFICATION = *mut USB_BUS_NOTIFICATION;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_COMPOSITE_DEVICE_INFO = *mut USB_COMPOSITE_DEVICE_INFO;
#[cfg(feature = "shared")]
pub type PUSB_COMPOSITE_FUNCTION_INFO = *mut USB_COMPOSITE_FUNCTION_INFO;
#[cfg(feature = "shared")]
pub type PUSB_CONNECTION_NOTIFICATION = *mut USB_CONNECTION_NOTIFICATION;
pub type PUSB_CONNECTION_STATUS = *mut USB_CONNECTION_STATUS;
#[cfg(feature = "shared")]
pub type PUSB_CONTROLLER_DEVICE_INFO = *mut USB_CONTROLLER_DEVICE_INFO;
#[cfg(feature = "shared")]
pub type PUSB_CYCLE_PORT_PARAMS = *mut USB_CYCLE_PORT_PARAMS;
#[cfg(feature = "shared")]
pub type PUSB_DESCRIPTOR_REQUEST = *mut USB_DESCRIPTOR_REQUEST;
#[cfg(feature = "shared")]
pub type PUSB_DEVICE_CHARACTERISTICS = *mut USB_DEVICE_CHARACTERISTICS;
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
pub type PUSB_DEVICE_INFO = *mut USB_DEVICE_INFO;
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
pub type PUSB_DEVICE_NODE_INFO = *mut USB_DEVICE_NODE_INFO;
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
pub type PUSB_DEVICE_PERFORMANCE_INFO = *mut USB_DEVICE_PERFORMANCE_INFO;
#[cfg(feature = "shared")]
pub type PUSB_DEVICE_STATE = *mut USB_DEVICE_STATE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION = *mut USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_HCD_DRIVERKEY_NAME = *mut USB_HCD_DRIVERKEY_NAME;
#[cfg(feature = "shared")]
pub type PUSB_HUB_CAPABILITIES = *mut USB_HUB_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PUSB_HUB_CAPABILITIES_EX = *mut USB_HUB_CAPABILITIES_EX;
#[cfg(feature = "shared")]
pub type PUSB_HUB_CAP_FLAGS = *mut USB_HUB_CAP_FLAGS;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_HUB_DEVICE_INFO = *mut USB_HUB_DEVICE_INFO;
#[cfg(feature = "shared")]
pub type PUSB_HUB_DEVICE_UXD_SETTINGS = *mut USB_HUB_DEVICE_UXD_SETTINGS;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_HUB_INFORMATION = *mut USB_HUB_INFORMATION;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_HUB_INFORMATION_EX = *mut USB_HUB_INFORMATION_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_HUB_NAME = *mut USB_HUB_NAME;
#[cfg(feature = "shared")]
pub type PUSB_HUB_PORT_INFORMATION = *mut USB_HUB_PORT_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_ID_STRING = *mut USB_ID_STRING;
#[cfg(feature = "shared")]
pub type PUSB_MI_PARENT_INFORMATION = *mut USB_MI_PARENT_INFORMATION;
#[cfg(feature = "shared")]
pub type PUSB_NODE_CONNECTION_ATTRIBUTES = *mut USB_NODE_CONNECTION_ATTRIBUTES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_NODE_CONNECTION_DRIVERKEY_NAME = *mut USB_NODE_CONNECTION_DRIVERKEY_NAME;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_NODE_CONNECTION_INFORMATION = *mut USB_NODE_CONNECTION_INFORMATION;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_NODE_CONNECTION_INFORMATION_EX = *mut USB_NODE_CONNECTION_INFORMATION_EX;
#[cfg(feature = "shared")]
pub type PUSB_NODE_CONNECTION_INFORMATION_EX_V2 = *mut USB_NODE_CONNECTION_INFORMATION_EX_V2;
#[cfg(feature = "shared")]
pub type PUSB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS = *mut USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_NODE_CONNECTION_NAME = *mut USB_NODE_CONNECTION_NAME;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION = *mut USB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_NODE_INFORMATION = *mut USB_NODE_INFORMATION;
pub type PUSB_NOTIFICATION = *mut USB_NOTIFICATION;
#[cfg(all(feature = "shared", feature = "usbspec"))]
pub type PUSB_PIPE_INFO = *mut USB_PIPE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_PORT_CONNECTOR_PROPERTIES = *mut USB_PORT_CONNECTOR_PROPERTIES;
#[cfg(feature = "shared")]
pub type PUSB_PORT_PROPERTIES = *mut USB_PORT_PROPERTIES;
#[cfg(feature = "shared")]
pub type PUSB_PROTOCOLS = *mut USB_PROTOCOLS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_ROOT_HUB_NAME = *mut USB_ROOT_HUB_NAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUSB_START_TRACKING_FOR_TIME_SYNC_INFORMATION = *mut USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION;
#[cfg(feature = "winnt")]
pub type PUSB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION = *mut USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION;
#[cfg(feature = "shared")]
pub type PUSB_TOPOLOGY_ADDRESS = *mut USB_TOPOLOGY_ADDRESS;
#[cfg(feature = "shared")]
pub type PUSB_TRANSPORT_CHARACTERISTICS = *mut USB_TRANSPORT_CHARACTERISTICS;
#[cfg(feature = "shared")]
pub type PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION = *mut USB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION;
#[cfg(feature = "shared")]
pub type PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION = *mut USB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION;
pub type PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION = *mut USB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION;
pub type PUSB_WMI_DEVICE_NODE_TYPE = *mut USB_WMI_DEVICE_NODE_TYPE;
pub const ResetOvercurrent: USB_NOTIFICATION_TYPE = 4;
pub const USBD_PORT_CONNECTED: i32 = 2;
pub const USBD_PORT_ENABLED: i32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_ACQUIRE_INFO {
    pub NotificationType: USB_NOTIFICATION_TYPE,
    pub TotalSize: super::ULONG,
    pub Buffer: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USB_ACQUIRE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_BUS_NOTIFICATION {
    pub NotificationType: USB_NOTIFICATION_TYPE,
    pub TotalBandwidth: super::ULONG,
    pub ConsumedBandwidth: super::ULONG,
    pub ControllerNameLength: super::ULONG,
}
pub type USB_CHANGE_REGISTRATION_HANDLE = *mut USB_CHANGE_REGISTRATION_HANDLE__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_CHANGE_REGISTRATION_HANDLE__ {
    pub unused: i32,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub struct USB_COMPOSITE_DEVICE_INFO {
    pub DeviceDescriptor: super::USB_DEVICE_DESCRIPTOR,
    pub CurrentConfigDescriptor: super::USB_CONFIGURATION_DESCRIPTOR,
    pub CurrentConfigurationValue: super::UCHAR,
    pub NumberOfFunctions: super::UCHAR,
    pub FunctionInfo: [USB_COMPOSITE_FUNCTION_INFO; 1],
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_COMPOSITE_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_COMPOSITE_FUNCTION_INFO {
    pub FunctionNumber: super::UCHAR,
    pub BaseInterfaceNumber: super::UCHAR,
    pub NumberOfInterfaces: super::UCHAR,
    pub FunctionIsIdle: super::BOOLEAN,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_CONNECTION_NOTIFICATION {
    pub NotificationType: USB_NOTIFICATION_TYPE,
    pub ConnectionNumber: super::ULONG,
    pub RequestedBandwidth: super::ULONG,
    pub EnumerationFailReason: super::ULONG,
    pub PowerRequested: super::ULONG,
    pub HubNameLength: super::ULONG,
}
pub type USB_CONNECTION_STATUS = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_CONTROLLER_DEVICE_INFO {
    pub PciVendorId: super::ULONG,
    pub PciDeviceId: super::ULONG,
    pub PciRevision: super::ULONG,
    pub NumberOfRootPorts: super::ULONG,
    pub HcFeatureFlags: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_CYCLE_PORT_PARAMS {
    pub ConnectionIndex: super::ULONG,
    pub StatusReturned: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct USB_DESCRIPTOR_REQUEST {
    pub ConnectionIndex: super::ULONG,
    pub SetupPacket: USB_DESCRIPTOR_REQUEST_0,
    pub Data: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for USB_DESCRIPTOR_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_DESCRIPTOR_REQUEST_0 {
    pub bmRequest: super::UCHAR,
    pub bRequest: super::UCHAR,
    pub wValue: super::USHORT,
    pub wIndex: super::USHORT,
    pub wLength: super::USHORT,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CHARACTERISTICS {
    pub Version: super::ULONG,
    pub Reserved: [super::ULONG; 2],
    pub UsbDeviceCharacteristicsFlags: super::ULONG,
    pub MaximumSendPathDelayInMilliSeconds: super::ULONG,
    pub MaximumCompletionPathDelayInMilliSeconds: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for USB_DEVICE_CHARACTERISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_DEVICE_CHARACTERISTICS_MAXIMUM_PATH_DELAYS_AVAILABLE: i32 = 1;
pub const USB_DEVICE_CHARACTERISTICS_VERSION_1: i32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_INFO {
    pub DeviceState: USB_DEVICE_STATE,
    pub PortNumber: super::USHORT,
    pub DeviceDescriptor: super::USB_DEVICE_DESCRIPTOR,
    pub CurrentConfigurationValue: super::UCHAR,
    pub Speed: super::USB_DEVICE_SPEED,
    pub DeviceAddress: super::USHORT,
    pub ConnectionIndex: super::ULONG,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
    pub PnpHardwareId: [super::WCHAR; 128],
    pub PnpCompatibleId: [super::WCHAR; 128],
    pub SerialNumberId: [super::WCHAR; 128],
    pub PnpDeviceDescription: [super::WCHAR; 128],
    pub NumberOfOpenPipes: super::ULONG,
    pub PipeList: [USB_PIPE_INFO; 1],
}
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
impl Default for USB_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_NODE_INFO {
    pub Sig: super::ULONG,
    pub LengthInBytes: super::ULONG,
    pub DeviceDescription: [super::WCHAR; 40],
    pub NodeType: USB_WMI_DEVICE_NODE_TYPE,
    pub BusAddress: USB_TOPOLOGY_ADDRESS,
    pub Anonymous: USB_DEVICE_NODE_INFO_0,
}
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
impl Default for USB_DEVICE_NODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union USB_DEVICE_NODE_INFO_0 {
    pub UsbDeviceInfo: USB_DEVICE_INFO,
    pub HubDeviceInfo: USB_HUB_DEVICE_INFO,
    pub CompositeDeviceInfo: USB_COMPOSITE_DEVICE_INFO,
    pub ControllerDeviceInfo: USB_CONTROLLER_DEVICE_INFO,
    pub DeviceInformation: [super::UCHAR; 4],
}
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
impl Default for USB_DEVICE_NODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_PERFORMANCE_INFO {
    pub BulkBytes: super::ULONG,
    pub ControlDataBytes: super::ULONG,
    pub IsoBytes: super::ULONG,
    pub InterruptBytes: super::ULONG,
    pub BulkUrbCount: super::ULONG,
    pub ControlUrbCount: super::ULONG,
    pub IsoUrbCount: super::ULONG,
    pub InterruptUrbCount: super::ULONG,
    pub AllocedInterrupt: [super::ULONG; 6],
    pub AllocedIso: super::ULONG,
    pub Total32secBandwidth: super::ULONG,
    pub TotalTtBandwidth: super::ULONG,
    pub DeviceDescription: [super::WCHAR; 60],
    pub DeviceSpeed: super::USB_DEVICE_SPEED,
    pub TotalIsoLatency: super::ULONG,
    pub DroppedIsoPackets: super::ULONG,
    pub TransferErrors: super::ULONG,
    pub PciInterruptCount: super::ULONG,
    pub HcIdleState: super::ULONG,
    pub HcAsyncIdleState: super::ULONG,
    pub HcAsyncCacheFlushCount: super::ULONG,
    pub HcPeriodicIdleState: super::ULONG,
    pub HcPeriodicCacheFlushCount: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
impl Default for USB_DEVICE_PERFORMANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_STATE {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::HANDLE,
    pub InputFrameNumber: super::ULONG,
    pub InputMicroFrameNumber: super::ULONG,
    pub QueryPerformanceCounterAtInputFrameOrMicroFrame: super::LARGE_INTEGER,
    pub QueryPerformanceCounterFrequency: super::LARGE_INTEGER,
    pub PredictedAccuracyInMicroSeconds: super::ULONG,
    pub CurrentGenerationID: super::ULONG,
    pub CurrentQueryPerformanceCounter: super::LARGE_INTEGER,
    pub CurrentHardwareFrameNumber: super::ULONG,
    pub CurrentHardwareMicroFrameNumber: super::ULONG,
    pub CurrentUSBFrameNumber: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_HCD_DRIVERKEY_NAME {
    pub ActualLength: super::ULONG,
    pub DriverKeyName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USB_HCD_DRIVERKEY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_CAPABILITIES {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct USB_HUB_CAPABILITIES_EX {
    pub CapabilityFlags: USB_HUB_CAP_FLAGS,
}
#[cfg(feature = "shared")]
impl Default for USB_HUB_CAPABILITIES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union USB_HUB_CAP_FLAGS {
    pub ul: super::ULONG,
    pub Anonymous: USB_HUB_CAP_FLAGS_0,
}
#[cfg(feature = "shared")]
impl Default for USB_HUB_CAP_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_CAP_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub struct USB_HUB_DEVICE_INFO {
    pub HubDescriptor: super::USB_HUB_DESCRIPTOR,
    pub HubNumber: super::ULONG,
    pub DeviceAddress: super::USHORT,
    pub HubIsSelfPowered: super::BOOLEAN,
    pub HubIsRootHub: super::BOOLEAN,
    pub HubCapabilities: USB_HUB_CAPABILITIES,
    pub NumberOfHubPorts: super::ULONG,
    pub PortInfo: [USB_HUB_PORT_INFORMATION; 1],
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_HUB_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct USB_HUB_DEVICE_UXD_SETTINGS {
    pub Version: super::ULONG,
    pub PnpGuid: windows_sys::core::GUID,
    pub OwnerGuid: windows_sys::core::GUID,
    pub DeleteOnShutdown: super::ULONG,
    pub DeleteOnReload: super::ULONG,
    pub DeleteOnDisconnect: super::ULONG,
    pub Reserved: [super::ULONG; 5],
}
#[cfg(feature = "shared")]
impl Default for USB_HUB_DEVICE_UXD_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_INFORMATION {
    pub HubDescriptor: super::USB_HUB_DESCRIPTOR,
    pub HubIsBusPowered: super::BOOLEAN,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub struct USB_HUB_INFORMATION_EX {
    pub HubType: USB_HUB_TYPE,
    pub HighestPortNumber: super::USHORT,
    pub u: USB_HUB_INFORMATION_EX_0,
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_HUB_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub union USB_HUB_INFORMATION_EX_0 {
    pub UsbHubDescriptor: super::USB_HUB_DESCRIPTOR,
    pub Usb30HubDescriptor: super::USB_30_HUB_DESCRIPTOR,
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_HUB_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_HUB_NAME {
    pub ActualLength: super::ULONG,
    pub HubName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USB_HUB_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type USB_HUB_NODE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_PORT_INFORMATION {
    pub DeviceState: USB_DEVICE_STATE,
    pub PortNumber: super::USHORT,
    pub DeviceAddress: super::USHORT,
    pub ConnectionIndex: super::ULONG,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
}
pub type USB_HUB_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct USB_ID_STRING {
    pub LanguageId: super::USHORT,
    pub Pad: super::USHORT,
    pub LengthInBytes: super::ULONG,
    pub Buffer: super::PWCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_MI_PARENT_INFORMATION {
    pub NumberOfInterfaces: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_NODE_CONNECTION_ATTRIBUTES {
    pub ConnectionIndex: super::ULONG,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
    pub PortAttributes: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_DRIVERKEY_NAME {
    pub ConnectionIndex: super::ULONG,
    pub ActualLength: super::ULONG,
    pub DriverKeyName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USB_NODE_CONNECTION_DRIVERKEY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_INFORMATION {
    pub ConnectionIndex: super::ULONG,
    pub DeviceDescriptor: super::USB_DEVICE_DESCRIPTOR,
    pub CurrentConfigurationValue: super::UCHAR,
    pub LowSpeed: super::BOOLEAN,
    pub DeviceIsHub: super::BOOLEAN,
    pub DeviceAddress: super::USHORT,
    pub NumberOfOpenPipes: super::ULONG,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
    pub PipeList: [USB_PIPE_INFO; 0],
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_NODE_CONNECTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_INFORMATION_EX {
    pub ConnectionIndex: super::ULONG,
    pub DeviceDescriptor: super::USB_DEVICE_DESCRIPTOR,
    pub CurrentConfigurationValue: super::UCHAR,
    pub Speed: super::UCHAR,
    pub DeviceIsHub: super::BOOLEAN,
    pub DeviceAddress: super::USHORT,
    pub NumberOfOpenPipes: super::ULONG,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
    pub PipeList: [USB_PIPE_INFO; 0],
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_NODE_CONNECTION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_INFORMATION_EX_V2 {
    pub ConnectionIndex: super::ULONG,
    pub Length: super::ULONG,
    pub SupportedUsbProtocols: USB_PROTOCOLS,
    pub Flags: USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS,
}
#[cfg(feature = "shared")]
impl Default for USB_NODE_CONNECTION_INFORMATION_EX_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS {
    pub ul: super::ULONG,
    pub Anonymous: USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS_0,
}
#[cfg(feature = "shared")]
impl Default for USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_NAME {
    pub ConnectionIndex: super::ULONG,
    pub ActualLength: super::ULONG,
    pub NodeName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USB_NODE_CONNECTION_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION {
    pub ConnectionIndex: super::ULONG,
    pub Length: super::ULONG,
    pub RxSuperSpeedPlus: super::USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED,
    pub RxLaneCount: super::ULONG,
    pub TxSuperSpeedPlus: super::USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED,
    pub TxLaneCount: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub struct USB_NODE_INFORMATION {
    pub NodeType: USB_HUB_NODE,
    pub u: USB_NODE_INFORMATION_0,
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_NODE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy)]
pub union USB_NODE_INFORMATION_0 {
    pub HubInformation: USB_HUB_INFORMATION,
    pub MiParentInformation: USB_MI_PARENT_INFORMATION,
}
#[cfg(all(feature = "shared", feature = "usbspec"))]
impl Default for USB_NODE_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_NODE_INFO_SIG: i32 = 1431519822;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_NOTIFICATION {
    pub NotificationType: USB_NOTIFICATION_TYPE,
}
pub type USB_NOTIFICATION_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "usbspec"))]
#[derive(Clone, Copy, Default)]
pub struct USB_PIPE_INFO {
    pub EndpointDescriptor: super::USB_ENDPOINT_DESCRIPTOR,
    pub ScheduleOffset: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_PORT_CONNECTOR_PROPERTIES {
    pub ConnectionIndex: super::ULONG,
    pub ActualLength: super::ULONG,
    pub UsbPortProperties: USB_PORT_PROPERTIES,
    pub CompanionIndex: super::USHORT,
    pub CompanionPortNumber: super::USHORT,
    pub CompanionHubSymbolicLinkName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USB_PORT_CONNECTOR_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union USB_PORT_PROPERTIES {
    pub ul: super::ULONG,
    pub Anonymous: USB_PORT_PROPERTIES_0,
}
#[cfg(feature = "shared")]
impl Default for USB_PORT_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_PORT_PROPERTIES_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union USB_PROTOCOLS {
    pub ul: super::ULONG,
    pub Anonymous: USB_PROTOCOLS_0,
}
#[cfg(feature = "shared")]
impl Default for USB_PROTOCOLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_PROTOCOLS_0 {
    pub _bitfield: u32,
}
pub const USB_REGISTER_FOR_TRANSPORT_BANDWIDTH_CHANGE: i32 = 2;
pub const USB_REGISTER_FOR_TRANSPORT_LATENCY_CHANGE: i32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct USB_ROOT_HUB_NAME {
    pub ActualLength: super::ULONG,
    pub RootHubName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for USB_ROOT_HUB_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::HANDLE,
    pub IsStartupDelayTolerable: super::BOOLEAN,
}
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct USB_TOPOLOGY_ADDRESS {
    pub PciBusNumber: super::ULONG,
    pub PciDeviceNumber: super::ULONG,
    pub PciFunctionNumber: super::ULONG,
    pub Reserved: super::ULONG,
    pub RootHubPortNumber: super::USHORT,
    pub HubPortNumber: [super::USHORT; 5],
    pub Reserved2: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for USB_TOPOLOGY_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_TRANSPORT_CHARACTERISTICS {
    pub Version: super::ULONG,
    pub TransportCharacteristicsFlags: super::ULONG,
    pub CurrentRoundtripLatencyInMilliSeconds: super::ULONG64,
    pub MaxPotentialBandwidth: super::ULONG64,
}
pub const USB_TRANSPORT_CHARACTERISTICS_BANDWIDTH_AVAILABLE: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION {
    pub Handle: USB_CHANGE_REGISTRATION_HANDLE,
    pub UsbTransportCharacteristics: USB_TRANSPORT_CHARACTERISTICS,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct USB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION {
    pub ChangeNotificationInputFlags: super::ULONG,
    pub Handle: USB_CHANGE_REGISTRATION_HANDLE,
    pub UsbTransportCharacteristics: USB_TRANSPORT_CHARACTERISTICS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION {
    pub Handle: USB_CHANGE_REGISTRATION_HANDLE,
}
pub const USB_TRANSPORT_CHARACTERISTICS_LATENCY_AVAILABLE: i32 = 1;
pub const USB_TRANSPORT_CHARACTERISTICS_VERSION_1: i32 = 1;
pub type USB_WMI_DEVICE_NODE_TYPE = i32;
pub const Usb20Hub: USB_HUB_TYPE = 2;
pub const Usb30Hub: USB_HUB_TYPE = 3;
pub const UsbController: USB_WMI_DEVICE_NODE_TYPE = 3;
pub const UsbDevice: USB_WMI_DEVICE_NODE_TYPE = 0;
pub const UsbHub: USB_HUB_NODE = 0;
pub const UsbMIParent: USB_HUB_NODE = 1;
pub const UsbRootHub: USB_HUB_TYPE = 1;
pub const WMI_USB_DEVICE_NODE_INFORMATION: i32 = 2;
pub const WMI_USB_DRIVER_INFORMATION: i32 = 0;
pub const WMI_USB_DRIVER_NOTIFICATION: i32 = 1;
pub const WMI_USB_HUB_NODE_INFORMATION: i32 = 4;
pub const WMI_USB_PERFORMANCE_INFORMATION: i32 = 1;
pub const WMI_USB_POWER_DEVICE_ENABLE: i32 = 2;
