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
#[derive(Clone, Copy)]
pub struct HCD_ISO_STAT_COUNTERS {
    pub LateUrbs: u16,
    pub DoubleBufferedPackets: u16,
    pub TransfersCF_5ms: u16,
    pub TransfersCF_2ms: u16,
    pub TransfersCF_1ms: u16,
    pub MaxInterruptLatency: u16,
    pub BadStartFrame: u16,
    pub StaleUrbs: u16,
    pub IsoPacketNotAccesed: u16,
    pub IsoPacketHWError: u16,
    pub SmallestUrbPacketCount: u16,
    pub LargestUrbPacketCount: u16,
    pub IsoCRC_Error: u16,
    pub IsoOVERRUN_Error: u16,
    pub IsoINTERNAL_Error: u16,
    pub IsoUNKNOWN_Error: u16,
    pub IsoBytesTransferred: u32,
    pub LateMissedCount: u16,
    pub HWIsoMissedCount: u16,
    pub Reserved7: [u32; 8],
}
impl Default for HCD_ISO_STAT_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HCD_STAT_COUNTERS {
    pub BytesTransferred: u32,
    pub IsoMissedCount: u16,
    pub DataOverrunErrorCount: u16,
    pub CrcErrorCount: u16,
    pub ScheduleOverrunCount: u16,
    pub TimeoutErrorCount: u16,
    pub InternalHcErrorCount: u16,
    pub BufferOverrunErrorCount: u16,
    pub SWErrorCount: u16,
    pub StallPidCount: u16,
    pub PortDisableCount: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HCD_STAT_INFORMATION_1 {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub ResetCounters: u32,
    pub TimeRead: i64,
    pub Counters: HCD_STAT_COUNTERS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HCD_STAT_INFORMATION_2 {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub ResetCounters: u32,
    pub TimeRead: i64,
    pub LockedMemoryUsed: i32,
    pub Counters: HCD_STAT_COUNTERS,
    pub IsoCounters: HCD_ISO_STAT_COUNTERS,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct HUB_DEVICE_CONFIG_INFO {
    pub Version: u32,
    pub Length: u32,
    pub HubFlags: USB_HUB_CAP_FLAGS,
    pub HardwareIds: USB_ID_STRING,
    pub CompatibleIds: USB_ID_STRING,
    pub DeviceDescription: USB_ID_STRING,
    pub Reserved: [u32; 19],
    pub UxdSettings: USB_HUB_DEVICE_UXD_SETTINGS,
}
#[cfg(feature = "Win32_winnt")]
impl Default for HUB_DEVICE_CONFIG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HubDevice: USB_WMI_DEVICE_NODE_TYPE = 1;
pub const HubNestedTooDeeply: USB_NOTIFICATION_TYPE = 10;
pub const HubOvercurrent: USB_NOTIFICATION_TYPE = 8;
pub const HubPowerChange: USB_NOTIFICATION_TYPE = 9;
pub const IOCTL_GET_HCD_DRIVERKEY_NAME: u32 = 2229284;
pub const IOCTL_INTERNAL_USB_CYCLE_PORT: u32 = 2228255;
pub const IOCTL_INTERNAL_USB_ENABLE_PORT: u32 = 2228247;
pub const IOCTL_INTERNAL_USB_FAIL_GET_STATUS_FROM_DEVICE: u32 = 2229347;
pub const IOCTL_INTERNAL_USB_GET_BUSGUID_INFO: u32 = 2229288;
pub const IOCTL_INTERNAL_USB_GET_BUS_INFO: u32 = 2229280;
pub const IOCTL_INTERNAL_USB_GET_CONTROLLER_NAME: u32 = 2229284;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO: u32 = 2229327;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE: u32 = 2229299;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX: u32 = 2229303;
pub const IOCTL_INTERNAL_USB_GET_HUB_COUNT: u32 = 2228251;
pub const IOCTL_INTERNAL_USB_GET_HUB_NAME: u32 = 2228256;
pub const IOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO: u32 = 2229292;
pub const IOCTL_INTERNAL_USB_GET_PORT_STATUS: u32 = 2228243;
pub const IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO: u32 = 2228239;
pub const IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS: u32 = 2229311;
pub const IOCTL_INTERNAL_USB_GET_TT_DEVICE_HANDLE: u32 = 2229307;
pub const IOCTL_INTERNAL_USB_NOTIFY_IDLE_READY: u32 = 2229315;
pub const IOCTL_INTERNAL_USB_RECORD_FAILURE: u32 = 2228267;
pub const IOCTL_INTERNAL_USB_REGISTER_COMPOSITE_DEVICE: u32 = 4784131;
pub const IOCTL_INTERNAL_USB_REQUEST_REMOTE_WAKE_NOTIFICATION: u32 = 4784139;
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME: u32 = 2229323;
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND: u32 = 2229319;
pub const IOCTL_INTERNAL_USB_RESET_PORT: u32 = 2228231;
pub const IOCTL_INTERNAL_USB_SUBMIT_IDLE_NOTIFICATION: u32 = 2228263;
pub const IOCTL_INTERNAL_USB_SUBMIT_URB: u32 = 2228227;
pub const IOCTL_INTERNAL_USB_UNREGISTER_COMPOSITE_DEVICE: u32 = 4784135;
pub const IOCTL_USB_DIAGNOSTIC_MODE_OFF: u32 = 2229252;
pub const IOCTL_USB_DIAGNOSTIC_MODE_ON: u32 = 2229248;
pub const IOCTL_USB_DIAG_IGNORE_HUBS_OFF: u32 = 2229276;
pub const IOCTL_USB_DIAG_IGNORE_HUBS_ON: u32 = 2229272;
pub const IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION: u32 = 2229264;
pub const IOCTL_USB_GET_DEVICE_CHARACTERISTICS: u32 = 2229376;
pub const IOCTL_USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC: u32 = 2229368;
pub const IOCTL_USB_GET_HUB_CAPABILITIES: u32 = 2229308;
pub const IOCTL_USB_GET_HUB_CAPABILITIES_EX: u32 = 2229328;
pub const IOCTL_USB_GET_HUB_INFORMATION_EX: u32 = 2229332;
pub const IOCTL_USB_GET_NODE_CONNECTION_ATTRIBUTES: u32 = 2229312;
pub const IOCTL_USB_GET_NODE_CONNECTION_DRIVERKEY_NAME: u32 = 2229280;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION: u32 = 2229260;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX: u32 = 2229320;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2: u32 = 2229340;
pub const IOCTL_USB_GET_NODE_CONNECTION_NAME: u32 = 2229268;
pub const IOCTL_USB_GET_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION: u32 = 2229380;
pub const IOCTL_USB_GET_NODE_INFORMATION: u32 = 2229256;
pub const IOCTL_USB_GET_PORT_CONNECTOR_PROPERTIES: u32 = 2229336;
pub const IOCTL_USB_GET_ROOT_HUB_NAME: u32 = 2229256;
pub const IOCTL_USB_GET_TRANSPORT_CHARACTERISTICS: u32 = 2229348;
pub const IOCTL_USB_HCD_DISABLE_PORT: u32 = 2229296;
pub const IOCTL_USB_HCD_ENABLE_PORT: u32 = 2229300;
pub const IOCTL_USB_HCD_GET_STATS_1: u32 = 2229244;
pub const IOCTL_USB_HCD_GET_STATS_2: u32 = 2229288;
pub const IOCTL_USB_HUB_CYCLE_PORT: u32 = 2229316;
pub const IOCTL_USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229356;
pub const IOCTL_USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229352;
pub const IOCTL_USB_RESET_HUB: u32 = 2229324;
pub const IOCTL_USB_START_TRACKING_FOR_TIME_SYNC: u32 = 2229364;
pub const IOCTL_USB_STOP_TRACKING_FOR_TIME_SYNC: u32 = 2229372;
pub const IOCTL_USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229360;
pub const InsufficentBandwidth: USB_NOTIFICATION_TYPE = 1;
pub const InsufficentPower: USB_NOTIFICATION_TYPE = 2;
pub const ModernDeviceInLegacyHub: USB_NOTIFICATION_TYPE = 11;
pub const NoDeviceConnected: USB_CONNECTION_STATUS = 0;
pub const OverCurrent: USB_NOTIFICATION_TYPE = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHCD_ISO_STAT_COUNTERS(pub *mut HCD_ISO_STAT_COUNTERS);
impl PHCD_ISO_STAT_COUNTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHCD_ISO_STAT_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHCD_STAT_COUNTERS(pub *mut HCD_STAT_COUNTERS);
impl PHCD_STAT_COUNTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHCD_STAT_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHCD_STAT_INFORMATION_1(pub *mut HCD_STAT_INFORMATION_1);
impl PHCD_STAT_INFORMATION_1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHCD_STAT_INFORMATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHCD_STAT_INFORMATION_2(pub *mut HCD_STAT_INFORMATION_2);
impl PHCD_STAT_INFORMATION_2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHCD_STAT_INFORMATION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHUB_DEVICE_CONFIG_INFO(pub *mut HUB_DEVICE_CONFIG_INFO);
#[cfg(feature = "Win32_winnt")]
impl PHUB_DEVICE_CONFIG_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PHUB_DEVICE_CONFIG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_ACQUIRE_INFO(pub *mut USB_ACQUIRE_INFO);
impl PUSB_ACQUIRE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_ACQUIRE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_BUS_NOTIFICATION(pub *mut USB_BUS_NOTIFICATION);
impl PUSB_BUS_NOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_BUS_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_COMPOSITE_DEVICE_INFO(pub *mut USB_COMPOSITE_DEVICE_INFO);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_COMPOSITE_DEVICE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_COMPOSITE_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_COMPOSITE_FUNCTION_INFO(pub *mut USB_COMPOSITE_FUNCTION_INFO);
impl PUSB_COMPOSITE_FUNCTION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_COMPOSITE_FUNCTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_CONNECTION_NOTIFICATION(pub *mut USB_CONNECTION_NOTIFICATION);
impl PUSB_CONNECTION_NOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_CONNECTION_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_CONNECTION_STATUS(pub *mut USB_CONNECTION_STATUS);
impl PUSB_CONNECTION_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_CONNECTION_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_CONTROLLER_DEVICE_INFO(pub *mut USB_CONTROLLER_DEVICE_INFO);
impl PUSB_CONTROLLER_DEVICE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_CONTROLLER_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_CYCLE_PORT_PARAMS(pub *mut USB_CYCLE_PORT_PARAMS);
impl PUSB_CYCLE_PORT_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_CYCLE_PORT_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_DESCRIPTOR_REQUEST(pub *mut USB_DESCRIPTOR_REQUEST);
impl PUSB_DESCRIPTOR_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_DESCRIPTOR_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_DEVICE_CHARACTERISTICS(pub *mut USB_DEVICE_CHARACTERISTICS);
impl PUSB_DEVICE_CHARACTERISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_DEVICE_CHARACTERISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_DEVICE_INFO(pub *mut USB_DEVICE_INFO);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_DEVICE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_DEVICE_NODE_INFO(pub *mut USB_DEVICE_NODE_INFO);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_DEVICE_NODE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_DEVICE_NODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_DEVICE_PERFORMANCE_INFO(pub *mut USB_DEVICE_PERFORMANCE_INFO);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_DEVICE_PERFORMANCE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_DEVICE_PERFORMANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_DEVICE_STATE(pub *mut USB_DEVICE_STATE);
impl PUSB_DEVICE_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_DEVICE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION(pub *mut USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION);
#[cfg(feature = "Win32_winnt")]
impl PUSB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PUSB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HCD_DRIVERKEY_NAME(pub *mut USB_HCD_DRIVERKEY_NAME);
impl PUSB_HCD_DRIVERKEY_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_HCD_DRIVERKEY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_CAPABILITIES(pub *mut USB_HUB_CAPABILITIES);
impl PUSB_HUB_CAPABILITIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_HUB_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_CAPABILITIES_EX(pub *mut USB_HUB_CAPABILITIES_EX);
impl PUSB_HUB_CAPABILITIES_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_HUB_CAPABILITIES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_CAP_FLAGS(pub *mut USB_HUB_CAP_FLAGS);
impl PUSB_HUB_CAP_FLAGS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_HUB_CAP_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_DEVICE_INFO(pub *mut USB_HUB_DEVICE_INFO);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_HUB_DEVICE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_HUB_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_DEVICE_UXD_SETTINGS(pub *mut USB_HUB_DEVICE_UXD_SETTINGS);
impl PUSB_HUB_DEVICE_UXD_SETTINGS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_HUB_DEVICE_UXD_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_INFORMATION(pub *mut USB_HUB_INFORMATION);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_HUB_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_HUB_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_INFORMATION_EX(pub *mut USB_HUB_INFORMATION_EX);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_HUB_INFORMATION_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_HUB_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_NAME(pub *mut USB_HUB_NAME);
impl PUSB_HUB_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_HUB_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_HUB_PORT_INFORMATION(pub *mut USB_HUB_PORT_INFORMATION);
impl PUSB_HUB_PORT_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_HUB_PORT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_ID_STRING(pub *mut USB_ID_STRING);
#[cfg(feature = "Win32_winnt")]
impl PUSB_ID_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PUSB_ID_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_MI_PARENT_INFORMATION(pub *mut USB_MI_PARENT_INFORMATION);
impl PUSB_MI_PARENT_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_MI_PARENT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_CONNECTION_ATTRIBUTES(pub *mut USB_NODE_CONNECTION_ATTRIBUTES);
impl PUSB_NODE_CONNECTION_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_NODE_CONNECTION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_CONNECTION_DRIVERKEY_NAME(pub *mut USB_NODE_CONNECTION_DRIVERKEY_NAME);
impl PUSB_NODE_CONNECTION_DRIVERKEY_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_NODE_CONNECTION_DRIVERKEY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_CONNECTION_INFORMATION(pub *mut USB_NODE_CONNECTION_INFORMATION);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_NODE_CONNECTION_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_NODE_CONNECTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_CONNECTION_INFORMATION_EX(pub *mut USB_NODE_CONNECTION_INFORMATION_EX);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_NODE_CONNECTION_INFORMATION_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_NODE_CONNECTION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_CONNECTION_INFORMATION_EX_V2(pub *mut USB_NODE_CONNECTION_INFORMATION_EX_V2);
impl PUSB_NODE_CONNECTION_INFORMATION_EX_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_NODE_CONNECTION_INFORMATION_EX_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS(pub *mut USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS);
impl PUSB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_CONNECTION_NAME(pub *mut USB_NODE_CONNECTION_NAME);
impl PUSB_NODE_CONNECTION_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_NODE_CONNECTION_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION(pub *mut USB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NODE_INFORMATION(pub *mut USB_NODE_INFORMATION);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_NODE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_NODE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_NOTIFICATION(pub *mut USB_NOTIFICATION);
impl PUSB_NOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_PIPE_INFO(pub *mut USB_PIPE_INFO);
#[cfg(feature = "Win32_usbspec")]
impl PUSB_PIPE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_usbspec")]
impl Default for PUSB_PIPE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_PORT_CONNECTOR_PROPERTIES(pub *mut USB_PORT_CONNECTOR_PROPERTIES);
impl PUSB_PORT_CONNECTOR_PROPERTIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_PORT_CONNECTOR_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_PORT_PROPERTIES(pub *mut USB_PORT_PROPERTIES);
impl PUSB_PORT_PROPERTIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_PORT_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_PROTOCOLS(pub *mut USB_PROTOCOLS);
impl PUSB_PROTOCOLS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_PROTOCOLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_ROOT_HUB_NAME(pub *mut USB_ROOT_HUB_NAME);
impl PUSB_ROOT_HUB_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_ROOT_HUB_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_START_TRACKING_FOR_TIME_SYNC_INFORMATION(pub *mut USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION);
#[cfg(feature = "Win32_winnt")]
impl PUSB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PUSB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION(pub *mut USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION);
#[cfg(feature = "Win32_winnt")]
impl PUSB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PUSB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_TOPOLOGY_ADDRESS(pub *mut USB_TOPOLOGY_ADDRESS);
impl PUSB_TOPOLOGY_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_TOPOLOGY_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_TRANSPORT_CHARACTERISTICS(pub *mut USB_TRANSPORT_CHARACTERISTICS);
impl PUSB_TRANSPORT_CHARACTERISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_TRANSPORT_CHARACTERISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION(pub *mut USB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION);
impl PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION(pub *mut USB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION);
impl PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION(pub *mut USB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION);
impl PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSB_WMI_DEVICE_NODE_TYPE(pub *mut USB_WMI_DEVICE_NODE_TYPE);
impl PUSB_WMI_DEVICE_NODE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSB_WMI_DEVICE_NODE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ResetOvercurrent: USB_NOTIFICATION_TYPE = 4;
pub const USBD_PORT_CONNECTED: u32 = 2;
pub const USBD_PORT_ENABLED: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_ACQUIRE_INFO {
    pub NotificationType: USB_NOTIFICATION_TYPE,
    pub TotalSize: u32,
    pub Buffer: [u16; 1],
}
impl Default for USB_ACQUIRE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_BUS_NOTIFICATION {
    pub NotificationType: USB_NOTIFICATION_TYPE,
    pub TotalBandwidth: u32,
    pub ConsumedBandwidth: u32,
    pub ControllerNameLength: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct USB_CHANGE_REGISTRATION_HANDLE(pub *mut core::ffi::c_void);
impl USB_CHANGE_REGISTRATION_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for USB_CHANGE_REGISTRATION_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_COMPOSITE_DEVICE_INFO {
    pub DeviceDescriptor: super::usbspec::USB_DEVICE_DESCRIPTOR,
    pub CurrentConfigDescriptor: super::usbspec::USB_CONFIGURATION_DESCRIPTOR,
    pub CurrentConfigurationValue: u8,
    pub NumberOfFunctions: u8,
    pub FunctionInfo: [USB_COMPOSITE_FUNCTION_INFO; 1],
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_COMPOSITE_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USB_COMPOSITE_FUNCTION_INFO {
    pub FunctionNumber: u8,
    pub BaseInterfaceNumber: u8,
    pub NumberOfInterfaces: u8,
    pub FunctionIsIdle: bool,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_CONNECTION_NOTIFICATION {
    pub NotificationType: USB_NOTIFICATION_TYPE,
    pub ConnectionNumber: u32,
    pub RequestedBandwidth: u32,
    pub EnumerationFailReason: u32,
    pub PowerRequested: u32,
    pub HubNameLength: u32,
}
pub type USB_CONNECTION_STATUS = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_CONTROLLER_DEVICE_INFO {
    pub PciVendorId: u32,
    pub PciDeviceId: u32,
    pub PciRevision: u32,
    pub NumberOfRootPorts: u32,
    pub HcFeatureFlags: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_CYCLE_PORT_PARAMS {
    pub ConnectionIndex: u32,
    pub StatusReturned: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DESCRIPTOR_REQUEST {
    pub ConnectionIndex: u32,
    pub SetupPacket: USB_DESCRIPTOR_REQUEST_0,
    pub Data: [u8; 0],
}
impl Default for USB_DESCRIPTOR_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DESCRIPTOR_REQUEST_0 {
    pub bmRequest: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_CHARACTERISTICS {
    pub Version: u32,
    pub Reserved: [u32; 2],
    pub UsbDeviceCharacteristicsFlags: u32,
    pub MaximumSendPathDelayInMilliSeconds: u32,
    pub MaximumCompletionPathDelayInMilliSeconds: u32,
}
impl Default for USB_DEVICE_CHARACTERISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_DEVICE_CHARACTERISTICS_MAXIMUM_PATH_DELAYS_AVAILABLE: u32 = 1;
pub const USB_DEVICE_CHARACTERISTICS_VERSION_1: u32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_INFO {
    pub DeviceState: USB_DEVICE_STATE,
    pub PortNumber: u16,
    pub DeviceDescriptor: super::usbspec::USB_DEVICE_DESCRIPTOR,
    pub CurrentConfigurationValue: u8,
    pub Speed: super::usbspec::USB_DEVICE_SPEED,
    pub DeviceAddress: u16,
    pub ConnectionIndex: u32,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
    pub PnpHardwareId: [u16; 128],
    pub PnpCompatibleId: [u16; 128],
    pub SerialNumberId: [u16; 128],
    pub PnpDeviceDescription: [u16; 128],
    pub NumberOfOpenPipes: u32,
    pub PipeList: [USB_PIPE_INFO; 1],
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_NODE_INFO {
    pub Sig: u32,
    pub LengthInBytes: u32,
    pub DeviceDescription: [u16; 40],
    pub NodeType: USB_WMI_DEVICE_NODE_TYPE,
    pub BusAddress: USB_TOPOLOGY_ADDRESS,
    pub Anonymous: USB_DEVICE_NODE_INFO_0,
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_DEVICE_NODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub union USB_DEVICE_NODE_INFO_0 {
    pub UsbDeviceInfo: USB_DEVICE_INFO,
    pub HubDeviceInfo: USB_HUB_DEVICE_INFO,
    pub CompositeDeviceInfo: USB_COMPOSITE_DEVICE_INFO,
    pub ControllerDeviceInfo: USB_CONTROLLER_DEVICE_INFO,
    pub DeviceInformation: [u8; 4],
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_DEVICE_NODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_DEVICE_PERFORMANCE_INFO {
    pub BulkBytes: u32,
    pub ControlDataBytes: u32,
    pub IsoBytes: u32,
    pub InterruptBytes: u32,
    pub BulkUrbCount: u32,
    pub ControlUrbCount: u32,
    pub IsoUrbCount: u32,
    pub InterruptUrbCount: u32,
    pub AllocedInterrupt: [u32; 6],
    pub AllocedIso: u32,
    pub Total32secBandwidth: u32,
    pub TotalTtBandwidth: u32,
    pub DeviceDescription: [u16; 60],
    pub DeviceSpeed: super::usbspec::USB_DEVICE_SPEED,
    pub TotalIsoLatency: u32,
    pub DroppedIsoPackets: u32,
    pub TransferErrors: u32,
    pub PciInterruptCount: u32,
    pub HcIdleState: u32,
    pub HcAsyncIdleState: u32,
    pub HcAsyncCacheFlushCount: u32,
    pub HcPeriodicIdleState: u32,
    pub HcPeriodicCacheFlushCount: u32,
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_DEVICE_PERFORMANCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_DEVICE_STATE {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::winnt::HANDLE,
    pub InputFrameNumber: u32,
    pub InputMicroFrameNumber: u32,
    pub QueryPerformanceCounterAtInputFrameOrMicroFrame: i64,
    pub QueryPerformanceCounterFrequency: i64,
    pub PredictedAccuracyInMicroSeconds: u32,
    pub CurrentGenerationID: u32,
    pub CurrentQueryPerformanceCounter: i64,
    pub CurrentHardwareFrameNumber: u32,
    pub CurrentHardwareMicroFrameNumber: u32,
    pub CurrentUSBFrameNumber: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_HCD_DRIVERKEY_NAME {
    pub ActualLength: u32,
    pub DriverKeyName: [u16; 1],
}
impl Default for USB_HCD_DRIVERKEY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_CAPABILITIES {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct USB_HUB_CAPABILITIES_EX {
    pub CapabilityFlags: USB_HUB_CAP_FLAGS,
}
impl Default for USB_HUB_CAPABILITIES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_HUB_CAP_FLAGS {
    pub ul: u32,
    pub Anonymous: USB_HUB_CAP_FLAGS_0,
}
impl Default for USB_HUB_CAP_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_CAP_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_HUB_DEVICE_INFO {
    pub HubDescriptor: super::usbspec::USB_HUB_DESCRIPTOR,
    pub HubNumber: u32,
    pub DeviceAddress: u16,
    pub HubIsSelfPowered: bool,
    pub HubIsRootHub: bool,
    pub HubCapabilities: USB_HUB_CAPABILITIES,
    pub NumberOfHubPorts: u32,
    pub PortInfo: [USB_HUB_PORT_INFORMATION; 1],
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_HUB_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_HUB_DEVICE_UXD_SETTINGS {
    pub Version: u32,
    pub PnpGuid: windows_core::GUID,
    pub OwnerGuid: windows_core::GUID,
    pub DeleteOnShutdown: u32,
    pub DeleteOnReload: u32,
    pub DeleteOnDisconnect: u32,
    pub Reserved: [u32; 5],
}
impl Default for USB_HUB_DEVICE_UXD_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_INFORMATION {
    pub HubDescriptor: super::usbspec::USB_HUB_DESCRIPTOR,
    pub HubIsBusPowered: bool,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_HUB_INFORMATION_EX {
    pub HubType: USB_HUB_TYPE,
    pub HighestPortNumber: u16,
    pub u: USB_HUB_INFORMATION_EX_0,
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_HUB_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub union USB_HUB_INFORMATION_EX_0 {
    pub UsbHubDescriptor: super::usbspec::USB_HUB_DESCRIPTOR,
    pub Usb30HubDescriptor: super::usbspec::USB_30_HUB_DESCRIPTOR,
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_HUB_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_HUB_NAME {
    pub ActualLength: u32,
    pub HubName: [u16; 1],
}
impl Default for USB_HUB_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type USB_HUB_NODE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_HUB_PORT_INFORMATION {
    pub DeviceState: USB_DEVICE_STATE,
    pub PortNumber: u16,
    pub DeviceAddress: u16,
    pub ConnectionIndex: u32,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
}
pub type USB_HUB_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct USB_ID_STRING {
    pub LanguageId: u16,
    pub Pad: u16,
    pub LengthInBytes: u32,
    pub Buffer: super::winnt::PWCHAR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_MI_PARENT_INFORMATION {
    pub NumberOfInterfaces: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_NODE_CONNECTION_ATTRIBUTES {
    pub ConnectionIndex: u32,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
    pub PortAttributes: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_DRIVERKEY_NAME {
    pub ConnectionIndex: u32,
    pub ActualLength: u32,
    pub DriverKeyName: [u16; 1],
}
impl Default for USB_NODE_CONNECTION_DRIVERKEY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_INFORMATION {
    pub ConnectionIndex: u32,
    pub DeviceDescriptor: super::usbspec::USB_DEVICE_DESCRIPTOR,
    pub CurrentConfigurationValue: u8,
    pub LowSpeed: bool,
    pub DeviceIsHub: bool,
    pub DeviceAddress: u16,
    pub NumberOfOpenPipes: u32,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
    pub PipeList: [USB_PIPE_INFO; 0],
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_NODE_CONNECTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_INFORMATION_EX {
    pub ConnectionIndex: u32,
    pub DeviceDescriptor: super::usbspec::USB_DEVICE_DESCRIPTOR,
    pub CurrentConfigurationValue: u8,
    pub Speed: u8,
    pub DeviceIsHub: bool,
    pub DeviceAddress: u16,
    pub NumberOfOpenPipes: u32,
    pub ConnectionStatus: USB_CONNECTION_STATUS,
    pub PipeList: [USB_PIPE_INFO; 0],
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_NODE_CONNECTION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_INFORMATION_EX_V2 {
    pub ConnectionIndex: u32,
    pub Length: u32,
    pub SupportedUsbProtocols: USB_PROTOCOLS,
    pub Flags: USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS,
}
impl Default for USB_NODE_CONNECTION_INFORMATION_EX_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS {
    pub ul: u32,
    pub Anonymous: USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS_0,
}
impl Default for USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_NAME {
    pub ConnectionIndex: u32,
    pub ActualLength: u32,
    pub NodeName: [u16; 1],
}
impl Default for USB_NODE_CONNECTION_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION {
    pub ConnectionIndex: u32,
    pub Length: u32,
    pub RxSuperSpeedPlus: super::usbspec::USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED,
    pub RxLaneCount: u32,
    pub TxSuperSpeedPlus: super::usbspec::USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED,
    pub TxLaneCount: u32,
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub struct USB_NODE_INFORMATION {
    pub NodeType: USB_HUB_NODE,
    pub u: USB_NODE_INFORMATION_0,
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_NODE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy)]
pub union USB_NODE_INFORMATION_0 {
    pub HubInformation: USB_HUB_INFORMATION,
    pub MiParentInformation: USB_MI_PARENT_INFORMATION,
}
#[cfg(feature = "Win32_usbspec")]
impl Default for USB_NODE_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_NODE_INFO_SIG: u32 = 1431519822;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_NOTIFICATION {
    pub NotificationType: USB_NOTIFICATION_TYPE,
}
pub type USB_NOTIFICATION_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_usbspec")]
#[derive(Clone, Copy, Default)]
pub struct USB_PIPE_INFO {
    pub EndpointDescriptor: super::usbspec::USB_ENDPOINT_DESCRIPTOR,
    pub ScheduleOffset: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_PORT_CONNECTOR_PROPERTIES {
    pub ConnectionIndex: u32,
    pub ActualLength: u32,
    pub UsbPortProperties: USB_PORT_PROPERTIES,
    pub CompanionIndex: u16,
    pub CompanionPortNumber: u16,
    pub CompanionHubSymbolicLinkName: [u16; 1],
}
impl Default for USB_PORT_CONNECTOR_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_PORT_PROPERTIES {
    pub ul: u32,
    pub Anonymous: USB_PORT_PROPERTIES_0,
}
impl Default for USB_PORT_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_PORT_PROPERTIES_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union USB_PROTOCOLS {
    pub ul: u32,
    pub Anonymous: USB_PROTOCOLS_0,
}
impl Default for USB_PROTOCOLS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_PROTOCOLS_0 {
    pub _bitfield: u32,
}
pub const USB_REGISTER_FOR_TRANSPORT_BANDWIDTH_CHANGE: u32 = 2;
pub const USB_REGISTER_FOR_TRANSPORT_LATENCY_CHANGE: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USB_ROOT_HUB_NAME {
    pub ActualLength: u32,
    pub RootHubName: [u16; 1],
}
impl Default for USB_ROOT_HUB_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::winnt::HANDLE,
    pub IsStartupDelayTolerable: bool,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::winnt::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USB_TOPOLOGY_ADDRESS {
    pub PciBusNumber: u32,
    pub PciDeviceNumber: u32,
    pub PciFunctionNumber: u32,
    pub Reserved: u32,
    pub RootHubPortNumber: u16,
    pub HubPortNumber: [u16; 5],
    pub Reserved2: u16,
}
impl Default for USB_TOPOLOGY_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_TRANSPORT_CHARACTERISTICS {
    pub Version: u32,
    pub TransportCharacteristicsFlags: u32,
    pub CurrentRoundtripLatencyInMilliSeconds: u64,
    pub MaxPotentialBandwidth: u64,
}
pub const USB_TRANSPORT_CHARACTERISTICS_BANDWIDTH_AVAILABLE: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION {
    pub Handle: USB_CHANGE_REGISTRATION_HANDLE,
    pub UsbTransportCharacteristics: USB_TRANSPORT_CHARACTERISTICS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION {
    pub ChangeNotificationInputFlags: u32,
    pub Handle: USB_CHANGE_REGISTRATION_HANDLE,
    pub UsbTransportCharacteristics: USB_TRANSPORT_CHARACTERISTICS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct USB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION {
    pub Handle: USB_CHANGE_REGISTRATION_HANDLE,
}
pub const USB_TRANSPORT_CHARACTERISTICS_LATENCY_AVAILABLE: u32 = 1;
pub const USB_TRANSPORT_CHARACTERISTICS_VERSION_1: u32 = 1;
pub type USB_WMI_DEVICE_NODE_TYPE = i32;
pub const Usb20Hub: USB_HUB_TYPE = 2;
pub const Usb30Hub: USB_HUB_TYPE = 3;
pub const UsbController: USB_WMI_DEVICE_NODE_TYPE = 3;
pub const UsbDevice: USB_WMI_DEVICE_NODE_TYPE = 0;
pub const UsbHub: USB_HUB_NODE = 0;
pub const UsbMIParent: USB_HUB_NODE = 1;
pub const UsbRootHub: USB_HUB_TYPE = 1;
pub const WMI_USB_DEVICE_NODE_INFORMATION: u32 = 2;
pub const WMI_USB_DRIVER_INFORMATION: u32 = 0;
pub const WMI_USB_DRIVER_NOTIFICATION: u32 = 1;
pub const WMI_USB_HUB_NODE_INFORMATION: u32 = 4;
pub const WMI_USB_PERFORMANCE_INFORMATION: u32 = 1;
pub const WMI_USB_POWER_DEVICE_ENABLE: u32 = 2;
