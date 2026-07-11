pub const FILE_DEVICE_USB: u32 = 34;
pub const GUID_DEVINTERFACE_USB_BILLBOARD: windows_core::GUID = windows_core::GUID::from_u128(0x5e9adaef_f879_473f_b807_4e5ea77d1b1c);
pub const GUID_DEVINTERFACE_USB_DEVICE: windows_core::GUID = windows_core::GUID::from_u128(0xa5dcbf10_6530_11d2_901f_00c04fb951ed);
pub const GUID_DEVINTERFACE_USB_HOST_CONTROLLER: windows_core::GUID = windows_core::GUID::from_u128(0x3abf6f2d_71c4_462a_8a92_1e6861e6af27);
pub const GUID_DEVINTERFACE_USB_HUB: windows_core::GUID = windows_core::GUID::from_u128(0xf18a0e88_c30c_11d0_8815_00a0c906bed8);
pub const GUID_USB_PERFORMANCE_TRACING: windows_core::GUID = windows_core::GUID::from_u128(0xd5de77a6_6ae9_425c_b1e2_f5615fd348a9);
pub const GUID_USB_TRANSFER_TRACING: windows_core::GUID = windows_core::GUID::from_u128(0x681eb8aa_403d_452c_9f8a_f0616fac9540);
pub const GUID_USB_WMI_DEVICE_PERF_INFO: windows_core::GUID = windows_core::GUID::from_u128(0x66c1aa3c_499f_49a0_a9a5_61e2359f6407);
pub const GUID_USB_WMI_NODE_INFO: windows_core::GUID = windows_core::GUID::from_u128(0x9c179357_dc7a_4f41_b66b_323b9ddcb5b1);
pub const GUID_USB_WMI_STD_DATA: windows_core::GUID = windows_core::GUID::from_u128(0x4e623b20_cb14_11d1_b331_00a0c959bbd2);
pub const GUID_USB_WMI_STD_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x4e623b20_cb14_11d1_b331_00a0c959bbd2);
pub const GUID_USB_WMI_SURPRISE_REMOVAL_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0x9bbbf831_a2f2_43b4_96d1_86944b5914b3);
pub const GUID_USB_WMI_TRACING: windows_core::GUID = windows_core::GUID::from_u128(0x3a61881b_b4e6_4bf9_ae0f_3cd8f394e52f);
pub const HCD_DIAGNOSTIC_MODE_OFF: u32 = 257;
pub const HCD_DIAGNOSTIC_MODE_ON: u32 = 256;
pub const HCD_DISABLE_PORT: u32 = 268;
pub const HCD_ENABLE_PORT: u32 = 269;
pub const HCD_GET_DRIVERKEY_NAME: u32 = 265;
pub const HCD_GET_ROOT_HUB_NAME: u32 = 258;
pub const HCD_GET_STATS_1: u32 = 255;
pub const HCD_GET_STATS_2: u32 = 266;
pub const HCD_TRACE_READ_REQUEST: u32 = 275;
pub const HCD_USER_REQUEST: u32 = 270;
pub type PUSB_IDLE_CALLBACK_INFO = *mut USB_IDLE_CALLBACK_INFO;
pub const USB_CYCLE_PORT: u32 = 7;
pub const USB_DIAG_IGNORE_HUBS_OFF: u32 = 263;
pub const USB_DIAG_IGNORE_HUBS_ON: u32 = 262;
pub const USB_ENABLE_PORT: u32 = 5;
pub const USB_FAIL_GET_STATUS: u32 = 280;
pub const USB_GET_BUSGUID_INFO: u32 = 266;
pub const USB_GET_BUS_INFO: u32 = 264;
pub const USB_GET_CONTROLLER_NAME: u32 = 265;
pub const USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION: u32 = 260;
pub const USB_GET_DEVICE_CHARACTERISTICS: u32 = 288;
pub const USB_GET_DEVICE_HANDLE: u32 = 268;
pub const USB_GET_DEVICE_HANDLE_EX: u32 = 269;
pub const USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC: u32 = 286;
pub const USB_GET_HUB_CAPABILITIES: u32 = 271;
pub const USB_GET_HUB_CAPABILITIES_EX: u32 = 276;
pub const USB_GET_HUB_CONFIG_INFO: u32 = 275;
pub const USB_GET_HUB_COUNT: u32 = 6;
pub const USB_GET_HUB_INFORMATION_EX: u32 = 277;
pub const USB_GET_HUB_NAME: u32 = 8;
pub const USB_GET_NODE_CONNECTION_ATTRIBUTES: u32 = 272;
pub const USB_GET_NODE_CONNECTION_DRIVERKEY_NAME: u32 = 264;
pub const USB_GET_NODE_CONNECTION_INFORMATION: u32 = 259;
pub const USB_GET_NODE_CONNECTION_INFORMATION_EX: u32 = 274;
pub const USB_GET_NODE_CONNECTION_INFORMATION_EX_V2: u32 = 279;
pub const USB_GET_NODE_CONNECTION_NAME: u32 = 261;
pub const USB_GET_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION: u32 = 289;
pub const USB_GET_NODE_INFORMATION: u32 = 258;
pub const USB_GET_PARENT_HUB_INFO: u32 = 267;
pub const USB_GET_PORT_CONNECTOR_PROPERTIES: u32 = 278;
pub const USB_GET_PORT_STATUS: u32 = 4;
pub const USB_GET_ROOTHUB_PDO: u32 = 3;
pub const USB_GET_TOPOLOGY_ADDRESS: u32 = 271;
pub const USB_GET_TRANSPORT_CHARACTERISTICS: u32 = 281;
pub const USB_GET_TT_DEVICE_HANDLE: u32 = 270;
pub const USB_HUB_CYCLE_PORT: u32 = 273;
pub type USB_IDLE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct USB_IDLE_CALLBACK_INFO {
    pub IdleCallback: USB_IDLE_CALLBACK,
    pub IdleContext: *mut core::ffi::c_void,
}
impl Default for USB_IDLE_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USB_IDLE_NOTIFICATION: u32 = 9;
pub const USB_IDLE_NOTIFICATION_EX: u32 = 272;
pub const USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 283;
pub const USB_RECORD_FAILURE: u32 = 10;
pub const USB_REGISTER_COMPOSITE_DEVICE: u32 = 0;
pub const USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 282;
pub const USB_REQUEST_REMOTE_WAKE_NOTIFICATION: u32 = 2;
pub const USB_REQ_GLOBAL_RESUME: u32 = 274;
pub const USB_REQ_GLOBAL_SUSPEND: u32 = 273;
pub const USB_RESERVED_USER_BASE: u32 = 1024;
pub const USB_RESET_HUB: u32 = 275;
pub const USB_RESET_PORT: u32 = 1;
pub const USB_START_TRACKING_FOR_TIME_SYNC: u32 = 285;
pub const USB_STOP_TRACKING_FOR_TIME_SYNC: u32 = 287;
pub const USB_SUBMIT_URB: u32 = 0;
pub const USB_UNREGISTER_COMPOSITE_DEVICE: u32 = 1;
pub const USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 284;
