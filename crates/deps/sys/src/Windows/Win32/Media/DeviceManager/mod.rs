#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
pub const EVENT_WMDM_CONTENT_TRANSFER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 865901556,
    data2: 48382,
    data3: 20184,
    data4: [148, 223, 234, 248, 194, 106, 182, 27],
};
#[repr(transparent)]
pub struct IComponentAuthenticate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPDevice3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPDeviceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPDirectTransfer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPEnumDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPEnumStorage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPObject2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPObjectInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPRevoked(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPStorage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPStorage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPStorage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPStorage4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDSPStorageGlobals(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDServiceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDServiceProvider2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMDServiceProvider3(pub *mut ::core::ffi::c_void);
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
#[repr(transparent)]
pub struct ISCPSecureAuthenticate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCPSecureAuthenticate2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCPSecureExchange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCPSecureExchange2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCPSecureExchange3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCPSecureQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCPSecureQuery2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCPSecureQuery3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCPSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMDevice3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMDeviceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMDeviceSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMEnumDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMEnumStorage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMLogger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMMetaData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMObjectInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMOperation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMOperation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMProgress2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMProgress3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMRevoked(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMStorage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMStorage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMStorage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMStorage4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMStorageControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMStorageControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMStorageControl3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDMStorageGlobals(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDeviceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDeviceManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWMDeviceManager3(pub *mut ::core::ffi::c_void);
pub const MDSP_READ: u32 = 1u32;
pub const MDSP_SEEK_BOF: u32 = 1u32;
pub const MDSP_SEEK_CUR: u32 = 2u32;
pub const MDSP_SEEK_EOF: u32 = 4u32;
pub const MDSP_WRITE: u32 = 2u32;
pub struct MTP_COMMAND_DATA_IN(i32);
pub struct MTP_COMMAND_DATA_OUT(i32);
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
pub const MTP_RESPONSE_OK: u16 = 8193u16;
pub struct MediaDevMgr(i32);
pub struct MediaDevMgrClassFactory(i32);
pub struct OPAQUECOMMAND(i32);
pub const RSA_KEY_LEN: u32 = 64u32;
pub const SAC_CERT_V1: u32 = 2u32;
pub const SAC_CERT_X509: u32 = 1u32;
pub const SAC_MAC_LEN: u32 = 8u32;
pub const SAC_PROTOCOL_V1: u32 = 2u32;
pub const SAC_PROTOCOL_WMDM: u32 = 1u32;
pub const SAC_SESSION_KEYLEN: u32 = 8u32;
pub const SCP_EVENTID_ACQSECURECLOCK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2250542281, data2: 19033, data3: 17378, data4: [145, 70, 72, 167, 243, 244, 20, 12] };
pub const SCP_EVENTID_DRMINFO: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 557699719, data2: 16850, data3: 17195, data4: [158, 63, 59, 79, 123, 53, 129, 221] };
pub const SCP_EVENTID_NEEDTOINDIV: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2275739591,
    data2: 46185,
    data3: 17286,
    data4: [185, 118, 213, 209, 206, 83, 138, 111],
};
pub const SCP_PARAMID_DRMVERSION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1104155997, data2: 31943, data3: 16919, data4: [173, 169, 0, 80, 116, 98, 77, 164] };
pub struct WMDMDATETIME(i32);
pub struct WMDMDetermineMaxPropStringLen(i32);
pub struct WMDMDevice(i32);
pub struct WMDMDeviceEnum(i32);
pub struct WMDMID(i32);
pub const WMDMID_LENGTH: u32 = 128u32;
pub struct WMDMLogger(i32);
pub struct WMDMMessage(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WMDMMetadataView(i32);
pub struct WMDMRIGHTS(i32);
pub struct WMDMStorage(i32);
pub struct WMDMStorageEnum(i32);
pub struct WMDMStorageGlobal(i32);
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
pub const WMDM_DEVICE_PROTOCOL_MSC: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2765275756,
    data2: 43137,
    data3: 17595,
    data4: [189, 93, 31, 112, 60, 113, 247, 169],
};
pub const WMDM_DEVICE_PROTOCOL_MTP: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2543736037,
    data2: 2812,
    data3: 17924,
    data4: [141, 147, 220, 121, 138, 75, 207, 69],
};
pub const WMDM_DEVICE_PROTOCOL_RAPI: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 705818001, data2: 35983, data3: 16868, data4: [130, 209, 131, 134, 224, 3, 86, 28] };
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
pub struct WMDM_ENUM_PROP_VALID_VALUES_FORM(i32);
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
pub struct WMDM_FIND_SCOPE(i32);
pub struct WMDM_FORMATCODE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_FORMAT_CAPABILITY(i32);
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_CONFIG(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_DESC(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_ENUM(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_RANGE(i32);
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
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2112383085,
    data2: 30958,
    data3: 17386,
    data4: [164, 150, 198, 37, 172, 145, 204, 93],
};
pub struct WMDM_SESSION_TYPE(i32);
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
pub struct WMDM_STORAGE_ENUM_MODE(i32);
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
pub struct WMDM_TAG_DATATYPE(i32);
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WMFILECAPABILITIES(i32);
pub struct _BITMAPINFOHEADER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct _VIDEOINFOHEADER(i32);
pub struct _WAVEFORMATEX(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct __MACINFO(i32);
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
