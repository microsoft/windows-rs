pub const BIO_UNIT_CONTROL_UNIT: u16 = 64;
pub const BIO_UNIT_DELETE_TEMPLATE: u16 = 32;
pub const BIO_UNIT_ENROLL: u16 = 16;
pub const BIO_UNIT_EXTENDED_ACCESS: u16 = 8;
pub const BIO_UNIT_MAINTENANCE: u16 = 2;
pub const BIO_UNIT_OPEN_SESSION: u16 = 4;
pub const BIO_UNIT_RAW: u16 = 1;
pub type PWINBIO_ACCOUNT_POLICY = *mut WINBIO_ACCOUNT_POLICY;
pub type PWINBIO_ANTI_SPOOF_POLICY = *mut WINBIO_ANTI_SPOOF_POLICY;
pub type PWINBIO_ANTI_SPOOF_POLICY_ACTION = *mut WINBIO_ANTI_SPOOF_POLICY_ACTION;
pub type PWINBIO_BDB_ANSI_381_HEADER = *mut WINBIO_BDB_ANSI_381_HEADER;
pub type PWINBIO_BDB_ANSI_381_RECORD = *mut WINBIO_BDB_ANSI_381_RECORD;
pub type PWINBIO_BIOMETRIC_SENSOR_SUBTYPE = *mut u32;
pub type PWINBIO_BIOMETRIC_SUBTYPE = *mut u8;
pub type PWINBIO_BIOMETRIC_TYPE = *mut u32;
pub type PWINBIO_BIR = *mut WINBIO_BIR;
pub type PWINBIO_BIR_DATA = *mut WINBIO_BIR_DATA;
pub type PWINBIO_BIR_DATA_FLAGS = *mut u8;
pub type PWINBIO_BIR_HEADER = *mut WINBIO_BIR_HEADER;
pub type PWINBIO_BIR_PURPOSE = *mut u8;
pub type PWINBIO_BIR_QUALITY = *mut i8;
pub type PWINBIO_BIR_VERSION = *mut u8;
pub type PWINBIO_BSP_SCHEMA = *mut WINBIO_BSP_SCHEMA;
pub type PWINBIO_CAPABILITIES = *mut u32;
pub type PWINBIO_COMPONENT = *mut u32;
pub type PWINBIO_CONNECTED_SENSOR = *mut WINBIO_CONNECTED_SENSOR;
pub type PWINBIO_CREDENTIAL_STATE = *mut WINBIO_CREDENTIAL_STATE;
pub type PWINBIO_ESS_STATE = *mut u64;
pub type PWINBIO_EVENT = *mut WINBIO_EVENT;
pub type PWINBIO_EVENT_TYPE = *mut u32;
pub type PWINBIO_EXTENDED_ENGINE_INFO = *mut WINBIO_EXTENDED_ENGINE_INFO;
pub type PWINBIO_EXTENDED_ENROLLMENT_PARAMETERS = *mut WINBIO_EXTENDED_ENROLLMENT_PARAMETERS;
#[cfg(feature = "Win32_windef")]
pub type PWINBIO_EXTENDED_ENROLLMENT_STATUS = *mut WINBIO_EXTENDED_ENROLLMENT_STATUS;
#[cfg(feature = "Win32_windef")]
pub type PWINBIO_EXTENDED_SENSOR_INFO = *mut WINBIO_EXTENDED_SENSOR_INFO;
pub type PWINBIO_EXTENDED_STORAGE_INFO = *mut WINBIO_EXTENDED_STORAGE_INFO;
pub type PWINBIO_EXTENDED_UNIT_STATUS = *mut WINBIO_EXTENDED_UNIT_STATUS;
pub type PWINBIO_FP_BU_STATE = *mut WINBIO_FP_BU_STATE;
pub type PWINBIO_FRAMEWORK_CHANGE_TYPE = *mut u32;
pub type PWINBIO_FRAMEWORK_HANDLE = *mut WINBIO_SESSION_HANDLE;
pub type PWINBIO_GESTURE_METADATA = *mut WINBIO_GESTURE_METADATA;
pub type PWINBIO_IDENTITY = *mut WINBIO_IDENTITY;
pub type PWINBIO_IDENTITY_TYPE = *mut u32;
pub type PWINBIO_INDICATOR_STATUS = *mut u32;
pub type PWINBIO_MATCH_TYPE = *mut u32;
pub type PWINBIO_OPERATION_TYPE = *mut u32;
pub type PWINBIO_ORIENTATION = *mut u32;
pub type PWINBIO_POLICY_SOURCE = *mut WINBIO_POLICY_SOURCE;
pub type PWINBIO_POOL_TYPE = *mut u32;
#[cfg(feature = "Win32_windef")]
pub type PWINBIO_PRESENCE = *mut WINBIO_PRESENCE;
pub type PWINBIO_PRESENCE_CHANGE = *mut u32;
#[cfg(feature = "Win32_windef")]
pub type PWINBIO_PRESENCE_PROPERTIES = *mut WINBIO_PRESENCE_PROPERTIES;
pub type PWINBIO_PROPERTY_ID = *mut u32;
pub type PWINBIO_PROPERTY_TYPE = *mut u32;
pub type PWINBIO_PROTECTION_POLICY = *mut WINBIO_PROTECTION_POLICY;
pub type PWINBIO_PROTECTION_TICKET = *mut u64;
pub type PWINBIO_PROTECTION_TYPE = *mut u32;
pub type PWINBIO_REGISTERED_FORMAT = *mut WINBIO_REGISTERED_FORMAT;
pub type PWINBIO_REJECT_DETAIL = *mut u32;
pub type PWINBIO_SECURE_BUFFER_HEADER_V1 = *mut WINBIO_SECURE_BUFFER_HEADER_V1;
pub type PWINBIO_SECURE_CONNECTION_DATA = *mut WINBIO_SECURE_CONNECTION_DATA;
pub type PWINBIO_SECURE_CONNECTION_PARAMS = *mut WINBIO_SECURE_CONNECTION_PARAMS;
pub type PWINBIO_SENSOR_MODE = *mut u32;
pub type PWINBIO_SENSOR_STATUS = *mut u32;
pub type PWINBIO_SESSION_FLAGS = *mut u32;
pub type PWINBIO_SESSION_HANDLE = *mut u32;
pub type PWINBIO_SETTING_SOURCE_TYPE = *mut u32;
pub type PWINBIO_SETTING_TYPE = *mut u32;
pub type PWINBIO_STORAGE_SCHEMA = *mut WINBIO_STORAGE_SCHEMA;
pub type PWINBIO_STRING = *mut WINBIO_STRING;
pub type PWINBIO_TELEMETRY_TYPE = *mut u32;
pub type PWINBIO_UNIT_ID = *mut u32;
pub type PWINBIO_UNIT_SCHEMA = *mut WINBIO_UNIT_SCHEMA;
pub type PWINBIO_UNIT_SECURITY_LEVEL = *mut u32;
pub type PWINBIO_UUID = *mut windows_sys::core::GUID;
pub type PWINBIO_VERSION = *mut WINBIO_VERSION;
pub type PWINBIO_WAKE_REASON = *mut u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_ACCOUNT_POLICY {
    pub Identity: WINBIO_IDENTITY,
    pub AntiSpoofBehavior: WINBIO_ANTI_SPOOF_POLICY_ACTION,
}
impl Default for WINBIO_ACCOUNT_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINBIO_ANSI_381_FORMAT_OWNER: u16 = 27;
pub const WINBIO_ANSI_381_FORMAT_TYPE: u16 = 1025;
pub const WINBIO_ANSI_381_IMG_ACQ_LEVEL_10: u16 = 10;
pub const WINBIO_ANSI_381_IMG_ACQ_LEVEL_20: u16 = 20;
pub const WINBIO_ANSI_381_IMG_ACQ_LEVEL_30: u16 = 30;
pub const WINBIO_ANSI_381_IMG_ACQ_LEVEL_31: u16 = 31;
pub const WINBIO_ANSI_381_IMG_ACQ_LEVEL_40: u16 = 40;
pub const WINBIO_ANSI_381_IMG_ACQ_LEVEL_41: u16 = 41;
pub const WINBIO_ANSI_381_IMG_BIT_PACKED: u8 = 1;
pub const WINBIO_ANSI_381_IMG_COMPRESSED_JPEG: u8 = 3;
pub const WINBIO_ANSI_381_IMG_COMPRESSED_JPEG2000: u8 = 4;
pub const WINBIO_ANSI_381_IMG_COMPRESSED_PNG: u8 = 5;
pub const WINBIO_ANSI_381_IMG_COMPRESSED_WSQ: u8 = 2;
pub const WINBIO_ANSI_381_IMG_UNCOMPRESSED: u8 = 0;
pub const WINBIO_ANSI_381_IMP_TYPE_LATENT: u8 = 7;
pub const WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_CONTACTLESS: u8 = 9;
pub const WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_PLAIN: u8 = 0;
pub const WINBIO_ANSI_381_IMP_TYPE_LIVE_SCAN_ROLLED: u8 = 1;
pub const WINBIO_ANSI_381_IMP_TYPE_NONLIVE_SCAN_PLAIN: u8 = 2;
pub const WINBIO_ANSI_381_IMP_TYPE_NONLIVE_SCAN_ROLLED: u8 = 3;
pub const WINBIO_ANSI_381_IMP_TYPE_SWIPE: u8 = 8;
pub const WINBIO_ANSI_381_PIXELS_PER_CM: u8 = 2;
pub const WINBIO_ANSI_381_PIXELS_PER_INCH: u8 = 1;
pub const WINBIO_ANSI_381_POS_LH_FOUR_FINGERS: WINBIO_BIOMETRIC_SUBTYPE = 14;
pub const WINBIO_ANSI_381_POS_LH_FULL_PALM: WINBIO_BIOMETRIC_SUBTYPE = 23;
pub const WINBIO_ANSI_381_POS_LH_HYPOTHENAR: WINBIO_BIOMETRIC_SUBTYPE = 36;
pub const WINBIO_ANSI_381_POS_LH_INDEX_FINGER: WINBIO_BIOMETRIC_SUBTYPE = 7;
pub const WINBIO_ANSI_381_POS_LH_INTERDIGITAL: WINBIO_BIOMETRIC_SUBTYPE = 34;
pub const WINBIO_ANSI_381_POS_LH_LITTLE_FINGER: WINBIO_BIOMETRIC_SUBTYPE = 10;
pub const WINBIO_ANSI_381_POS_LH_LOWER_PALM: WINBIO_BIOMETRIC_SUBTYPE = 27;
pub const WINBIO_ANSI_381_POS_LH_MIDDLE_FINGER: WINBIO_BIOMETRIC_SUBTYPE = 8;
pub const WINBIO_ANSI_381_POS_LH_OTHER: WINBIO_BIOMETRIC_SUBTYPE = 30;
pub const WINBIO_ANSI_381_POS_LH_RING_FINGER: WINBIO_BIOMETRIC_SUBTYPE = 9;
pub const WINBIO_ANSI_381_POS_LH_THENAR: WINBIO_BIOMETRIC_SUBTYPE = 35;
pub const WINBIO_ANSI_381_POS_LH_THUMB: WINBIO_BIOMETRIC_SUBTYPE = 6;
pub const WINBIO_ANSI_381_POS_LH_UPPER_PALM: WINBIO_BIOMETRIC_SUBTYPE = 28;
pub const WINBIO_ANSI_381_POS_LH_WRITERS_PALM: WINBIO_BIOMETRIC_SUBTYPE = 24;
pub const WINBIO_ANSI_381_POS_RH_FOUR_FINGERS: WINBIO_BIOMETRIC_SUBTYPE = 13;
pub const WINBIO_ANSI_381_POS_RH_FULL_PALM: WINBIO_BIOMETRIC_SUBTYPE = 21;
pub const WINBIO_ANSI_381_POS_RH_HYPOTHENAR: WINBIO_BIOMETRIC_SUBTYPE = 33;
pub const WINBIO_ANSI_381_POS_RH_INDEX_FINGER: WINBIO_BIOMETRIC_SUBTYPE = 2;
pub const WINBIO_ANSI_381_POS_RH_INTERDIGITAL: WINBIO_BIOMETRIC_SUBTYPE = 31;
pub const WINBIO_ANSI_381_POS_RH_LITTLE_FINGER: WINBIO_BIOMETRIC_SUBTYPE = 5;
pub const WINBIO_ANSI_381_POS_RH_LOWER_PALM: WINBIO_BIOMETRIC_SUBTYPE = 25;
pub const WINBIO_ANSI_381_POS_RH_MIDDLE_FINGER: WINBIO_BIOMETRIC_SUBTYPE = 3;
pub const WINBIO_ANSI_381_POS_RH_OTHER: WINBIO_BIOMETRIC_SUBTYPE = 29;
pub const WINBIO_ANSI_381_POS_RH_RING_FINGER: WINBIO_BIOMETRIC_SUBTYPE = 4;
pub const WINBIO_ANSI_381_POS_RH_THENAR: WINBIO_BIOMETRIC_SUBTYPE = 32;
pub const WINBIO_ANSI_381_POS_RH_THUMB: WINBIO_BIOMETRIC_SUBTYPE = 1;
pub const WINBIO_ANSI_381_POS_RH_UPPER_PALM: WINBIO_BIOMETRIC_SUBTYPE = 26;
pub const WINBIO_ANSI_381_POS_RH_WRITERS_PALM: WINBIO_BIOMETRIC_SUBTYPE = 22;
pub const WINBIO_ANSI_381_POS_TWO_THUMBS: WINBIO_BIOMETRIC_SUBTYPE = 15;
pub const WINBIO_ANSI_381_POS_UNKNOWN: WINBIO_BIOMETRIC_SUBTYPE = 0;
pub const WINBIO_ANSI_381_POS_UNKNOWN_PALM: WINBIO_BIOMETRIC_SUBTYPE = 20;
pub const WINBIO_ANSI_385_FACE_FRONTAL_FULL: WINBIO_BIOMETRIC_SUBTYPE = 1;
pub const WINBIO_ANSI_385_FACE_FRONTAL_TOKEN: WINBIO_BIOMETRIC_SUBTYPE = 2;
pub const WINBIO_ANSI_385_FACE_TYPE_UNKNOWN: WINBIO_BIOMETRIC_SUBTYPE = 0;
pub const WINBIO_ANTI_SPOOF_DISABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = 0;
pub const WINBIO_ANTI_SPOOF_ENABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_ANTI_SPOOF_POLICY {
    pub Action: WINBIO_ANTI_SPOOF_POLICY_ACTION,
    pub Source: WINBIO_POLICY_SOURCE,
}
pub type WINBIO_ANTI_SPOOF_POLICY_ACTION = i32;
pub const WINBIO_ANTI_SPOOF_REMOVE: WINBIO_ANTI_SPOOF_POLICY_ACTION = 2;
pub const WINBIO_ANTI_SPOOF_TURN_SIDE_TO_SIDE: WINBIO_REJECT_DETAIL = 16777216;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_BDB_ANSI_381_HEADER {
    pub RecordLength: u64,
    pub FormatIdentifier: u32,
    pub VersionNumber: u32,
    pub ProductId: WINBIO_REGISTERED_FORMAT,
    pub CaptureDeviceId: u16,
    pub ImageAcquisitionLevel: u16,
    pub HorizontalScanResolution: u16,
    pub VerticalScanResolution: u16,
    pub HorizontalImageResolution: u16,
    pub VerticalImageResolution: u16,
    pub ElementCount: u8,
    pub ScaleUnits: u8,
    pub PixelDepth: u8,
    pub ImageCompressionAlg: u8,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_BDB_ANSI_381_RECORD {
    pub BlockLength: u32,
    pub HorizontalLineLength: u16,
    pub VerticalLineLength: u16,
    pub Position: WINBIO_BIOMETRIC_SUBTYPE,
    pub CountOfViews: u8,
    pub ViewNumber: u8,
    pub ImageQuality: u8,
    pub ImpressionType: u8,
    pub Reserved: u8,
}
pub type WINBIO_BIOMETRIC_SENSOR_SUBTYPE = u32;
pub type WINBIO_BIOMETRIC_SUBTYPE = u8;
pub type WINBIO_BIOMETRIC_TYPE = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_BIR {
    pub HeaderBlock: WINBIO_BIR_DATA,
    pub StandardDataBlock: WINBIO_BIR_DATA,
    pub VendorDataBlock: WINBIO_BIR_DATA,
    pub SignatureBlock: WINBIO_BIR_DATA,
}
pub const WINBIO_BIR_ALGIN_SIZE: u32 = 8;
pub const WINBIO_BIR_ALIGN_SIZE: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_BIR_DATA {
    pub Size: u32,
    pub Offset: u32,
}
pub type WINBIO_BIR_DATA_FLAGS = u8;
pub const WINBIO_BIR_FIELD_BIOMETRIC_CONDITION: u16 = 2048;
pub const WINBIO_BIR_FIELD_BIOMETRIC_PURPOSE: u16 = 1024;
pub const WINBIO_BIR_FIELD_BIOMETRIC_SUBTYPE: u16 = 128;
pub const WINBIO_BIR_FIELD_BIOMETRIC_TYPE: u16 = 64;
pub const WINBIO_BIR_FIELD_CBEFF_HEADER_VERSION: u16 = 256;
pub const WINBIO_BIR_FIELD_CHALLENGE: u16 = 16384;
pub const WINBIO_BIR_FIELD_CREATION_DATE: u16 = 16;
pub const WINBIO_BIR_FIELD_CREATOR: u16 = 8192;
pub const WINBIO_BIR_FIELD_INDEX: u16 = 8;
pub const WINBIO_BIR_FIELD_NEVER_VALID: u32 = 49165;
pub const WINBIO_BIR_FIELD_PATRON_HEADER_VERSION: u16 = 512;
pub const WINBIO_BIR_FIELD_PATRON_ID: u16 = 4;
pub const WINBIO_BIR_FIELD_PAYLOAD: u16 = 32768;
pub const WINBIO_BIR_FIELD_PRODUCT_ID: u16 = 2;
pub const WINBIO_BIR_FIELD_QUALITY: u16 = 4096;
pub const WINBIO_BIR_FIELD_SUBHEAD_COUNT: u16 = 1;
pub const WINBIO_BIR_FIELD_VALIDITY_PERIOD: u16 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_BIR_HEADER {
    pub ValidFields: u16,
    pub HeaderVersion: WINBIO_BIR_VERSION,
    pub PatronHeaderVersion: WINBIO_BIR_VERSION,
    pub DataFlags: WINBIO_BIR_DATA_FLAGS,
    pub Type: WINBIO_BIOMETRIC_TYPE,
    pub Subtype: WINBIO_BIOMETRIC_SUBTYPE,
    pub Purpose: WINBIO_BIR_PURPOSE,
    pub DataQuality: WINBIO_BIR_QUALITY,
    pub CreationDate: i64,
    pub ValidityPeriod: WINBIO_BIR_HEADER_0,
    pub BiometricDataFormat: WINBIO_REGISTERED_FORMAT,
    pub ProductId: WINBIO_REGISTERED_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_BIR_HEADER_0 {
    pub BeginDate: i64,
    pub EndDate: i64,
}
pub type WINBIO_BIR_PURPOSE = u8;
pub type WINBIO_BIR_QUALITY = i8;
pub type WINBIO_BIR_VERSION = u8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_BSP_SCHEMA {
    pub BiometricFactor: WINBIO_BIOMETRIC_TYPE,
    pub BspId: WINBIO_UUID,
    pub Description: WINBIO_STRING,
    pub Vendor: WINBIO_STRING,
    pub Version: WINBIO_VERSION,
}
impl Default for WINBIO_BSP_SCHEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINBIO_CAPABILITIES = u32;
pub const WINBIO_CAPABILITY_DATABASE: WINBIO_CAPABILITIES = 4;
pub const WINBIO_CAPABILITY_ENCRYPTION: WINBIO_CAPABILITIES = 16;
pub const WINBIO_CAPABILITY_INDICATOR: WINBIO_CAPABILITIES = 64;
pub const WINBIO_CAPABILITY_MATCHING: WINBIO_CAPABILITIES = 2;
pub const WINBIO_CAPABILITY_NAVIGATION: WINBIO_CAPABILITIES = 32;
pub const WINBIO_CAPABILITY_PROCESSING: WINBIO_CAPABILITIES = 8;
pub const WINBIO_CAPABILITY_SCP_V1: WINBIO_CAPABILITIES = 512;
pub const WINBIO_CAPABILITY_SECURE_SENSOR: WINBIO_CAPABILITIES = 256;
pub const WINBIO_CAPABILITY_SENSOR: WINBIO_CAPABILITIES = 1;
pub const WINBIO_CAPABILITY_VIRTUAL_SENSOR: WINBIO_CAPABILITIES = 128;
pub const WINBIO_CAPABILITY_WAKE: WINBIO_CAPABILITIES = 1024;
pub const WINBIO_CBEFF_HEADER_VERSION: WINBIO_BIR_VERSION = 17;
pub type WINBIO_COMPONENT = u32;
pub const WINBIO_COMPONENT_ENGINE: WINBIO_COMPONENT = 2;
pub const WINBIO_COMPONENT_SENSOR: WINBIO_COMPONENT = 1;
pub const WINBIO_COMPONENT_STORAGE: WINBIO_COMPONENT = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_CONNECTED_SENSOR {
    pub biometricType: WINBIO_BIOMETRIC_TYPE,
    pub isEnhancedSignInSecurityCapable: windows_sys::core::BOOL,
}
pub const WINBIO_CREDENTIAL_ALL: WINBIO_CREDENTIAL_TYPE = -1;
pub type WINBIO_CREDENTIAL_FORMAT = i32;
pub const WINBIO_CREDENTIAL_NOT_SET: WINBIO_CREDENTIAL_STATE = 1;
pub const WINBIO_CREDENTIAL_PASSWORD: WINBIO_CREDENTIAL_TYPE = 1;
pub const WINBIO_CREDENTIAL_SET: WINBIO_CREDENTIAL_STATE = 2;
pub type WINBIO_CREDENTIAL_STATE = i32;
pub type WINBIO_CREDENTIAL_TYPE = i32;
pub const WINBIO_DATABASE_FLAG_MASK: u32 = 4294901760;
pub const WINBIO_DATABASE_FLAG_REMOTE: u32 = 131072;
pub const WINBIO_DATABASE_FLAG_REMOVABLE: u32 = 65536;
pub const WINBIO_DATABASE_TYPE_DBMS: u32 = 2;
pub const WINBIO_DATABASE_TYPE_FILE: u32 = 1;
pub const WINBIO_DATABASE_TYPE_MASK: u32 = 65535;
pub const WINBIO_DATABASE_TYPE_ONCHIP: u32 = 3;
pub const WINBIO_DATABASE_TYPE_SMARTCARD: u32 = 4;
pub const WINBIO_DATA_FLAG_INTEGRITY: u8 = 1;
pub const WINBIO_DATA_FLAG_INTERMEDIATE: u8 = 64;
pub const WINBIO_DATA_FLAG_OPTION_MASK_PRESENT: u8 = 8;
pub const WINBIO_DATA_FLAG_PRIVACY: u8 = 2;
pub const WINBIO_DATA_FLAG_PROCESSED: u8 = 128;
pub const WINBIO_DATA_FLAG_RAW: u8 = 32;
pub const WINBIO_DATA_FLAG_SIGNED: u8 = 4;
pub const WINBIO_DATA_QUALITY_NOT_SET: WINBIO_BIR_QUALITY = -1;
pub const WINBIO_DATA_QUALITY_NOT_SUPPORTED: WINBIO_BIR_QUALITY = -2;
pub const WINBIO_ENG_CAP_ITERATIVE_IMPROVEMENT: WINBIO_CAPABILITIES = 1;
pub const WINBIO_ENG_CAP_SPOOF_DETECTION: WINBIO_CAPABILITIES = 2;
pub const WINBIO_ESS_BLOCKED_NON_ESS_CAMERA: WINBIO_ESS_STATE_FLAGS = 16384;
pub const WINBIO_ESS_BLOCKED_NON_ESS_FPR: WINBIO_ESS_STATE_FLAGS = 8192;
pub const WINBIO_ESS_MANAGED_BY_POLICY: WINBIO_ESS_STATE_FLAGS = 128;
pub const WINBIO_ESS_REQUIRES_ENABLEMENT: WINBIO_ESS_STATE_FLAGS = 64;
pub const WINBIO_ESS_REQUIRES_FACE_SENSOR: WINBIO_ESS_STATE_FLAGS = 1024;
pub const WINBIO_ESS_REQUIRES_FPR_SENSOR: WINBIO_ESS_STATE_FLAGS = 2048;
pub const WINBIO_ESS_REQUIRES_ISOLATED_PROCESS: WINBIO_ESS_STATE_FLAGS = 4096;
pub const WINBIO_ESS_REQUIRES_NON_VBS_BIOMETRIC_ENROLLMENT_ABSENCE: WINBIO_ESS_STATE_FLAGS = 256;
pub const WINBIO_ESS_REQUIRES_NON_VBS_WINDOWS_HELLO_ABSENCE: WINBIO_ESS_STATE_FLAGS = 4;
pub const WINBIO_ESS_REQUIRES_TPM2: WINBIO_ESS_STATE_FLAGS = 1;
pub const WINBIO_ESS_REQUIRES_VBS_BIOMETRIC_ENROLLMENT: WINBIO_ESS_STATE_FLAGS = 512;
pub const WINBIO_ESS_REQUIRES_VBS_CAPABLE: WINBIO_ESS_STATE_FLAGS = 2;
pub const WINBIO_ESS_REQUIRES_VBS_ENCRYPTION_KEY: WINBIO_ESS_STATE_FLAGS = 32;
pub const WINBIO_ESS_REQUIRES_VBS_RUNNING: WINBIO_ESS_STATE_FLAGS = 16;
pub const WINBIO_ESS_REQUIRES_VBS_WINDOWS_HELLO: WINBIO_ESS_STATE_FLAGS = 8;
pub const WINBIO_ESS_SOURCE_DEFAULT: WINBIO_ESS_STATE_FLAGS = 32768;
pub type WINBIO_ESS_STATE = u64;
pub type WINBIO_ESS_STATE_FLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EVENT {
    pub Type: WINBIO_EVENT_TYPE,
    pub Parameters: WINBIO_EVENT_0,
}
impl Default for WINBIO_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_EVENT_0 {
    pub Unclaimed: WINBIO_EVENT_0_0,
    pub UnclaimedIdentify: WINBIO_EVENT_0_1,
    pub Error: WINBIO_EVENT_0_2,
}
impl Default for WINBIO_EVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EVENT_0_0 {
    pub UnitId: WINBIO_UNIT_ID,
    pub RejectDetail: WINBIO_REJECT_DETAIL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EVENT_0_1 {
    pub UnitId: WINBIO_UNIT_ID,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: WINBIO_BIOMETRIC_SUBTYPE,
    pub RejectDetail: WINBIO_REJECT_DETAIL,
}
impl Default for WINBIO_EVENT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EVENT_0_2 {
    pub ErrorCode: windows_sys::core::HRESULT,
}
pub const WINBIO_EVENT_ERROR: WINBIO_EVENT_TYPE = 4294967295;
pub const WINBIO_EVENT_FP_UNCLAIMED: WINBIO_EVENT_TYPE = 1;
pub const WINBIO_EVENT_FP_UNCLAIMED_IDENTIFY: WINBIO_EVENT_TYPE = 2;
pub type WINBIO_EVENT_TYPE = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_ENGINE_INFO {
    pub GenericEngineCapabilities: WINBIO_CAPABILITIES,
    pub Factor: WINBIO_BIOMETRIC_TYPE,
    pub Specific: WINBIO_EXTENDED_ENGINE_INFO_0,
}
impl Default for WINBIO_EXTENDED_ENGINE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_EXTENDED_ENGINE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENGINE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENGINE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_ENGINE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_ENGINE_INFO_0_3,
}
impl Default for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    pub Capabilities: WINBIO_CAPABILITIES,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    pub Null: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    pub Capabilities: WINBIO_CAPABILITIES,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_1_0,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    pub Capabilities: WINBIO_CAPABILITIES,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_2_0,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    pub Null: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    pub Capabilities: WINBIO_CAPABILITIES,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_3_0,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    pub Null: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    pub Size: usize,
    pub SubFactor: WINBIO_BIOMETRIC_SUBTYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS {
    pub TemplateStatus: windows_sys::core::HRESULT,
    pub RejectDetail: WINBIO_REJECT_DETAIL,
    pub PercentComplete: u32,
    pub Factor: WINBIO_BIOMETRIC_TYPE,
    pub SubFactor: WINBIO_BIOMETRIC_SUBTYPE,
    pub Specific: WINBIO_EXTENDED_ENROLLMENT_STATUS_0,
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub union WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1,
    pub Iris: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2,
    pub Voice: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3,
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    pub BoundingBox: super::windef::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    pub AdapterId: WINBIO_UUID,
    pub Data: [u32; 78],
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    pub EyeBoundingBox_1: super::windef::RECT,
    pub EyeBoundingBox_2: super::windef::RECT,
    pub PupilCenter_1: super::windef::POINT,
    pub PupilCenter_2: super::windef::POINT,
    pub Distance: i32,
    pub GridPointCompletionPercent: u32,
    pub GridPointIndex: u16,
    pub Point3D: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0,
    pub StopCaptureAndShowCriticalFeedback: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_SENSOR_INFO {
    pub GenericSensorCapabilities: WINBIO_CAPABILITIES,
    pub Factor: WINBIO_BIOMETRIC_TYPE,
    pub Specific: WINBIO_EXTENDED_SENSOR_INFO_0,
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_EXTENDED_SENSOR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub union WINBIO_EXTENDED_SENSOR_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_SENSOR_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_SENSOR_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_SENSOR_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_SENSOR_INFO_0_3,
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    pub FrameSize: super::windef::RECT,
    pub FrameOffset: super::windef::POINT,
    pub MandatoryOrientation: WINBIO_ORIENTATION,
    pub HardwareInfo: WINBIO_EXTENDED_SENSOR_INFO_0_0_0,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    pub ColorSensorId: [u16; 260],
    pub InfraredSensorId: [u16; 260],
    pub InfraredSensorRotationAngle: u32,
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    pub FrameSize: super::windef::RECT,
    pub FrameOffset: super::windef::POINT,
    pub MandatoryOrientation: WINBIO_ORIENTATION,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_EXTENDED_STORAGE_INFO {
    pub GenericStorageCapabilities: WINBIO_CAPABILITIES,
    pub Factor: WINBIO_BIOMETRIC_TYPE,
    pub Specific: WINBIO_EXTENDED_STORAGE_INFO_0,
}
impl Default for WINBIO_EXTENDED_STORAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_EXTENDED_STORAGE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_STORAGE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_STORAGE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_STORAGE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_STORAGE_INFO_0_3,
}
impl Default for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    pub Capabilities: WINBIO_CAPABILITIES,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    pub Capabilities: WINBIO_CAPABILITIES,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    pub Capabilities: WINBIO_CAPABILITIES,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    pub Capabilities: WINBIO_CAPABILITIES,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_EXTENDED_UNIT_STATUS {
    pub Availability: WINBIO_SENSOR_STATUS,
    pub ReasonCode: u32,
}
pub const WINBIO_FACE_AMBIGUOUS_TARGET: WINBIO_REJECT_DETAIL = 5;
pub const WINBIO_FACE_EYES_OCCLUDED: WINBIO_REJECT_DETAIL = 6;
pub const WINBIO_FACE_OCCLUDED: WINBIO_REJECT_DETAIL = 6;
pub const WINBIO_FACE_POOR_QUALITY: WINBIO_REJECT_DETAIL = 1;
pub const WINBIO_FACE_SPOOF_DETECTED: WINBIO_REJECT_DETAIL = 4;
pub const WINBIO_FACE_TOO_BRIGHT: WINBIO_REJECT_DETAIL = 2;
pub const WINBIO_FACE_TOO_DARK: WINBIO_REJECT_DETAIL = 3;
pub const WINBIO_FACE_TOO_FAR: WINBIO_REJECT_DETAIL = 2097152;
pub const WINBIO_FACE_TOO_HIGH: WINBIO_REJECT_DETAIL = 65536;
pub const WINBIO_FACE_TOO_LEFT: WINBIO_REJECT_DETAIL = 262144;
pub const WINBIO_FACE_TOO_LOW: WINBIO_REJECT_DETAIL = 131072;
pub const WINBIO_FACE_TOO_NEAR: WINBIO_REJECT_DETAIL = 1048576;
pub const WINBIO_FACE_TOO_RIGHT: WINBIO_REJECT_DETAIL = 524288;
pub const WINBIO_FACE_WRONG_ORIENTATION: WINBIO_REJECT_DETAIL = 7;
pub const WINBIO_FINGER_UNSPECIFIED_POS_01: WINBIO_BIOMETRIC_SUBTYPE = 245;
pub const WINBIO_FINGER_UNSPECIFIED_POS_02: WINBIO_BIOMETRIC_SUBTYPE = 246;
pub const WINBIO_FINGER_UNSPECIFIED_POS_03: WINBIO_BIOMETRIC_SUBTYPE = 247;
pub const WINBIO_FINGER_UNSPECIFIED_POS_04: WINBIO_BIOMETRIC_SUBTYPE = 248;
pub const WINBIO_FINGER_UNSPECIFIED_POS_05: WINBIO_BIOMETRIC_SUBTYPE = 249;
pub const WINBIO_FINGER_UNSPECIFIED_POS_06: WINBIO_BIOMETRIC_SUBTYPE = 250;
pub const WINBIO_FINGER_UNSPECIFIED_POS_07: WINBIO_BIOMETRIC_SUBTYPE = 251;
pub const WINBIO_FINGER_UNSPECIFIED_POS_08: WINBIO_BIOMETRIC_SUBTYPE = 252;
pub const WINBIO_FINGER_UNSPECIFIED_POS_09: WINBIO_BIOMETRIC_SUBTYPE = 253;
pub const WINBIO_FINGER_UNSPECIFIED_POS_10: WINBIO_BIOMETRIC_SUBTYPE = 254;
pub const WINBIO_FLAG_ADVANCED: u32 = 131072;
pub const WINBIO_FLAG_BASIC: u32 = 65536;
pub const WINBIO_FLAG_DEFAULT: WINBIO_SESSION_FLAGS = 0;
pub const WINBIO_FLAG_MAINTENANCE: u32 = 2;
pub const WINBIO_FLAG_RAW: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_FP_BU_STATE {
    pub SensorAttached: windows_sys::core::BOOL,
    pub CreationResult: windows_sys::core::HRESULT,
}
pub const WINBIO_FP_MERGE_FAILURE: WINBIO_REJECT_DETAIL = 10;
pub const WINBIO_FP_POOR_QUALITY: WINBIO_REJECT_DETAIL = 7;
pub const WINBIO_FP_SENSOR_SUBTYPE_SWIPE: WINBIO_BIOMETRIC_SENSOR_SUBTYPE = 1;
pub const WINBIO_FP_SENSOR_SUBTYPE_TOUCH: WINBIO_BIOMETRIC_SENSOR_SUBTYPE = 2;
pub const WINBIO_FP_TOO_FAST: WINBIO_REJECT_DETAIL = 5;
pub const WINBIO_FP_TOO_HIGH: WINBIO_REJECT_DETAIL = 1;
pub const WINBIO_FP_TOO_LEFT: WINBIO_REJECT_DETAIL = 3;
pub const WINBIO_FP_TOO_LOW: WINBIO_REJECT_DETAIL = 2;
pub const WINBIO_FP_TOO_RIGHT: WINBIO_REJECT_DETAIL = 4;
pub const WINBIO_FP_TOO_SHORT: WINBIO_REJECT_DETAIL = 9;
pub const WINBIO_FP_TOO_SKEWED: WINBIO_REJECT_DETAIL = 8;
pub const WINBIO_FP_TOO_SLOW: WINBIO_REJECT_DETAIL = 6;
pub type WINBIO_FRAMEWORK_CHANGE_TYPE = u32;
pub const WINBIO_FRAMEWORK_CHANGE_UNIT: WINBIO_FRAMEWORK_CHANGE_TYPE = 1;
pub const WINBIO_FRAMEWORK_CHANGE_UNIT_STATUS: WINBIO_FRAMEWORK_CHANGE_TYPE = 2;
pub type WINBIO_FRAMEWORK_HANDLE = WINBIO_SESSION_HANDLE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_GESTURE_METADATA {
    pub Size: usize,
    pub BiometricType: WINBIO_BIOMETRIC_TYPE,
    pub MatchType: WINBIO_MATCH_TYPE,
    pub ProtectionType: WINBIO_PROTECTION_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_IDENTITY {
    pub Type: WINBIO_IDENTITY_TYPE,
    pub Value: WINBIO_IDENTITY_0,
}
impl Default for WINBIO_IDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WINBIO_IDENTITY_0 {
    pub Null: u32,
    pub Wildcard: u32,
    pub TemplateGuid: windows_sys::core::GUID,
    pub AccountSid: WINBIO_IDENTITY_0_0,
    pub SecureId: [u8; 32],
}
impl Default for WINBIO_IDENTITY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_IDENTITY_0_0 {
    pub Size: u32,
    pub Data: [u8; 68],
}
impl Default for WINBIO_IDENTITY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINBIO_IDENTITY_SECURE_ID_SIZE: u32 = 32;
pub type WINBIO_IDENTITY_TYPE = u32;
pub const WINBIO_IDENTITY_WILDCARD: u32 = 621175426;
pub const WINBIO_ID_TYPE_GUID: WINBIO_IDENTITY_TYPE = 2;
pub const WINBIO_ID_TYPE_NULL: WINBIO_IDENTITY_TYPE = 0;
pub const WINBIO_ID_TYPE_SECURE_ID: WINBIO_IDENTITY_TYPE = 4;
pub const WINBIO_ID_TYPE_SID: WINBIO_IDENTITY_TYPE = 3;
pub const WINBIO_ID_TYPE_WILDCARD: WINBIO_IDENTITY_TYPE = 1;
pub const WINBIO_INDICATOR_OFF: WINBIO_INDICATOR_STATUS = 2;
pub const WINBIO_INDICATOR_ON: WINBIO_INDICATOR_STATUS = 1;
pub type WINBIO_INDICATOR_STATUS = u32;
pub const WINBIO_IRIS_BOTH_EYES: WINBIO_BIOMETRIC_SUBTYPE = 249;
pub const WINBIO_IRIS_DIRTY_LENS: WINBIO_REJECT_DETAIL = 8;
pub const WINBIO_IRIS_EITHER_EYE: WINBIO_BIOMETRIC_SUBTYPE = 250;
pub const WINBIO_IRIS_GLARE: WINBIO_REJECT_DETAIL = 7;
pub const WINBIO_IRIS_LEFT_EYE: WINBIO_BIOMETRIC_SUBTYPE = 245;
pub const WINBIO_IRIS_POOR_FOCUS: WINBIO_REJECT_DETAIL = 9;
pub const WINBIO_IRIS_POOR_QUALITY: WINBIO_REJECT_DETAIL = 1;
pub const WINBIO_IRIS_RIGHT_EYE: WINBIO_BIOMETRIC_SUBTYPE = 246;
pub const WINBIO_IRIS_SPOOF_DETECTED: WINBIO_REJECT_DETAIL = 4;
pub const WINBIO_IRIS_TOO_BRIGHT: WINBIO_REJECT_DETAIL = 2;
pub const WINBIO_IRIS_TOO_CLOSED: WINBIO_REJECT_DETAIL = 6;
pub const WINBIO_IRIS_TOO_DARK: WINBIO_REJECT_DETAIL = 3;
pub const WINBIO_IRIS_TOO_FAR: WINBIO_REJECT_DETAIL = 2097152;
pub const WINBIO_IRIS_TOO_HIGH: WINBIO_REJECT_DETAIL = 65536;
pub const WINBIO_IRIS_TOO_LEFT: WINBIO_REJECT_DETAIL = 262144;
pub const WINBIO_IRIS_TOO_LOW: WINBIO_REJECT_DETAIL = 131072;
pub const WINBIO_IRIS_TOO_NEAR: WINBIO_REJECT_DETAIL = 1048576;
pub const WINBIO_IRIS_TOO_RIGHT: WINBIO_REJECT_DETAIL = 524288;
pub const WINBIO_IRIS_TOO_SKEWED: WINBIO_REJECT_DETAIL = 5;
pub const WINBIO_IRIS_TYPE_UNKNOWN: WINBIO_BIOMETRIC_SUBTYPE = 0;
pub const WINBIO_IRIS_UNSPECIFIED_POS_01: WINBIO_BIOMETRIC_SUBTYPE = 247;
pub const WINBIO_IRIS_UNSPECIFIED_POS_02: WINBIO_BIOMETRIC_SUBTYPE = 248;
pub const WINBIO_IRIS_WRONG_ORIENTATION: WINBIO_REJECT_DETAIL = 10;
pub const WINBIO_MATCH_ON_CHIP: WINBIO_MATCH_TYPE = 3;
pub const WINBIO_MATCH_SOFTWARE: WINBIO_MATCH_TYPE = 1;
pub const WINBIO_MATCH_TRUSTED_EXECUTION_ENVIRONMENT: WINBIO_MATCH_TYPE = 2;
pub type WINBIO_MATCH_TYPE = u32;
pub const WINBIO_MAX_PRIVATE_SENSOR_TYPE_INFO_BUFFER_SIZE: u32 = 4096;
pub const WINBIO_MAX_SAMPLE_BUFFER_SIZE: u32 = 2147483647;
pub const WINBIO_MAX_SET_PROPERTY_BUFFER_SIZE: u32 = 4096;
pub const WINBIO_MAX_STRING_LEN: u32 = 256;
pub const WINBIO_NO_FORMAT_OWNER_AVAILABLE: u16 = 0;
pub const WINBIO_NO_FORMAT_TYPE_AVAILABLE: u16 = 0;
pub const WINBIO_NO_PURPOSE_AVAILABLE: WINBIO_BIR_PURPOSE = 0;
pub const WINBIO_NO_TYPE_AVAILABLE: WINBIO_BIOMETRIC_TYPE = 0;
pub const WINBIO_OPAQUE_ENGINE_DATA_ITEM_COUNT: u32 = 78;
pub const WINBIO_OPERATION_CAPTURE_SAMPLE: WINBIO_OPERATION_TYPE = 12;
pub const WINBIO_OPERATION_CLOSE: WINBIO_OPERATION_TYPE = 2;
pub const WINBIO_OPERATION_CLOSE_FRAMEWORK: WINBIO_OPERATION_TYPE = 21;
pub const WINBIO_OPERATION_CONTROL_UNIT: WINBIO_OPERATION_TYPE = 18;
pub const WINBIO_OPERATION_CONTROL_UNIT_PRIVILEGED: WINBIO_OPERATION_TYPE = 19;
pub const WINBIO_OPERATION_DELETE_TEMPLATE: WINBIO_OPERATION_TYPE = 11;
pub const WINBIO_OPERATION_ENROLL_AUTHORIZE: WINBIO_OPERATION_TYPE = 31;
pub const WINBIO_OPERATION_ENROLL_BEGIN: WINBIO_OPERATION_TYPE = 6;
pub const WINBIO_OPERATION_ENROLL_CAPTURE: WINBIO_OPERATION_TYPE = 7;
pub const WINBIO_OPERATION_ENROLL_COMMIT: WINBIO_OPERATION_TYPE = 8;
pub const WINBIO_OPERATION_ENROLL_DISCARD: WINBIO_OPERATION_TYPE = 9;
pub const WINBIO_OPERATION_ENROLL_REVOKE: WINBIO_OPERATION_TYPE = 32;
pub const WINBIO_OPERATION_ENROLL_SELECT: WINBIO_OPERATION_TYPE = 30;
pub const WINBIO_OPERATION_ENUM_BIOMETRIC_UNITS: WINBIO_OPERATION_TYPE = 23;
pub const WINBIO_OPERATION_ENUM_DATABASES: WINBIO_OPERATION_TYPE = 24;
pub const WINBIO_OPERATION_ENUM_ENROLLMENTS: WINBIO_OPERATION_TYPE = 10;
pub const WINBIO_OPERATION_ENUM_SERVICE_PROVIDERS: WINBIO_OPERATION_TYPE = 22;
pub const WINBIO_OPERATION_GET_EVENT: WINBIO_OPERATION_TYPE = 15;
pub const WINBIO_OPERATION_GET_PROPERTY: WINBIO_OPERATION_TYPE = 13;
pub const WINBIO_OPERATION_GET_PROTECTION_POLICY: WINBIO_OPERATION_TYPE = 33;
pub const WINBIO_OPERATION_IDENTIFY: WINBIO_OPERATION_TYPE = 4;
pub const WINBIO_OPERATION_IDENTIFY_AND_RELEASE_TICKET: WINBIO_OPERATION_TYPE = 27;
pub const WINBIO_OPERATION_IMPROVE_BEGIN: WINBIO_OPERATION_TYPE = 35;
pub const WINBIO_OPERATION_IMPROVE_END: WINBIO_OPERATION_TYPE = 36;
pub const WINBIO_OPERATION_LOCATE_SENSOR: WINBIO_OPERATION_TYPE = 5;
pub const WINBIO_OPERATION_LOCK_UNIT: WINBIO_OPERATION_TYPE = 16;
pub const WINBIO_OPERATION_MONITOR_PRESENCE: WINBIO_OPERATION_TYPE = 29;
pub const WINBIO_OPERATION_NONE: WINBIO_OPERATION_TYPE = 0;
pub const WINBIO_OPERATION_NOTIFY_UNIT_STATUS_CHANGE: WINBIO_OPERATION_TYPE = 34;
pub const WINBIO_OPERATION_OPEN: WINBIO_OPERATION_TYPE = 1;
pub const WINBIO_OPERATION_OPEN_FRAMEWORK: WINBIO_OPERATION_TYPE = 20;
pub const WINBIO_OPERATION_SET_PROPERTY: WINBIO_OPERATION_TYPE = 14;
pub type WINBIO_OPERATION_TYPE = u32;
pub const WINBIO_OPERATION_UNIT_ARRIVAL: WINBIO_OPERATION_TYPE = 25;
pub const WINBIO_OPERATION_UNIT_REMOVAL: WINBIO_OPERATION_TYPE = 26;
pub const WINBIO_OPERATION_UNLOCK_UNIT: WINBIO_OPERATION_TYPE = 17;
pub const WINBIO_OPERATION_VERIFY: WINBIO_OPERATION_TYPE = 3;
pub const WINBIO_OPERATION_VERIFY_AND_RELEASE_TICKET: WINBIO_OPERATION_TYPE = 28;
pub type WINBIO_ORIENTATION = u32;
pub const WINBIO_ORIENTATION_ANY: WINBIO_ORIENTATION = 3;
pub const WINBIO_ORIENTATION_LANDSCAPE: WINBIO_ORIENTATION = 1;
pub const WINBIO_ORIENTATION_PORTRAIT: WINBIO_ORIENTATION = 2;
pub const WINBIO_ORIENTATION_UNSPECIFIED: WINBIO_ORIENTATION = 0;
pub const WINBIO_PASSWORD_GENERIC: WINBIO_CREDENTIAL_FORMAT = 1;
pub const WINBIO_PASSWORD_PACKED: WINBIO_CREDENTIAL_FORMAT = 2;
pub const WINBIO_PASSWORD_PROTECTED: WINBIO_CREDENTIAL_FORMAT = 3;
pub const WINBIO_PATRON_HEADER_VERSION: WINBIO_BIR_VERSION = 17;
pub const WINBIO_POLICY_ADMIN: WINBIO_POLICY_SOURCE = 3;
pub const WINBIO_POLICY_DEFAULT: WINBIO_POLICY_SOURCE = 1;
pub const WINBIO_POLICY_LOCAL: WINBIO_POLICY_SOURCE = 2;
pub type WINBIO_POLICY_SOURCE = i32;
pub const WINBIO_POLICY_UNKNOWN: WINBIO_POLICY_SOURCE = 0;
pub const WINBIO_POOL_PRIVATE: WINBIO_POOL_TYPE = 2;
pub const WINBIO_POOL_SYSTEM: WINBIO_POOL_TYPE = 1;
pub type WINBIO_POOL_TYPE = u32;
pub const WINBIO_POOL_UNASSIGNED: WINBIO_POOL_TYPE = 3;
pub const WINBIO_POOL_UNKNOWN: WINBIO_POOL_TYPE = 0;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct WINBIO_PRESENCE {
    pub Factor: WINBIO_BIOMETRIC_TYPE,
    pub SubFactor: WINBIO_BIOMETRIC_SUBTYPE,
    pub Status: windows_sys::core::HRESULT,
    pub RejectDetail: WINBIO_REJECT_DETAIL,
    pub Identity: WINBIO_IDENTITY,
    pub TrackingId: u64,
    pub Ticket: WINBIO_PROTECTION_TICKET,
    pub Properties: WINBIO_PRESENCE_PROPERTIES,
    pub Authorization: WINBIO_PRESENCE_0,
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_PRESENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct WINBIO_PRESENCE_0 {
    pub Size: u32,
    pub Data: [u8; 32],
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_PRESENCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINBIO_PRESENCE_CHANGE = u32;
pub const WINBIO_PRESENCE_CHANGE_TYPE_ARRIVE: WINBIO_PRESENCE_CHANGE = 2;
pub const WINBIO_PRESENCE_CHANGE_TYPE_DEPART: WINBIO_PRESENCE_CHANGE = 4;
pub const WINBIO_PRESENCE_CHANGE_TYPE_RECOGNIZE: WINBIO_PRESENCE_CHANGE = 3;
pub const WINBIO_PRESENCE_CHANGE_TYPE_TRACK: WINBIO_PRESENCE_CHANGE = 5;
pub const WINBIO_PRESENCE_CHANGE_TYPE_UNKNOWN: WINBIO_PRESENCE_CHANGE = 0;
pub const WINBIO_PRESENCE_CHANGE_TYPE_UPDATE_ALL: WINBIO_PRESENCE_CHANGE = 1;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub union WINBIO_PRESENCE_PROPERTIES {
    pub FacialFeatures: WINBIO_PRESENCE_PROPERTIES_0,
    pub Iris: WINBIO_PRESENCE_PROPERTIES_1,
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_PRESENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_PRESENCE_PROPERTIES_0 {
    pub BoundingBox: super::windef::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_PRESENCE_PROPERTIES_0_0,
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct WINBIO_PRESENCE_PROPERTIES_0_0 {
    pub AdapterId: WINBIO_UUID,
    pub Data: [u32; 78],
}
#[cfg(feature = "Win32_windef")]
impl Default for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_PRESENCE_PROPERTIES_1 {
    pub EyeBoundingBox_1: super::windef::RECT,
    pub EyeBoundingBox_2: super::windef::RECT,
    pub PupilCenter_1: super::windef::POINT,
    pub PupilCenter_2: super::windef::POINT,
    pub Distance: i32,
}
pub const WINBIO_PROPERTY_ANTI_SPOOF_POLICY: WINBIO_PROPERTY_ID = 1;
pub const WINBIO_PROPERTY_EXTENDED_ENGINE_INFO: WINBIO_PROPERTY_ID = 3;
pub const WINBIO_PROPERTY_EXTENDED_ENROLLMENT_STATUS: WINBIO_PROPERTY_ID = 5;
pub const WINBIO_PROPERTY_EXTENDED_SENSOR_INFO: WINBIO_PROPERTY_ID = 2;
pub const WINBIO_PROPERTY_EXTENDED_STORAGE_INFO: WINBIO_PROPERTY_ID = 4;
pub const WINBIO_PROPERTY_EXTENDED_UNIT_STATUS: WINBIO_PROPERTY_ID = 6;
pub const WINBIO_PROPERTY_FP_BU_STATE: WINBIO_PROPERTY_ID = 8;
pub const WINBIO_PROPERTY_FP_IS_IMPROVING: WINBIO_PROPERTY_ID = 9;
pub type WINBIO_PROPERTY_ID = u32;
pub const WINBIO_PROPERTY_SAMPLE_HINT: WINBIO_PROPERTY_ID = 1;
pub type WINBIO_PROPERTY_TYPE = u32;
pub const WINBIO_PROPERTY_TYPE_ACCOUNT: WINBIO_PROPERTY_TYPE = 4;
pub const WINBIO_PROPERTY_TYPE_SESSION: WINBIO_PROPERTY_TYPE = 1;
pub const WINBIO_PROPERTY_TYPE_TEMPLATE: WINBIO_PROPERTY_TYPE = 3;
pub const WINBIO_PROPERTY_TYPE_UNIT: WINBIO_PROPERTY_TYPE = 2;
pub const WINBIO_PROPERTY_UNIT_SECURITY_LEVEL: WINBIO_PROPERTY_ID = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_PROTECTION_POLICY {
    pub Version: u32,
    pub Identity: WINBIO_IDENTITY,
    pub DatabaseId: WINBIO_UUID,
    pub UserState: u64,
    pub PolicySize: usize,
    pub Policy: [u8; 128],
}
impl Default for WINBIO_PROTECTION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINBIO_PROTECTION_SOFTWARE: WINBIO_PROTECTION_TYPE = 1;
pub type WINBIO_PROTECTION_TICKET = u64;
pub const WINBIO_PROTECTION_TRUSTED_EXECUTION_ENVIRONMENT: WINBIO_PROTECTION_TYPE = 2;
pub type WINBIO_PROTECTION_TYPE = u32;
pub const WINBIO_PURPOSE_AUDIT: WINBIO_BIR_PURPOSE = 128;
pub const WINBIO_PURPOSE_ENROLL: WINBIO_BIR_PURPOSE = 4;
pub const WINBIO_PURPOSE_ENROLL_FOR_IDENTIFICATION: WINBIO_BIR_PURPOSE = 16;
pub const WINBIO_PURPOSE_ENROLL_FOR_VERIFICATION: WINBIO_BIR_PURPOSE = 8;
pub const WINBIO_PURPOSE_IDENTIFY: WINBIO_BIR_PURPOSE = 2;
pub const WINBIO_PURPOSE_VERIFY: WINBIO_BIR_PURPOSE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_REGISTERED_FORMAT {
    pub Owner: u16,
    pub Type: u16,
}
pub type WINBIO_REJECT_DETAIL = u32;
pub const WINBIO_REJECT_DETAIL_ANTI_SPOOF_MASK: WINBIO_REJECT_DETAIL = 4278190080;
pub const WINBIO_REJECT_DETAIL_POSITION_MASK: WINBIO_REJECT_DETAIL = 16711680;
pub const WINBIO_REJECT_DETAIL_REASON_MASK: WINBIO_REJECT_DETAIL = 65535;
pub const WINBIO_SCP_CURVE_FIELD_SIZE_V1: u32 = 32;
pub const WINBIO_SCP_DIGEST_SIZE_V1: u32 = 32;
pub const WINBIO_SCP_ENCRYPTION_BLOCK_SIZE_V1: u32 = 16;
pub const WINBIO_SCP_ENCRYPTION_KEY_SIZE_V1: u32 = 32;
pub type WINBIO_SCP_FLAGS = u16;
pub const WINBIO_SCP_FLAG_RECONNECT: WINBIO_SCP_FLAGS = 1;
pub const WINBIO_SCP_PRIVATE_KEY_SIZE_V1: u32 = 32;
pub const WINBIO_SCP_PUBLIC_KEY_SIZE_V1: u32 = 65;
pub const WINBIO_SCP_RANDOM_SIZE_V1: u32 = 32;
pub const WINBIO_SCP_SIGNATURE_SIZE_V1: u32 = 64;
pub type WINBIO_SCP_VERSION = u16;
pub const WINBIO_SCP_VERSION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_SECURE_BUFFER_HEADER_V1 {
    pub Type: u32,
    pub Size: u32,
    pub Flags: u32,
    pub ValidationTag: u64,
}
pub const WINBIO_SECURE_BUFFER_TYPE_V1: u32 = 3116236801;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_SECURE_CONNECTION_DATA {
    pub Size: u32,
    pub Version: WINBIO_SCP_VERSION,
    pub Flags: WINBIO_SCP_FLAGS,
    pub ModelCertificateSize: u32,
    pub IntermediateCA1Size: u32,
    pub IntermediateCA2Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_SECURE_CONNECTION_PARAMS {
    pub PayloadSize: u32,
    pub Version: WINBIO_SCP_VERSION,
    pub Flags: WINBIO_SCP_FLAGS,
}
pub const WINBIO_SENSOR_ACCEPT: WINBIO_SENSOR_STATUS = 1;
pub const WINBIO_SENSOR_ADVANCED_MODE: WINBIO_SENSOR_MODE = 2;
pub const WINBIO_SENSOR_AVAILABLE: WINBIO_SENSOR_STATUS = 7;
pub const WINBIO_SENSOR_BASIC_MODE: WINBIO_SENSOR_MODE = 1;
pub const WINBIO_SENSOR_BUSY: WINBIO_SENSOR_STATUS = 4;
pub const WINBIO_SENSOR_FAILURE: WINBIO_SENSOR_STATUS = 6;
pub type WINBIO_SENSOR_MODE = u32;
pub const WINBIO_SENSOR_NAVIGATION_MODE: WINBIO_SENSOR_MODE = 3;
pub const WINBIO_SENSOR_NOT_CALIBRATED: WINBIO_SENSOR_STATUS = 5;
pub const WINBIO_SENSOR_READY: WINBIO_SENSOR_STATUS = 3;
pub const WINBIO_SENSOR_REJECT: WINBIO_SENSOR_STATUS = 2;
pub const WINBIO_SENSOR_SLEEP_MODE: WINBIO_SENSOR_MODE = 4;
pub type WINBIO_SENSOR_STATUS = u32;
pub const WINBIO_SENSOR_STATUS_UNKNOWN: WINBIO_SENSOR_STATUS = 0;
pub const WINBIO_SENSOR_SUBTYPE_UNKNOWN: WINBIO_BIOMETRIC_SENSOR_SUBTYPE = 0;
pub const WINBIO_SENSOR_UNAVAILABLE: WINBIO_SENSOR_STATUS = 8;
pub const WINBIO_SENSOR_UNKNOWN_MODE: WINBIO_SENSOR_MODE = 0;
pub type WINBIO_SESSION_FLAGS = u32;
pub type WINBIO_SESSION_HANDLE = u32;
pub const WINBIO_SETTING_SOURCE_DEFAULT: WINBIO_SETTING_SOURCE_TYPE = 1;
pub const WINBIO_SETTING_SOURCE_INVALID: WINBIO_SETTING_SOURCE_TYPE = 0;
pub const WINBIO_SETTING_SOURCE_LOCAL: WINBIO_SETTING_SOURCE_TYPE = 3;
pub const WINBIO_SETTING_SOURCE_POLICY: WINBIO_SETTING_SOURCE_TYPE = 2;
pub type WINBIO_SETTING_SOURCE_TYPE = u32;
pub type WINBIO_SETTING_TYPE = u32;
pub const WINBIO_SETTING_TYPE_ESS_ENABLED: WINBIO_SETTING_TYPE = 1;
pub const WINBIO_SETTING_TYPE_PERIPHERALS_WITH_ESS: WINBIO_SETTING_TYPE = 0;
pub const WINBIO_STANDARD_TYPE_MASK: WINBIO_BIOMETRIC_TYPE = 16777215;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_STORAGE_SCHEMA {
    pub BiometricFactor: WINBIO_BIOMETRIC_TYPE,
    pub DatabaseId: WINBIO_UUID,
    pub DataFormat: WINBIO_UUID,
    pub Attributes: u32,
    pub FilePath: WINBIO_STRING,
    pub ConnectionString: WINBIO_STRING,
}
impl Default for WINBIO_STORAGE_SCHEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINBIO_STRING = [u16; 256];
pub const WINBIO_SUBTYPE_ANY: WINBIO_BIOMETRIC_SUBTYPE = 255;
pub const WINBIO_SUBTYPE_NO_INFORMATION: WINBIO_BIOMETRIC_SUBTYPE = 0;
pub const WINBIO_TELEMETRY_AUTH: WINBIO_TELEMETRY_TYPE = 1;
pub const WINBIO_TELEMETRY_ENROLLMENT: WINBIO_TELEMETRY_TYPE = 2;
pub type WINBIO_TELEMETRY_TYPE = u32;
pub const WINBIO_TYPE_ANY: i32 = -1056964609;
pub const WINBIO_TYPE_DNA: WINBIO_BIOMETRIC_TYPE = 16384;
pub const WINBIO_TYPE_EAR_SHAPE: WINBIO_BIOMETRIC_TYPE = 32768;
pub const WINBIO_TYPE_FACIAL_FEATURES: WINBIO_BIOMETRIC_TYPE = 2;
pub const WINBIO_TYPE_FINGERPRINT: WINBIO_BIOMETRIC_TYPE = 8;
pub const WINBIO_TYPE_FINGER_GEOMETRY: WINBIO_BIOMETRIC_TYPE = 65536;
pub const WINBIO_TYPE_FOOT_PRINT: WINBIO_BIOMETRIC_TYPE = 524288;
pub const WINBIO_TYPE_GAIT: WINBIO_BIOMETRIC_TYPE = 4096;
pub const WINBIO_TYPE_HAND_GEOMETRY: WINBIO_BIOMETRIC_TYPE = 64;
pub const WINBIO_TYPE_IRIS: WINBIO_BIOMETRIC_TYPE = 16;
pub const WINBIO_TYPE_KEYSTROKE_DYNAMICS: WINBIO_BIOMETRIC_TYPE = 256;
pub const WINBIO_TYPE_LIP_MOVEMENT: WINBIO_BIOMETRIC_TYPE = 512;
pub const WINBIO_TYPE_MULTIPLE: WINBIO_BIOMETRIC_TYPE = 1;
pub const WINBIO_TYPE_OTHER: WINBIO_BIOMETRIC_TYPE = 1073741824;
pub const WINBIO_TYPE_PALM_PRINT: WINBIO_BIOMETRIC_TYPE = 131072;
pub const WINBIO_TYPE_PASSWORD: WINBIO_BIOMETRIC_TYPE = 2147483648;
pub const WINBIO_TYPE_RETINA: WINBIO_BIOMETRIC_TYPE = 32;
pub const WINBIO_TYPE_SCENT: WINBIO_BIOMETRIC_TYPE = 8192;
pub const WINBIO_TYPE_SIGNATURE_DYNAMICS: WINBIO_BIOMETRIC_TYPE = 128;
pub const WINBIO_TYPE_THERMAL_FACE_IMAGE: WINBIO_BIOMETRIC_TYPE = 1024;
pub const WINBIO_TYPE_THERMAL_HAND_IMAGE: WINBIO_BIOMETRIC_TYPE = 2048;
pub const WINBIO_TYPE_VEIN_PATTERN: WINBIO_BIOMETRIC_TYPE = 262144;
pub const WINBIO_TYPE_VOICE: WINBIO_BIOMETRIC_TYPE = 4;
pub type WINBIO_UNIT_ID = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINBIO_UNIT_SCHEMA {
    pub UnitId: WINBIO_UNIT_ID,
    pub PoolType: WINBIO_POOL_TYPE,
    pub BiometricFactor: WINBIO_BIOMETRIC_TYPE,
    pub SensorSubType: WINBIO_BIOMETRIC_SENSOR_SUBTYPE,
    pub Capabilities: WINBIO_CAPABILITIES,
    pub DeviceInstanceId: WINBIO_STRING,
    pub Description: WINBIO_STRING,
    pub Manufacturer: WINBIO_STRING,
    pub Model: WINBIO_STRING,
    pub SerialNumber: WINBIO_STRING,
    pub FirmwareVersion: WINBIO_VERSION,
}
impl Default for WINBIO_UNIT_SCHEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WINBIO_UNIT_SECURITY_LEVEL = u32;
pub const WINBIO_UNIT_SECURITY_LEVEL_NORMAL: WINBIO_UNIT_SECURITY_LEVEL = 0;
pub const WINBIO_UNIT_SECURITY_LEVEL_VBS: WINBIO_UNIT_SECURITY_LEVEL = 1;
pub type WINBIO_UUID = windows_sys::core::GUID;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINBIO_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
pub const WINBIO_VOICE_MAX_UTTERANCE: WINBIO_BIOMETRIC_SUBTYPE = 32;
pub const WINBIO_VOICE_MIN_UTTERANCE: WINBIO_BIOMETRIC_SUBTYPE = 1;
pub const WINBIO_VOICE_NO_KEYWORD: WINBIO_REJECT_DETAIL = 4;
pub const WINBIO_VOICE_POOR_QUALITY: WINBIO_REJECT_DETAIL = 1;
pub const WINBIO_VOICE_PROCESSING_ERROR: WINBIO_REJECT_DETAIL = 5;
pub const WINBIO_VOICE_TOO_FAST: WINBIO_REJECT_DETAIL = 3;
pub const WINBIO_VOICE_TOO_SLOW: WINBIO_REJECT_DETAIL = 2;
pub const WINBIO_VOICE_TYPE_UNKNOWN: WINBIO_BIOMETRIC_SUBTYPE = 0;
pub type WINBIO_WAKE_REASON = u32;
pub const WINBIO_WAKE_REASON_TOUCH: WINBIO_WAKE_REASON = 1;
pub const WINBIO_WAKE_REASON_UNKNOWN: WINBIO_WAKE_REASON = 0;
