#[cfg(feature = "minwindef")]
windows_link::link!("sti.dll" "system" fn StiCreateInstanceW(hinst : super::HINSTANCE, dwver : u32, ppsti : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const CLSID_Sti: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb323f8e0_2e68_11d0_90ea_00aa0060f86c);
pub type DIAG = STI_DIAG;
pub const GUID_DeviceArrivedLaunch: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x740d9ee6_70f1_11d1_ad10_00a02438ad48);
pub const GUID_STIUserDefined1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc00eb795_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc77ae9c5_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc77ae9c6_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanFaxImage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanImage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanPrintImage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IStiDeviceW(pub u8);
pub type LPDIAG = LPSTI_DIAG;
pub type LPSTINOTIFY = *mut STINOTIFY;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type LPSTISUBSCRIBE = *mut STISUBSCRIBE;
pub type LPSTI_DIAG = *mut STI_DIAG;
pub const MAX_NOTIFICATION_DATA: i32 = 64;
pub type PSTIDEVICEW = *mut IStiDeviceW;
pub type PSTI_DEVICE_INFORMATION = PSTI_DEVICE_INFORMATIONW;
pub type PSTI_DEVICE_INFORMATIONW = *mut STI_DEVICE_INFORMATIONW;
pub type PSTI_DEVICE_STATUS = *mut STI_DEVICE_STATUS;
pub type PSTI_DEV_CAPS = *mut STI_DEV_CAPS;
pub type PSTI_ERROR_INFO = *mut STI_ERROR_INFO;
pub type PSTI_ERROR_INFOW = *mut STI_ERROR_INFOW;
pub type PSTI_WIA_DEVICE_INFORMATION = PSTI_WIA_DEVICE_INFORMATIONW;
pub type PSTI_WIA_DEVICE_INFORMATIONW = *mut STI_WIA_DEVICE_INFORMATIONW;
pub const STIEDFL_ALLDEVICES: i32 = 0;
pub const STIEDFL_ATTACHEDONLY: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STINOTIFY {
    pub dwSize: u32,
    pub guidNotificationCode: windows_sys::core::GUID,
    pub abNotificationData: [u8; 64],
}
impl Default for STINOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STISUBSCRIBE {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFilter: u32,
    pub hWndNotify: super::HWND,
    pub hEvent: super::HANDLE,
    pub uiNotificationMessage: u32,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for STISUBSCRIBE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STI_ADD_DEVICE_BROADCAST_ACTION: windows_sys::core::PCSTR = windows_sys::core::s!("Arrival");
pub const STI_DEVICE_CREATE_BOTH: i32 = 3;
pub const STI_DEVICE_CREATE_DATA: i32 = 2;
pub const STI_DEVICE_CREATE_MASK: i32 = 65535;
pub const STI_DEVICE_CREATE_STATUS: i32 = 1;
pub type STI_DEVICE_INFORMATION = STI_DEVICE_INFORMATIONW;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: STI_DEVICE_TYPE,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilities: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: windows_sys::core::PWSTR,
    pub pszDeviceDescription: windows_sys::core::PWSTR,
    pub pszPortName: windows_sys::core::PWSTR,
    pub pszPropProvider: windows_sys::core::PWSTR,
    pub pszLocalName: windows_sys::core::PWSTR,
}
impl Default for STI_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type STI_DEVICE_MJ_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STI_DEVICE_STATUS {
    pub dwSize: u32,
    pub StatusMask: u32,
    pub dwOnlineState: u32,
    pub dwHardwareStatusCode: u32,
    pub dwEventHandlingState: u32,
    pub dwPollingInterval: u32,
}
pub type STI_DEVICE_TYPE = u32;
pub const STI_DEVSTATUS_EVENTS_STATE: i32 = 2;
pub const STI_DEVSTATUS_ONLINE_STATE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STI_DEV_CAPS {
    pub dwGeneric: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STI_DIAG {
    pub dwSize: u32,
    pub dwBasicDiagCode: u32,
    pub dwVendorDiagCode: u32,
    pub dwStatusMask: u32,
    pub sErrorInfo: STI_ERROR_INFO,
}
pub const STI_DIAGCODE_HWPRESENCE: i32 = 1;
pub type STI_ERROR_INFO = STI_ERROR_INFOW;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STI_ERROR_INFOW {
    pub dwSize: u32,
    pub dwGenericError: u32,
    pub dwVendorError: u32,
    pub szExtendedErrorText: [u16; 255],
}
impl Default for STI_ERROR_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STI_EVENTHANDLING_ENABLED: i32 = 1;
pub const STI_EVENTHANDLING_PENDING: i32 = 4;
pub const STI_EVENTHANDLING_POLLING: i32 = 2;
pub const STI_GENCAP_AUTO_PORTSELECT: i32 = 8;
pub const STI_GENCAP_COMMON_MASK: u32 = 255;
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: i32 = 4;
pub const STI_GENCAP_NOTIFICATIONS: i32 = 1;
pub const STI_GENCAP_POLLING_NEEDED: i32 = 2;
pub const STI_GENCAP_SUBSET: i32 = 32;
pub const STI_GENCAP_WIA: i32 = 16;
pub const STI_HW_CONFIG_PARALLEL: i32 = 16;
pub const STI_HW_CONFIG_SCSI: i32 = 2;
pub const STI_HW_CONFIG_SERIAL: i32 = 8;
pub const STI_HW_CONFIG_UNKNOWN: i32 = 1;
pub const STI_HW_CONFIG_USB: i32 = 4;
pub const STI_MAX_INTERNAL_NAME_LENGTH: i32 = 128;
pub const STI_ONLINESTATE_BUSY: i32 = 256;
pub const STI_ONLINESTATE_ERROR: i32 = 4;
pub const STI_ONLINESTATE_INITIALIZING: i32 = 1024;
pub const STI_ONLINESTATE_IO_ACTIVE: i32 = 128;
pub const STI_ONLINESTATE_OFFLINE: i32 = 64;
pub const STI_ONLINESTATE_OPERATIONAL: i32 = 1;
pub const STI_ONLINESTATE_PAPER_JAM: i32 = 16;
pub const STI_ONLINESTATE_PAPER_PROBLEM: i32 = 32;
pub const STI_ONLINESTATE_PAUSED: i32 = 8;
pub const STI_ONLINESTATE_PENDING: i32 = 2;
pub const STI_ONLINESTATE_POWER_SAVE: i32 = 8192;
pub const STI_ONLINESTATE_TRANSFERRING: i32 = 512;
pub const STI_ONLINESTATE_USER_INTERVENTION: i32 = 4096;
pub const STI_ONLINESTATE_WARMING_UP: i32 = 2048;
pub type STI_RAW_CONTROL_CODE = u32;
pub const STI_RAW_RESERVED: i32 = 4096;
pub const STI_REMOVE_DEVICE_BROADCAST_ACTION: windows_sys::core::PCSTR = windows_sys::core::s!("Removal");
pub const STI_SUBSCRIBE_FLAG_EVENT: i32 = 2;
pub const STI_SUBSCRIBE_FLAG_WINDOW: i32 = 1;
pub const STI_TRACE_ERROR: i32 = 4;
pub const STI_TRACE_INFORMATION: i32 = 1;
pub const STI_TRACE_WARNING: i32 = 2;
pub const STI_UNICODE: i32 = 1;
pub const STI_VERSION: i32 = 2;
pub const STI_VERSION_3: i32 = 16777219;
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080;
pub const STI_VERSION_FLAG_UNICODE: i32 = 16777216;
pub const STI_VERSION_MIN_ALLOWED: i32 = 2;
pub const STI_VERSION_REAL: i32 = 2;
pub type STI_WIA_DEVICE_INFORMATION = STI_WIA_DEVICE_INFORMATIONW;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: STI_DEVICE_TYPE,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilities: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: windows_sys::core::PWSTR,
    pub pszDeviceDescription: windows_sys::core::PWSTR,
    pub pszPortName: windows_sys::core::PWSTR,
    pub pszPropProvider: windows_sys::core::PWSTR,
    pub pszLocalName: windows_sys::core::PWSTR,
    pub pszUiDll: windows_sys::core::PWSTR,
    pub pszServer: windows_sys::core::PWSTR,
}
impl Default for STI_WIA_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = 0;
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = 2;
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = 1;
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = 3;
pub const WIA_INCOMPAT_XP: i32 = 1;
