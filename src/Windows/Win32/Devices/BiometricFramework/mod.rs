#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const FACILITY_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const FACILITY_WINBIO: u32 = 9u32;
pub const GUID_DEVINTERFACE_BIOMETRIC_READER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3803519034, 39402, 19651, [173, 107, 128, 202, 141, 113, 91, 128]);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const IOCTL_BIOMETRIC_VENDOR: u32 = 4464640u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, typeinfobufferaddress: *const u8, typeinfobuffersize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplebuffer: *const WINBIO_BIR, samplesize: usize, purpose: u8, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACTIVATE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ATTACH_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, duplicate: *mut super::super::Foundation::BOOLEAN) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CLEAR_CONTEXT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_COMMIT_ENROLLMENT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, payloadblob: *const u8, payloadblobsize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CONTROL_UNIT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *mut *mut u8, noncesize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_ENROLLMENT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_KEY_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, key: *const u8, keysize: usize, keyidentifier: *mut u8, keyidentifiersize: usize, resultsize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DEACTIVATE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DETACH_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DISCARD_ENROLLMENT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, flags: u8, samplebuffer: *mut *mut WINBIO_BIR, samplesize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, hashvalue: *mut *mut u8, hashsize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_ALL_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, presencecount: *mut usize, presencearray: *mut *mut WINBIO_PRESENCE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *const u8, noncesize: usize, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32, authentication: *mut *mut u8, authenticationsize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, payloadblob: *mut *mut u8, payloadblobsize: *mut usize, hashvalue: *mut *mut u8, hashsize: *mut usize, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *const u8, noncesize: usize, keyidentifier: *const u8, keyidentifiersize: usize, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32, authorization: *mut *mut u8, authorizationsize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_PIPELINE_CLEANUP_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_PIPELINE_INIT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, discardandrepeatcapture: *mut super::super::Foundation::BOOLEAN, calibrationbuffer: *mut u8, calibrationbuffersize: *mut usize, maxbuffersize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, enrollmentstatus: *mut WINBIO_EXTENDED_ENROLLMENT_STATUS, enrollmentstatussize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, engineinfo: *mut WINBIO_EXTENDED_ENGINE_INFO, engineinfosize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, algorithmcount: *mut usize, algorithmbuffersize: *mut usize, algorithmbuffer: *mut *mut u8) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indexelementcount: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, standardformat: *mut WINBIO_REGISTERED_FORMAT, vendorformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplehint: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_REFRESH_CACHE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_RESERVED_1_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, formatarray: *const ::windows::runtime::GUID, formatcount: usize, selectedformat: *mut ::windows::runtime::GUID, maxbuffersize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, policyitemarray: *const WINBIO_ACCOUNT_POLICY, policyitemcount: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, parameters: *const WINBIO_EXTENDED_ENROLLMENT_PARAMETERS) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, selectorvalue: u64) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_HASH_ALGORITHM_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, algorithmbuffersize: usize, algorithmbuffer: *const u8) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_UPDATE_ENROLLMENT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_VERIFY_FEATURE_SET_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, r#match: *mut super::super::Foundation::BOOLEAN, payloadblob: *mut *mut u8, payloadblobsize: *mut usize, hashvalue: *mut *mut u8, hashsize: *mut usize, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, allocationsize: usize, address: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_FREE_MEMORY_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, address: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_GET_PROPERTY_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, propertytype: u32, propertyid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::std::ffi::c_void, propertybuffersize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::runtime::GUID, securebufferaddress: *mut *mut ::std::ffi::c_void, securebuffersize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, extendedstatus: *const WINBIO_EXTENDED_UNIT_STATUS, extendedstatussize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, requiredcapacity: *mut usize, maxbuffersize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, bufferaddress: *mut u8, buffersize: usize, returneddatasize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, requiredcapacity: usize, maxbuffersize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, bufferaddress: *const u8, buffersize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, authentication: *const u8, authenticationsize: usize, iv: *const u8, ivsize: usize, encrypteddata: *mut u8, encrypteddatasize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, secureidentitycount: *mut usize, secureidentities: *mut *mut WINBIO_IDENTITY) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, reserved1: usize, reserved2: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, reserved1: *mut u8, reserved2: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, calibrationbuffer: *const u8, calibrationbuffersize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ACTIVATE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rawbufferaddress: *const u8, rawbuffersize: usize, resultbufferaddress: *mut *mut u8, resultbuffersize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::runtime::GUID, metadatabufferaddress: *const u8, metadatabuffersize: usize, resultbufferaddress: *mut *mut u8, resultbuffersize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ATTACH_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CANCEL_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CLEAR_CONTEXT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONNECT_SECURE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, connectionparams: *const WINBIO_SECURE_CONNECTION_PARAMS, connectiondata: *mut *mut WINBIO_SECURE_CONNECTION_DATA) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONTROL_UNIT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_DEACTIVATE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_DETACH_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplebuffer: *mut *mut WINBIO_BIR, samplesize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_FINISH_CAPTURE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, reason: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_GET_INDICATOR_STATUS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indicatorstatus: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PIPELINE_CLEANUP_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PIPELINE_INIT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, flags: u8, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, formatarray: *mut ::windows::runtime::GUID, formatarraysize: usize, formatcount: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, sensorinfo: *mut WINBIO_EXTENDED_SENSOR_INFO, sensorinfosize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, typeinfobufferaddress: *mut u8, typeinfobuffersize: usize, typeinfodatasize: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_STATUS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, status: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_RESET_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, format: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_INDICATOR_STATUS_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indicatorstatus: u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_MODE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, mode: u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_CAPTURE_EX_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, nonce: *const u8, noncesize: usize, flags: u8, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_CAPTURE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_NOTIFY_WAKE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ACTIVATE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ADD_RECORD_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *const WINBIO_STORAGE_RECORD) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ATTACH_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CLEAR_CONTEXT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CLOSE_DATABASE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CONTROL_UNIT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CREATE_DATABASE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::runtime::GUID, factor: u32, format: *const ::windows::runtime::GUID, filepath: super::super::Foundation::PWSTR, connectstring: super::super::Foundation::PWSTR, indexelementcount: usize, initialsize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DEACTIVATE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DELETE_RECORD_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DETACH_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ERASE_DATABASE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::runtime::GUID, filepath: super::super::Foundation::PWSTR, connectstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_FIRST_RECORD_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_CURRENT_RECORD_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *mut WINBIO_STORAGE_RECORD) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_DATABASE_SIZE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, availablerecordcount: *mut usize, totalrecordcount: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_DATA_FORMAT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, format: *mut ::windows::runtime::GUID, version: *mut WINBIO_VERSION) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_RECORD_COUNT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcount: *mut usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NEXT_RECORD_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordsadded: super::super::Foundation::BOOLEAN) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_OPEN_DATABASE_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::runtime::GUID, filepath: super::super::Foundation::PWSTR, connectstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_PIPELINE_CLEANUP_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_PIPELINE_INIT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_BY_CONTENT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, subfactor: u8, indexvector: *const u32, indexelementcount: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_BY_SUBJECT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, storageinfo: *mut WINBIO_EXTENDED_STORAGE_INFO, storageinfosize: usize) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_RESERVED_1_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, reserved1: *mut u64, reserved2: *mut u64) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_RESERVED_2_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, recordcontents: *mut WINBIO_STORAGE_RECORD) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN = unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *const WINBIO_STORAGE_RECORD) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWINBIO_ASYNC_COMPLETION_CALLBACK = unsafe extern "system" fn(asyncresult: *const WINBIO_ASYNC_RESULT);
pub type PWINBIO_CAPTURE_CALLBACK = unsafe extern "system" fn(capturecallbackcontext: *const ::std::ffi::c_void, operationstatus: ::windows::runtime::HRESULT, unitid: u32, sample: *const WINBIO_BIR, samplesize: usize, rejectdetail: u32);
pub type PWINBIO_ENROLL_CAPTURE_CALLBACK = unsafe extern "system" fn(enrollcallbackcontext: *const ::std::ffi::c_void, operationstatus: ::windows::runtime::HRESULT, rejectdetail: u32);
pub type PWINBIO_EVENT_CALLBACK = unsafe extern "system" fn(eventcallbackcontext: *const ::std::ffi::c_void, operationstatus: ::windows::runtime::HRESULT, event: *const WINBIO_EVENT);
pub type PWINBIO_IDENTIFY_CALLBACK = unsafe extern "system" fn(identifycallbackcontext: *const ::std::ffi::c_void, operationstatus: ::windows::runtime::HRESULT, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, rejectdetail: u32);
pub type PWINBIO_LOCATE_SENSOR_CALLBACK = unsafe extern "system" fn(locatecallbackcontext: *const ::std::ffi::c_void, operationstatus: ::windows::runtime::HRESULT, unitid: u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_ENGINE_INTERFACE_FN = unsafe extern "system" fn(engineinterface: *mut *mut WINBIO_ENGINE_INTERFACE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_SENSOR_INTERFACE_FN = unsafe extern "system" fn(sensorinterface: *mut *mut WINBIO_SENSOR_INTERFACE) -> ::windows::runtime::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_STORAGE_INTERFACE_FN = unsafe extern "system" fn(storageinterface: *mut *mut WINBIO_STORAGE_INTERFACE) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWINBIO_VERIFY_CALLBACK = unsafe extern "system" fn(verifycallbackcontext: *const ::std::ffi::c_void, operationstatus: ::windows::runtime::HRESULT, unitid: u32, r#match: super::super::Foundation::BOOLEAN, rejectdetail: u32);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ACCOUNT_POLICY {
    pub Identity: WINBIO_IDENTITY,
    pub AntiSpoofBehavior: WINBIO_ANTI_SPOOF_POLICY_ACTION,
}
impl WINBIO_ACCOUNT_POLICY {}
impl ::std::default::Default for WINBIO_ACCOUNT_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ACCOUNT_POLICY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ACCOUNT_POLICY {}
unsafe impl ::windows::runtime::Abi for WINBIO_ACCOUNT_POLICY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ADAPTER_INTERFACE_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl WINBIO_ADAPTER_INTERFACE_VERSION {}
impl ::std::default::Default for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_ADAPTER_INTERFACE_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::std::cmp::Eq for WINBIO_ADAPTER_INTERFACE_VERSION {}
unsafe impl ::windows::runtime::Abi for WINBIO_ADAPTER_INTERFACE_VERSION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ANTI_SPOOF_POLICY {
    pub Action: WINBIO_ANTI_SPOOF_POLICY_ACTION,
    pub Source: WINBIO_POLICY_SOURCE,
}
impl WINBIO_ANTI_SPOOF_POLICY {}
impl ::std::default::Default for WINBIO_ANTI_SPOOF_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ANTI_SPOOF_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_ANTI_SPOOF_POLICY").field("Action", &self.Action).field("Source", &self.Source).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ANTI_SPOOF_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Action == other.Action && self.Source == other.Source
    }
}
impl ::std::cmp::Eq for WINBIO_ANTI_SPOOF_POLICY {}
unsafe impl ::windows::runtime::Abi for WINBIO_ANTI_SPOOF_POLICY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_ANTI_SPOOF_POLICY_ACTION(pub i32);
pub const WINBIO_ANTI_SPOOF_DISABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(0i32);
pub const WINBIO_ANTI_SPOOF_ENABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(1i32);
pub const WINBIO_ANTI_SPOOF_REMOVE: WINBIO_ANTI_SPOOF_POLICY_ACTION = WINBIO_ANTI_SPOOF_POLICY_ACTION(2i32);
impl ::std::convert::From<i32> for WINBIO_ANTI_SPOOF_POLICY_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_ANTI_SPOOF_POLICY_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_ASYNC_NOTIFICATION_METHOD(pub i32);
pub const WINBIO_ASYNC_NOTIFY_NONE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(0i32);
pub const WINBIO_ASYNC_NOTIFY_CALLBACK: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(1i32);
pub const WINBIO_ASYNC_NOTIFY_MESSAGE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(2i32);
pub const WINBIO_ASYNC_NOTIFY_MAXIMUM_VALUE: WINBIO_ASYNC_NOTIFICATION_METHOD = WINBIO_ASYNC_NOTIFICATION_METHOD(3i32);
impl ::std::convert::From<i32> for WINBIO_ASYNC_NOTIFICATION_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_NOTIFICATION_METHOD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_ASYNC_RESULT {
    pub SessionHandle: u32,
    pub Operation: u32,
    pub SequenceNumber: u64,
    pub TimeStamp: i64,
    pub ApiStatus: ::windows::runtime::HRESULT,
    pub UnitId: u32,
    pub UserData: *mut ::std::ffi::c_void,
    pub Parameters: WINBIO_ASYNC_RESULT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_ASYNC_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub union WINBIO_ASYNC_RESULT_0 {
    pub Verify: WINBIO_ASYNC_RESULT_0_20,
    pub Identify: WINBIO_ASYNC_RESULT_0_15,
    pub EnrollBegin: WINBIO_ASYNC_RESULT_0_3,
    pub EnrollCapture: WINBIO_ASYNC_RESULT_0_4,
    pub EnrollCommit: WINBIO_ASYNC_RESULT_0_5,
    pub EnumEnrollments: WINBIO_ASYNC_RESULT_0_9,
    pub CaptureSample: WINBIO_ASYNC_RESULT_0_0,
    pub DeleteTemplate: WINBIO_ASYNC_RESULT_0_2,
    pub GetProperty: WINBIO_ASYNC_RESULT_0_12,
    pub SetProperty: WINBIO_ASYNC_RESULT_0_18,
    pub GetEvent: WINBIO_ASYNC_RESULT_0_11,
    pub ControlUnit: WINBIO_ASYNC_RESULT_0_1,
    pub EnumServiceProviders: WINBIO_ASYNC_RESULT_0_10,
    pub EnumBiometricUnits: WINBIO_ASYNC_RESULT_0_7,
    pub EnumDatabases: WINBIO_ASYNC_RESULT_0_8,
    pub VerifyAndReleaseTicket: WINBIO_ASYNC_RESULT_0_19,
    pub IdentifyAndReleaseTicket: WINBIO_ASYNC_RESULT_0_14,
    pub EnrollSelect: WINBIO_ASYNC_RESULT_0_6,
    pub MonitorPresence: WINBIO_ASYNC_RESULT_0_16,
    pub GetProtectionPolicy: WINBIO_ASYNC_RESULT_0_13,
    pub NotifyUnitStatusChange: WINBIO_ASYNC_RESULT_0_17,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_ASYNC_RESULT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_0 {
    pub Sample: *mut WINBIO_BIR,
    pub SampleSize: usize,
    pub RejectDetail: u32,
}
impl WINBIO_ASYNC_RESULT_0_0 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_CaptureSample_e__Struct").field("Sample", &self.Sample).field("SampleSize", &self.SampleSize).field("RejectDetail", &self.RejectDetail).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Sample == other.Sample && self.SampleSize == other.SampleSize && self.RejectDetail == other.RejectDetail
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_1 {
    pub Component: WINBIO_COMPONENT,
    pub ControlCode: u32,
    pub OperationStatus: u32,
    pub SendBuffer: *mut u8,
    pub SendBufferSize: usize,
    pub ReceiveBuffer: *mut u8,
    pub ReceiveBufferSize: usize,
    pub ReceiveDataSize: usize,
}
impl WINBIO_ASYNC_RESULT_0_1 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ControlUnit_e__Struct")
            .field("Component", &self.Component)
            .field("ControlCode", &self.ControlCode)
            .field("OperationStatus", &self.OperationStatus)
            .field("SendBuffer", &self.SendBuffer)
            .field("SendBufferSize", &self.SendBufferSize)
            .field("ReceiveBuffer", &self.ReceiveBuffer)
            .field("ReceiveBufferSize", &self.ReceiveBufferSize)
            .field("ReceiveDataSize", &self.ReceiveDataSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Component == other.Component && self.ControlCode == other.ControlCode && self.OperationStatus == other.OperationStatus && self.SendBuffer == other.SendBuffer && self.SendBufferSize == other.SendBufferSize && self.ReceiveBuffer == other.ReceiveBuffer && self.ReceiveBufferSize == other.ReceiveBufferSize && self.ReceiveDataSize == other.ReceiveDataSize
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_1 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_2 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
}
impl WINBIO_ASYNC_RESULT_0_2 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_2 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_3 {
    pub SubFactor: u8,
}
impl WINBIO_ASYNC_RESULT_0_3 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnrollBegin_e__Struct").field("SubFactor", &self.SubFactor).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.SubFactor == other.SubFactor
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_3 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_4 {
    pub RejectDetail: u32,
}
impl WINBIO_ASYNC_RESULT_0_4 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnrollCapture_e__Struct").field("RejectDetail", &self.RejectDetail).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.RejectDetail == other.RejectDetail
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_4 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_4 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_ASYNC_RESULT_0_5 {
    pub Identity: WINBIO_IDENTITY,
    pub IsNewTemplate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_ASYNC_RESULT_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_5 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_5 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_5 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_6 {
    pub SelectorValue: u64,
}
impl WINBIO_ASYNC_RESULT_0_6 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnrollSelect_e__Struct").field("SelectorValue", &self.SelectorValue).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.SelectorValue == other.SelectorValue
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_6 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_6 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_7 {
    pub UnitCount: usize,
    pub UnitSchemaArray: *mut WINBIO_UNIT_SCHEMA,
}
impl WINBIO_ASYNC_RESULT_0_7 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_7 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnumBiometricUnits_e__Struct").field("UnitCount", &self.UnitCount).field("UnitSchemaArray", &self.UnitSchemaArray).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.UnitCount == other.UnitCount && self.UnitSchemaArray == other.UnitSchemaArray
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_7 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_7 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_8 {
    pub StorageCount: usize,
    pub StorageSchemaArray: *mut WINBIO_STORAGE_SCHEMA,
}
impl WINBIO_ASYNC_RESULT_0_8 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_8 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_8 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnumDatabases_e__Struct").field("StorageCount", &self.StorageCount).field("StorageSchemaArray", &self.StorageSchemaArray).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_8 {
    fn eq(&self, other: &Self) -> bool {
        self.StorageCount == other.StorageCount && self.StorageSchemaArray == other.StorageSchemaArray
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_8 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_8 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_9 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactorCount: usize,
    pub SubFactorArray: *mut u8,
}
impl WINBIO_ASYNC_RESULT_0_9 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_9 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_9 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_9 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_9 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_10 {
    pub BspCount: usize,
    pub BspSchemaArray: *mut WINBIO_BSP_SCHEMA,
}
impl WINBIO_ASYNC_RESULT_0_10 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_10 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_10 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnumServiceProviders_e__Struct").field("BspCount", &self.BspCount).field("BspSchemaArray", &self.BspSchemaArray).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_10 {
    fn eq(&self, other: &Self) -> bool {
        self.BspCount == other.BspCount && self.BspSchemaArray == other.BspSchemaArray
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_10 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_10 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_11 {
    pub Event: WINBIO_EVENT,
}
impl WINBIO_ASYNC_RESULT_0_11 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_11 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_11 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_11 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_11 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_12 {
    pub PropertyType: u32,
    pub PropertyId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut ::std::ffi::c_void,
}
impl WINBIO_ASYNC_RESULT_0_12 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_12 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_12 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_12 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_12 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_13 {
    pub Identity: WINBIO_IDENTITY,
    pub Policy: WINBIO_PROTECTION_POLICY,
}
impl WINBIO_ASYNC_RESULT_0_13 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_13 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_13 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_13 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_13 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_14 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
    pub Ticket: u64,
}
impl WINBIO_ASYNC_RESULT_0_14 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_14 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_14 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_14 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_14 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_15 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
}
impl WINBIO_ASYNC_RESULT_0_15 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_15 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_15 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_15 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_15 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_ASYNC_RESULT_0_16 {
    pub ChangeType: u32,
    pub PresenceCount: usize,
    pub PresenceArray: *mut WINBIO_PRESENCE,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_ASYNC_RESULT_0_16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_16 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_16 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_MonitorPresence_e__Struct").field("ChangeType", &self.ChangeType).field("PresenceCount", &self.PresenceCount).field("PresenceArray", &self.PresenceArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_16 {
    fn eq(&self, other: &Self) -> bool {
        self.ChangeType == other.ChangeType && self.PresenceCount == other.PresenceCount && self.PresenceArray == other.PresenceArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_16 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_16 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_17 {
    pub ExtendedStatus: WINBIO_EXTENDED_UNIT_STATUS,
}
impl WINBIO_ASYNC_RESULT_0_17 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_17 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_17 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_NotifyUnitStatusChange_e__Struct").field("ExtendedStatus", &self.ExtendedStatus).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_17 {
    fn eq(&self, other: &Self) -> bool {
        self.ExtendedStatus == other.ExtendedStatus
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_17 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_17 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ASYNC_RESULT_0_18 {
    pub PropertyType: u32,
    pub PropertyId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut ::std::ffi::c_void,
}
impl WINBIO_ASYNC_RESULT_0_18 {}
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_18 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_18 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_18 {}
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_18 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_ASYNC_RESULT_0_19 {
    pub Match: super::super::Foundation::BOOLEAN,
    pub RejectDetail: u32,
    pub Ticket: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_ASYNC_RESULT_0_19 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_19 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_19 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_VerifyAndReleaseTicket_e__Struct").field("Match", &self.Match).field("RejectDetail", &self.RejectDetail).field("Ticket", &self.Ticket).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_19 {
    fn eq(&self, other: &Self) -> bool {
        self.Match == other.Match && self.RejectDetail == other.RejectDetail && self.Ticket == other.Ticket
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_19 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_19 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_ASYNC_RESULT_0_20 {
    pub Match: super::super::Foundation::BOOLEAN,
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_ASYNC_RESULT_0_20 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_ASYNC_RESULT_0_20 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_ASYNC_RESULT_0_20 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Verify_e__Struct").field("Match", &self.Match).field("RejectDetail", &self.RejectDetail).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_20 {
    fn eq(&self, other: &Self) -> bool {
        self.Match == other.Match && self.RejectDetail == other.RejectDetail
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_ASYNC_RESULT_0_20 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_ASYNC_RESULT_0_20 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
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
impl WINBIO_BDB_ANSI_381_HEADER {}
impl ::std::default::Default for WINBIO_BDB_ANSI_381_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_BDB_ANSI_381_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_BDB_ANSI_381_HEADER")
            .field("RecordLength", &self.RecordLength)
            .field("FormatIdentifier", &self.FormatIdentifier)
            .field("VersionNumber", &self.VersionNumber)
            .field("ProductId", &self.ProductId)
            .field("CaptureDeviceId", &self.CaptureDeviceId)
            .field("ImageAcquisitionLevel", &self.ImageAcquisitionLevel)
            .field("HorizontalScanResolution", &self.HorizontalScanResolution)
            .field("VerticalScanResolution", &self.VerticalScanResolution)
            .field("HorizontalImageResolution", &self.HorizontalImageResolution)
            .field("VerticalImageResolution", &self.VerticalImageResolution)
            .field("ElementCount", &self.ElementCount)
            .field("ScaleUnits", &self.ScaleUnits)
            .field("PixelDepth", &self.PixelDepth)
            .field("ImageCompressionAlg", &self.ImageCompressionAlg)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_BDB_ANSI_381_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RecordLength == other.RecordLength
            && self.FormatIdentifier == other.FormatIdentifier
            && self.VersionNumber == other.VersionNumber
            && self.ProductId == other.ProductId
            && self.CaptureDeviceId == other.CaptureDeviceId
            && self.ImageAcquisitionLevel == other.ImageAcquisitionLevel
            && self.HorizontalScanResolution == other.HorizontalScanResolution
            && self.VerticalScanResolution == other.VerticalScanResolution
            && self.HorizontalImageResolution == other.HorizontalImageResolution
            && self.VerticalImageResolution == other.VerticalImageResolution
            && self.ElementCount == other.ElementCount
            && self.ScaleUnits == other.ScaleUnits
            && self.PixelDepth == other.PixelDepth
            && self.ImageCompressionAlg == other.ImageCompressionAlg
            && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WINBIO_BDB_ANSI_381_HEADER {}
unsafe impl ::windows::runtime::Abi for WINBIO_BDB_ANSI_381_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_BDB_ANSI_381_RECORD {
    pub BlockLength: u32,
    pub HorizontalLineLength: u16,
    pub VerticalLineLength: u16,
    pub Position: u8,
    pub CountOfViews: u8,
    pub ViewNumber: u8,
    pub ImageQuality: u8,
    pub ImpressionType: u8,
    pub Reserved: u8,
}
impl WINBIO_BDB_ANSI_381_RECORD {}
impl ::std::default::Default for WINBIO_BDB_ANSI_381_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_BDB_ANSI_381_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_BDB_ANSI_381_RECORD")
            .field("BlockLength", &self.BlockLength)
            .field("HorizontalLineLength", &self.HorizontalLineLength)
            .field("VerticalLineLength", &self.VerticalLineLength)
            .field("Position", &self.Position)
            .field("CountOfViews", &self.CountOfViews)
            .field("ViewNumber", &self.ViewNumber)
            .field("ImageQuality", &self.ImageQuality)
            .field("ImpressionType", &self.ImpressionType)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_BDB_ANSI_381_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.BlockLength == other.BlockLength && self.HorizontalLineLength == other.HorizontalLineLength && self.VerticalLineLength == other.VerticalLineLength && self.Position == other.Position && self.CountOfViews == other.CountOfViews && self.ViewNumber == other.ViewNumber && self.ImageQuality == other.ImageQuality && self.ImpressionType == other.ImpressionType && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WINBIO_BDB_ANSI_381_RECORD {}
unsafe impl ::windows::runtime::Abi for WINBIO_BDB_ANSI_381_RECORD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_BIR {
    pub HeaderBlock: WINBIO_BIR_DATA,
    pub StandardDataBlock: WINBIO_BIR_DATA,
    pub VendorDataBlock: WINBIO_BIR_DATA,
    pub SignatureBlock: WINBIO_BIR_DATA,
}
impl WINBIO_BIR {}
impl ::std::default::Default for WINBIO_BIR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_BIR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_BIR").field("HeaderBlock", &self.HeaderBlock).field("StandardDataBlock", &self.StandardDataBlock).field("VendorDataBlock", &self.VendorDataBlock).field("SignatureBlock", &self.SignatureBlock).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_BIR {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderBlock == other.HeaderBlock && self.StandardDataBlock == other.StandardDataBlock && self.VendorDataBlock == other.VendorDataBlock && self.SignatureBlock == other.SignatureBlock
    }
}
impl ::std::cmp::Eq for WINBIO_BIR {}
unsafe impl ::windows::runtime::Abi for WINBIO_BIR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_BIR_ALGIN_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_BIR_ALIGN_SIZE: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_BIR_DATA {
    pub Size: u32,
    pub Offset: u32,
}
impl WINBIO_BIR_DATA {}
impl ::std::default::Default for WINBIO_BIR_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_BIR_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_BIR_DATA").field("Size", &self.Size).field("Offset", &self.Offset).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_BIR_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Offset == other.Offset
    }
}
impl ::std::cmp::Eq for WINBIO_BIR_DATA {}
unsafe impl ::windows::runtime::Abi for WINBIO_BIR_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_BIR_HEADER {
    pub ValidFields: u16,
    pub HeaderVersion: u8,
    pub PatronHeaderVersion: u8,
    pub DataFlags: u8,
    pub Type: u32,
    pub Subtype: u8,
    pub Purpose: u8,
    pub DataQuality: i8,
    pub CreationDate: i64,
    pub ValidityPeriod: WINBIO_BIR_HEADER_0,
    pub BiometricDataFormat: WINBIO_REGISTERED_FORMAT,
    pub ProductId: WINBIO_REGISTERED_FORMAT,
}
impl WINBIO_BIR_HEADER {}
impl ::std::default::Default for WINBIO_BIR_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_BIR_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_BIR_HEADER")
            .field("ValidFields", &self.ValidFields)
            .field("HeaderVersion", &self.HeaderVersion)
            .field("PatronHeaderVersion", &self.PatronHeaderVersion)
            .field("DataFlags", &self.DataFlags)
            .field("Type", &self.Type)
            .field("Subtype", &self.Subtype)
            .field("Purpose", &self.Purpose)
            .field("DataQuality", &self.DataQuality)
            .field("CreationDate", &self.CreationDate)
            .field("ValidityPeriod", &self.ValidityPeriod)
            .field("BiometricDataFormat", &self.BiometricDataFormat)
            .field("ProductId", &self.ProductId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_BIR_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.ValidFields == other.ValidFields
            && self.HeaderVersion == other.HeaderVersion
            && self.PatronHeaderVersion == other.PatronHeaderVersion
            && self.DataFlags == other.DataFlags
            && self.Type == other.Type
            && self.Subtype == other.Subtype
            && self.Purpose == other.Purpose
            && self.DataQuality == other.DataQuality
            && self.CreationDate == other.CreationDate
            && self.ValidityPeriod == other.ValidityPeriod
            && self.BiometricDataFormat == other.BiometricDataFormat
            && self.ProductId == other.ProductId
    }
}
impl ::std::cmp::Eq for WINBIO_BIR_HEADER {}
unsafe impl ::windows::runtime::Abi for WINBIO_BIR_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_BIR_HEADER_0 {
    pub BeginDate: i64,
    pub EndDate: i64,
}
impl WINBIO_BIR_HEADER_0 {}
impl ::std::default::Default for WINBIO_BIR_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_BIR_HEADER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ValidityPeriod_e__Struct").field("BeginDate", &self.BeginDate).field("EndDate", &self.EndDate).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_BIR_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BeginDate == other.BeginDate && self.EndDate == other.EndDate
    }
}
impl ::std::cmp::Eq for WINBIO_BIR_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_BIR_HEADER_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_BLANK_PAYLOAD {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
}
impl WINBIO_BLANK_PAYLOAD {}
impl ::std::default::Default for WINBIO_BLANK_PAYLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_BLANK_PAYLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_BLANK_PAYLOAD").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_BLANK_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult
    }
}
impl ::std::cmp::Eq for WINBIO_BLANK_PAYLOAD {}
unsafe impl ::windows::runtime::Abi for WINBIO_BLANK_PAYLOAD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_BSP_SCHEMA {
    pub BiometricFactor: u32,
    pub BspId: ::windows::runtime::GUID,
    pub Description: [u16; 256],
    pub Vendor: [u16; 256],
    pub Version: WINBIO_VERSION,
}
impl WINBIO_BSP_SCHEMA {}
impl ::std::default::Default for WINBIO_BSP_SCHEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_BSP_SCHEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_BSP_SCHEMA").field("BiometricFactor", &self.BiometricFactor).field("BspId", &self.BspId).field("Description", &self.Description).field("Vendor", &self.Vendor).field("Version", &self.Version).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_BSP_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.BiometricFactor == other.BiometricFactor && self.BspId == other.BspId && self.Description == other.Description && self.Vendor == other.Vendor && self.Version == other.Version
    }
}
impl ::std::cmp::Eq for WINBIO_BSP_SCHEMA {}
unsafe impl ::windows::runtime::Abi for WINBIO_BSP_SCHEMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_CALIBRATION_INFO {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
    pub CalibrationData: WINBIO_DATA,
}
impl WINBIO_CALIBRATION_INFO {}
impl ::std::default::Default for WINBIO_CALIBRATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_CALIBRATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_CALIBRATION_INFO").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("CalibrationData", &self.CalibrationData).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_CALIBRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.CalibrationData == other.CalibrationData
    }
}
impl ::std::cmp::Eq for WINBIO_CALIBRATION_INFO {}
unsafe impl ::windows::runtime::Abi for WINBIO_CALIBRATION_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_CAPTURE_DATA {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
    pub SensorStatus: u32,
    pub RejectDetail: u32,
    pub CaptureData: WINBIO_DATA,
}
impl WINBIO_CAPTURE_DATA {}
impl ::std::default::Default for WINBIO_CAPTURE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_CAPTURE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_CAPTURE_DATA").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("SensorStatus", &self.SensorStatus).field("RejectDetail", &self.RejectDetail).field("CaptureData", &self.CaptureData).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_CAPTURE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.SensorStatus == other.SensorStatus && self.RejectDetail == other.RejectDetail && self.CaptureData == other.CaptureData
    }
}
impl ::std::cmp::Eq for WINBIO_CAPTURE_DATA {}
unsafe impl ::windows::runtime::Abi for WINBIO_CAPTURE_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_CAPTURE_PARAMETERS {
    pub PayloadSize: u32,
    pub Purpose: u8,
    pub Format: WINBIO_REGISTERED_FORMAT,
    pub VendorFormat: ::windows::runtime::GUID,
    pub Flags: u8,
}
impl WINBIO_CAPTURE_PARAMETERS {}
impl ::std::default::Default for WINBIO_CAPTURE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_CAPTURE_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_CAPTURE_PARAMETERS").field("PayloadSize", &self.PayloadSize).field("Purpose", &self.Purpose).field("Format", &self.Format).field("VendorFormat", &self.VendorFormat).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_CAPTURE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Purpose == other.Purpose && self.Format == other.Format && self.VendorFormat == other.VendorFormat && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for WINBIO_CAPTURE_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for WINBIO_CAPTURE_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_COMPONENT(pub u32);
pub const WINBIO_COMPONENT_SENSOR: WINBIO_COMPONENT = WINBIO_COMPONENT(1u32);
pub const WINBIO_COMPONENT_ENGINE: WINBIO_COMPONENT = WINBIO_COMPONENT(2u32);
pub const WINBIO_COMPONENT_STORAGE: WINBIO_COMPONENT = WINBIO_COMPONENT(3u32);
impl ::std::convert::From<u32> for WINBIO_COMPONENT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_COMPONENT {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINBIO_COMPONENT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINBIO_COMPONENT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINBIO_COMPONENT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINBIO_COMPONENT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINBIO_COMPONENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_CREDENTIAL_FORMAT(pub i32);
pub const WINBIO_PASSWORD_GENERIC: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(1i32);
pub const WINBIO_PASSWORD_PACKED: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(2i32);
pub const WINBIO_PASSWORD_PROTECTED: WINBIO_CREDENTIAL_FORMAT = WINBIO_CREDENTIAL_FORMAT(3i32);
impl ::std::convert::From<i32> for WINBIO_CREDENTIAL_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_CREDENTIAL_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_CREDENTIAL_STATE(pub i32);
pub const WINBIO_CREDENTIAL_NOT_SET: WINBIO_CREDENTIAL_STATE = WINBIO_CREDENTIAL_STATE(1i32);
pub const WINBIO_CREDENTIAL_SET: WINBIO_CREDENTIAL_STATE = WINBIO_CREDENTIAL_STATE(2i32);
impl ::std::convert::From<i32> for WINBIO_CREDENTIAL_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_CREDENTIAL_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_CREDENTIAL_TYPE(pub i32);
pub const WINBIO_CREDENTIAL_PASSWORD: WINBIO_CREDENTIAL_TYPE = WINBIO_CREDENTIAL_TYPE(1i32);
pub const WINBIO_CREDENTIAL_ALL: WINBIO_CREDENTIAL_TYPE = WINBIO_CREDENTIAL_TYPE(-1i32);
impl ::std::convert::From<i32> for WINBIO_CREDENTIAL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_CREDENTIAL_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_DATA {
    pub Size: u32,
    pub Data: [u8; 1],
}
impl WINBIO_DATA {}
impl ::std::default::Default for WINBIO_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_DATA").field("Size", &self.Size).field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for WINBIO_DATA {}
unsafe impl ::windows::runtime::Abi for WINBIO_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_DIAGNOSTICS {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
    pub SensorStatus: u32,
    pub VendorDiagnostics: WINBIO_DATA,
}
impl WINBIO_DIAGNOSTICS {}
impl ::std::default::Default for WINBIO_DIAGNOSTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_DIAGNOSTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_DIAGNOSTICS").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("SensorStatus", &self.SensorStatus).field("VendorDiagnostics", &self.VendorDiagnostics).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_DIAGNOSTICS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.SensorStatus == other.SensorStatus && self.VendorDiagnostics == other.VendorDiagnostics
    }
}
impl ::std::cmp::Eq for WINBIO_DIAGNOSTICS {}
unsafe impl ::windows::runtime::Abi for WINBIO_DIAGNOSTICS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    pub PayloadSize: u32,
    pub Purpose: u8,
    pub Format: WINBIO_REGISTERED_FORMAT,
    pub VendorFormat: ::windows::runtime::GUID,
    pub Flags: u8,
    pub NonceSize: u32,
}
impl WINBIO_ENCRYPTED_CAPTURE_PARAMS {}
impl ::std::default::Default for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_ENCRYPTED_CAPTURE_PARAMS").field("PayloadSize", &self.PayloadSize).field("Purpose", &self.Purpose).field("Format", &self.Format).field("VendorFormat", &self.VendorFormat).field("Flags", &self.Flags).field("NonceSize", &self.NonceSize).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Purpose == other.Purpose && self.Format == other.Format && self.VendorFormat == other.VendorFormat && self.Flags == other.Flags && self.NonceSize == other.NonceSize
    }
}
impl ::std::cmp::Eq for WINBIO_ENCRYPTED_CAPTURE_PARAMS {}
unsafe impl ::windows::runtime::Abi for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WINBIO_ENGINE_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::runtime::GUID,
    pub Attach: ::std::option::Option<PIBIO_ENGINE_ATTACH_FN>,
    pub Detach: ::std::option::Option<PIBIO_ENGINE_DETACH_FN>,
    pub ClearContext: ::std::option::Option<PIBIO_ENGINE_CLEAR_CONTEXT_FN>,
    pub QueryPreferredFormat: ::std::option::Option<PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN>,
    pub QueryIndexVectorSize: ::std::option::Option<PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN>,
    pub QueryHashAlgorithms: ::std::option::Option<PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN>,
    pub SetHashAlgorithm: ::std::option::Option<PIBIO_ENGINE_SET_HASH_ALGORITHM_FN>,
    pub QuerySampleHint: ::std::option::Option<PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN>,
    pub AcceptSampleData: ::std::option::Option<PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN>,
    pub ExportEngineData: ::std::option::Option<PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN>,
    pub VerifyFeatureSet: ::std::option::Option<PIBIO_ENGINE_VERIFY_FEATURE_SET_FN>,
    pub IdentifyFeatureSet: ::std::option::Option<PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN>,
    pub CreateEnrollment: ::std::option::Option<PIBIO_ENGINE_CREATE_ENROLLMENT_FN>,
    pub UpdateEnrollment: ::std::option::Option<PIBIO_ENGINE_UPDATE_ENROLLMENT_FN>,
    pub GetEnrollmentStatus: ::std::option::Option<PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN>,
    pub GetEnrollmentHash: ::std::option::Option<PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN>,
    pub CheckForDuplicate: ::std::option::Option<PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN>,
    pub CommitEnrollment: ::std::option::Option<PIBIO_ENGINE_COMMIT_ENROLLMENT_FN>,
    pub DiscardEnrollment: ::std::option::Option<PIBIO_ENGINE_DISCARD_ENROLLMENT_FN>,
    pub ControlUnit: ::std::option::Option<PIBIO_ENGINE_CONTROL_UNIT_FN>,
    pub ControlUnitPrivileged: ::std::option::Option<PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN>,
    pub NotifyPowerChange: ::std::option::Option<PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN>,
    pub Reserved_1: ::std::option::Option<PIBIO_ENGINE_RESERVED_1_FN>,
    pub PipelineInit: ::std::option::Option<PIBIO_ENGINE_PIPELINE_INIT_FN>,
    pub PipelineCleanup: ::std::option::Option<PIBIO_ENGINE_PIPELINE_CLEANUP_FN>,
    pub Activate: ::std::option::Option<PIBIO_ENGINE_ACTIVATE_FN>,
    pub Deactivate: ::std::option::Option<PIBIO_ENGINE_DEACTIVATE_FN>,
    pub QueryExtendedInfo: ::std::option::Option<PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN>,
    pub IdentifyAll: ::std::option::Option<PIBIO_ENGINE_IDENTIFY_ALL_FN>,
    pub SetEnrollmentSelector: ::std::option::Option<PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN>,
    pub SetEnrollmentParameters: ::std::option::Option<PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN>,
    pub QueryExtendedEnrollmentStatus: ::std::option::Option<PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN>,
    pub RefreshCache: ::std::option::Option<PIBIO_ENGINE_REFRESH_CACHE_FN>,
    pub SelectCalibrationFormat: ::std::option::Option<PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN>,
    pub QueryCalibrationData: ::std::option::Option<PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN>,
    pub SetAccountPolicy: ::std::option::Option<PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN>,
    pub CreateKey: ::std::option::Option<PIBIO_ENGINE_CREATE_KEY_FN>,
    pub IdentifyFeatureSetSecure: ::std::option::Option<PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN>,
    pub AcceptPrivateSensorTypeInfo: ::std::option::Option<PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN>,
    pub CreateEnrollmentAuthenticated: ::std::option::Option<PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN>,
    pub IdentifyFeatureSetAuthenticated: ::std::option::Option<PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WINBIO_ENGINE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WINBIO_ENGINE_INTERFACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WINBIO_ENGINE_INTERFACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_ENGINE_INTERFACE").field("Version", &self.Version).field("Type", &self.Type).field("Size", &self.Size).field("AdapterId", &self.AdapterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WINBIO_ENGINE_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Type == other.Type
            && self.Size == other.Size
            && self.AdapterId == other.AdapterId
            && self.Attach.map(|f| f as usize) == other.Attach.map(|f| f as usize)
            && self.Detach.map(|f| f as usize) == other.Detach.map(|f| f as usize)
            && self.ClearContext.map(|f| f as usize) == other.ClearContext.map(|f| f as usize)
            && self.QueryPreferredFormat.map(|f| f as usize) == other.QueryPreferredFormat.map(|f| f as usize)
            && self.QueryIndexVectorSize.map(|f| f as usize) == other.QueryIndexVectorSize.map(|f| f as usize)
            && self.QueryHashAlgorithms.map(|f| f as usize) == other.QueryHashAlgorithms.map(|f| f as usize)
            && self.SetHashAlgorithm.map(|f| f as usize) == other.SetHashAlgorithm.map(|f| f as usize)
            && self.QuerySampleHint.map(|f| f as usize) == other.QuerySampleHint.map(|f| f as usize)
            && self.AcceptSampleData.map(|f| f as usize) == other.AcceptSampleData.map(|f| f as usize)
            && self.ExportEngineData.map(|f| f as usize) == other.ExportEngineData.map(|f| f as usize)
            && self.VerifyFeatureSet.map(|f| f as usize) == other.VerifyFeatureSet.map(|f| f as usize)
            && self.IdentifyFeatureSet.map(|f| f as usize) == other.IdentifyFeatureSet.map(|f| f as usize)
            && self.CreateEnrollment.map(|f| f as usize) == other.CreateEnrollment.map(|f| f as usize)
            && self.UpdateEnrollment.map(|f| f as usize) == other.UpdateEnrollment.map(|f| f as usize)
            && self.GetEnrollmentStatus.map(|f| f as usize) == other.GetEnrollmentStatus.map(|f| f as usize)
            && self.GetEnrollmentHash.map(|f| f as usize) == other.GetEnrollmentHash.map(|f| f as usize)
            && self.CheckForDuplicate.map(|f| f as usize) == other.CheckForDuplicate.map(|f| f as usize)
            && self.CommitEnrollment.map(|f| f as usize) == other.CommitEnrollment.map(|f| f as usize)
            && self.DiscardEnrollment.map(|f| f as usize) == other.DiscardEnrollment.map(|f| f as usize)
            && self.ControlUnit.map(|f| f as usize) == other.ControlUnit.map(|f| f as usize)
            && self.ControlUnitPrivileged.map(|f| f as usize) == other.ControlUnitPrivileged.map(|f| f as usize)
            && self.NotifyPowerChange.map(|f| f as usize) == other.NotifyPowerChange.map(|f| f as usize)
            && self.Reserved_1.map(|f| f as usize) == other.Reserved_1.map(|f| f as usize)
            && self.PipelineInit.map(|f| f as usize) == other.PipelineInit.map(|f| f as usize)
            && self.PipelineCleanup.map(|f| f as usize) == other.PipelineCleanup.map(|f| f as usize)
            && self.Activate.map(|f| f as usize) == other.Activate.map(|f| f as usize)
            && self.Deactivate.map(|f| f as usize) == other.Deactivate.map(|f| f as usize)
            && self.QueryExtendedInfo.map(|f| f as usize) == other.QueryExtendedInfo.map(|f| f as usize)
            && self.IdentifyAll.map(|f| f as usize) == other.IdentifyAll.map(|f| f as usize)
            && self.SetEnrollmentSelector.map(|f| f as usize) == other.SetEnrollmentSelector.map(|f| f as usize)
            && self.SetEnrollmentParameters.map(|f| f as usize) == other.SetEnrollmentParameters.map(|f| f as usize)
            && self.QueryExtendedEnrollmentStatus.map(|f| f as usize) == other.QueryExtendedEnrollmentStatus.map(|f| f as usize)
            && self.RefreshCache.map(|f| f as usize) == other.RefreshCache.map(|f| f as usize)
            && self.SelectCalibrationFormat.map(|f| f as usize) == other.SelectCalibrationFormat.map(|f| f as usize)
            && self.QueryCalibrationData.map(|f| f as usize) == other.QueryCalibrationData.map(|f| f as usize)
            && self.SetAccountPolicy.map(|f| f as usize) == other.SetAccountPolicy.map(|f| f as usize)
            && self.CreateKey.map(|f| f as usize) == other.CreateKey.map(|f| f as usize)
            && self.IdentifyFeatureSetSecure.map(|f| f as usize) == other.IdentifyFeatureSetSecure.map(|f| f as usize)
            && self.AcceptPrivateSensorTypeInfo.map(|f| f as usize) == other.AcceptPrivateSensorTypeInfo.map(|f| f as usize)
            && self.CreateEnrollmentAuthenticated.map(|f| f as usize) == other.CreateEnrollmentAuthenticated.map(|f| f as usize)
            && self.IdentifyFeatureSetAuthenticated.map(|f| f as usize) == other.IdentifyFeatureSetAuthenticated.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WINBIO_ENGINE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WINBIO_ENGINE_INTERFACE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EVENT {
    pub Type: u32,
    pub Parameters: WINBIO_EVENT_0,
}
impl WINBIO_EVENT {}
impl ::std::default::Default for WINBIO_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_EVENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_EVENT {}
unsafe impl ::windows::runtime::Abi for WINBIO_EVENT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub union WINBIO_EVENT_0 {
    pub Unclaimed: WINBIO_EVENT_0_2,
    pub UnclaimedIdentify: WINBIO_EVENT_0_1,
    pub Error: WINBIO_EVENT_0_0,
}
impl WINBIO_EVENT_0 {}
impl ::std::default::Default for WINBIO_EVENT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_EVENT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_EVENT_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EVENT_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EVENT_0_0 {
    pub ErrorCode: ::windows::runtime::HRESULT,
}
impl WINBIO_EVENT_0_0 {}
impl ::std::default::Default for WINBIO_EVENT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EVENT_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Error_e__Struct").field("ErrorCode", &self.ErrorCode).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EVENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ErrorCode == other.ErrorCode
    }
}
impl ::std::cmp::Eq for WINBIO_EVENT_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EVENT_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EVENT_0_1 {
    pub UnitId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
}
impl WINBIO_EVENT_0_1 {}
impl ::std::default::Default for WINBIO_EVENT_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_EVENT_0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_EVENT_0_1 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EVENT_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EVENT_0_2 {
    pub UnitId: u32,
    pub RejectDetail: u32,
}
impl WINBIO_EVENT_0_2 {}
impl ::std::default::Default for WINBIO_EVENT_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EVENT_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Unclaimed_e__Struct").field("UnitId", &self.UnitId).field("RejectDetail", &self.RejectDetail).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EVENT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.RejectDetail == other.RejectDetail
    }
}
impl ::std::cmp::Eq for WINBIO_EVENT_0_2 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EVENT_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO {
    pub GenericEngineCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_ENGINE_INFO_0,
}
impl WINBIO_EXTENDED_ENGINE_INFO {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub union WINBIO_EXTENDED_ENGINE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENGINE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENGINE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_ENGINE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_ENGINE_INFO_0_3,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_0_0,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FacialFeatures_e__Struct").field("Capabilities", &self.Capabilities).field("EnrollmentRequirements", &self.EnrollmentRequirements).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.EnrollmentRequirements == other.EnrollmentRequirements
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    pub Null: u32,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnrollmentRequirements_e__Struct").field("Null", &self.Null).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Null == other.Null
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_1_0,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0_1 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Fingerprint_e__Struct").field("Capabilities", &self.Capabilities).field("EnrollmentRequirements", &self.EnrollmentRequirements).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.EnrollmentRequirements == other.EnrollmentRequirements
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_1 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnrollmentRequirements_e__Struct").field("GeneralSamples", &self.GeneralSamples).field("Center", &self.Center).field("TopEdge", &self.TopEdge).field("BottomEdge", &self.BottomEdge).field("LeftEdge", &self.LeftEdge).field("RightEdge", &self.RightEdge).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.GeneralSamples == other.GeneralSamples && self.Center == other.Center && self.TopEdge == other.TopEdge && self.BottomEdge == other.BottomEdge && self.LeftEdge == other.LeftEdge && self.RightEdge == other.RightEdge
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_2_0,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0_2 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Iris_e__Struct").field("Capabilities", &self.Capabilities).field("EnrollmentRequirements", &self.EnrollmentRequirements).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.EnrollmentRequirements == other.EnrollmentRequirements
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_2 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    pub Null: u32,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnrollmentRequirements_e__Struct").field("Null", &self.Null).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Null == other.Null
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_3_0,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0_3 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Voice_e__Struct").field("Capabilities", &self.Capabilities).field("EnrollmentRequirements", &self.EnrollmentRequirements).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.EnrollmentRequirements == other.EnrollmentRequirements
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_3 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    pub Null: u32,
}
impl WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_EnrollmentRequirements_e__Struct").field("Null", &self.Null).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Null == other.Null
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    pub Size: usize,
    pub SubFactor: u8,
}
impl WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {}
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_EXTENDED_ENROLLMENT_PARAMETERS").field("Size", &self.Size).field("SubFactor", &self.SubFactor).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SubFactor == other.SubFactor
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS {
    pub TemplateStatus: ::windows::runtime::HRESULT,
    pub RejectDetail: u32,
    pub PercentComplete: u32,
    pub Factor: u32,
    pub SubFactor: u8,
    pub Specific: WINBIO_EXTENDED_ENROLLMENT_STATUS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_EXTENDED_ENROLLMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub union WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1,
    pub Iris: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2,
    pub Voice: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    pub BoundingBox: super::super::Foundation::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FacialFeatures_e__Struct").field("BoundingBox", &self.BoundingBox).field("Distance", &self.Distance).field("OpaqueEngineData", &self.OpaqueEngineData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BoundingBox == other.BoundingBox && self.Distance == other.Distance && self.OpaqueEngineData == other.OpaqueEngineData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    pub AdapterId: ::windows::runtime::GUID,
    pub Data: [u32; 78],
}
impl WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_OpaqueEngineData_e__Struct").field("AdapterId", &self.AdapterId).field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterId == other.AdapterId && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
impl WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Fingerprint_e__Struct").field("GeneralSamples", &self.GeneralSamples).field("Center", &self.Center).field("TopEdge", &self.TopEdge).field("BottomEdge", &self.BottomEdge).field("LeftEdge", &self.LeftEdge).field("RightEdge", &self.RightEdge).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.GeneralSamples == other.GeneralSamples && self.Center == other.Center && self.TopEdge == other.TopEdge && self.BottomEdge == other.BottomEdge && self.LeftEdge == other.LeftEdge && self.RightEdge == other.RightEdge
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    pub EyeBoundingBox_1: super::super::Foundation::RECT,
    pub EyeBoundingBox_2: super::super::Foundation::RECT,
    pub PupilCenter_1: super::super::Foundation::POINT,
    pub PupilCenter_2: super::super::Foundation::POINT,
    pub Distance: i32,
    pub GridPointCompletionPercent: u32,
    pub GridPointIndex: u16,
    pub Point3D: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0,
    pub StopCaptureAndShowCriticalFeedback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Iris_e__Struct")
            .field("EyeBoundingBox_1", &self.EyeBoundingBox_1)
            .field("EyeBoundingBox_2", &self.EyeBoundingBox_2)
            .field("PupilCenter_1", &self.PupilCenter_1)
            .field("PupilCenter_2", &self.PupilCenter_2)
            .field("Distance", &self.Distance)
            .field("GridPointCompletionPercent", &self.GridPointCompletionPercent)
            .field("GridPointIndex", &self.GridPointIndex)
            .field("Point3D", &self.Point3D)
            .field("StopCaptureAndShowCriticalFeedback", &self.StopCaptureAndShowCriticalFeedback)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.EyeBoundingBox_1 == other.EyeBoundingBox_1 && self.EyeBoundingBox_2 == other.EyeBoundingBox_2 && self.PupilCenter_1 == other.PupilCenter_1 && self.PupilCenter_2 == other.PupilCenter_2 && self.Distance == other.Distance && self.GridPointCompletionPercent == other.GridPointCompletionPercent && self.GridPointIndex == other.GridPointIndex && self.Point3D == other.Point3D && self.StopCaptureAndShowCriticalFeedback == other.StopCaptureAndShowCriticalFeedback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
