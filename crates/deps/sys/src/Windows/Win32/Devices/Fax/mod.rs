#![allow(non_snake_case, non_camel_case_types)]
pub const CLSID_Sti: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3005479136, data2: 11880, data3: 4560, data4: [144, 234, 0, 170, 0, 96, 248, 108] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Fax`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WIA_DeviceType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Fax`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WIA_USDClassId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] },
    pid: 3u32,
};
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAXDEVRECEIVE_SIZE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAXDEVREPORTSTATUS_SIZE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_CONFIG_QUERY: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_CONFIG_SET: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_BAD_GROUP_CONFIGURATION: i32 = 7003i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED: i32 = 7010i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_DIRECTORY_IN_USE: i32 = 7007i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_END: i32 = 7013i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_FILE_ACCESS_DENIED: i32 = 7008i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_GROUP_IN_USE: i32 = 7004i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_GROUP_NOT_FOUND: i32 = 7002i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_MESSAGE_NOT_FOUND: i32 = 7009i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_NOT_NTFS: i32 = 7006i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU: i32 = 7011i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_RECIPIENTS_LIMIT: i32 = 7013i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_RULE_NOT_FOUND: i32 = 7005i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_SRV_OUTOFMEMORY: i32 = 7001i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_START: i32 = 7001i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_ERR_VERSION_MISMATCH: i32 = 7012i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_BAD_GROUP_CONFIGURATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214501i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_DEVICE_NUM_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214494i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_DIRECTORY_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214497i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_FILE_ACCESS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214496i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_GROUP_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214500i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_GROUP_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214502i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_MESSAGE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214495i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_NOT_NTFS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214498i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_NOT_SUPPORTED_ON_THIS_SKU: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214493i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_RECIPIENTS_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214491i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_RULE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214499i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_SRV_OUTOFMEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214503i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_E_VERSION_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214492i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_JOB_MANAGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_JOB_QUERY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_JOB_SUBMIT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_PORT_QUERY: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FAX_PORT_SET: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_ABORTING: u32 = 15u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_ANSWERED: u32 = 21u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_BAD_ADDRESS: u32 = 7u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_BUSY: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_CALL_BLACKLISTED: u32 = 13u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_CALL_DELAYED: u32 = 12u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_COMPLETED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_DELETED: u32 = 23u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_DIALING: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_DISCONNECTED: u32 = 9u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_FATAL_ERROR: u32 = 10u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_FAXSVC_ENDED: u32 = 20u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_FAXSVC_STARTED: u32 = 27u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_HANDLED: u32 = 26u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_IDLE: u32 = 19u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_INITIALIZING: u32 = 24u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_JOB_QUEUED: u32 = 22u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_LINE_UNAVAILABLE: u32 = 25u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_MODEM_POWERED_OFF: u32 = 18u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_MODEM_POWERED_ON: u32 = 17u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_NEVENTS: u32 = 27u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_NOT_FAX_CALL: u32 = 11u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_NO_ANSWER: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_NO_DIAL_TONE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_RECEIVING: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_RINGING: u32 = 14u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_ROUTING: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FEI_SENDING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPF_RECEIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPF_SEND: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPF_VIRTUAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_ABORTING: u32 = 538968064u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_ANSWERED: u32 = 545259520u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_AVAILABLE: u32 = 537919488u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_BAD_ADDRESS: u32 = 536871168u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_BUSY: u32 = 536870976u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_CALL_BLACKLISTED: u32 = 536887296u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_CALL_DELAYED: u32 = 536879104u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_COMPLETED: u32 = 536870920u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_DIALING: u32 = 536870913u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_DISCONNECTED: u32 = 536871936u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_FATAL_ERROR: u32 = 536872960u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_HANDLED: u32 = 536870928u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_INITIALIZING: u32 = 536903680u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_NOT_FAX_CALL: u32 = 536875008u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_NO_ANSWER: u32 = 536871040u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_NO_DIAL_TONE: u32 = 536871424u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_OFFLINE: u32 = 536936448u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_RECEIVING: u32 = 536870916u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_RINGING: u32 = 537001984u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_ROUTING: u32 = 541065216u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_SENDING: u32 = 536870914u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FPS_UNAVAILABLE: u32 = 536870944u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_ANSWERED: u32 = 545259520u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_BAD_ADDRESS: u32 = 536871168u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_BUSY: u32 = 536870976u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_CALL_BLACKLISTED: u32 = 536887296u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_CALL_DELAYED: u32 = 536879104u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_COMPLETED: u32 = 536870920u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_DIALING: u32 = 536870913u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_DISCONNECTED: u32 = 536871936u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_FATAL_ERROR: u32 = 536872960u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_HANDLED: u32 = 536870928u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_INITIALIZING: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_LINE_UNAVAILABLE: u32 = 536870944u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_NOT_FAX_CALL: u32 = 536875008u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_NO_ANSWER: u32 = 536871040u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_NO_DIAL_TONE: u32 = 536871424u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_RECEIVING: u32 = 536870916u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_TRANSMITTING: u32 = 536870914u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const FS_USER_ABORT: u32 = 538968064u32;
pub const GUID_DeviceArrivedLaunch: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1947049702, data2: 28913, data3: 4561, data4: [173, 16, 0, 160, 36, 56, 173, 72] };
pub const GUID_STIUserDefined1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3222189973, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_STIUserDefined2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3346721221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_STIUserDefined3: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3346721222, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanFaxImage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3222189971, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanImage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2797971221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanPrintImage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3024221221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const IS_DIGITAL_CAMERA_VAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JS_DELETING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JS_FAILED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JS_INPROGRESS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JS_NOLINE: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JS_PAUSED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JS_PENDING: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JS_RETRIES_EXCEEDED: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JS_RETRYING: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JT_FAIL_RECEIVE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JT_RECEIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JT_ROUTING: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JT_SEND: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const JT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const MAX_NOTIFICATION_DATA: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIEDFL_ALLDEVICES: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIEDFL_ATTACHEDONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147023649i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_BADDRIVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024777i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_BETA_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147023743i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_DEVICENOTREG: i32 = -2147221164i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_DEVICE_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024863i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_DEVICE_NOTREADY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024875i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_GENERIC: i32 = -2147467259i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_HANDLEEXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024713i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_INVALID_DEVICE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024773i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_INVALID_HW_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024883i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_INVALID_PARAM: i32 = -2147024809i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_NEEDS_LOCK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024738i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_NOEVENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024637i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_NOINTERFACE: i32 = -2147467262i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_NOTINITIALIZED: i32 = -2147024891i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024875i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_OBJECTNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024894i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_OLD_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147023746i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_OUTOFMEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_READONLY: i32 = -2147024891i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_SHARING_VIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024864i32 as _);
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STIERR_UNSUPPORTED: i32 = -2147467263i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_CHANGENOEFFECT: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_DEVICE_CREATE_BOTH: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_DEVICE_CREATE_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_DEVICE_CREATE_FOR_MONITOR: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_DEVICE_CREATE_MASK: u32 = 65535u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_DEVICE_CREATE_STATUS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_DIAGCODE_HWPRESENCE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ERROR_NO_ERROR: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_EVENTHANDLING_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_EVENTHANDLING_PENDING: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_EVENTHANDLING_POLLING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_GENCAP_AUTO_PORTSELECT: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_GENCAP_NOTIFICATIONS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_GENCAP_POLLING_NEEDED: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_GENCAP_SUBSET: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_GENCAP_WIA: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_HW_CONFIG_PARALLEL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_HW_CONFIG_SCSI: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_HW_CONFIG_SERIAL: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_HW_CONFIG_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_HW_CONFIG_USB: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_MAX_INTERNAL_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_NOTCONNECTED: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_OK: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_BUSY: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_ERROR: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_INITIALIZING: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_IO_ACTIVE: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_OFFLINE: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_OPERATIONAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_PAPER_JAM: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_PAPER_PROBLEM: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_PAUSED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_PENDING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_POWER_SAVE: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_TRANSFERRING: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_USER_INTERVENTION: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_ONLINESTATE_WARMING_UP: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_RAW_RESERVED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_TRACE_ERROR: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_TRACE_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_TRACE_WARNING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_UNICODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_USD_GENCAP_NATIVE_PUSHSUPPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_VERSION_FLAG_UNICODE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_VERSION_MIN_ALLOWED: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const STI_VERSION_REAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const SUPPORTS_MSCPLUS_VAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const WIA_INCOMPAT_XP: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const lDEFAULT_PREFETCH_SIZE: i32 = 100i32;
#[doc = "*Required features: `Win32_Devices_Fax`*"]
pub const wcharREASSIGN_RECIPIENTS_DELIMITER: u16 = 59u16;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAbort(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAccessCheck(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxClose(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerA(machinename: super::super::Foundation::PSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerW(machinename: super::super::Foundation::PWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsA(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsW(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsA(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsW(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsA(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsW(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`*"]
    pub fn FaxFreeBuffer(buffer: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusA(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusW(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPageData(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxInitializeEventQueue(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxOpenPort(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterRoutingExtensionW(faxhandle: super::super::Foundation::HANDLE, extensionname: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterServiceProviderW(deviceprovider: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, tspname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentA(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastA(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastW(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentW(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobA(printername: super::super::Foundation::PSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobW(printername: super::super::Foundation::PWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxUnregisterServiceProviderW(deviceprovider: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendToFaxRecipient(sndmode: SendToMode, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StiCreateInstanceW(hinst: super::super::Foundation::HINSTANCE, dwver: u32, ppsti: *mut IStillImageW, punkouter: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
