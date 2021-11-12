#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const FACILITY_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const FACILITY_WINBIO: u32 = 9u32;
pub const GUID_DEVINTERFACE_BIOMETRIC_READER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3803519034,
    data2: 39402,
    data3: 19651,
    data4: [173, 107, 128, 202, 141, 113, 91, 128],
};
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const IOCTL_BIOMETRIC_VENDOR: u32 = 4464640u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_BIR_ALGIN_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_BIR_ALIGN_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_ADAPTER_INTEGRITY_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860995i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_AUTO_LOGON_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860989i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_BAD_CAPTURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861048i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860975i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_LARGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860976i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860977i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CANCELED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861052i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CAPTURE_ABORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861050i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CONFIGURATION_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861005i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CRED_PROV_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861008i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CRED_PROV_NO_CREDENTIAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861007i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CRED_PROV_SECURITY_LOCKOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860985i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861034i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_BAD_INDEX_VECTOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861022i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_CLOSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861037i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_CREATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861039i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_ERASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861036i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_FIND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861035i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861038i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CORRUPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861030i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_EOF: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861023i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_FULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861032i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861031i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_NO_MORE_RECORDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861024i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_NO_RESULTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861025i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_NO_SUCH_RECORD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861029i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_READ_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861027i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_WRITE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861026i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATA_COLLECTION_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861045i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATA_PROTECTION_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860986i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DEADLOCK_DETECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860992i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DEVICE_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861040i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DEVICE_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861002i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861006i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DUPLICATE_ENROLLMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861028i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DUPLICATE_TEMPLATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861013i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_ENROLLMENT_CANCELED_BY_SUSPEND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860965i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_ENROLLMENT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861049i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_EVENT_MONITOR_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860999i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_FAST_USER_SWITCH_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861001i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INCORRECT_BSP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861020i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INCORRECT_SENSOR_POOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861019i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INCORRECT_SESSION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860994i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INSECURE_SENSOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860969i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_BUFFER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860967i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_BUFFER_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860968i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_CALIBRATION_FORMAT_ARRAY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860980i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_CONTROL_CODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861047i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_DEVICE_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861041i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_KEY_IDENTIFIER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860974i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_OPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861012i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_PROPERTY_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860997i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_PROPERTY_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860998i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_SENSOR_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861017i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_SUBFACTOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860981i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_TICKET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860988i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_UNIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861054i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_KEY_CREATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860973i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_KEY_IDENTIFIER_BUFFER_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860972i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_LOCK_VIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861014i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_MAX_ERROR_COUNT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860990i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NOT_ACTIVE_CONSOLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861000i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NO_CAPTURE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861018i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NO_MATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861051i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NO_PREBOOT_IDENTITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860991i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NO_SUPPORTED_CALIBRATION_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860979i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_POLICY_PROTECTION_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860970i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_PRESENCE_MONITOR_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860982i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_PROPERTY_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860971i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SAS_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861003i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SELECTION_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860983i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SENSOR_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861004i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SESSION_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861011i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SESSION_HANDLE_CLOSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860993i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_TICKET_QUOTA_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860987i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_TRUSTLET_INTEGRITY_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860966i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNKNOWN_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861053i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_DATA_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861044i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_DATA_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861043i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_FACTOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861055i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_POOL_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860984i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860996i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_PURPOSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861042i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_SENSOR_CALIBRATION_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860978i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_I_EXTENDED_STATUS_INFORMATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(589826i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_I_MORE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(589825i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_MAX_STRING_LEN: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_CURVE_FIELD_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_DIGEST_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_ENCRYPTION_BLOCK_SIZE_V1: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_ENCRYPTION_KEY_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_PRIVATE_KEY_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_PUBLIC_KEY_SIZE_V1: u32 = 65u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_RANDOM_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_SIGNATURE_SIZE_V1: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_SCP_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_WBDI_MAJOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_WBDI_MINOR_VERSION: u32 = 0u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAcquireFocus() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioAsyncOpenFramework(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: *const ::core::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, frameworkhandle: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioAsyncOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows_sys::core::GUID, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: *const ::core::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, sessionhandle: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCancel(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: *mut u32, sample: *mut *mut WINBIO_BIR, samplesize: *mut usize, rejectdetail: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: PWINBIO_CAPTURE_CALLBACK, capturecallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCloseFramework(frameworkhandle: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCloseSession(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCapture(sessionhandle: u32, rejectdetail: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: PWINBIO_ENROLL_CAPTURE_CALLBACK, enrollcallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCommit(sessionhandle: u32, identity: *mut WINBIO_IDENTITY, isnewtemplate: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollDiscard(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioFree(address: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetCredentialState(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE, credentialstate: *mut WINBIO_CREDENTIAL_STATE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY, enrolledfactors: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::core::ffi::c_void, propertybuffersize: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioIdentify(sessionhandle: u32, unitid: *mut u32, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: PWINBIO_IDENTIFY_CALLBACK, identifycallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioImproveEnd(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLocateSensor(sessionhandle: u32, unitid: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: PWINBIO_LOCATE_SENSOR_CALLBACK, locatecallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows_sys::core::GUID, sessionhandle: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: PWINBIO_EVENT_CALLBACK, eventcallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioReleaseFocus() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveAllCredentials() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveAllDomainCredentials() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveCredential(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: *const u8, credentialsize: usize, format: WINBIO_CREDENTIAL_FORMAT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *const ::core::ffi::c_void, propertybuffersize: usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: *mut u32, r#match: *mut u8, rejectdetail: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: PWINBIO_VERIFY_CALLBACK, verifycallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioWait(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
}
