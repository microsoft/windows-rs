#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
pub const EVENT_WMDM_CONTENT_TRANSFER: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 865901556,
    data2: 48382,
    data3: 20184,
    data4: [148, 223, 234, 248, 194, 106, 182, 27],
};
#[repr(transparent)]
pub struct IComponentAuthenticate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponentAuthenticate {}
impl ::core::clone::Clone for IComponentAuthenticate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPDevice {}
impl ::core::clone::Clone for IMDSPDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPDevice2 {}
impl ::core::clone::Clone for IMDSPDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPDevice3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPDevice3 {}
impl ::core::clone::Clone for IMDSPDevice3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPDeviceControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPDeviceControl {}
impl ::core::clone::Clone for IMDSPDeviceControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPDirectTransfer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPDirectTransfer {}
impl ::core::clone::Clone for IMDSPDirectTransfer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPEnumDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPEnumDevice {}
impl ::core::clone::Clone for IMDSPEnumDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPEnumStorage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPEnumStorage {}
impl ::core::clone::Clone for IMDSPEnumStorage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPObject {}
impl ::core::clone::Clone for IMDSPObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPObject2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPObject2 {}
impl ::core::clone::Clone for IMDSPObject2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPObjectInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPObjectInfo {}
impl ::core::clone::Clone for IMDSPObjectInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPRevoked(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPRevoked {}
impl ::core::clone::Clone for IMDSPRevoked {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPStorage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPStorage {}
impl ::core::clone::Clone for IMDSPStorage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPStorage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPStorage2 {}
impl ::core::clone::Clone for IMDSPStorage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPStorage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPStorage3 {}
impl ::core::clone::Clone for IMDSPStorage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPStorage4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPStorage4 {}
impl ::core::clone::Clone for IMDSPStorage4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDSPStorageGlobals(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDSPStorageGlobals {}
impl ::core::clone::Clone for IMDSPStorageGlobals {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDServiceProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDServiceProvider {}
impl ::core::clone::Clone for IMDServiceProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDServiceProvider2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDServiceProvider2 {}
impl ::core::clone::Clone for IMDServiceProvider2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMDServiceProvider3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMDServiceProvider3 {}
impl ::core::clone::Clone for IMDServiceProvider3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
#[repr(transparent)]
pub struct ISCPSecureAuthenticate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSecureAuthenticate {}
impl ::core::clone::Clone for ISCPSecureAuthenticate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCPSecureAuthenticate2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSecureAuthenticate2 {}
impl ::core::clone::Clone for ISCPSecureAuthenticate2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCPSecureExchange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSecureExchange {}
impl ::core::clone::Clone for ISCPSecureExchange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCPSecureExchange2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSecureExchange2 {}
impl ::core::clone::Clone for ISCPSecureExchange2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCPSecureExchange3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSecureExchange3 {}
impl ::core::clone::Clone for ISCPSecureExchange3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCPSecureQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSecureQuery {}
impl ::core::clone::Clone for ISCPSecureQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCPSecureQuery2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSecureQuery2 {}
impl ::core::clone::Clone for ISCPSecureQuery2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCPSecureQuery3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSecureQuery3 {}
impl ::core::clone::Clone for ISCPSecureQuery3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCPSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCPSession {}
impl ::core::clone::Clone for ISCPSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMDevice {}
impl ::core::clone::Clone for IWMDMDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMDevice2 {}
impl ::core::clone::Clone for IWMDMDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMDevice3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMDevice3 {}
impl ::core::clone::Clone for IWMDMDevice3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMDeviceControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMDeviceControl {}
impl ::core::clone::Clone for IWMDMDeviceControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMDeviceSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMDeviceSession {}
impl ::core::clone::Clone for IWMDMDeviceSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMEnumDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMEnumDevice {}
impl ::core::clone::Clone for IWMDMEnumDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMEnumStorage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMEnumStorage {}
impl ::core::clone::Clone for IWMDMEnumStorage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMLogger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMLogger {}
impl ::core::clone::Clone for IWMDMLogger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMMetaData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMMetaData {}
impl ::core::clone::Clone for IWMDMMetaData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMNotification(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMNotification {}
impl ::core::clone::Clone for IWMDMNotification {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMObjectInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMObjectInfo {}
impl ::core::clone::Clone for IWMDMObjectInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMOperation {}
impl ::core::clone::Clone for IWMDMOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMOperation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMOperation2 {}
impl ::core::clone::Clone for IWMDMOperation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMOperation3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMOperation3 {}
impl ::core::clone::Clone for IWMDMOperation3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMProgress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMProgress {}
impl ::core::clone::Clone for IWMDMProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMProgress2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMProgress2 {}
impl ::core::clone::Clone for IWMDMProgress2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMProgress3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMProgress3 {}
impl ::core::clone::Clone for IWMDMProgress3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMRevoked(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMRevoked {}
impl ::core::clone::Clone for IWMDMRevoked {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMStorage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMStorage {}
impl ::core::clone::Clone for IWMDMStorage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMStorage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMStorage2 {}
impl ::core::clone::Clone for IWMDMStorage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMStorage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMStorage3 {}
impl ::core::clone::Clone for IWMDMStorage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMStorage4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMStorage4 {}
impl ::core::clone::Clone for IWMDMStorage4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMStorageControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMStorageControl {}
impl ::core::clone::Clone for IWMDMStorageControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMStorageControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMStorageControl2 {}
impl ::core::clone::Clone for IWMDMStorageControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMStorageControl3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMStorageControl3 {}
impl ::core::clone::Clone for IWMDMStorageControl3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDMStorageGlobals(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDMStorageGlobals {}
impl ::core::clone::Clone for IWMDMStorageGlobals {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDeviceManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDeviceManager {}
impl ::core::clone::Clone for IWMDeviceManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDeviceManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDeviceManager2 {}
impl ::core::clone::Clone for IWMDeviceManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWMDeviceManager3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWMDeviceManager3 {}
impl ::core::clone::Clone for IWMDeviceManager3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MDSP_READ: u32 = 1u32;
pub const MDSP_SEEK_BOF: u32 = 1u32;
pub const MDSP_SEEK_CUR: u32 = 2u32;
pub const MDSP_SEEK_EOF: u32 = 4u32;
pub const MDSP_WRITE: u32 = 2u32;
#[repr(C, packed(1))]
pub struct MTP_COMMAND_DATA_IN {
    pub OpCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub NextPhase: u32,
    pub CommandWriteDataSize: u32,
    pub CommandWriteData: [u8; 1],
}
impl ::core::marker::Copy for MTP_COMMAND_DATA_IN {}
impl ::core::clone::Clone for MTP_COMMAND_DATA_IN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct MTP_COMMAND_DATA_OUT {
    pub ResponseCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub CommandReadDataSize: u32,
    pub CommandReadData: [u8; 1],
}
impl ::core::marker::Copy for MTP_COMMAND_DATA_OUT {}
impl ::core::clone::Clone for MTP_COMMAND_DATA_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
pub const MTP_RESPONSE_OK: u16 = 8193u16;
pub const MediaDevMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 632991105, data2: 13664, data3: 4563, data4: [132, 113, 0, 192, 79, 121, 219, 192] };
pub const MediaDevMgrClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1342442525,
    data2: 48575,
    data3: 18724,
    data4: [184, 115, 241, 77, 108, 91, 253, 102],
};
#[repr(C)]
pub struct OPAQUECOMMAND {
    pub guidCommand: ::windows_sys::core::GUID,
    pub dwDataLen: u32,
    pub pData: *mut u8,
    pub abMAC: [u8; 20],
}
impl ::core::marker::Copy for OPAQUECOMMAND {}
impl ::core::clone::Clone for OPAQUECOMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RSA_KEY_LEN: u32 = 64u32;
pub const SAC_CERT_V1: u32 = 2u32;
pub const SAC_CERT_X509: u32 = 1u32;
pub const SAC_MAC_LEN: u32 = 8u32;
pub const SAC_PROTOCOL_V1: u32 = 2u32;
pub const SAC_PROTOCOL_WMDM: u32 = 1u32;
pub const SAC_SESSION_KEYLEN: u32 = 8u32;
pub const SCP_EVENTID_ACQSECURECLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2250542281, data2: 19033, data3: 17378, data4: [145, 70, 72, 167, 243, 244, 20, 12] };
pub const SCP_EVENTID_DRMINFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 557699719, data2: 16850, data3: 17195, data4: [158, 63, 59, 79, 123, 53, 129, 221] };
pub const SCP_EVENTID_NEEDTOINDIV: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2275739591,
    data2: 46185,
    data3: 17286,
    data4: [185, 118, 213, 209, 206, 83, 138, 111],
};
pub const SCP_PARAMID_DRMVERSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1104155997, data2: 31943, data3: 16919, data4: [173, 169, 0, 80, 116, 98, 77, 164] };
#[repr(C)]
pub struct WMDMDATETIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
}
impl ::core::marker::Copy for WMDMDATETIME {}
impl ::core::clone::Clone for WMDMDATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WMDMDetermineMaxPropStringLen {
    pub sz001: [u16; 27],
    pub sz002: [u16; 31],
    pub sz003: [u16; 14],
    pub sz004: [u16; 16],
    pub sz005: [u16; 22],
    pub sz006: [u16; 14],
    pub sz007: [u16; 20],
    pub sz008: [u16; 20],
    pub sz009: [u16; 22],
    pub sz010: [u16; 11],
    pub sz011: [u16; 12],
    pub sz012: [u16; 17],
    pub sz013: [u16; 17],
    pub sz014: [u16; 16],
    pub sz015: [u16; 17],
    pub sz016: [u16; 11],
    pub sz017: [u16; 11],
    pub sz018: [u16; 15],
    pub sz019: [u16; 22],
    pub sz020: [u16; 20],
    pub sz021: [u16; 22],
    pub sz022: [u16; 21],
    pub sz023: [u16; 24],
    pub sz024: [u16; 20],
    pub sz025: [u16; 10],
    pub sz026: [u16; 14],
    pub sz027: [u16; 11],
    pub sz028: [u16; 11],
    pub sz029: [u16; 13],
    pub sz030: [u16; 17],
    pub sz031: [u16; 16],
    pub sz032: [u16; 17],
    pub sz033: [u16; 20],
    pub sz034: [u16; 19],
    pub sz035: [u16; 18],
    pub sz036: [u16; 18],
    pub sz037: [u16; 15],
    pub sz041: [u16; 14],
    pub sz043: [u16; 22],
    pub sz044: [u16; 16],
    pub sz045: [u16; 20],
    pub sz046: [u16; 14],
    pub sz047: [u16; 14],
    pub sz048: [u16; 12],
    pub sz049: [u16; 25],
    pub sz050: [u16; 26],
    pub sz051: [u16; 25],
    pub sz052: [u16; 16],
    pub sz053: [u16; 24],
    pub sz054: [u16; 15],
    pub sz055: [u16; 21],
    pub sz056: [u16; 16],
    pub sz057: [u16; 22],
    pub sz058: [u16; 14],
    pub sz059: [u16; 25],
    pub sz060: [u16; 18],
    pub sz061: [u16; 22],
    pub sz062: [u16; 26],
    pub sz063: [u16; 36],
    pub sz064: [u16; 23],
    pub sz065: [u16; 12],
    pub sz066: [u16; 24],
    pub sz067: [u16; 11],
    pub sz068: [u16; 12],
    pub sz069: [u16; 14],
    pub sz070: [u16; 20],
    pub sz071: [u16; 15],
    pub sz072: [u16; 14],
    pub sz073: [u16; 31],
    pub sz074: [u16; 24],
    pub sz075: [u16; 22],
    pub sz076: [u16; 24],
    pub sz077: [u16; 21],
    pub sz078: [u16; 27],
    pub sz079: [u16; 27],
    pub sz080: [u16; 20],
    pub sz081: [u16; 33],
    pub sz082: [u16; 21],
    pub sz083: [u16; 32],
    pub sz084: [u16; 26],
    pub sz085: [u16; 18],
    pub sz086: [u16; 30],
}
impl ::core::marker::Copy for WMDMDetermineMaxPropStringLen {}
impl ::core::clone::Clone for WMDMDetermineMaxPropStringLen {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WMDMDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2155560159, data2: 13690, data3: 4563, data4: [132, 113, 0, 192, 79, 121, 219, 192] };
pub const WMDMDeviceEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1125004719, data2: 14705, data3: 4563, data4: [132, 116, 0, 192, 79, 121, 219, 192] };
#[repr(C)]
pub struct WMDMID {
    pub cbSize: u32,
    pub dwVendorID: u32,
    pub pID: [u8; 128],
    pub SerialNumberLength: u32,
}
impl ::core::marker::Copy for WMDMID {}
impl ::core::clone::Clone for WMDMID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WMDMID_LENGTH: u32 = 128u32;
pub const WMDMLogger: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 285880834, data2: 23161, data3: 4563, data4: [141, 120, 68, 69, 83, 84, 0, 0] };
pub const WMDM_MSG_DEVICE_ARRIVAL: i32 = 0i32;
pub const WMDM_MSG_DEVICE_REMOVAL: i32 = 1i32;
pub const WMDM_MSG_MEDIA_ARRIVAL: i32 = 2i32;
pub const WMDM_MSG_MEDIA_REMOVAL: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WMDMMetadataView {
    pub pwszViewName: super::super::Foundation::PWSTR,
    pub nDepth: u32,
    pub ppwszTags: *mut *mut u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WMDMMetadataView {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WMDMMetadataView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WMDMRIGHTS {
    pub cbSize: u32,
    pub dwContentType: u32,
    pub fuFlags: u32,
    pub fuRights: u32,
    pub dwAppSec: u32,
    pub dwPlaybackCount: u32,
    pub ExpirationDate: WMDMDATETIME,
}
impl ::core::marker::Copy for WMDMRIGHTS {}
impl ::core::clone::Clone for WMDMRIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WMDMStorage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2155560160, data2: 13690, data3: 4563, data4: [132, 113, 0, 192, 79, 121, 219, 192] };
pub const WMDMStorageEnum: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3946846779, data2: 15095, data3: 4563, data4: [132, 116, 0, 192, 79, 121, 219, 192] };
pub const WMDMStorageGlobal: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2155560161, data2: 13690, data3: 4563, data4: [132, 113, 0, 192, 79, 121, 219, 192] };
pub const WMDM_APP_REVOKED: u32 = 2u32;
pub const WMDM_CONTENT_FILE: u32 = 4u32;
pub const WMDM_CONTENT_FOLDER: u32 = 8u32;
pub const WMDM_CONTENT_OPERATIONINTERFACE: u32 = 16u32;
pub const WMDM_DEVICECAP_CANPAUSE: u32 = 16u32;
pub const WMDM_DEVICECAP_CANPLAY: u32 = 1u32;
pub const WMDM_DEVICECAP_CANRECORD: u32 = 4u32;
pub const WMDM_DEVICECAP_CANRESUME: u32 = 32u32;
pub const WMDM_DEVICECAP_CANSEEK: u32 = 128u32;
pub const WMDM_DEVICECAP_CANSTOP: u32 = 64u32;
pub const WMDM_DEVICECAP_CANSTREAMPLAY: u32 = 2u32;
pub const WMDM_DEVICECAP_CANSTREAMRECORD: u32 = 8u32;
pub const WMDM_DEVICECAP_HASSECURECLOCK: u32 = 256u32;
pub const WMDM_DEVICE_PROTOCOL_MSC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2765275756,
    data2: 43137,
    data3: 17595,
    data4: [189, 93, 31, 112, 60, 113, 247, 169],
};
pub const WMDM_DEVICE_PROTOCOL_MTP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2543736037,
    data2: 2812,
    data3: 17924,
    data4: [141, 147, 220, 121, 138, 75, 207, 69],
};
pub const WMDM_DEVICE_PROTOCOL_RAPI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 705818001, data2: 35983, data3: 16868, data4: [130, 209, 131, 134, 224, 3, 86, 28] };
pub const WMDM_DEVICE_TYPE_DECODE: u32 = 4u32;
pub const WMDM_DEVICE_TYPE_ENCODE: u32 = 8u32;
pub const WMDM_DEVICE_TYPE_FILELISTRESYNC: u32 = 512u32;
pub const WMDM_DEVICE_TYPE_NONREENTRANT: u32 = 256u32;
pub const WMDM_DEVICE_TYPE_NONSDMI: u32 = 128u32;
pub const WMDM_DEVICE_TYPE_PLAYBACK: u32 = 1u32;
pub const WMDM_DEVICE_TYPE_RECORD: u32 = 2u32;
pub const WMDM_DEVICE_TYPE_SDMI: u32 = 64u32;
pub const WMDM_DEVICE_TYPE_STORAGE: u32 = 16u32;
pub const WMDM_DEVICE_TYPE_VIEW_PREF_METADATAVIEW: u32 = 1024u32;
pub const WMDM_DEVICE_TYPE_VIRTUAL: u32 = 32u32;
pub const WMDM_ENUM_PROP_VALID_VALUES_ANY: i32 = 0i32;
pub const WMDM_ENUM_PROP_VALID_VALUES_RANGE: i32 = 1i32;
pub const WMDM_ENUM_PROP_VALID_VALUES_ENUM: i32 = 2i32;
pub const WMDM_E_BUFFERTOOSMALL: i32 = -2147201016i32;
pub const WMDM_E_BUSY: i32 = -2147201024i32;
pub const WMDM_E_CALL_OUT_OF_SEQUENCE: i32 = -2147201017i32;
pub const WMDM_E_CANTOPEN_PMSN_SERVICE_PIPE: i32 = -2147201005i32;
pub const WMDM_E_INCORRECT_APPSEC: i32 = -2147201008i32;
pub const WMDM_E_INCORRECT_RIGHTS: i32 = -2147201007i32;
pub const WMDM_E_INTERFACEDEAD: i32 = -2147201023i32;
pub const WMDM_E_INVALIDTYPE: i32 = -2147201022i32;
pub const WMDM_E_LICENSE_EXPIRED: i32 = -2147201006i32;
pub const WMDM_E_LICENSE_NOTEXIST: i32 = -2147201009i32;
pub const WMDM_E_MAC_CHECK_FAILED: i32 = -2147201014i32;
pub const WMDM_E_MOREDATA: i32 = -2147201015i32;
pub const WMDM_E_NORIGHTS: i32 = -2147201018i32;
pub const WMDM_E_NOTCERTIFIED: i32 = -2147201019i32;
pub const WMDM_E_NOTSUPPORTED: i32 = -2147201020i32;
pub const WMDM_E_PROCESSFAILED: i32 = -2147201021i32;
pub const WMDM_E_REVOKED: i32 = -2147201010i32;
pub const WMDM_E_SDMI_NOMORECOPIES: i32 = -2147201011i32;
pub const WMDM_E_SDMI_TRIGGER: i32 = -2147201012i32;
pub const WMDM_E_TOO_MANY_SESSIONS: i32 = -2147201005i32;
pub const WMDM_E_USER_CANCELLED: i32 = -2147201013i32;
pub const WMDM_FILE_ATTR_AUDIO: u32 = 4096u32;
pub const WMDM_FILE_ATTR_AUDIOBOOK: u32 = 2097152u32;
pub const WMDM_FILE_ATTR_CANDELETE: u32 = 32768u32;
pub const WMDM_FILE_ATTR_CANMOVE: u32 = 65536u32;
pub const WMDM_FILE_ATTR_CANPLAY: u32 = 16384u32;
pub const WMDM_FILE_ATTR_CANREAD: u32 = 262144u32;
pub const WMDM_FILE_ATTR_CANRENAME: u32 = 131072u32;
pub const WMDM_FILE_ATTR_DATA: u32 = 8192u32;
pub const WMDM_FILE_ATTR_FILE: u32 = 32u32;
pub const WMDM_FILE_ATTR_FOLDER: u32 = 8u32;
pub const WMDM_FILE_ATTR_HIDDEN: u32 = 4194304u32;
pub const WMDM_FILE_ATTR_LINK: u32 = 16u32;
pub const WMDM_FILE_ATTR_MUSIC: u32 = 524288u32;
pub const WMDM_FILE_ATTR_READONLY: u32 = 16777216u32;
pub const WMDM_FILE_ATTR_SYSTEM: u32 = 8388608u32;
pub const WMDM_FILE_ATTR_VIDEO: u32 = 64u32;
pub const WMDM_FILE_CREATE_OVERWRITE: u32 = 1048576u32;
pub const WMDM_FIND_SCOPE_GLOBAL: i32 = 0i32;
pub const WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN: i32 = 1i32;
pub const WMDM_FORMATCODE_NOTUSED: i32 = 0i32;
pub const WMDM_FORMATCODE_ALLIMAGES: i32 = -1i32;
pub const WMDM_FORMATCODE_UNDEFINED: i32 = 12288i32;
pub const WMDM_FORMATCODE_ASSOCIATION: i32 = 12289i32;
pub const WMDM_FORMATCODE_SCRIPT: i32 = 12290i32;
pub const WMDM_FORMATCODE_EXECUTABLE: i32 = 12291i32;
pub const WMDM_FORMATCODE_TEXT: i32 = 12292i32;
pub const WMDM_FORMATCODE_HTML: i32 = 12293i32;
pub const WMDM_FORMATCODE_DPOF: i32 = 12294i32;
pub const WMDM_FORMATCODE_AIFF: i32 = 12295i32;
pub const WMDM_FORMATCODE_WAVE: i32 = 12296i32;
pub const WMDM_FORMATCODE_MP3: i32 = 12297i32;
pub const WMDM_FORMATCODE_AVI: i32 = 12298i32;
pub const WMDM_FORMATCODE_MPEG: i32 = 12299i32;
pub const WMDM_FORMATCODE_ASF: i32 = 12300i32;
pub const WMDM_FORMATCODE_RESERVED_FIRST: i32 = 12301i32;
pub const WMDM_FORMATCODE_RESERVED_LAST: i32 = 14335i32;
pub const WMDM_FORMATCODE_IMAGE_UNDEFINED: i32 = 14336i32;
pub const WMDM_FORMATCODE_IMAGE_EXIF: i32 = 14337i32;
pub const WMDM_FORMATCODE_IMAGE_TIFFEP: i32 = 14338i32;
pub const WMDM_FORMATCODE_IMAGE_FLASHPIX: i32 = 14339i32;
pub const WMDM_FORMATCODE_IMAGE_BMP: i32 = 14340i32;
pub const WMDM_FORMATCODE_IMAGE_CIFF: i32 = 14341i32;
pub const WMDM_FORMATCODE_IMAGE_GIF: i32 = 14343i32;
pub const WMDM_FORMATCODE_IMAGE_JFIF: i32 = 14344i32;
pub const WMDM_FORMATCODE_IMAGE_PCD: i32 = 14345i32;
pub const WMDM_FORMATCODE_IMAGE_PICT: i32 = 14346i32;
pub const WMDM_FORMATCODE_IMAGE_PNG: i32 = 14347i32;
pub const WMDM_FORMATCODE_IMAGE_TIFF: i32 = 14349i32;
pub const WMDM_FORMATCODE_IMAGE_TIFFIT: i32 = 14350i32;
pub const WMDM_FORMATCODE_IMAGE_JP2: i32 = 14351i32;
pub const WMDM_FORMATCODE_IMAGE_JPX: i32 = 14352i32;
pub const WMDM_FORMATCODE_IMAGE_RESERVED_FIRST: i32 = 14353i32;
pub const WMDM_FORMATCODE_IMAGE_RESERVED_LAST: i32 = 16383i32;
pub const WMDM_FORMATCODE_UNDEFINEDFIRMWARE: i32 = 47106i32;
pub const WMDM_FORMATCODE_WBMP: i32 = 47107i32;
pub const WMDM_FORMATCODE_JPEGXR: i32 = 47108i32;
pub const WMDM_FORMATCODE_WINDOWSIMAGEFORMAT: i32 = 47233i32;
pub const WMDM_FORMATCODE_UNDEFINEDAUDIO: i32 = 47360i32;
pub const WMDM_FORMATCODE_WMA: i32 = 47361i32;
pub const WMDM_FORMATCODE_OGG: i32 = 47362i32;
pub const WMDM_FORMATCODE_AAC: i32 = 47363i32;
pub const WMDM_FORMATCODE_AUDIBLE: i32 = 47364i32;
pub const WMDM_FORMATCODE_FLAC: i32 = 47366i32;
pub const WMDM_FORMATCODE_QCELP: i32 = 47367i32;
pub const WMDM_FORMATCODE_AMR: i32 = 47368i32;
pub const WMDM_FORMATCODE_UNDEFINEDVIDEO: i32 = 47488i32;
pub const WMDM_FORMATCODE_WMV: i32 = 47489i32;
pub const WMDM_FORMATCODE_MP4: i32 = 47490i32;
pub const WMDM_FORMATCODE_MP2: i32 = 47491i32;
pub const WMDM_FORMATCODE_3GP: i32 = 47492i32;
pub const WMDM_FORMATCODE_3G2: i32 = 47493i32;
pub const WMDM_FORMATCODE_AVCHD: i32 = 47494i32;
pub const WMDM_FORMATCODE_ATSCTS: i32 = 47495i32;
pub const WMDM_FORMATCODE_DVBTS: i32 = 47496i32;
pub const WMDM_FORMATCODE_MKV: i32 = 47497i32;
pub const WMDM_FORMATCODE_MKA: i32 = 47498i32;
pub const WMDM_FORMATCODE_MK3D: i32 = 47499i32;
pub const WMDM_FORMATCODE_UNDEFINEDCOLLECTION: i32 = 47616i32;
pub const WMDM_FORMATCODE_ABSTRACTMULTIMEDIAALBUM: i32 = 47617i32;
pub const WMDM_FORMATCODE_ABSTRACTIMAGEALBUM: i32 = 47618i32;
pub const WMDM_FORMATCODE_ABSTRACTAUDIOALBUM: i32 = 47619i32;
pub const WMDM_FORMATCODE_ABSTRACTVIDEOALBUM: i32 = 47620i32;
pub const WMDM_FORMATCODE_ABSTRACTAUDIOVIDEOPLAYLIST: i32 = 47621i32;
pub const WMDM_FORMATCODE_ABSTRACTCONTACTGROUP: i32 = 47622i32;
pub const WMDM_FORMATCODE_ABSTRACTMESSAGEFOLDER: i32 = 47623i32;
pub const WMDM_FORMATCODE_ABSTRACTCHAPTEREDPRODUCTION: i32 = 47624i32;
pub const WMDM_FORMATCODE_MEDIA_CAST: i32 = 47627i32;
pub const WMDM_FORMATCODE_WPLPLAYLIST: i32 = 47632i32;
pub const WMDM_FORMATCODE_M3UPLAYLIST: i32 = 47633i32;
pub const WMDM_FORMATCODE_MPLPLAYLIST: i32 = 47634i32;
pub const WMDM_FORMATCODE_ASXPLAYLIST: i32 = 47635i32;
pub const WMDM_FORMATCODE_PLSPLAYLIST: i32 = 47636i32;
pub const WMDM_FORMATCODE_UNDEFINEDDOCUMENT: i32 = 47744i32;
pub const WMDM_FORMATCODE_ABSTRACTDOCUMENT: i32 = 47745i32;
pub const WMDM_FORMATCODE_XMLDOCUMENT: i32 = 47746i32;
pub const WMDM_FORMATCODE_MICROSOFTWORDDOCUMENT: i32 = 47747i32;
pub const WMDM_FORMATCODE_MHTCOMPILEDHTMLDOCUMENT: i32 = 47748i32;
pub const WMDM_FORMATCODE_MICROSOFTEXCELSPREADSHEET: i32 = 47749i32;
pub const WMDM_FORMATCODE_MICROSOFTPOWERPOINTDOCUMENT: i32 = 47750i32;
pub const WMDM_FORMATCODE_UNDEFINEDMESSAGE: i32 = 47872i32;
pub const WMDM_FORMATCODE_ABSTRACTMESSAGE: i32 = 47873i32;
pub const WMDM_FORMATCODE_UNDEFINEDCONTACT: i32 = 48000i32;
pub const WMDM_FORMATCODE_ABSTRACTCONTACT: i32 = 48001i32;
pub const WMDM_FORMATCODE_VCARD2: i32 = 48002i32;
pub const WMDM_FORMATCODE_VCARD3: i32 = 48003i32;
pub const WMDM_FORMATCODE_UNDEFINEDCALENDARITEM: i32 = 48640i32;
pub const WMDM_FORMATCODE_ABSTRACTCALENDARITEM: i32 = 48641i32;
pub const WMDM_FORMATCODE_VCALENDAR1: i32 = 48642i32;
pub const WMDM_FORMATCODE_VCALENDAR2: i32 = 48643i32;
pub const WMDM_FORMATCODE_UNDEFINEDWINDOWSEXECUTABLE: i32 = 48768i32;
pub const WMDM_FORMATCODE_M4A: i32 = 1297101889i32;
pub const WMDM_FORMATCODE_3GPA: i32 = 860311617i32;
pub const WMDM_FORMATCODE_3G2A: i32 = 860303937i32;
pub const WMDM_FORMATCODE_SECTION: i32 = 48770i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_FORMAT_CAPABILITY {
    pub nPropConfig: u32,
    pub pConfigs: *mut WMDM_PROP_CONFIG,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_FORMAT_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WMDM_GET_FORMAT_SUPPORT_AUDIO: u32 = 1u32;
pub const WMDM_GET_FORMAT_SUPPORT_FILE: u32 = 4u32;
pub const WMDM_GET_FORMAT_SUPPORT_VIDEO: u32 = 2u32;
pub const WMDM_LOG_NOTIMESTAMP: u32 = 16u32;
pub const WMDM_LOG_SEV_ERROR: u32 = 4u32;
pub const WMDM_LOG_SEV_INFO: u32 = 1u32;
pub const WMDM_LOG_SEV_WARN: u32 = 2u32;
pub const WMDM_MAC_LENGTH: u32 = 8u32;
pub const WMDM_MODE_BLOCK: u32 = 1u32;
pub const WMDM_MODE_PROGRESS: u32 = 64u32;
pub const WMDM_MODE_QUERY: u32 = 32u32;
pub const WMDM_MODE_RECURSIVE: u32 = 4096u32;
pub const WMDM_MODE_THREAD: u32 = 2u32;
pub const WMDM_MODE_TRANSFER_PROTECTED: u32 = 128u32;
pub const WMDM_MODE_TRANSFER_UNPROTECTED: u32 = 256u32;
pub const WMDM_POWER_CAP_BATTERY: u32 = 1u32;
pub const WMDM_POWER_CAP_EXTERNAL: u32 = 2u32;
pub const WMDM_POWER_IS_BATTERY: u32 = 4u32;
pub const WMDM_POWER_IS_EXTERNAL: u32 = 8u32;
pub const WMDM_POWER_PERCENT_AVAILABLE: u32 = 16u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_CONFIG {
    pub nPreference: u32,
    pub nPropDesc: u32,
    pub pPropDesc: *mut WMDM_PROP_DESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_DESC {
    pub pwszPropName: super::super::Foundation::PWSTR,
    pub ValidValuesForm: WMDM_ENUM_PROP_VALID_VALUES_FORM,
    pub ValidValues: WMDM_PROP_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub union WMDM_PROP_DESC_0 {
    pub ValidValuesRange: WMDM_PROP_VALUES_RANGE,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_ENUM {
    pub cEnumValues: u32,
    pub pValues: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_RANGE {
    pub rangeMin: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeMax: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeStep: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_VALUES_RANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WMDM_RIGHTS_COPY_TO_CD: u32 = 8u32;
pub const WMDM_RIGHTS_COPY_TO_NON_SDMI_DEVICE: u32 = 2u32;
pub const WMDM_RIGHTS_COPY_TO_SDMI_DEVICE: u32 = 16u32;
pub const WMDM_RIGHTS_EXPIRATIONDATE: u32 = 2u32;
pub const WMDM_RIGHTS_FREESERIALIDS: u32 = 8u32;
pub const WMDM_RIGHTS_GROUPID: u32 = 4u32;
pub const WMDM_RIGHTS_NAMEDSERIALIDS: u32 = 16u32;
pub const WMDM_RIGHTS_PLAYBACKCOUNT: u32 = 1u32;
pub const WMDM_RIGHTS_PLAY_ON_PC: u32 = 1u32;
pub const WMDM_SCP_DECIDE_DATA: i32 = 8i32;
pub const WMDM_SCP_DRMINFO_NOT_DRMPROTECTED: i32 = 0i32;
pub const WMDM_SCP_DRMINFO_V1HEADER: i32 = 1i32;
pub const WMDM_SCP_DRMINFO_V2HEADER: i32 = 2i32;
pub const WMDM_SCP_EXAMINE_DATA: i32 = 2i32;
pub const WMDM_SCP_EXAMINE_EXTENSION: i32 = 1i32;
pub const WMDM_SCP_NO_MORE_CHANGES: i32 = 64i32;
pub const WMDM_SCP_PROTECTED_OUTPUT: i32 = 16i32;
pub const WMDM_SCP_REVOKED: u32 = 8u32;
pub const WMDM_SCP_RIGHTS_DATA: i32 = 64i32;
pub const WMDM_SCP_TRANSFER_OBJECTDATA: i32 = 32i32;
pub const WMDM_SCP_UNPROTECTED_OUTPUT: i32 = 32i32;
pub const WMDM_SEEK_BEGIN: u32 = 1u32;
pub const WMDM_SEEK_CURRENT: u32 = 2u32;
pub const WMDM_SEEK_END: u32 = 8u32;
pub const WMDM_SEEK_REMOTECONTROL: u32 = 1u32;
pub const WMDM_SEEK_STREAMINGAUDIO: u32 = 2u32;
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2112383085,
    data2: 30958,
    data3: 17386,
    data4: [164, 150, 198, 37, 172, 145, 204, 93],
};
pub const WMDM_SESSION_NONE: i32 = 0i32;
pub const WMDM_SESSION_TRANSFER_TO_DEVICE: i32 = 1i32;
pub const WMDM_SESSION_TRANSFER_FROM_DEVICE: i32 = 16i32;
pub const WMDM_SESSION_DELETE: i32 = 256i32;
pub const WMDM_SESSION_CUSTOM: i32 = 4096i32;
pub const WMDM_SP_REVOKED: u32 = 4u32;
pub const WMDM_STATUS_BUSY: u32 = 2u32;
pub const WMDM_STATUS_DEVICECONTROL_PAUSED: u32 = 32u32;
pub const WMDM_STATUS_DEVICECONTROL_PLAYING: u32 = 8u32;
pub const WMDM_STATUS_DEVICECONTROL_RECORDING: u32 = 16u32;
pub const WMDM_STATUS_DEVICECONTROL_REMOTE: u32 = 64u32;
pub const WMDM_STATUS_DEVICECONTROL_STREAM: u32 = 128u32;
pub const WMDM_STATUS_DEVICE_NOTPRESENT: u32 = 4u32;
pub const WMDM_STATUS_READY: u32 = 1u32;
pub const WMDM_STATUS_STORAGECONTROL_APPENDING: u32 = 32768u32;
pub const WMDM_STATUS_STORAGECONTROL_DELETING: u32 = 16384u32;
pub const WMDM_STATUS_STORAGECONTROL_INSERTING: u32 = 8192u32;
pub const WMDM_STATUS_STORAGECONTROL_MOVING: u32 = 65536u32;
pub const WMDM_STATUS_STORAGECONTROL_READING: u32 = 131072u32;
pub const WMDM_STATUS_STORAGE_BROKEN: u32 = 1024u32;
pub const WMDM_STATUS_STORAGE_INITIALIZING: u32 = 512u32;
pub const WMDM_STATUS_STORAGE_NOTPRESENT: u32 = 256u32;
pub const WMDM_STATUS_STORAGE_NOTSUPPORTED: u32 = 2048u32;
pub const WMDM_STATUS_STORAGE_UNFORMATTED: u32 = 4096u32;
pub const WMDM_STORAGECAP_FILELIMITEXISTS: u32 = 32u32;
pub const WMDM_STORAGECAP_FILESINFOLDERS: u32 = 8u32;
pub const WMDM_STORAGECAP_FILESINROOT: u32 = 2u32;
pub const WMDM_STORAGECAP_FOLDERLIMITEXISTS: u32 = 16u32;
pub const WMDM_STORAGECAP_FOLDERSINFOLDERS: u32 = 4u32;
pub const WMDM_STORAGECAP_FOLDERSINROOT: u32 = 1u32;
pub const WMDM_STORAGECAP_NOT_INITIALIZABLE: u32 = 64u32;
pub const WMDM_STORAGECONTROL_INSERTAFTER: u32 = 1024u32;
pub const WMDM_STORAGECONTROL_INSERTBEFORE: u32 = 512u32;
pub const WMDM_STORAGECONTROL_INSERTINTO: u32 = 2048u32;
pub const WMDM_STORAGE_ATTR_CANEDITMETADATA: u32 = 128u32;
pub const WMDM_STORAGE_ATTR_FILESYSTEM: u32 = 1u32;
pub const WMDM_STORAGE_ATTR_FOLDERS: u32 = 256u32;
pub const WMDM_STORAGE_ATTR_HAS_FILES: u32 = 67108864u32;
pub const WMDM_STORAGE_ATTR_HAS_FOLDERS: u32 = 33554432u32;
pub const WMDM_STORAGE_ATTR_NONREMOVABLE: u32 = 4u32;
pub const WMDM_STORAGE_ATTR_REMOVABLE: u32 = 2u32;
pub const WMDM_STORAGE_ATTR_VIRTUAL: u32 = 536870912u32;
pub const WMDM_STORAGE_CONTAINS_DEFAULT: u32 = 268435456u32;
pub const ENUM_MODE_RAW: i32 = 0i32;
pub const ENUM_MODE_USE_DEVICE_PREF: i32 = 1i32;
pub const ENUM_MODE_METADATA_VIEWS: i32 = 2i32;
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
pub const WMDM_TYPE_DWORD: i32 = 0i32;
pub const WMDM_TYPE_STRING: i32 = 1i32;
pub const WMDM_TYPE_BINARY: i32 = 2i32;
pub const WMDM_TYPE_BOOL: i32 = 3i32;
pub const WMDM_TYPE_QWORD: i32 = 4i32;
pub const WMDM_TYPE_WORD: i32 = 5i32;
pub const WMDM_TYPE_GUID: i32 = 6i32;
pub const WMDM_TYPE_DATE: i32 = 7i32;
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WMFILECAPABILITIES {
    pub pwszMimeType: super::super::Foundation::PWSTR,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WMFILECAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WMFILECAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
impl ::core::marker::Copy for _BITMAPINFOHEADER {}
impl ::core::clone::Clone for _BITMAPINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct _VIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: _BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _VIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for _WAVEFORMATEX {}
impl ::core::clone::Clone for _WAVEFORMATEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct __MACINFO {
    pub fUsed: super::super::Foundation::BOOL,
    pub abMacState: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for __MACINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for __MACINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const g_wszAudioWAVECodec: &'static str = "WMDM/AudioWAVECodec";
pub const g_wszVideoFourCCCodec: &'static str = "WMDM/VideoFourCCCodec";
pub const g_wszWMDMAlbumArt: &'static str = "WMDM/AlbumArt";
pub const g_wszWMDMAlbumArtist: &'static str = "WMDM/AlbumArtist";
pub const g_wszWMDMAlbumCoverData: &'static str = "WMDM/AlbumCoverData";
pub const g_wszWMDMAlbumCoverDuration: &'static str = "WMDM/AlbumCoverDuration";
pub const g_wszWMDMAlbumCoverFormat: &'static str = "WMDM/AlbumCoverFormat";
pub const g_wszWMDMAlbumCoverHeight: &'static str = "WMDM/AlbumCoverHeight";
pub const g_wszWMDMAlbumCoverSize: &'static str = "WMDM/AlbumCoverSize";
pub const g_wszWMDMAlbumCoverWidth: &'static str = "WMDM/AlbumCoverWidth";
pub const g_wszWMDMAlbumTitle: &'static str = "WMDM/AlbumTitle";
pub const g_wszWMDMAudioBitDepth: &'static str = "WMDM/AudioBitDepth";
pub const g_wszWMDMAuthor: &'static str = "WMDM/Author";
pub const g_wszWMDMAuthorDate: &'static str = "WMDM/AuthorDate";
pub const g_wszWMDMBitRateType: &'static str = "WMDM/BitRateType";
pub const g_wszWMDMBitrate: &'static str = "WMDM/Bitrate";
pub const g_wszWMDMBlockAlignment: &'static str = "WMDM/BlockAlignment";
pub const g_wszWMDMBufferSize: &'static str = "WMDM/BufferSize";
pub const g_wszWMDMBuyNow: &'static str = "WMDM/BuyNow";
pub const g_wszWMDMByteBookmark: &'static str = "WMDM/ByteBookmark";
pub const g_wszWMDMCategory: &'static str = "WMDM/Category";
pub const g_wszWMDMCodec: &'static str = "WMDM/Codec";
pub const g_wszWMDMCollectionID: &'static str = "WMDM/CollectionID";
pub const g_wszWMDMComposer: &'static str = "WMDM/Composer";
pub const g_wszWMDMDRMId: &'static str = "WMDM/DRMId";
pub const g_wszWMDMDataLength: &'static str = "WMDM/DataLength";
pub const g_wszWMDMDataOffset: &'static str = "WMDM/DataOffset";
pub const g_wszWMDMDataUnits: &'static str = "WMDM/DataUnits";
pub const g_wszWMDMDescription: &'static str = "WMDM/Description";
pub const g_wszWMDMDestinationURL: &'static str = "WMDM/DestinationURL";
pub const g_wszWMDMDeviceFirmwareVersion: &'static str = "WMDM/DeviceFirmwareVersion";
pub const g_wszWMDMDeviceFriendlyName: &'static str = "WMDM/DeviceFriendlyName";
pub const g_wszWMDMDeviceModelName: &'static str = "WMDM/DeviceModelName";
pub const g_wszWMDMDevicePlayCount: &'static str = "WMDM/DevicePlayCount";
pub const g_wszWMDMDeviceProtocol: &'static str = "WMDM/DeviceProtocol";
pub const g_wszWMDMDeviceRevocationInfo: &'static str = "WMDM/DeviceRevocationInfo";
pub const g_wszWMDMDeviceServiceProviderVendor: &'static str = "WMDM/DeviceServiceProviderVendor";
pub const g_wszWMDMDeviceVendorExtension: &'static str = "WMDM/DeviceVendorExtension";
pub const g_wszWMDMDuration: &'static str = "WMDM/Duration";
pub const g_wszWMDMEditor: &'static str = "WMDM/Editor";
pub const g_wszWMDMEncodingProfile: &'static str = "WMDM/EncodingProfile";
pub const g_wszWMDMFileAttributes: &'static str = "WMDM/FileAttributes";
pub const g_wszWMDMFileCreationDate: &'static str = "WMDM/FileCreationDate";
pub const g_wszWMDMFileName: &'static str = "WMDM/FileName";
pub const g_wszWMDMFileSize: &'static str = "WMDM/FileSize";
pub const g_wszWMDMFormatCode: &'static str = "WMDM/FormatCode";
pub const g_wszWMDMFormatsSupported: &'static str = "WMDM/FormatsSupported";
pub const g_wszWMDMFormatsSupportedAreOrdered: &'static str = "WMDM/FormatsSupportedAreOrdered";
pub const g_wszWMDMFrameRate: &'static str = "WMDM/FrameRate";
pub const g_wszWMDMGenre: &'static str = "WMDM/Genre";
pub const g_wszWMDMHeight: &'static str = "WMDM/Height";
pub const g_wszWMDMIsProtected: &'static str = "WMDM/IsProtected";
pub const g_wszWMDMIsRepeat: &'static str = "WMDM/IsRepeat";
pub const g_wszWMDMKeyFrameDistance: &'static str = "WMDM/KeyFrameDistance";
pub const g_wszWMDMLastModifiedDate: &'static str = "WMDM/LastModifiedDate";
pub const g_wszWMDMMediaClassSecondaryID: &'static str = "WMDM/MediaClassSecondaryID";
pub const g_wszWMDMMediaCredits: &'static str = "WMDM/MediaCredits";
pub const g_wszWMDMMediaGuid: &'static str = "WMDM/MediaGuid";
pub const g_wszWMDMMediaOriginalBroadcastDateTime: &'static str = "WMDM/MediaOriginalBroadcastDateTime";
pub const g_wszWMDMMediaOriginalChannel: &'static str = "WMDM/MediaOriginalChannel";
pub const g_wszWMDMMediaStationName: &'static str = "WMDM/MediaStationName";
pub const g_wszWMDMMetaGenre: &'static str = "WMDM/MetaGenre";
pub const g_wszWMDMNonConsumable: &'static str = "WMDM/NonConsumable";
pub const g_wszWMDMNumChannels: &'static str = "WMDM/NumChannels";
pub const g_wszWMDMObjectBookmark: &'static str = "WMDM/ObjectBookmark";
pub const g_wszWMDMOwner: &'static str = "WMDM/Owner";
pub const g_wszWMDMParentalRating: &'static str = "WMDM/ParentalRating";
pub const g_wszWMDMPersistentUniqueID: &'static str = "WMDM/PersistentUniqueID";
pub const g_wszWMDMPlayCount: &'static str = "WMDM/PlayCount";
pub const g_wszWMDMProviderCopyright: &'static str = "WMDM/ProviderCopyright";
pub const g_wszWMDMQualitySetting: &'static str = "WMDM/QualitySetting";
pub const g_wszWMDMSampleRate: &'static str = "WMDM/SampleRate";
pub const g_wszWMDMScanType: &'static str = "WMDM/ScanType";
pub const g_wszWMDMSourceURL: &'static str = "WMDM/SourceURL";
pub const g_wszWMDMSubTitle: &'static str = "WMDM/SubTitle";
pub const g_wszWMDMSubTitleDescription: &'static str = "WMDM/SubTitleDescription";
pub const g_wszWMDMSupportedDeviceProperties: &'static str = "WMDM/SupportedDeviceProperties";
pub const g_wszWMDMSyncID: &'static str = "WMDM/SyncID";
pub const g_wszWMDMSyncRelationshipID: &'static str = "WMDM/SyncRelationshipID";
pub const g_wszWMDMSyncTime: &'static str = "WMDM/SyncTime";
pub const g_wszWMDMTimeBookmark: &'static str = "WMDM/TimeBookmark";
pub const g_wszWMDMTimeToLive: &'static str = "WMDM/TimeToLive";
pub const g_wszWMDMTitle: &'static str = "WMDM/Title";
pub const g_wszWMDMTotalBitrate: &'static str = "WMDM/TotalBitrate";
pub const g_wszWMDMTrack: &'static str = "WMDM/Track";
pub const g_wszWMDMTrackMood: &'static str = "WMDM/TrackMood";
pub const g_wszWMDMUserEffectiveRating: &'static str = "WMDM/UserEffectiveRating";
pub const g_wszWMDMUserLastPlayTime: &'static str = "WMDM/UserLastPlayTime";
pub const g_wszWMDMUserRating: &'static str = "WMDM/UserRating";
pub const g_wszWMDMUserRatingOnDevice: &'static str = "WMDM/UserRatingOnDevice";
pub const g_wszWMDMVideoBitrate: &'static str = "WMDM/VideoBitrate";
pub const g_wszWMDMWebmaster: &'static str = "WMDM/Webmaster";
pub const g_wszWMDMWidth: &'static str = "WMDM/Width";
pub const g_wszWMDMYear: &'static str = "WMDM/Year";
pub const g_wszWMDMediaClassPrimaryID: &'static str = "WMDM/MediaClassPrimaryID";
pub const g_wszWPDPassthroughPropertyValues: &'static str = "WPD/PassthroughPropertyValues";