impl WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Point3D_e__Struct").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    pub Reserved: u32,
}
impl WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {}
impl ::std::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Voice_e__Struct").field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_EXTENDED_SENSOR_INFO {
    pub GenericSensorCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_SENSOR_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_EXTENDED_SENSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_EXTENDED_SENSOR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_SENSOR_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub union WINBIO_EXTENDED_SENSOR_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_SENSOR_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_SENSOR_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_SENSOR_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_SENSOR_INFO_0_3,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_EXTENDED_SENSOR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_SENSOR_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    pub FrameSize: super::super::Foundation::RECT,
    pub FrameOffset: super::super::Foundation::POINT,
    pub MandatoryOrientation: u32,
    pub HardwareInfo: WINBIO_EXTENDED_SENSOR_INFO_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_EXTENDED_SENSOR_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FacialFeatures_e__Struct").field("FrameSize", &self.FrameSize).field("FrameOffset", &self.FrameOffset).field("MandatoryOrientation", &self.MandatoryOrientation).field("HardwareInfo", &self.HardwareInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.FrameSize == other.FrameSize && self.FrameOffset == other.FrameOffset && self.MandatoryOrientation == other.MandatoryOrientation && self.HardwareInfo == other.HardwareInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    pub ColorSensorId: [u16; 260],
    pub InfraredSensorId: [u16; 260],
    pub InfraredSensorRotationAngle: u32,
}
impl WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_HardwareInfo_e__Struct").field("ColorSensorId", &self.ColorSensorId).field("InfraredSensorId", &self.InfraredSensorId).field("InfraredSensorRotationAngle", &self.InfraredSensorRotationAngle).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ColorSensorId == other.ColorSensorId && self.InfraredSensorId == other.InfraredSensorId && self.InfraredSensorRotationAngle == other.InfraredSensorRotationAngle
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    pub Reserved: u32,
}
impl WINBIO_EXTENDED_SENSOR_INFO_0_1 {}
impl ::std::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Fingerprint_e__Struct").field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_1 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    pub FrameSize: super::super::Foundation::RECT,
    pub FrameOffset: super::super::Foundation::POINT,
    pub MandatoryOrientation: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_EXTENDED_SENSOR_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Iris_e__Struct").field("FrameSize", &self.FrameSize).field("FrameOffset", &self.FrameOffset).field("MandatoryOrientation", &self.MandatoryOrientation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.FrameSize == other.FrameSize && self.FrameOffset == other.FrameOffset && self.MandatoryOrientation == other.MandatoryOrientation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    pub Reserved: u32,
}
impl WINBIO_EXTENDED_SENSOR_INFO_0_3 {}
impl ::std::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Voice_e__Struct").field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_3 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO {
    pub GenericStorageCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_STORAGE_INFO_0,
}
impl WINBIO_EXTENDED_STORAGE_INFO {}
impl ::std::default::Default for WINBIO_EXTENDED_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_STORAGE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub union WINBIO_EXTENDED_STORAGE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_STORAGE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_STORAGE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_STORAGE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_STORAGE_INFO_0_3,
}
impl WINBIO_EXTENDED_STORAGE_INFO_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_STORAGE_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    pub Capabilities: u32,
}
impl WINBIO_EXTENDED_STORAGE_INFO_0_0 {}
impl ::std::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FacialFeatures_e__Struct").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    pub Capabilities: u32,
}
impl WINBIO_EXTENDED_STORAGE_INFO_0_1 {}
impl ::std::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Fingerprint_e__Struct").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_1 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    pub Capabilities: u32,
}
impl WINBIO_EXTENDED_STORAGE_INFO_0_2 {}
impl ::std::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Iris_e__Struct").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_2 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    pub Capabilities: u32,
}
impl WINBIO_EXTENDED_STORAGE_INFO_0_3 {}
impl ::std::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Voice_e__Struct").field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_3 {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_EXTENDED_UNIT_STATUS {
    pub Availability: u32,
    pub ReasonCode: u32,
}
impl WINBIO_EXTENDED_UNIT_STATUS {}
impl ::std::default::Default for WINBIO_EXTENDED_UNIT_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_EXTENDED_UNIT_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_EXTENDED_UNIT_STATUS").field("Availability", &self.Availability).field("ReasonCode", &self.ReasonCode).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_EXTENDED_UNIT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Availability == other.Availability && self.ReasonCode == other.ReasonCode
    }
}
impl ::std::cmp::Eq for WINBIO_EXTENDED_UNIT_STATUS {}
unsafe impl ::windows::runtime::Abi for WINBIO_EXTENDED_UNIT_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_ADAPTER_INTEGRITY_FAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860995i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_AUTO_LOGON_DISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860989i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_BAD_CAPTURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861048i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860975i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_LARGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860976i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_SMALL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860977i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CANCELED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861052i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CAPTURE_ABORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861050i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CONFIGURATION_FAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861005i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CRED_PROV_DISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861008i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CRED_PROV_NO_CREDENTIAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861007i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_CRED_PROV_SECURITY_LOCKOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860985i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861034i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_BAD_INDEX_VECTOR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861022i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_CLOSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861037i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_CREATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861039i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_ERASE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861036i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_FIND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861035i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CANT_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861038i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_CORRUPTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861030i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_EOF: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861023i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_FULL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861032i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861031i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_NO_MORE_RECORDS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861024i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_NO_RESULTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861025i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_NO_SUCH_RECORD: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861029i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_READ_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861027i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATABASE_WRITE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861026i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATA_COLLECTION_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861045i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DATA_PROTECTION_FAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860986i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DEADLOCK_DETECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860992i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DEVICE_BUSY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861040i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DEVICE_FAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861002i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861006i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DUPLICATE_ENROLLMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861028i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_DUPLICATE_TEMPLATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861013i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_ENROLLMENT_CANCELED_BY_SUSPEND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860965i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_ENROLLMENT_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861049i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_EVENT_MONITOR_ACTIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860999i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_FAST_USER_SWITCH_DISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861001i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INCORRECT_BSP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861020i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INCORRECT_SENSOR_POOL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861019i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INCORRECT_SESSION_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860994i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INSECURE_SENSOR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860969i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_BUFFER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860967i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_BUFFER_ID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860968i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_CALIBRATION_FORMAT_ARRAY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860980i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_CONTROL_CODE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861047i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_DEVICE_STATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861041i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_KEY_IDENTIFIER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860974i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_OPERATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861012i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_PROPERTY_ID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860997i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_PROPERTY_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860998i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_SENSOR_MODE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861017i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_SUBFACTOR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860981i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_TICKET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860988i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_INVALID_UNIT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861054i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_KEY_CREATION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860973i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_KEY_IDENTIFIER_BUFFER_TOO_SMALL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860972i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_LOCK_VIOLATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861014i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_MAX_ERROR_COUNT_EXCEEDED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860990i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NOT_ACTIVE_CONSOLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861000i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NO_CAPTURE_DATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861018i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NO_MATCH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861051i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NO_PREBOOT_IDENTITY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860991i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_NO_SUPPORTED_CALIBRATION_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860979i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_POLICY_PROTECTION_UNAVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860970i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_PRESENCE_MONITOR_ACTIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860982i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_PROPERTY_UNAVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860971i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SAS_ENABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861003i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SELECTION_REQUIRED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860983i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SENSOR_UNAVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861004i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SESSION_BUSY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861011i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_SESSION_HANDLE_CLOSED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860993i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_TICKET_QUOTA_EXCEEDED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860987i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_TRUSTLET_INTEGRITY_FAIL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860966i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNKNOWN_ID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861053i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_DATA_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861044i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_DATA_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861043i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_FACTOR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861055i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_POOL_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860984i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_PROPERTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860996i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_PURPOSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146861042i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_E_UNSUPPORTED_SENSOR_CALIBRATION_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2146860978i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_FP_BU_STATE {
    pub SensorAttached: super::super::Foundation::BOOL,
    pub CreationResult: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_FP_BU_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_FP_BU_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_FP_BU_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_FP_BU_STATE").field("SensorAttached", &self.SensorAttached).field("CreationResult", &self.CreationResult).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_FP_BU_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.SensorAttached == other.SensorAttached && self.CreationResult == other.CreationResult
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_FP_BU_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_FP_BU_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WINBIO_FRAMEWORK_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::runtime::GUID,
    pub SetUnitStatus: ::std::option::Option<PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN>,
    pub VsmStorageAttach: ::std::option::Option<PIBIO_STORAGE_ATTACH_FN>,
    pub VsmStorageDetach: ::std::option::Option<PIBIO_STORAGE_DETACH_FN>,
    pub VsmStorageClearContext: ::std::option::Option<PIBIO_STORAGE_CLEAR_CONTEXT_FN>,
    pub VsmStorageCreateDatabase: ::std::option::Option<PIBIO_STORAGE_CREATE_DATABASE_FN>,
    pub VsmStorageOpenDatabase: ::std::option::Option<PIBIO_STORAGE_OPEN_DATABASE_FN>,
    pub VsmStorageCloseDatabase: ::std::option::Option<PIBIO_STORAGE_CLOSE_DATABASE_FN>,
    pub VsmStorageDeleteRecord: ::std::option::Option<PIBIO_STORAGE_DELETE_RECORD_FN>,
    pub VsmStorageNotifyPowerChange: ::std::option::Option<PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN>,
    pub VsmStoragePipelineInit: ::std::option::Option<PIBIO_STORAGE_PIPELINE_INIT_FN>,
    pub VsmStoragePipelineCleanup: ::std::option::Option<PIBIO_STORAGE_PIPELINE_CLEANUP_FN>,
    pub VsmStorageActivate: ::std::option::Option<PIBIO_STORAGE_ACTIVATE_FN>,
    pub VsmStorageDeactivate: ::std::option::Option<PIBIO_STORAGE_DEACTIVATE_FN>,
    pub VsmStorageQueryExtendedInfo: ::std::option::Option<PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN>,
    pub VsmStorageCacheClear: ::std::option::Option<PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN>,
    pub VsmStorageCacheImportBegin: ::std::option::Option<PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN>,
    pub VsmStorageCacheImportNext: ::std::option::Option<PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN>,
    pub VsmStorageCacheImportEnd: ::std::option::Option<PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN>,
    pub VsmStorageCacheExportBegin: ::std::option::Option<PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN>,
    pub VsmStorageCacheExportNext: ::std::option::Option<PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN>,
    pub VsmStorageCacheExportEnd: ::std::option::Option<PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN>,
    pub VsmSensorAttach: ::std::option::Option<PIBIO_SENSOR_ATTACH_FN>,
    pub VsmSensorDetach: ::std::option::Option<PIBIO_SENSOR_DETACH_FN>,
    pub VsmSensorClearContext: ::std::option::Option<PIBIO_SENSOR_CLEAR_CONTEXT_FN>,
    pub VsmSensorPushDataToEngine: ::std::option::Option<PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN>,
    pub VsmSensorNotifyPowerChange: ::std::option::Option<PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN>,
    pub VsmSensorPipelineInit: ::std::option::Option<PIBIO_SENSOR_PIPELINE_INIT_FN>,
    pub VsmSensorPipelineCleanup: ::std::option::Option<PIBIO_SENSOR_PIPELINE_CLEANUP_FN>,
    pub VsmSensorActivate: ::std::option::Option<PIBIO_SENSOR_ACTIVATE_FN>,
    pub VsmSensorDeactivate: ::std::option::Option<PIBIO_SENSOR_DEACTIVATE_FN>,
    pub VsmSensorAsyncImportRawBuffer: ::std::option::Option<PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN>,
    pub VsmSensorAsyncImportSecureBuffer: ::std::option::Option<PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN>,
    pub Reserved1: ::std::option::Option<PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN>,
    pub Reserved2: ::std::option::Option<PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN>,
    pub Reserved3: ::std::option::Option<PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN>,
    pub Reserved4: ::std::option::Option<PIBIO_STORAGE_RESERVED_1_FN>,
    pub Reserved5: ::std::option::Option<PIBIO_STORAGE_RESERVED_2_FN>,
    pub AllocateMemory: ::std::option::Option<PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN>,
    pub FreeMemory: ::std::option::Option<PIBIO_FRAMEWORK_FREE_MEMORY_FN>,
    pub GetProperty: ::std::option::Option<PIBIO_FRAMEWORK_GET_PROPERTY_FN>,
    pub LockAndValidateSecureBuffer: ::std::option::Option<PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN>,
    pub ReleaseSecureBuffer: ::std::option::Option<PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN>,
    pub QueryAuthorizedEnrollments: ::std::option::Option<PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN>,
    pub DecryptSample: ::std::option::Option<PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WINBIO_FRAMEWORK_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WINBIO_FRAMEWORK_INTERFACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WINBIO_FRAMEWORK_INTERFACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_FRAMEWORK_INTERFACE").field("Version", &self.Version).field("Type", &self.Type).field("Size", &self.Size).field("AdapterId", &self.AdapterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WINBIO_FRAMEWORK_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Type == other.Type
            && self.Size == other.Size
            && self.AdapterId == other.AdapterId
            && self.SetUnitStatus.map(|f| f as usize) == other.SetUnitStatus.map(|f| f as usize)
            && self.VsmStorageAttach.map(|f| f as usize) == other.VsmStorageAttach.map(|f| f as usize)
            && self.VsmStorageDetach.map(|f| f as usize) == other.VsmStorageDetach.map(|f| f as usize)
            && self.VsmStorageClearContext.map(|f| f as usize) == other.VsmStorageClearContext.map(|f| f as usize)
            && self.VsmStorageCreateDatabase.map(|f| f as usize) == other.VsmStorageCreateDatabase.map(|f| f as usize)
            && self.VsmStorageOpenDatabase.map(|f| f as usize) == other.VsmStorageOpenDatabase.map(|f| f as usize)
            && self.VsmStorageCloseDatabase.map(|f| f as usize) == other.VsmStorageCloseDatabase.map(|f| f as usize)
            && self.VsmStorageDeleteRecord.map(|f| f as usize) == other.VsmStorageDeleteRecord.map(|f| f as usize)
            && self.VsmStorageNotifyPowerChange.map(|f| f as usize) == other.VsmStorageNotifyPowerChange.map(|f| f as usize)
            && self.VsmStoragePipelineInit.map(|f| f as usize) == other.VsmStoragePipelineInit.map(|f| f as usize)
            && self.VsmStoragePipelineCleanup.map(|f| f as usize) == other.VsmStoragePipelineCleanup.map(|f| f as usize)
            && self.VsmStorageActivate.map(|f| f as usize) == other.VsmStorageActivate.map(|f| f as usize)
            && self.VsmStorageDeactivate.map(|f| f as usize) == other.VsmStorageDeactivate.map(|f| f as usize)
            && self.VsmStorageQueryExtendedInfo.map(|f| f as usize) == other.VsmStorageQueryExtendedInfo.map(|f| f as usize)
            && self.VsmStorageCacheClear.map(|f| f as usize) == other.VsmStorageCacheClear.map(|f| f as usize)
            && self.VsmStorageCacheImportBegin.map(|f| f as usize) == other.VsmStorageCacheImportBegin.map(|f| f as usize)
            && self.VsmStorageCacheImportNext.map(|f| f as usize) == other.VsmStorageCacheImportNext.map(|f| f as usize)
            && self.VsmStorageCacheImportEnd.map(|f| f as usize) == other.VsmStorageCacheImportEnd.map(|f| f as usize)
            && self.VsmStorageCacheExportBegin.map(|f| f as usize) == other.VsmStorageCacheExportBegin.map(|f| f as usize)
            && self.VsmStorageCacheExportNext.map(|f| f as usize) == other.VsmStorageCacheExportNext.map(|f| f as usize)
            && self.VsmStorageCacheExportEnd.map(|f| f as usize) == other.VsmStorageCacheExportEnd.map(|f| f as usize)
            && self.VsmSensorAttach.map(|f| f as usize) == other.VsmSensorAttach.map(|f| f as usize)
            && self.VsmSensorDetach.map(|f| f as usize) == other.VsmSensorDetach.map(|f| f as usize)
            && self.VsmSensorClearContext.map(|f| f as usize) == other.VsmSensorClearContext.map(|f| f as usize)
            && self.VsmSensorPushDataToEngine.map(|f| f as usize) == other.VsmSensorPushDataToEngine.map(|f| f as usize)
            && self.VsmSensorNotifyPowerChange.map(|f| f as usize) == other.VsmSensorNotifyPowerChange.map(|f| f as usize)
            && self.VsmSensorPipelineInit.map(|f| f as usize) == other.VsmSensorPipelineInit.map(|f| f as usize)
            && self.VsmSensorPipelineCleanup.map(|f| f as usize) == other.VsmSensorPipelineCleanup.map(|f| f as usize)
            && self.VsmSensorActivate.map(|f| f as usize) == other.VsmSensorActivate.map(|f| f as usize)
            && self.VsmSensorDeactivate.map(|f| f as usize) == other.VsmSensorDeactivate.map(|f| f as usize)
            && self.VsmSensorAsyncImportRawBuffer.map(|f| f as usize) == other.VsmSensorAsyncImportRawBuffer.map(|f| f as usize)
            && self.VsmSensorAsyncImportSecureBuffer.map(|f| f as usize) == other.VsmSensorAsyncImportSecureBuffer.map(|f| f as usize)
            && self.Reserved1.map(|f| f as usize) == other.Reserved1.map(|f| f as usize)
            && self.Reserved2.map(|f| f as usize) == other.Reserved2.map(|f| f as usize)
            && self.Reserved3.map(|f| f as usize) == other.Reserved3.map(|f| f as usize)
            && self.Reserved4.map(|f| f as usize) == other.Reserved4.map(|f| f as usize)
            && self.Reserved5.map(|f| f as usize) == other.Reserved5.map(|f| f as usize)
            && self.AllocateMemory.map(|f| f as usize) == other.AllocateMemory.map(|f| f as usize)
            && self.FreeMemory.map(|f| f as usize) == other.FreeMemory.map(|f| f as usize)
            && self.GetProperty.map(|f| f as usize) == other.GetProperty.map(|f| f as usize)
            && self.LockAndValidateSecureBuffer.map(|f| f as usize) == other.LockAndValidateSecureBuffer.map(|f| f as usize)
            && self.ReleaseSecureBuffer.map(|f| f as usize) == other.ReleaseSecureBuffer.map(|f| f as usize)
            && self.QueryAuthorizedEnrollments.map(|f| f as usize) == other.QueryAuthorizedEnrollments.map(|f| f as usize)
            && self.DecryptSample.map(|f| f as usize) == other.DecryptSample.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WINBIO_FRAMEWORK_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WINBIO_FRAMEWORK_INTERFACE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_GESTURE_METADATA {
    pub Size: usize,
    pub BiometricType: u32,
    pub MatchType: u32,
    pub ProtectionType: u32,
}
impl WINBIO_GESTURE_METADATA {}
impl ::std::default::Default for WINBIO_GESTURE_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_GESTURE_METADATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_GESTURE_METADATA").field("Size", &self.Size).field("BiometricType", &self.BiometricType).field("MatchType", &self.MatchType).field("ProtectionType", &self.ProtectionType).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_GESTURE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.BiometricType == other.BiometricType && self.MatchType == other.MatchType && self.ProtectionType == other.ProtectionType
    }
}
impl ::std::cmp::Eq for WINBIO_GESTURE_METADATA {}
unsafe impl ::windows::runtime::Abi for WINBIO_GESTURE_METADATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_GET_INDICATOR {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
    pub IndicatorStatus: u32,
}
impl WINBIO_GET_INDICATOR {}
impl ::std::default::Default for WINBIO_GET_INDICATOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_GET_INDICATOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_GET_INDICATOR").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("IndicatorStatus", &self.IndicatorStatus).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_GET_INDICATOR {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.IndicatorStatus == other.IndicatorStatus
    }
}
impl ::std::cmp::Eq for WINBIO_GET_INDICATOR {}
unsafe impl ::windows::runtime::Abi for WINBIO_GET_INDICATOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_IDENTITY {
    pub Type: u32,
    pub Value: WINBIO_IDENTITY_0,
}
impl WINBIO_IDENTITY {}
impl ::std::default::Default for WINBIO_IDENTITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_IDENTITY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_IDENTITY {}
unsafe impl ::windows::runtime::Abi for WINBIO_IDENTITY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub union WINBIO_IDENTITY_0 {
    pub Null: u32,
    pub Wildcard: u32,
    pub TemplateGuid: ::windows::runtime::GUID,
    pub AccountSid: WINBIO_IDENTITY_0_0,
    pub SecureId: [u8; 32],
}
impl WINBIO_IDENTITY_0 {}
impl ::std::default::Default for WINBIO_IDENTITY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_IDENTITY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_IDENTITY_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_IDENTITY_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_IDENTITY_0_0 {
    pub Size: u32,
    pub Data: [u8; 68],
}
impl WINBIO_IDENTITY_0_0 {}
impl ::std::default::Default for WINBIO_IDENTITY_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_IDENTITY_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_AccountSid_e__Struct").field("Size", &self.Size).field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_IDENTITY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for WINBIO_IDENTITY_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_IDENTITY_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_I_EXTENDED_STATUS_INFORMATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(589826i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_I_MORE_DATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(589825i32 as _);
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_MAX_STRING_LEN: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_NOTIFY_WAKE {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
    pub Reason: u32,
}
impl WINBIO_NOTIFY_WAKE {}
impl ::std::default::Default for WINBIO_NOTIFY_WAKE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_NOTIFY_WAKE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_NOTIFY_WAKE").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("Reason", &self.Reason).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_NOTIFY_WAKE {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.Reason == other.Reason
    }
}
impl ::std::cmp::Eq for WINBIO_NOTIFY_WAKE {}
unsafe impl ::windows::runtime::Abi for WINBIO_NOTIFY_WAKE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WINBIO_PIPELINE {
    pub SensorHandle: super::super::Foundation::HANDLE,
    pub EngineHandle: super::super::Foundation::HANDLE,
    pub StorageHandle: super::super::Foundation::HANDLE,
    pub SensorInterface: *mut WINBIO_SENSOR_INTERFACE,
    pub EngineInterface: *mut WINBIO_ENGINE_INTERFACE,
    pub StorageInterface: *mut WINBIO_STORAGE_INTERFACE,
    pub SensorContext: *mut _WINIBIO_SENSOR_CONTEXT,
    pub EngineContext: *mut _WINIBIO_ENGINE_CONTEXT,
    pub StorageContext: *mut _WINIBIO_STORAGE_CONTEXT,
    pub FrameworkInterface: *mut WINBIO_FRAMEWORK_INTERFACE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WINBIO_PIPELINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WINBIO_PIPELINE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WINBIO_PIPELINE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_PIPELINE")
            .field("SensorHandle", &self.SensorHandle)
            .field("EngineHandle", &self.EngineHandle)
            .field("StorageHandle", &self.StorageHandle)
            .field("SensorInterface", &self.SensorInterface)
            .field("EngineInterface", &self.EngineInterface)
            .field("StorageInterface", &self.StorageInterface)
            .field("SensorContext", &self.SensorContext)
            .field("EngineContext", &self.EngineContext)
            .field("StorageContext", &self.StorageContext)
            .field("FrameworkInterface", &self.FrameworkInterface)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WINBIO_PIPELINE {
    fn eq(&self, other: &Self) -> bool {
        self.SensorHandle == other.SensorHandle && self.EngineHandle == other.EngineHandle && self.StorageHandle == other.StorageHandle && self.SensorInterface == other.SensorInterface && self.EngineInterface == other.EngineInterface && self.StorageInterface == other.StorageInterface && self.SensorContext == other.SensorContext && self.EngineContext == other.EngineContext && self.StorageContext == other.StorageContext && self.FrameworkInterface == other.FrameworkInterface
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WINBIO_PIPELINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WINBIO_PIPELINE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_POLICY_SOURCE(pub i32);
pub const WINBIO_POLICY_UNKNOWN: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(0i32);
pub const WINBIO_POLICY_DEFAULT: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(1i32);
pub const WINBIO_POLICY_LOCAL: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(2i32);
pub const WINBIO_POLICY_ADMIN: WINBIO_POLICY_SOURCE = WINBIO_POLICY_SOURCE(3i32);
impl ::std::convert::From<i32> for WINBIO_POLICY_SOURCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_POLICY_SOURCE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_POOL(pub u32);
pub const WINBIO_POOL_SYSTEM: WINBIO_POOL = WINBIO_POOL(1u32);
pub const WINBIO_POOL_PRIVATE: WINBIO_POOL = WINBIO_POOL(2u32);
impl ::std::convert::From<u32> for WINBIO_POOL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_POOL {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINBIO_POOL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINBIO_POOL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINBIO_POOL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINBIO_POOL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINBIO_POOL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_PRESENCE {
    pub Factor: u32,
    pub SubFactor: u8,
    pub Status: ::windows::runtime::HRESULT,
    pub RejectDetail: u32,
    pub Identity: WINBIO_IDENTITY,
    pub TrackingId: u64,
    pub Ticket: u64,
    pub Properties: WINBIO_PRESENCE_PROPERTIES,
    pub Authorization: WINBIO_PRESENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_PRESENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_PRESENCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_PRESENCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_PRESENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_PRESENCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_PRESENCE_0 {
    pub Size: u32,
    pub Data: [u8; 32],
}
impl WINBIO_PRESENCE_0 {}
impl ::std::default::Default for WINBIO_PRESENCE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_PRESENCE_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Authorization_e__Struct").field("Size", &self.Size).field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_PRESENCE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for WINBIO_PRESENCE_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_PRESENCE_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub union WINBIO_PRESENCE_PROPERTIES {
    pub FacialFeatures: WINBIO_PRESENCE_PROPERTIES_0,
    pub Iris: WINBIO_PRESENCE_PROPERTIES_1,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_PRESENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_PRESENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_PRESENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_PRESENCE_PROPERTIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_PRESENCE_PROPERTIES_0 {
    pub BoundingBox: super::super::Foundation::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_PRESENCE_PROPERTIES_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_PRESENCE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_PRESENCE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_PRESENCE_PROPERTIES_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_FacialFeatures_e__Struct").field("BoundingBox", &self.BoundingBox).field("Distance", &self.Distance).field("OpaqueEngineData", &self.OpaqueEngineData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BoundingBox == other.BoundingBox && self.Distance == other.Distance && self.OpaqueEngineData == other.OpaqueEngineData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_PRESENCE_PROPERTIES_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_PRESENCE_PROPERTIES_0_0 {
    pub AdapterId: ::windows::runtime::GUID,
    pub Data: [u32; 78],
}
impl WINBIO_PRESENCE_PROPERTIES_0_0 {}
impl ::std::default::Default for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_OpaqueEngineData_e__Struct").field("AdapterId", &self.AdapterId).field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterId == other.AdapterId && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_0_0 {}
unsafe impl ::windows::runtime::Abi for WINBIO_PRESENCE_PROPERTIES_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
pub struct WINBIO_PRESENCE_PROPERTIES_1 {
    pub EyeBoundingBox_1: super::super::Foundation::RECT,
    pub EyeBoundingBox_2: super::super::Foundation::RECT,
    pub PupilCenter_1: super::super::Foundation::POINT,
    pub PupilCenter_2: super::super::Foundation::POINT,
    pub Distance: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl WINBIO_PRESENCE_PROPERTIES_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WINBIO_PRESENCE_PROPERTIES_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WINBIO_PRESENCE_PROPERTIES_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Iris_e__Struct").field("EyeBoundingBox_1", &self.EyeBoundingBox_1).field("EyeBoundingBox_2", &self.EyeBoundingBox_2).field("PupilCenter_1", &self.PupilCenter_1).field("PupilCenter_2", &self.PupilCenter_2).field("Distance", &self.Distance).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_1 {
    fn eq(&self, other: &Self) -> bool {
        self.EyeBoundingBox_1 == other.EyeBoundingBox_1 && self.EyeBoundingBox_2 == other.EyeBoundingBox_2 && self.PupilCenter_1 == other.PupilCenter_1 && self.PupilCenter_2 == other.PupilCenter_2 && self.Distance == other.Distance
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WINBIO_PRESENCE_PROPERTIES_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
    pub PrivateSensorTypeInfo: WINBIO_DATA,
}
impl WINBIO_PRIVATE_SENSOR_TYPE_INFO {}
impl ::std::default::Default for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_PRIVATE_SENSOR_TYPE_INFO").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("PrivateSensorTypeInfo", &self.PrivateSensorTypeInfo).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.PrivateSensorTypeInfo == other.PrivateSensorTypeInfo
    }
}
impl ::std::cmp::Eq for WINBIO_PRIVATE_SENSOR_TYPE_INFO {}
unsafe impl ::windows::runtime::Abi for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_PROTECTION_POLICY {
    pub Version: u32,
    pub Identity: WINBIO_IDENTITY,
    pub DatabaseId: ::windows::runtime::GUID,
    pub UserState: u64,
    pub PolicySize: usize,
    pub Policy: [u8; 128],
}
impl WINBIO_PROTECTION_POLICY {}
impl ::std::default::Default for WINBIO_PROTECTION_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WINBIO_PROTECTION_POLICY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WINBIO_PROTECTION_POLICY {}
unsafe impl ::windows::runtime::Abi for WINBIO_PROTECTION_POLICY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_REGISTERED_FORMAT {
    pub Owner: u16,
    pub Type: u16,
}
impl WINBIO_REGISTERED_FORMAT {}
impl ::std::default::Default for WINBIO_REGISTERED_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_REGISTERED_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_REGISTERED_FORMAT").field("Owner", &self.Owner).field("Type", &self.Type).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_REGISTERED_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Owner == other.Owner && self.Type == other.Type
    }
}
impl ::std::cmp::Eq for WINBIO_REGISTERED_FORMAT {}
unsafe impl ::windows::runtime::Abi for WINBIO_REGISTERED_FORMAT {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_SECURE_BUFFER_HEADER_V1 {
    pub Type: u32,
    pub Size: u32,
    pub Flags: u32,
    pub ValidationTag: u64,
}
impl WINBIO_SECURE_BUFFER_HEADER_V1 {}
impl ::std::default::Default for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_SECURE_BUFFER_HEADER_V1").field("Type", &self.Type).field("Size", &self.Size).field("Flags", &self.Flags).field("ValidationTag", &self.ValidationTag).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Size == other.Size && self.Flags == other.Flags && self.ValidationTag == other.ValidationTag
    }
}
impl ::std::cmp::Eq for WINBIO_SECURE_BUFFER_HEADER_V1 {}
unsafe impl ::windows::runtime::Abi for WINBIO_SECURE_BUFFER_HEADER_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_SECURE_CONNECTION_DATA {
    pub Size: u32,
    pub Version: u16,
    pub Flags: u16,
    pub ModelCertificateSize: u32,
    pub IntermediateCA1Size: u32,
    pub IntermediateCA2Size: u32,
}
impl WINBIO_SECURE_CONNECTION_DATA {}
impl ::std::default::Default for WINBIO_SECURE_CONNECTION_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_SECURE_CONNECTION_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_SECURE_CONNECTION_DATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ModelCertificateSize", &self.ModelCertificateSize)
            .field("IntermediateCA1Size", &self.IntermediateCA1Size)
            .field("IntermediateCA2Size", &self.IntermediateCA2Size)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_SECURE_CONNECTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Flags == other.Flags && self.ModelCertificateSize == other.ModelCertificateSize && self.IntermediateCA1Size == other.IntermediateCA1Size && self.IntermediateCA2Size == other.IntermediateCA2Size
    }
}
impl ::std::cmp::Eq for WINBIO_SECURE_CONNECTION_DATA {}
unsafe impl ::windows::runtime::Abi for WINBIO_SECURE_CONNECTION_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_SECURE_CONNECTION_PARAMS {
    pub PayloadSize: u32,
    pub Version: u16,
    pub Flags: u16,
}
impl WINBIO_SECURE_CONNECTION_PARAMS {}
impl ::std::default::Default for WINBIO_SECURE_CONNECTION_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_SECURE_CONNECTION_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_SECURE_CONNECTION_PARAMS").field("PayloadSize", &self.PayloadSize).field("Version", &self.Version).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_SECURE_CONNECTION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.Version == other.Version && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for WINBIO_SECURE_CONNECTION_PARAMS {}
unsafe impl ::windows::runtime::Abi for WINBIO_SECURE_CONNECTION_PARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_SENSOR_ATTRIBUTES {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
    pub WinBioVersion: WINBIO_VERSION,
    pub SensorType: u32,
    pub SensorSubType: u32,
    pub Capabilities: u32,
    pub ManufacturerName: [u16; 256],
    pub ModelName: [u16; 256],
    pub SerialNumber: [u16; 256],
    pub FirmwareVersion: WINBIO_VERSION,
    pub SupportedFormatEntries: u32,
    pub SupportedFormat: [WINBIO_REGISTERED_FORMAT; 1],
}
impl WINBIO_SENSOR_ATTRIBUTES {}
impl ::std::default::Default for WINBIO_SENSOR_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_SENSOR_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_SENSOR_ATTRIBUTES")
            .field("PayloadSize", &self.PayloadSize)
            .field("WinBioHresult", &self.WinBioHresult)
            .field("WinBioVersion", &self.WinBioVersion)
            .field("SensorType", &self.SensorType)
            .field("SensorSubType", &self.SensorSubType)
            .field("Capabilities", &self.Capabilities)
            .field("ManufacturerName", &self.ManufacturerName)
            .field("ModelName", &self.ModelName)
            .field("SerialNumber", &self.SerialNumber)
            .field("FirmwareVersion", &self.FirmwareVersion)
            .field("SupportedFormatEntries", &self.SupportedFormatEntries)
            .field("SupportedFormat", &self.SupportedFormat)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_SENSOR_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize
            && self.WinBioHresult == other.WinBioHresult
            && self.WinBioVersion == other.WinBioVersion
            && self.SensorType == other.SensorType
            && self.SensorSubType == other.SensorSubType
            && self.Capabilities == other.Capabilities
            && self.ManufacturerName == other.ManufacturerName
            && self.ModelName == other.ModelName
            && self.SerialNumber == other.SerialNumber
            && self.FirmwareVersion == other.FirmwareVersion
            && self.SupportedFormatEntries == other.SupportedFormatEntries
            && self.SupportedFormat == other.SupportedFormat
    }
}
impl ::std::cmp::Eq for WINBIO_SENSOR_ATTRIBUTES {}
unsafe impl ::windows::runtime::Abi for WINBIO_SENSOR_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WINBIO_SENSOR_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::runtime::GUID,
    pub Attach: ::std::option::Option<PIBIO_SENSOR_ATTACH_FN>,
    pub Detach: ::std::option::Option<PIBIO_SENSOR_DETACH_FN>,
    pub ClearContext: ::std::option::Option<PIBIO_SENSOR_CLEAR_CONTEXT_FN>,
    pub QueryStatus: ::std::option::Option<PIBIO_SENSOR_QUERY_STATUS_FN>,
    pub Reset: ::std::option::Option<PIBIO_SENSOR_RESET_FN>,
    pub SetMode: ::std::option::Option<PIBIO_SENSOR_SET_MODE_FN>,
    pub SetIndicatorStatus: ::std::option::Option<PIBIO_SENSOR_SET_INDICATOR_STATUS_FN>,
    pub GetIndicatorStatus: ::std::option::Option<PIBIO_SENSOR_GET_INDICATOR_STATUS_FN>,
    pub StartCapture: ::std::option::Option<PIBIO_SENSOR_START_CAPTURE_FN>,
    pub FinishCapture: ::std::option::Option<PIBIO_SENSOR_FINISH_CAPTURE_FN>,
    pub ExportSensorData: ::std::option::Option<PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN>,
    pub Cancel: ::std::option::Option<PIBIO_SENSOR_CANCEL_FN>,
    pub PushDataToEngine: ::std::option::Option<PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN>,
    pub ControlUnit: ::std::option::Option<PIBIO_SENSOR_CONTROL_UNIT_FN>,
    pub ControlUnitPrivileged: ::std::option::Option<PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN>,
    pub NotifyPowerChange: ::std::option::Option<PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN>,
    pub PipelineInit: ::std::option::Option<PIBIO_SENSOR_PIPELINE_INIT_FN>,
    pub PipelineCleanup: ::std::option::Option<PIBIO_SENSOR_PIPELINE_CLEANUP_FN>,
    pub Activate: ::std::option::Option<PIBIO_SENSOR_ACTIVATE_FN>,
    pub Deactivate: ::std::option::Option<PIBIO_SENSOR_DEACTIVATE_FN>,
    pub QueryExtendedInfo: ::std::option::Option<PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN>,
    pub QueryCalibrationFormats: ::std::option::Option<PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN>,
    pub SetCalibrationFormat: ::std::option::Option<PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN>,
    pub AcceptCalibrationData: ::std::option::Option<PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN>,
    pub AsyncImportRawBuffer: ::std::option::Option<PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN>,
    pub AsyncImportSecureBuffer: ::std::option::Option<PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN>,
    pub QueryPrivateSensorType: ::std::option::Option<PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN>,
    pub ConnectSecure: ::std::option::Option<PIBIO_SENSOR_CONNECT_SECURE_FN>,
    pub StartCaptureEx: ::std::option::Option<PIBIO_SENSOR_START_CAPTURE_EX_FN>,
    pub StartNotifyWake: ::std::option::Option<PIBIO_SENSOR_START_NOTIFY_WAKE_FN>,
    pub FinishNotifyWake: ::std::option::Option<PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WINBIO_SENSOR_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WINBIO_SENSOR_INTERFACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WINBIO_SENSOR_INTERFACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_SENSOR_INTERFACE").field("Version", &self.Version).field("Type", &self.Type).field("Size", &self.Size).field("AdapterId", &self.AdapterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WINBIO_SENSOR_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Type == other.Type
            && self.Size == other.Size
            && self.AdapterId == other.AdapterId
            && self.Attach.map(|f| f as usize) == other.Attach.map(|f| f as usize)
            && self.Detach.map(|f| f as usize) == other.Detach.map(|f| f as usize)
            && self.ClearContext.map(|f| f as usize) == other.ClearContext.map(|f| f as usize)
            && self.QueryStatus.map(|f| f as usize) == other.QueryStatus.map(|f| f as usize)
            && self.Reset.map(|f| f as usize) == other.Reset.map(|f| f as usize)
            && self.SetMode.map(|f| f as usize) == other.SetMode.map(|f| f as usize)
            && self.SetIndicatorStatus.map(|f| f as usize) == other.SetIndicatorStatus.map(|f| f as usize)
            && self.GetIndicatorStatus.map(|f| f as usize) == other.GetIndicatorStatus.map(|f| f as usize)
            && self.StartCapture.map(|f| f as usize) == other.StartCapture.map(|f| f as usize)
            && self.FinishCapture.map(|f| f as usize) == other.FinishCapture.map(|f| f as usize)
            && self.ExportSensorData.map(|f| f as usize) == other.ExportSensorData.map(|f| f as usize)
            && self.Cancel.map(|f| f as usize) == other.Cancel.map(|f| f as usize)
            && self.PushDataToEngine.map(|f| f as usize) == other.PushDataToEngine.map(|f| f as usize)
            && self.ControlUnit.map(|f| f as usize) == other.ControlUnit.map(|f| f as usize)
            && self.ControlUnitPrivileged.map(|f| f as usize) == other.ControlUnitPrivileged.map(|f| f as usize)
            && self.NotifyPowerChange.map(|f| f as usize) == other.NotifyPowerChange.map(|f| f as usize)
            && self.PipelineInit.map(|f| f as usize) == other.PipelineInit.map(|f| f as usize)
            && self.PipelineCleanup.map(|f| f as usize) == other.PipelineCleanup.map(|f| f as usize)
            && self.Activate.map(|f| f as usize) == other.Activate.map(|f| f as usize)
            && self.Deactivate.map(|f| f as usize) == other.Deactivate.map(|f| f as usize)
            && self.QueryExtendedInfo.map(|f| f as usize) == other.QueryExtendedInfo.map(|f| f as usize)
            && self.QueryCalibrationFormats.map(|f| f as usize) == other.QueryCalibrationFormats.map(|f| f as usize)
            && self.SetCalibrationFormat.map(|f| f as usize) == other.SetCalibrationFormat.map(|f| f as usize)
            && self.AcceptCalibrationData.map(|f| f as usize) == other.AcceptCalibrationData.map(|f| f as usize)
            && self.AsyncImportRawBuffer.map(|f| f as usize) == other.AsyncImportRawBuffer.map(|f| f as usize)
            && self.AsyncImportSecureBuffer.map(|f| f as usize) == other.AsyncImportSecureBuffer.map(|f| f as usize)
            && self.QueryPrivateSensorType.map(|f| f as usize) == other.QueryPrivateSensorType.map(|f| f as usize)
            && self.ConnectSecure.map(|f| f as usize) == other.ConnectSecure.map(|f| f as usize)
            && self.StartCaptureEx.map(|f| f as usize) == other.StartCaptureEx.map(|f| f as usize)
            && self.StartNotifyWake.map(|f| f as usize) == other.StartNotifyWake.map(|f| f as usize)
            && self.FinishNotifyWake.map(|f| f as usize) == other.FinishNotifyWake.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WINBIO_SENSOR_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WINBIO_SENSOR_INTERFACE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINBIO_SETTING_SOURCE(pub u32);
