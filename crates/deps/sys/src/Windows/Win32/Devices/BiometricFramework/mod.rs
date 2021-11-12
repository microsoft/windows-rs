#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn WinBioAcquireFocus() -> ::windows_sys::core::HRESULT;
    pub fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioAsyncOpenFramework(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: *const ::core::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, frameworkhandle: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioAsyncOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows_sys::core::GUID, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: *const ::core::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, sessionhandle: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioCancel(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: *mut u32, sample: *mut *mut WINBIO_BIR, samplesize: *mut usize, rejectdetail: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: PWINBIO_CAPTURE_CALLBACK, capturecallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WinBioCloseFramework(frameworkhandle: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioCloseSession(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnrollCapture(sessionhandle: u32, rejectdetail: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: PWINBIO_ENROLL_CAPTURE_CALLBACK, enrollcallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnrollCommit(sessionhandle: u32, identity: *mut WINBIO_IDENTITY, isnewtemplate: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnrollDiscard(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: *mut usize) -> ::windows_sys::core::HRESULT;
    pub fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> ::windows_sys::core::HRESULT;
    pub fn WinBioFree(address: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WinBioGetCredentialState(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE, credentialstate: *mut WINBIO_CREDENTIAL_STATE) -> ::windows_sys::core::HRESULT;
    pub fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    pub fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    pub fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY, enrolledfactors: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
    pub fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::core::ffi::c_void, propertybuffersize: *mut usize) -> ::windows_sys::core::HRESULT;
    pub fn WinBioIdentify(sessionhandle: u32, unitid: *mut u32, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: PWINBIO_IDENTIFY_CALLBACK, identifycallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioImproveEnd(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioLocateSensor(sessionhandle: u32, unitid: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: PWINBIO_LOCATE_SENSOR_CALLBACK, locatecallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows_sys::core::GUID, sessionhandle: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: PWINBIO_EVENT_CALLBACK, eventcallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WinBioReleaseFocus() -> ::windows_sys::core::HRESULT;
    pub fn WinBioRemoveAllCredentials() -> ::windows_sys::core::HRESULT;
    pub fn WinBioRemoveAllDomainCredentials() -> ::windows_sys::core::HRESULT;
    pub fn WinBioRemoveCredential(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows_sys::core::HRESULT;
    pub fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: *const u8, credentialsize: usize, format: WINBIO_CREDENTIAL_FORMAT) -> ::windows_sys::core::HRESULT;
    pub fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *const ::core::ffi::c_void, propertybuffersize: usize) -> ::windows_sys::core::HRESULT;
    pub fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
    pub fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: *mut u32, r#match: *mut u8, rejectdetail: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: PWINBIO_VERIFY_CALLBACK, verifycallbackcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WinBioWait(sessionhandle: u32) -> ::windows_sys::core::HRESULT;
}
pub const FACILITY_NONE: u32 = 0u32;
pub const FACILITY_WINBIO: u32 = 9u32;
pub const GUID_DEVINTERFACE_BIOMETRIC_READER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3803519034,
    data2: 39402,
    data3: 19651,
    data4: [173, 107, 128, 202, 141, 113, 91, 128],
};
pub const IOCTL_BIOMETRIC_VENDOR: u32 = 4464640u32;
#[repr(C)]
pub struct PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_ACTIVATE_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_ATTACH_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_CLEAR_CONTEXT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_COMMIT_ENROLLMENT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_CONTROL_UNIT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_CREATE_ENROLLMENT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_CREATE_KEY_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_DEACTIVATE_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_DETACH_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_DISCARD_ENROLLMENT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_IDENTIFY_ALL_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_PIPELINE_CLEANUP_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_PIPELINE_INIT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_REFRESH_CACHE_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_RESERVED_1_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_SET_HASH_ALGORITHM_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_UPDATE_ENROLLMENT_FN(i32);
#[repr(C)]
pub struct PIBIO_ENGINE_VERIFY_FEATURE_SET_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_FREE_MEMORY_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_GET_PROPERTY_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN(i32);
#[repr(C)]
pub struct PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_ACTIVATE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_ATTACH_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_CANCEL_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_CLEAR_CONTEXT_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_CONNECT_SECURE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_CONTROL_UNIT_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_DEACTIVATE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_DETACH_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_FINISH_CAPTURE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_GET_INDICATOR_STATUS_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_PIPELINE_CLEANUP_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_PIPELINE_INIT_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_QUERY_STATUS_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_RESET_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_SET_INDICATOR_STATUS_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_SET_MODE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_START_CAPTURE_EX_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_START_CAPTURE_FN(i32);
#[repr(C)]
pub struct PIBIO_SENSOR_START_NOTIFY_WAKE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_ACTIVATE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_ADD_RECORD_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_ATTACH_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_CLEAR_CONTEXT_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_CLOSE_DATABASE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_CONTROL_UNIT_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_CREATE_DATABASE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_DEACTIVATE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_DELETE_RECORD_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_DETACH_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_ERASE_DATABASE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_FIRST_RECORD_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_GET_CURRENT_RECORD_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_GET_DATABASE_SIZE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_GET_DATA_FORMAT_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_GET_RECORD_COUNT_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_NEXT_RECORD_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_OPEN_DATABASE_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_PIPELINE_CLEANUP_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_PIPELINE_INIT_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_QUERY_BY_CONTENT_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_QUERY_BY_SUBJECT_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_RESERVED_1_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_RESERVED_2_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN(i32);
#[repr(C)]
pub struct PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN(i32);
#[repr(C)]
pub struct PWINBIO_ASYNC_COMPLETION_CALLBACK(i32);
#[repr(C)]
pub struct PWINBIO_CAPTURE_CALLBACK(i32);
#[repr(C)]
pub struct PWINBIO_ENROLL_CAPTURE_CALLBACK(i32);
#[repr(C)]
pub struct PWINBIO_EVENT_CALLBACK(i32);
#[repr(C)]
pub struct PWINBIO_IDENTIFY_CALLBACK(i32);
#[repr(C)]
pub struct PWINBIO_LOCATE_SENSOR_CALLBACK(i32);
#[repr(C)]
pub struct PWINBIO_QUERY_ENGINE_INTERFACE_FN(i32);
#[repr(C)]
pub struct PWINBIO_QUERY_SENSOR_INTERFACE_FN(i32);
#[repr(C)]
pub struct PWINBIO_QUERY_STORAGE_INTERFACE_FN(i32);
#[repr(C)]
pub struct PWINBIO_VERIFY_CALLBACK(i32);
#[repr(C)]
pub struct WINBIO_ACCOUNT_POLICY(i32);
#[repr(C)]
pub struct WINBIO_ADAPTER_INTERFACE_VERSION(i32);
#[repr(C)]
pub struct WINBIO_ANTI_SPOOF_POLICY(i32);
#[repr(C)]
pub struct WINBIO_ANTI_SPOOF_POLICY_ACTION(i32);
#[repr(C)]
pub struct WINBIO_ASYNC_NOTIFICATION_METHOD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINBIO_ASYNC_RESULT(i32);
#[repr(C)]
pub struct WINBIO_BDB_ANSI_381_HEADER(i32);
#[repr(C)]
pub struct WINBIO_BDB_ANSI_381_RECORD(i32);
#[repr(C)]
pub struct WINBIO_BIR(i32);
pub const WINBIO_BIR_ALGIN_SIZE: u32 = 8u32;
pub const WINBIO_BIR_ALIGN_SIZE: u32 = 8u32;
#[repr(C)]
pub struct WINBIO_BIR_DATA(i32);
#[repr(C)]
pub struct WINBIO_BIR_HEADER(i32);
#[repr(C)]
pub struct WINBIO_BLANK_PAYLOAD(i32);
#[repr(C)]
pub struct WINBIO_BSP_SCHEMA(i32);
#[repr(C)]
pub struct WINBIO_CALIBRATION_INFO(i32);
#[repr(C)]
pub struct WINBIO_CAPTURE_DATA(i32);
#[repr(C)]
pub struct WINBIO_CAPTURE_PARAMETERS(i32);
#[repr(C)]
pub struct WINBIO_COMPONENT(i32);
#[repr(C)]
pub struct WINBIO_CREDENTIAL_FORMAT(i32);
#[repr(C)]
pub struct WINBIO_CREDENTIAL_STATE(i32);
#[repr(C)]
pub struct WINBIO_CREDENTIAL_TYPE(i32);
#[repr(C)]
pub struct WINBIO_DATA(i32);
#[repr(C)]
pub struct WINBIO_DIAGNOSTICS(i32);
#[repr(C)]
pub struct WINBIO_ENCRYPTED_CAPTURE_PARAMS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct WINBIO_ENGINE_INTERFACE(i32);
#[repr(C)]
pub struct WINBIO_EVENT(i32);
#[repr(C)]
pub struct WINBIO_EXTENDED_ENGINE_INFO(i32);
#[repr(C)]
pub struct WINBIO_EXTENDED_ENROLLMENT_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINBIO_EXTENDED_SENSOR_INFO(i32);
#[repr(C)]
pub struct WINBIO_EXTENDED_STORAGE_INFO(i32);
#[repr(C)]
pub struct WINBIO_EXTENDED_UNIT_STATUS(i32);
pub const WINBIO_E_ADAPTER_INTEGRITY_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860995i32 as _);
pub const WINBIO_E_AUTO_LOGON_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860989i32 as _);
pub const WINBIO_E_BAD_CAPTURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861048i32 as _);
pub const WINBIO_E_CALIBRATION_BUFFER_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860975i32 as _);
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_LARGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860976i32 as _);
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860977i32 as _);
pub const WINBIO_E_CANCELED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861052i32 as _);
pub const WINBIO_E_CAPTURE_ABORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861050i32 as _);
pub const WINBIO_E_CONFIGURATION_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861005i32 as _);
pub const WINBIO_E_CRED_PROV_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861008i32 as _);
pub const WINBIO_E_CRED_PROV_NO_CREDENTIAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861007i32 as _);
pub const WINBIO_E_CRED_PROV_SECURITY_LOCKOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860985i32 as _);
pub const WINBIO_E_DATABASE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861034i32 as _);
pub const WINBIO_E_DATABASE_BAD_INDEX_VECTOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861022i32 as _);
pub const WINBIO_E_DATABASE_CANT_CLOSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861037i32 as _);
pub const WINBIO_E_DATABASE_CANT_CREATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861039i32 as _);
pub const WINBIO_E_DATABASE_CANT_ERASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861036i32 as _);
pub const WINBIO_E_DATABASE_CANT_FIND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861035i32 as _);
pub const WINBIO_E_DATABASE_CANT_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861038i32 as _);
pub const WINBIO_E_DATABASE_CORRUPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861030i32 as _);
pub const WINBIO_E_DATABASE_EOF: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861023i32 as _);
pub const WINBIO_E_DATABASE_FULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861032i32 as _);
pub const WINBIO_E_DATABASE_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861031i32 as _);
pub const WINBIO_E_DATABASE_NO_MORE_RECORDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861024i32 as _);
pub const WINBIO_E_DATABASE_NO_RESULTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861025i32 as _);
pub const WINBIO_E_DATABASE_NO_SUCH_RECORD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861029i32 as _);
pub const WINBIO_E_DATABASE_READ_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861027i32 as _);
pub const WINBIO_E_DATABASE_WRITE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861026i32 as _);
pub const WINBIO_E_DATA_COLLECTION_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861045i32 as _);
pub const WINBIO_E_DATA_PROTECTION_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860986i32 as _);
pub const WINBIO_E_DEADLOCK_DETECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860992i32 as _);
pub const WINBIO_E_DEVICE_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861040i32 as _);
pub const WINBIO_E_DEVICE_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861002i32 as _);
pub const WINBIO_E_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861006i32 as _);
pub const WINBIO_E_DUPLICATE_ENROLLMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861028i32 as _);
pub const WINBIO_E_DUPLICATE_TEMPLATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861013i32 as _);
pub const WINBIO_E_ENROLLMENT_CANCELED_BY_SUSPEND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860965i32 as _);
pub const WINBIO_E_ENROLLMENT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861049i32 as _);
pub const WINBIO_E_EVENT_MONITOR_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860999i32 as _);
pub const WINBIO_E_FAST_USER_SWITCH_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861001i32 as _);
pub const WINBIO_E_INCORRECT_BSP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861020i32 as _);
pub const WINBIO_E_INCORRECT_SENSOR_POOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861019i32 as _);
pub const WINBIO_E_INCORRECT_SESSION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860994i32 as _);
pub const WINBIO_E_INSECURE_SENSOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860969i32 as _);
pub const WINBIO_E_INVALID_BUFFER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860967i32 as _);
pub const WINBIO_E_INVALID_BUFFER_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860968i32 as _);
pub const WINBIO_E_INVALID_CALIBRATION_FORMAT_ARRAY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860980i32 as _);
pub const WINBIO_E_INVALID_CONTROL_CODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861047i32 as _);
pub const WINBIO_E_INVALID_DEVICE_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861041i32 as _);
pub const WINBIO_E_INVALID_KEY_IDENTIFIER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860974i32 as _);
pub const WINBIO_E_INVALID_OPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861012i32 as _);
pub const WINBIO_E_INVALID_PROPERTY_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860997i32 as _);
pub const WINBIO_E_INVALID_PROPERTY_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860998i32 as _);
pub const WINBIO_E_INVALID_SENSOR_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861017i32 as _);
pub const WINBIO_E_INVALID_SUBFACTOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860981i32 as _);
pub const WINBIO_E_INVALID_TICKET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860988i32 as _);
pub const WINBIO_E_INVALID_UNIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861054i32 as _);
pub const WINBIO_E_KEY_CREATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860973i32 as _);
pub const WINBIO_E_KEY_IDENTIFIER_BUFFER_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860972i32 as _);
pub const WINBIO_E_LOCK_VIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861014i32 as _);
pub const WINBIO_E_MAX_ERROR_COUNT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860990i32 as _);
pub const WINBIO_E_NOT_ACTIVE_CONSOLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861000i32 as _);
pub const WINBIO_E_NO_CAPTURE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861018i32 as _);
pub const WINBIO_E_NO_MATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861051i32 as _);
pub const WINBIO_E_NO_PREBOOT_IDENTITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860991i32 as _);
pub const WINBIO_E_NO_SUPPORTED_CALIBRATION_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860979i32 as _);
pub const WINBIO_E_POLICY_PROTECTION_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860970i32 as _);
pub const WINBIO_E_PRESENCE_MONITOR_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860982i32 as _);
pub const WINBIO_E_PROPERTY_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860971i32 as _);
pub const WINBIO_E_SAS_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861003i32 as _);
pub const WINBIO_E_SELECTION_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860983i32 as _);
pub const WINBIO_E_SENSOR_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861004i32 as _);
pub const WINBIO_E_SESSION_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861011i32 as _);
pub const WINBIO_E_SESSION_HANDLE_CLOSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860993i32 as _);
pub const WINBIO_E_TICKET_QUOTA_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860987i32 as _);
pub const WINBIO_E_TRUSTLET_INTEGRITY_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860966i32 as _);
pub const WINBIO_E_UNKNOWN_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861053i32 as _);
pub const WINBIO_E_UNSUPPORTED_DATA_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861044i32 as _);
pub const WINBIO_E_UNSUPPORTED_DATA_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861043i32 as _);
pub const WINBIO_E_UNSUPPORTED_FACTOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861055i32 as _);
pub const WINBIO_E_UNSUPPORTED_POOL_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860984i32 as _);
pub const WINBIO_E_UNSUPPORTED_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860996i32 as _);
pub const WINBIO_E_UNSUPPORTED_PURPOSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146861042i32 as _);
pub const WINBIO_E_UNSUPPORTED_SENSOR_CALIBRATION_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2146860978i32 as _);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINBIO_FP_BU_STATE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct WINBIO_FRAMEWORK_INTERFACE(i32);
#[repr(C)]
pub struct WINBIO_GESTURE_METADATA(i32);
#[repr(C)]
pub struct WINBIO_GET_INDICATOR(i32);
#[repr(C)]
pub struct WINBIO_IDENTITY(i32);
pub const WINBIO_I_EXTENDED_STATUS_INFORMATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(589826i32 as _);
pub const WINBIO_I_MORE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(589825i32 as _);
pub const WINBIO_MAX_STRING_LEN: u32 = 256u32;
#[repr(C)]
pub struct WINBIO_NOTIFY_WAKE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct WINBIO_PIPELINE(i32);
#[repr(C)]
pub struct WINBIO_POLICY_SOURCE(i32);
#[repr(C)]
pub struct WINBIO_POOL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINBIO_PRESENCE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINBIO_PRESENCE_PROPERTIES(i32);
#[repr(C)]
pub struct WINBIO_PRIVATE_SENSOR_TYPE_INFO(i32);
#[repr(C)]
pub struct WINBIO_PROTECTION_POLICY(i32);
#[repr(C)]
pub struct WINBIO_REGISTERED_FORMAT(i32);
pub const WINBIO_SCP_CURVE_FIELD_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_DIGEST_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_ENCRYPTION_BLOCK_SIZE_V1: u32 = 16u32;
pub const WINBIO_SCP_ENCRYPTION_KEY_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_PRIVATE_KEY_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_PUBLIC_KEY_SIZE_V1: u32 = 65u32;
pub const WINBIO_SCP_RANDOM_SIZE_V1: u32 = 32u32;
pub const WINBIO_SCP_SIGNATURE_SIZE_V1: u32 = 64u32;
pub const WINBIO_SCP_VERSION_1: u32 = 1u32;
#[repr(C)]
pub struct WINBIO_SECURE_BUFFER_HEADER_V1(i32);
#[repr(C)]
pub struct WINBIO_SECURE_CONNECTION_DATA(i32);
#[repr(C)]
pub struct WINBIO_SECURE_CONNECTION_PARAMS(i32);
#[repr(C)]
pub struct WINBIO_SENSOR_ATTRIBUTES(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct WINBIO_SENSOR_INTERFACE(i32);
#[repr(C)]
pub struct WINBIO_SETTING_SOURCE(i32);
#[repr(C)]
pub struct WINBIO_SET_INDICATOR(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct WINBIO_STORAGE_INTERFACE(i32);
#[repr(C)]
pub struct WINBIO_STORAGE_RECORD(i32);
#[repr(C)]
pub struct WINBIO_STORAGE_SCHEMA(i32);
#[repr(C)]
pub struct WINBIO_SUPPORTED_ALGORITHMS(i32);
#[repr(C)]
pub struct WINBIO_UNIT_SCHEMA(i32);
#[repr(C)]
pub struct WINBIO_UPDATE_FIRMWARE(i32);
#[repr(C)]
pub struct WINBIO_VERSION(i32);
pub const WINBIO_WBDI_MAJOR_VERSION: u32 = 1u32;
pub const WINBIO_WBDI_MINOR_VERSION: u32 = 0u32;
#[repr(C)]
pub struct _WINIBIO_ENGINE_CONTEXT(i32);
#[repr(C)]
pub struct _WINIBIO_SENSOR_CONTEXT(i32);
#[repr(C)]
pub struct _WINIBIO_STORAGE_CONTEXT(i32);