pub const WINBIO_SETTING_SOURCE_INVALID: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(0u32);
pub const WINBIO_SETTING_SOURCE_DEFAULT: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(1u32);
pub const WINBIO_SETTING_SOURCE_LOCAL: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(3u32);
pub const WINBIO_SETTING_SOURCE_POLICY: WINBIO_SETTING_SOURCE = WINBIO_SETTING_SOURCE(2u32);
impl ::std::convert::From<u32> for WINBIO_SETTING_SOURCE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINBIO_SETTING_SOURCE {
    type Abi = Self;
}
impl ::std::ops::BitOr for WINBIO_SETTING_SOURCE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WINBIO_SETTING_SOURCE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WINBIO_SETTING_SOURCE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WINBIO_SETTING_SOURCE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WINBIO_SETTING_SOURCE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_SET_INDICATOR {
    pub PayloadSize: u32,
    pub IndicatorStatus: u32,
}
impl WINBIO_SET_INDICATOR {}
impl ::std::default::Default for WINBIO_SET_INDICATOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_SET_INDICATOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_SET_INDICATOR").field("PayloadSize", &self.PayloadSize).field("IndicatorStatus", &self.IndicatorStatus).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_SET_INDICATOR {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.IndicatorStatus == other.IndicatorStatus
    }
}
impl ::std::cmp::Eq for WINBIO_SET_INDICATOR {}
unsafe impl ::windows::runtime::Abi for WINBIO_SET_INDICATOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct WINBIO_STORAGE_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::runtime::GUID,
    pub Attach: ::std::option::Option<PIBIO_STORAGE_ATTACH_FN>,
    pub Detach: ::std::option::Option<PIBIO_STORAGE_DETACH_FN>,
    pub ClearContext: ::std::option::Option<PIBIO_STORAGE_CLEAR_CONTEXT_FN>,
    pub CreateDatabase: ::std::option::Option<PIBIO_STORAGE_CREATE_DATABASE_FN>,
    pub EraseDatabase: ::std::option::Option<PIBIO_STORAGE_ERASE_DATABASE_FN>,
    pub OpenDatabase: ::std::option::Option<PIBIO_STORAGE_OPEN_DATABASE_FN>,
    pub CloseDatabase: ::std::option::Option<PIBIO_STORAGE_CLOSE_DATABASE_FN>,
    pub GetDataFormat: ::std::option::Option<PIBIO_STORAGE_GET_DATA_FORMAT_FN>,
    pub GetDatabaseSize: ::std::option::Option<PIBIO_STORAGE_GET_DATABASE_SIZE_FN>,
    pub AddRecord: ::std::option::Option<PIBIO_STORAGE_ADD_RECORD_FN>,
    pub DeleteRecord: ::std::option::Option<PIBIO_STORAGE_DELETE_RECORD_FN>,
    pub QueryBySubject: ::std::option::Option<PIBIO_STORAGE_QUERY_BY_SUBJECT_FN>,
    pub QueryByContent: ::std::option::Option<PIBIO_STORAGE_QUERY_BY_CONTENT_FN>,
    pub GetRecordCount: ::std::option::Option<PIBIO_STORAGE_GET_RECORD_COUNT_FN>,
    pub FirstRecord: ::std::option::Option<PIBIO_STORAGE_FIRST_RECORD_FN>,
    pub NextRecord: ::std::option::Option<PIBIO_STORAGE_NEXT_RECORD_FN>,
    pub GetCurrentRecord: ::std::option::Option<PIBIO_STORAGE_GET_CURRENT_RECORD_FN>,
    pub ControlUnit: ::std::option::Option<PIBIO_STORAGE_CONTROL_UNIT_FN>,
    pub ControlUnitPrivileged: ::std::option::Option<PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN>,
    pub NotifyPowerChange: ::std::option::Option<PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN>,
    pub PipelineInit: ::std::option::Option<PIBIO_STORAGE_PIPELINE_INIT_FN>,
    pub PipelineCleanup: ::std::option::Option<PIBIO_STORAGE_PIPELINE_CLEANUP_FN>,
    pub Activate: ::std::option::Option<PIBIO_STORAGE_ACTIVATE_FN>,
    pub Deactivate: ::std::option::Option<PIBIO_STORAGE_DEACTIVATE_FN>,
    pub QueryExtendedInfo: ::std::option::Option<PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN>,
    pub NotifyDatabaseChange: ::std::option::Option<PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN>,
    pub Reserved1: ::std::option::Option<PIBIO_STORAGE_RESERVED_1_FN>,
    pub Reserved2: ::std::option::Option<PIBIO_STORAGE_RESERVED_2_FN>,
    pub UpdateRecordBegin: ::std::option::Option<PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN>,
    pub UpdateRecordCommit: ::std::option::Option<PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl WINBIO_STORAGE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for WINBIO_STORAGE_INTERFACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for WINBIO_STORAGE_INTERFACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_STORAGE_INTERFACE").field("Version", &self.Version).field("Type", &self.Type).field("Size", &self.Size).field("AdapterId", &self.AdapterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for WINBIO_STORAGE_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Type == other.Type
            && self.Size == other.Size
            && self.AdapterId == other.AdapterId
            && self.Attach.map(|f| f as usize) == other.Attach.map(|f| f as usize)
            && self.Detach.map(|f| f as usize) == other.Detach.map(|f| f as usize)
            && self.ClearContext.map(|f| f as usize) == other.ClearContext.map(|f| f as usize)
            && self.CreateDatabase.map(|f| f as usize) == other.CreateDatabase.map(|f| f as usize)
            && self.EraseDatabase.map(|f| f as usize) == other.EraseDatabase.map(|f| f as usize)
            && self.OpenDatabase.map(|f| f as usize) == other.OpenDatabase.map(|f| f as usize)
            && self.CloseDatabase.map(|f| f as usize) == other.CloseDatabase.map(|f| f as usize)
            && self.GetDataFormat.map(|f| f as usize) == other.GetDataFormat.map(|f| f as usize)
            && self.GetDatabaseSize.map(|f| f as usize) == other.GetDatabaseSize.map(|f| f as usize)
            && self.AddRecord.map(|f| f as usize) == other.AddRecord.map(|f| f as usize)
            && self.DeleteRecord.map(|f| f as usize) == other.DeleteRecord.map(|f| f as usize)
            && self.QueryBySubject.map(|f| f as usize) == other.QueryBySubject.map(|f| f as usize)
            && self.QueryByContent.map(|f| f as usize) == other.QueryByContent.map(|f| f as usize)
            && self.GetRecordCount.map(|f| f as usize) == other.GetRecordCount.map(|f| f as usize)
            && self.FirstRecord.map(|f| f as usize) == other.FirstRecord.map(|f| f as usize)
            && self.NextRecord.map(|f| f as usize) == other.NextRecord.map(|f| f as usize)
            && self.GetCurrentRecord.map(|f| f as usize) == other.GetCurrentRecord.map(|f| f as usize)
            && self.ControlUnit.map(|f| f as usize) == other.ControlUnit.map(|f| f as usize)
            && self.ControlUnitPrivileged.map(|f| f as usize) == other.ControlUnitPrivileged.map(|f| f as usize)
            && self.NotifyPowerChange.map(|f| f as usize) == other.NotifyPowerChange.map(|f| f as usize)
            && self.PipelineInit.map(|f| f as usize) == other.PipelineInit.map(|f| f as usize)
            && self.PipelineCleanup.map(|f| f as usize) == other.PipelineCleanup.map(|f| f as usize)
            && self.Activate.map(|f| f as usize) == other.Activate.map(|f| f as usize)
            && self.Deactivate.map(|f| f as usize) == other.Deactivate.map(|f| f as usize)
            && self.QueryExtendedInfo.map(|f| f as usize) == other.QueryExtendedInfo.map(|f| f as usize)
            && self.NotifyDatabaseChange.map(|f| f as usize) == other.NotifyDatabaseChange.map(|f| f as usize)
            && self.Reserved1.map(|f| f as usize) == other.Reserved1.map(|f| f as usize)
            && self.Reserved2.map(|f| f as usize) == other.Reserved2.map(|f| f as usize)
            && self.UpdateRecordBegin.map(|f| f as usize) == other.UpdateRecordBegin.map(|f| f as usize)
            && self.UpdateRecordCommit.map(|f| f as usize) == other.UpdateRecordCommit.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for WINBIO_STORAGE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for WINBIO_STORAGE_INTERFACE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_STORAGE_RECORD {
    pub Identity: *mut WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub IndexVector: *mut u32,
    pub IndexElementCount: usize,
    pub TemplateBlob: *mut u8,
    pub TemplateBlobSize: usize,
    pub PayloadBlob: *mut u8,
    pub PayloadBlobSize: usize,
}
impl WINBIO_STORAGE_RECORD {}
impl ::std::default::Default for WINBIO_STORAGE_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_STORAGE_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_STORAGE_RECORD")
            .field("Identity", &self.Identity)
            .field("SubFactor", &self.SubFactor)
            .field("IndexVector", &self.IndexVector)
            .field("IndexElementCount", &self.IndexElementCount)
            .field("TemplateBlob", &self.TemplateBlob)
            .field("TemplateBlobSize", &self.TemplateBlobSize)
            .field("PayloadBlob", &self.PayloadBlob)
            .field("PayloadBlobSize", &self.PayloadBlobSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_STORAGE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Identity == other.Identity && self.SubFactor == other.SubFactor && self.IndexVector == other.IndexVector && self.IndexElementCount == other.IndexElementCount && self.TemplateBlob == other.TemplateBlob && self.TemplateBlobSize == other.TemplateBlobSize && self.PayloadBlob == other.PayloadBlob && self.PayloadBlobSize == other.PayloadBlobSize
    }
}
impl ::std::cmp::Eq for WINBIO_STORAGE_RECORD {}
unsafe impl ::windows::runtime::Abi for WINBIO_STORAGE_RECORD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_STORAGE_SCHEMA {
    pub BiometricFactor: u32,
    pub DatabaseId: ::windows::runtime::GUID,
    pub DataFormat: ::windows::runtime::GUID,
    pub Attributes: u32,
    pub FilePath: [u16; 256],
    pub ConnectionString: [u16; 256],
}
impl WINBIO_STORAGE_SCHEMA {}
impl ::std::default::Default for WINBIO_STORAGE_SCHEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_STORAGE_SCHEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_STORAGE_SCHEMA").field("BiometricFactor", &self.BiometricFactor).field("DatabaseId", &self.DatabaseId).field("DataFormat", &self.DataFormat).field("Attributes", &self.Attributes).field("FilePath", &self.FilePath).field("ConnectionString", &self.ConnectionString).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_STORAGE_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.BiometricFactor == other.BiometricFactor && self.DatabaseId == other.DatabaseId && self.DataFormat == other.DataFormat && self.Attributes == other.Attributes && self.FilePath == other.FilePath && self.ConnectionString == other.ConnectionString
    }
}
impl ::std::cmp::Eq for WINBIO_STORAGE_SCHEMA {}
unsafe impl ::windows::runtime::Abi for WINBIO_STORAGE_SCHEMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_SUPPORTED_ALGORITHMS {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::runtime::HRESULT,
    pub NumberOfAlgorithms: u32,
    pub AlgorithmData: WINBIO_DATA,
}
impl WINBIO_SUPPORTED_ALGORITHMS {}
impl ::std::default::Default for WINBIO_SUPPORTED_ALGORITHMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_SUPPORTED_ALGORITHMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_SUPPORTED_ALGORITHMS").field("PayloadSize", &self.PayloadSize).field("WinBioHresult", &self.WinBioHresult).field("NumberOfAlgorithms", &self.NumberOfAlgorithms).field("AlgorithmData", &self.AlgorithmData).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_SUPPORTED_ALGORITHMS {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.WinBioHresult == other.WinBioHresult && self.NumberOfAlgorithms == other.NumberOfAlgorithms && self.AlgorithmData == other.AlgorithmData
    }
}
impl ::std::cmp::Eq for WINBIO_SUPPORTED_ALGORITHMS {}
unsafe impl ::windows::runtime::Abi for WINBIO_SUPPORTED_ALGORITHMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_UNIT_SCHEMA {
    pub UnitId: u32,
    pub PoolType: u32,
    pub BiometricFactor: u32,
    pub SensorSubType: u32,
    pub Capabilities: u32,
    pub DeviceInstanceId: [u16; 256],
    pub Description: [u16; 256],
    pub Manufacturer: [u16; 256],
    pub Model: [u16; 256],
    pub SerialNumber: [u16; 256],
    pub FirmwareVersion: WINBIO_VERSION,
}
impl WINBIO_UNIT_SCHEMA {}
impl ::std::default::Default for WINBIO_UNIT_SCHEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_UNIT_SCHEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_UNIT_SCHEMA")
            .field("UnitId", &self.UnitId)
            .field("PoolType", &self.PoolType)
            .field("BiometricFactor", &self.BiometricFactor)
            .field("SensorSubType", &self.SensorSubType)
            .field("Capabilities", &self.Capabilities)
            .field("DeviceInstanceId", &self.DeviceInstanceId)
            .field("Description", &self.Description)
            .field("Manufacturer", &self.Manufacturer)
            .field("Model", &self.Model)
            .field("SerialNumber", &self.SerialNumber)
            .field("FirmwareVersion", &self.FirmwareVersion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_UNIT_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.PoolType == other.PoolType && self.BiometricFactor == other.BiometricFactor && self.SensorSubType == other.SensorSubType && self.Capabilities == other.Capabilities && self.DeviceInstanceId == other.DeviceInstanceId && self.Description == other.Description && self.Manufacturer == other.Manufacturer && self.Model == other.Model && self.SerialNumber == other.SerialNumber && self.FirmwareVersion == other.FirmwareVersion
    }
}
impl ::std::cmp::Eq for WINBIO_UNIT_SCHEMA {}
unsafe impl ::windows::runtime::Abi for WINBIO_UNIT_SCHEMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_UPDATE_FIRMWARE {
    pub PayloadSize: u32,
    pub FirmwareData: WINBIO_DATA,
}
impl WINBIO_UPDATE_FIRMWARE {}
impl ::std::default::Default for WINBIO_UPDATE_FIRMWARE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_UPDATE_FIRMWARE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_UPDATE_FIRMWARE").field("PayloadSize", &self.PayloadSize).field("FirmwareData", &self.FirmwareData).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_UPDATE_FIRMWARE {
    fn eq(&self, other: &Self) -> bool {
        self.PayloadSize == other.PayloadSize && self.FirmwareData == other.FirmwareData
    }
}
impl ::std::cmp::Eq for WINBIO_UPDATE_FIRMWARE {}
unsafe impl ::windows::runtime::Abi for WINBIO_UPDATE_FIRMWARE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub struct WINBIO_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl WINBIO_VERSION {}
impl ::std::default::Default for WINBIO_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINBIO_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINBIO_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::std::cmp::PartialEq for WINBIO_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::std::cmp::Eq for WINBIO_VERSION {}
unsafe impl ::windows::runtime::Abi for WINBIO_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_WBDI_MAJOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
pub const WINBIO_WBDI_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioAcquireFocus() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAcquireFocus() -> ::windows::runtime::HRESULT;
        }
        WinBioAcquireFocus().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioAsyncEnumBiometricUnits(::std::mem::transmute(frameworkhandle), ::std::mem::transmute(factor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioAsyncEnumDatabases(::std::mem::transmute(frameworkhandle), ::std::mem::transmute(factor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioAsyncEnumServiceProviders(::std::mem::transmute(frameworkhandle), ::std::mem::transmute(factor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioAsyncMonitorFrameworkChanges(::std::mem::transmute(frameworkhandle), ::std::mem::transmute(changetypes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioAsyncOpenFramework<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: Param1, messagecode: u32, callbackroutine: ::std::option::Option<PWINBIO_ASYNC_COMPLETION_CALLBACK>, userdata: *const ::std::ffi::c_void, asynchronousopen: Param5) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncOpenFramework(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: ::windows::runtime::RawPtr, userdata: *const ::std::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, frameworkhandle: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinBioAsyncOpenFramework(::std::mem::transmute(notificationmethod), targetwindow.into_param().abi(), ::std::mem::transmute(messagecode), ::std::mem::transmute(callbackroutine), ::std::mem::transmute(userdata), asynchronousopen.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioAsyncOpenSession<'a, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param11: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
    factor: u32,
    pooltype: WINBIO_POOL,
    flags: u32,
    unitarray: *const u32,
    unitcount: usize,
    databaseid: *const ::windows::runtime::GUID,
    notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD,
    targetwindow: Param7,
    messagecode: u32,
    callbackroutine: ::std::option::Option<PWINBIO_ASYNC_COMPLETION_CALLBACK>,
    userdata: *const ::std::ffi::c_void,
    asynchronousopen: Param11,
) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::runtime::GUID, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: ::windows::runtime::RawPtr, userdata: *const ::std::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, sessionhandle: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinBioAsyncOpenSession(
            ::std::mem::transmute(factor),
            ::std::mem::transmute(pooltype),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(unitarray),
            ::std::mem::transmute(unitcount),
            ::std::mem::transmute(databaseid),
            ::std::mem::transmute(notificationmethod),
            targetwindow.into_param().abi(),
            ::std::mem::transmute(messagecode),
            ::std::mem::transmute(callbackroutine),
            ::std::mem::transmute(userdata),
            asynchronousopen.into_param().abi(),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioCancel(sessionhandle: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCancel(sessionhandle: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioCancel(::std::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: *mut u32, sample: *mut *mut WINBIO_BIR, samplesize: *mut usize, rejectdetail: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: *mut u32, sample: *mut *mut WINBIO_BIR, samplesize: *mut usize, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WinBioCaptureSample(::std::mem::transmute(sessionhandle), ::std::mem::transmute(purpose), ::std::mem::transmute(flags), ::std::mem::transmute(unitid), ::std::mem::transmute(sample), ::std::mem::transmute(samplesize), ::std::mem::transmute(rejectdetail)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: ::std::option::Option<PWINBIO_CAPTURE_CALLBACK>, capturecallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: ::windows::runtime::RawPtr, capturecallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WinBioCaptureSampleWithCallback(::std::mem::transmute(sessionhandle), ::std::mem::transmute(purpose), ::std::mem::transmute(flags), ::std::mem::transmute(capturecallback), ::std::mem::transmute(capturecallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioCloseFramework(frameworkhandle: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCloseFramework(frameworkhandle: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioCloseFramework(::std::mem::transmute(frameworkhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioCloseSession(sessionhandle: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCloseSession(sessionhandle: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioCloseSession(::std::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WinBioControlUnit(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(unitid),
            ::std::mem::transmute(component),
            ::std::mem::transmute(controlcode),
            ::std::mem::transmute(sendbuffer),
            ::std::mem::transmute(sendbuffersize),
            ::std::mem::transmute(receivebuffer),
            ::std::mem::transmute(receivebuffersize),
            ::std::mem::transmute(receivedatasize),
            ::std::mem::transmute(operationstatus),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WinBioControlUnitPrivileged(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(unitid),
            ::std::mem::transmute(component),
            ::std::mem::transmute(controlcode),
            ::std::mem::transmute(sendbuffer),
            ::std::mem::transmute(sendbuffersize),
            ::std::mem::transmute(receivebuffer),
            ::std::mem::transmute(receivebuffersize),
            ::std::mem::transmute(receivedatasize),
            ::std::mem::transmute(operationstatus),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::runtime::HRESULT;
        }
        WinBioDeleteTemplate(::std::mem::transmute(sessionhandle), ::std::mem::transmute(unitid), ::std::mem::transmute(identity), ::std::mem::transmute(subfactor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioEnrollBegin(::std::mem::transmute(sessionhandle), ::std::mem::transmute(subfactor), ::std::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnrollCapture(sessionhandle: u32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollCapture(sessionhandle: u32, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinBioEnrollCapture(::std::mem::transmute(sessionhandle), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: ::std::option::Option<PWINBIO_ENROLL_CAPTURE_CALLBACK>, enrollcallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: ::windows::runtime::RawPtr, enrollcallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WinBioEnrollCaptureWithCallback(::std::mem::transmute(sessionhandle), ::std::mem::transmute(enrollcallback), ::std::mem::transmute(enrollcallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnrollCommit(sessionhandle: u32, identity: *mut WINBIO_IDENTITY, isnewtemplate: *mut u8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollCommit(sessionhandle: u32, identity: *mut WINBIO_IDENTITY, isnewtemplate: *mut u8) -> ::windows::runtime::HRESULT;
        }
        WinBioEnrollCommit(::std::mem::transmute(sessionhandle), ::std::mem::transmute(identity), ::std::mem::transmute(isnewtemplate)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnrollDiscard(sessionhandle: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollDiscard(sessionhandle: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioEnrollDiscard(::std::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> ::windows::runtime::HRESULT;
        }
        WinBioEnrollSelect(::std::mem::transmute(sessionhandle), ::std::mem::transmute(selectorvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> ::windows::runtime::HRESULT;
        }
        WinBioEnumBiometricUnits(::std::mem::transmute(factor), ::std::mem::transmute(unitschemaarray), ::std::mem::transmute(unitcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> ::windows::runtime::HRESULT;
        }
        WinBioEnumDatabases(::std::mem::transmute(factor), ::std::mem::transmute(storageschemaarray), ::std::mem::transmute(storagecount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: *mut usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: *mut usize) -> ::windows::runtime::HRESULT;
        }
        WinBioEnumEnrollments(::std::mem::transmute(sessionhandle), ::std::mem::transmute(unitid), ::std::mem::transmute(identity), ::std::mem::transmute(subfactorarray), ::std::mem::transmute(subfactorcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> ::windows::runtime::HRESULT;
        }
        WinBioEnumServiceProviders(::std::mem::transmute(factor), ::std::mem::transmute(bspschemaarray), ::std::mem::transmute(bspcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioFree(address: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioFree(address: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WinBioFree(::std::mem::transmute(address)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioGetCredentialState<'a, Param0: ::windows::runtime::IntoParam<'a, WINBIO_IDENTITY>>(identity: Param0, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::runtime::Result<WINBIO_CREDENTIAL_STATE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetCredentialState(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE, credentialstate: *mut WINBIO_CREDENTIAL_STATE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WINBIO_CREDENTIAL_STATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinBioGetCredentialState(identity.into_param().abi(), ::std::mem::transmute(r#type), &mut result__).from_abi::<WINBIO_CREDENTIAL_STATE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
        }
        ::std::mem::transmute(WinBioGetDomainLogonSetting(::std::mem::transmute(value), ::std::mem::transmute(source)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
        }
        ::std::mem::transmute(WinBioGetEnabledSetting(::std::mem::transmute(value), ::std::mem::transmute(source)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY, enrolledfactors: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinBioGetEnrolledFactors(::std::mem::transmute(accountowner), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
        }
        ::std::mem::transmute(WinBioGetLogonSetting(::std::mem::transmute(value), ::std::mem::transmute(source)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::std::ffi::c_void, propertybuffersize: *mut usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::std::ffi::c_void, propertybuffersize: *mut usize) -> ::windows::runtime::HRESULT;
        }
        WinBioGetProperty(::std::mem::transmute(sessionhandle), ::std::mem::transmute(propertytype), ::std::mem::transmute(propertyid), ::std::mem::transmute(unitid), ::std::mem::transmute(identity), ::std::mem::transmute(subfactor), ::std::mem::transmute(propertybuffer), ::std::mem::transmute(propertybuffersize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioIdentify(sessionhandle: u32, unitid: *mut u32, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioIdentify(sessionhandle: u32, unitid: *mut u32, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WinBioIdentify(::std::mem::transmute(sessionhandle), ::std::mem::transmute(unitid), ::std::mem::transmute(identity), ::std::mem::transmute(subfactor), ::std::mem::transmute(rejectdetail)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: ::std::option::Option<PWINBIO_IDENTIFY_CALLBACK>, identifycallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: ::windows::runtime::RawPtr, identifycallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WinBioIdentifyWithCallback(::std::mem::transmute(sessionhandle), ::std::mem::transmute(identifycallback), ::std::mem::transmute(identifycallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioImproveBegin(::std::mem::transmute(sessionhandle), ::std::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioImproveEnd(sessionhandle: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioImproveEnd(sessionhandle: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioImproveEnd(::std::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioLocateSensor(sessionhandle: u32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioLocateSensor(sessionhandle: u32, unitid: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinBioLocateSensor(::std::mem::transmute(sessionhandle), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: ::std::option::Option<PWINBIO_LOCATE_SENSOR_CALLBACK>, locatecallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: ::windows::runtime::RawPtr, locatecallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WinBioLocateSensorWithCallback(::std::mem::transmute(sessionhandle), ::std::mem::transmute(locatecallback), ::std::mem::transmute(locatecallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioLockUnit(::std::mem::transmute(sessionhandle), ::std::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioLogonIdentifiedUser(::std::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioMonitorPresence(::std::mem::transmute(sessionhandle), ::std::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::runtime::GUID, sessionhandle: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WinBioOpenSession(::std::mem::transmute(factor), ::std::mem::transmute(pooltype), ::std::mem::transmute(flags), ::std::mem::transmute(unitarray), ::std::mem::transmute(unitcount), ::std::mem::transmute(databaseid), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: ::std::option::Option<PWINBIO_EVENT_CALLBACK>, eventcallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: ::windows::runtime::RawPtr, eventcallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WinBioRegisterEventMonitor(::std::mem::transmute(sessionhandle), ::std::mem::transmute(eventmask), ::std::mem::transmute(eventcallback), ::std::mem::transmute(eventcallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioReleaseFocus() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioReleaseFocus() -> ::windows::runtime::HRESULT;
        }
        WinBioReleaseFocus().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioRemoveAllCredentials() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioRemoveAllCredentials() -> ::windows::runtime::HRESULT;
        }
        WinBioRemoveAllCredentials().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioRemoveAllDomainCredentials() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioRemoveAllDomainCredentials() -> ::windows::runtime::HRESULT;
        }
        WinBioRemoveAllDomainCredentials().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioRemoveCredential<'a, Param0: ::windows::runtime::IntoParam<'a, WINBIO_IDENTITY>>(identity: Param0, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioRemoveCredential(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::runtime::HRESULT;
        }
        WinBioRemoveCredential(identity.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: *const u8, credentialsize: usize, format: WINBIO_CREDENTIAL_FORMAT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: *const u8, credentialsize: usize, format: WINBIO_CREDENTIAL_FORMAT) -> ::windows::runtime::HRESULT;
        }
        WinBioSetCredential(::std::mem::transmute(r#type), ::std::mem::transmute(credential), ::std::mem::transmute(credentialsize), ::std::mem::transmute(format)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *const ::std::ffi::c_void, propertybuffersize: usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *const ::std::ffi::c_void, propertybuffersize: usize) -> ::windows::runtime::HRESULT;
        }
        WinBioSetProperty(::std::mem::transmute(sessionhandle), ::std::mem::transmute(propertytype), ::std::mem::transmute(propertyid), ::std::mem::transmute(unitid), ::std::mem::transmute(identity), ::std::mem::transmute(subfactor), ::std::mem::transmute(propertybuffer), ::std::mem::transmute(propertybuffersize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioUnlockUnit(::std::mem::transmute(sessionhandle), ::std::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioUnregisterEventMonitor(::std::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: *mut u32, r#match: *mut u8, rejectdetail: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: *mut u32, r#match: *mut u8, rejectdetail: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WinBioVerify(::std::mem::transmute(sessionhandle), ::std::mem::transmute(identity), ::std::mem::transmute(subfactor), ::std::mem::transmute(unitid), ::std::mem::transmute(r#match), ::std::mem::transmute(rejectdetail)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: ::std::option::Option<PWINBIO_VERIFY_CALLBACK>, verifycallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: ::windows::runtime::RawPtr, verifycallbackcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        WinBioVerifyWithCallback(::std::mem::transmute(sessionhandle), ::std::mem::transmute(identity), ::std::mem::transmute(subfactor), ::std::mem::transmute(verifycallback), ::std::mem::transmute(verifycallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
#[inline]
pub unsafe fn WinBioWait(sessionhandle: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioWait(sessionhandle: u32) -> ::windows::runtime::HRESULT;
        }
        WinBioWait(::std::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _WINIBIO_ENGINE_CONTEXT(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _WINIBIO_SENSOR_CONTEXT(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _WINIBIO_STORAGE_CONTEXT(pub u8);
