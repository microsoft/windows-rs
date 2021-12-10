#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const FACILITY_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const FACILITY_WINBIO: u32 = 9u32;
pub const GUID_DEVINTERFACE_BIOMETRIC_READER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b5183a_99ea_4cc3_ad6b_80ca8d715b80);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const IOCTL_BIOMETRIC_VENDOR: u32 = 4464640u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, typeinfobufferaddress: *const u8, typeinfobuffersize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplebuffer: *const WINBIO_BIR, samplesize: usize, purpose: u8, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_ATTACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, duplicate: *mut super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CLEAR_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_COMMIT_ENROLLMENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, payloadblob: *const u8, payloadblobsize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CONTROL_UNIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *mut *mut u8, noncesize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_ENROLLMENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_CREATE_KEY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, key: *const u8, keysize: usize, keyidentifier: *mut u8, keyidentifiersize: usize, resultsize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DEACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DETACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_DISCARD_ENROLLMENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, flags: u8, samplebuffer: *mut *mut WINBIO_BIR, samplesize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, hashvalue: *mut *mut u8, hashsize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_ALL_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, presencecount: *mut usize, presencearray: *mut *mut WINBIO_PRESENCE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *const u8, noncesize: usize, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32, authentication: *mut *mut u8, authenticationsize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, payloadblob: *mut *mut u8, payloadblobsize: *mut usize, hashvalue: *mut *mut u8, hashsize: *mut usize, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, nonce: *const u8, noncesize: usize, keyidentifier: *const u8, keyidentifiersize: usize, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32, authorization: *mut *mut u8, authorizationsize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_PIPELINE_CLEANUP_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_PIPELINE_INIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, discardandrepeatcapture: *mut super::super::Foundation::BOOLEAN, calibrationbuffer: *mut u8, calibrationbuffersize: *mut usize, maxbuffersize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, enrollmentstatus: *mut WINBIO_EXTENDED_ENROLLMENT_STATUS, enrollmentstatussize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, engineinfo: *mut WINBIO_EXTENDED_ENGINE_INFO, engineinfosize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, algorithmcount: *mut usize, algorithmbuffersize: *mut usize, algorithmbuffer: *mut *mut u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indexelementcount: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, standardformat: *mut WINBIO_REGISTERED_FORMAT, vendorformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplehint: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_REFRESH_CACHE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_RESERVED_1_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, formatarray: *const ::windows::core::GUID, formatcount: usize, selectedformat: *mut ::windows::core::GUID, maxbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, policyitemarray: *const WINBIO_ACCOUNT_POLICY, policyitemcount: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, parameters: *const WINBIO_EXTENDED_ENROLLMENT_PARAMETERS) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, selectorvalue: u64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_SET_HASH_ALGORITHM_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, algorithmbuffersize: usize, algorithmbuffer: *const u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_UPDATE_ENROLLMENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_ENGINE_VERIFY_FEATURE_SET_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, r#match: *mut super::super::Foundation::BOOLEAN, payloadblob: *mut *mut u8, payloadblobsize: *mut usize, hashvalue: *mut *mut u8, hashsize: *mut usize, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, allocationsize: usize, address: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_FREE_MEMORY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, address: *const ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_GET_PROPERTY_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, propertytype: u32, propertyid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::core::ffi::c_void, propertybuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::core::GUID, securebufferaddress: *mut *mut ::core::ffi::c_void, securebuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::core::GUID) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, extendedstatus: *const WINBIO_EXTENDED_UNIT_STATUS, extendedstatussize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, requiredcapacity: *mut usize, maxbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, bufferaddress: *mut u8, buffersize: usize, returneddatasize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, requiredcapacity: usize, maxbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, bufferaddress: *const u8, buffersize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, authentication: *const u8, authenticationsize: usize, iv: *const u8, ivsize: usize, encrypteddata: *mut u8, encrypteddatasize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, secureidentitycount: *mut usize, secureidentities: *mut *mut WINBIO_IDENTITY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, reserved1: usize, reserved2: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, reserved1: *mut u8, reserved2: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, calibrationbuffer: *const u8, calibrationbuffersize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rawbufferaddress: *const u8, rawbuffersize: usize, resultbufferaddress: *mut *mut u8, resultbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, securebufferidentifier: ::windows::core::GUID, metadatabufferaddress: *const u8, metadatabuffersize: usize, resultbufferaddress: *mut *mut u8, resultbuffersize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_ATTACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CANCEL_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CLEAR_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONNECT_SECURE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, connectionparams: *const WINBIO_SECURE_CONNECTION_PARAMS, connectiondata: *mut *mut WINBIO_SECURE_CONNECTION_DATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONTROL_UNIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_DEACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_DETACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, samplebuffer: *mut *mut WINBIO_BIR, samplesize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_FINISH_CAPTURE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, reason: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_GET_INDICATOR_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indicatorstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PIPELINE_CLEANUP_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PIPELINE_INIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, flags: u8, rejectdetail: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, formatarray: *mut ::windows::core::GUID, formatarraysize: usize, formatcount: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, sensorinfo: *mut WINBIO_EXTENDED_SENSOR_INFO, sensorinfosize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, typeinfobufferaddress: *mut u8, typeinfobuffersize: usize, typeinfodatasize: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_QUERY_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, status: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_RESET_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, format: *const ::windows::core::GUID) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_INDICATOR_STATUS_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, indicatorstatus: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_SET_MODE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, mode: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_CAPTURE_EX_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, nonce: *const u8, noncesize: usize, flags: u8, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_CAPTURE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, purpose: u8, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_SENSOR_START_NOTIFY_WAKE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, overlapped: *mut *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ADD_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *const WINBIO_STORAGE_RECORD) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ATTACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CLEAR_CONTEXT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CLOSE_DATABASE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CONTROL_UNIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_CREATE_DATABASE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::core::GUID, factor: u32, format: *const ::windows::core::GUID, filepath: super::super::Foundation::PWSTR, connectstring: super::super::Foundation::PWSTR, indexelementcount: usize, initialsize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DEACTIVATE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DELETE_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_DETACH_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_ERASE_DATABASE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::core::GUID, filepath: super::super::Foundation::PWSTR, connectstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_FIRST_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_CURRENT_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *mut WINBIO_STORAGE_RECORD) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_DATABASE_SIZE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, availablerecordcount: *mut usize, totalrecordcount: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_DATA_FORMAT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, format: *mut ::windows::core::GUID, version: *mut WINBIO_VERSION) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_GET_RECORD_COUNT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcount: *mut usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NEXT_RECORD_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordsadded: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, powereventtype: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_OPEN_DATABASE_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, databaseid: *const ::windows::core::GUID, filepath: super::super::Foundation::PWSTR, connectstring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_PIPELINE_CLEANUP_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_PIPELINE_INIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_BY_CONTENT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, subfactor: u8, indexvector: *const u32, indexelementcount: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_BY_SUBJECT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, storageinfo: *mut WINBIO_EXTENDED_STORAGE_INFO, storageinfosize: usize) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_RESERVED_1_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY, reserved1: *mut u64, reserved2: *mut u64) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_RESERVED_2_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *mut WINBIO_IDENTITY) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, identity: *const WINBIO_IDENTITY, subfactor: u8, recordcontents: *mut WINBIO_STORAGE_RECORD) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN = ::core::option::Option<unsafe extern "system" fn(pipeline: *mut WINBIO_PIPELINE, recordcontents: *const WINBIO_STORAGE_RECORD) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWINBIO_ASYNC_COMPLETION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(asyncresult: *const WINBIO_ASYNC_RESULT)>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type PWINBIO_CAPTURE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(capturecallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, unitid: u32, sample: *const WINBIO_BIR, samplesize: usize, rejectdetail: u32)>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type PWINBIO_ENROLL_CAPTURE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(enrollcallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, rejectdetail: u32)>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type PWINBIO_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(eventcallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, event: *const WINBIO_EVENT)>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type PWINBIO_IDENTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(identifycallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, rejectdetail: u32)>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type PWINBIO_LOCATE_SENSOR_CALLBACK = ::core::option::Option<unsafe extern "system" fn(locatecallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, unitid: u32)>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_ENGINE_INTERFACE_FN = ::core::option::Option<unsafe extern "system" fn(engineinterface: *mut *mut WINBIO_ENGINE_INTERFACE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_SENSOR_INTERFACE_FN = ::core::option::Option<unsafe extern "system" fn(sensorinterface: *mut *mut WINBIO_SENSOR_INTERFACE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PWINBIO_QUERY_STORAGE_INTERFACE_FN = ::core::option::Option<unsafe extern "system" fn(storageinterface: *mut *mut WINBIO_STORAGE_INTERFACE) -> ::windows::core::HRESULT>;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWINBIO_VERIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(verifycallbackcontext: *const ::core::ffi::c_void, operationstatus: ::windows::core::HRESULT, unitid: u32, r#match: super::super::Foundation::BOOLEAN, rejectdetail: u32)>;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_ACCOUNT_POLICY {
    pub Identity: WINBIO_IDENTITY,
    pub AntiSpoofBehavior: WINBIO_ANTI_SPOOF_POLICY_ACTION,
}
impl ::core::marker::Copy for WINBIO_ACCOUNT_POLICY {}
impl ::core::clone::Clone for WINBIO_ACCOUNT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_ACCOUNT_POLICY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_ACCOUNT_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ACCOUNT_POLICY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_ACCOUNT_POLICY {}
impl ::core::default::Default for WINBIO_ACCOUNT_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_ADAPTER_INTERFACE_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for WINBIO_ADAPTER_INTERFACE_VERSION {}
impl ::core::clone::Clone for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_ADAPTER_INTERFACE_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ADAPTER_INTERFACE_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_ADAPTER_INTERFACE_VERSION {}
impl ::core::default::Default for WINBIO_ADAPTER_INTERFACE_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_ANTI_SPOOF_POLICY {
    pub Action: WINBIO_ANTI_SPOOF_POLICY_ACTION,
    pub Source: WINBIO_POLICY_SOURCE,
}
impl ::core::marker::Copy for WINBIO_ANTI_SPOOF_POLICY {}
impl ::core::clone::Clone for WINBIO_ANTI_SPOOF_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_ANTI_SPOOF_POLICY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_ANTI_SPOOF_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ANTI_SPOOF_POLICY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_ANTI_SPOOF_POLICY {}
impl ::core::default::Default for WINBIO_ANTI_SPOOF_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_ANTI_SPOOF_POLICY_ACTION = i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_ANTI_SPOOF_DISABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = 0i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_ANTI_SPOOF_ENABLE: WINBIO_ANTI_SPOOF_POLICY_ACTION = 1i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_ANTI_SPOOF_REMOVE: WINBIO_ANTI_SPOOF_POLICY_ACTION = 2i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_ASYNC_NOTIFICATION_METHOD = i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_ASYNC_NOTIFY_NONE: WINBIO_ASYNC_NOTIFICATION_METHOD = 0i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_ASYNC_NOTIFY_CALLBACK: WINBIO_ASYNC_NOTIFICATION_METHOD = 1i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_ASYNC_NOTIFY_MESSAGE: WINBIO_ASYNC_NOTIFICATION_METHOD = 2i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_ASYNC_NOTIFY_MAXIMUM_VALUE: WINBIO_ASYNC_NOTIFICATION_METHOD = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT {
    pub SessionHandle: u32,
    pub Operation: u32,
    pub SequenceNumber: u64,
    pub TimeStamp: i64,
    pub ApiStatus: ::windows::core::HRESULT,
    pub UnitId: u32,
    pub UserData: *mut ::core::ffi::c_void,
    pub Parameters: WINBIO_ASYNC_RESULT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_0 {
    pub Sample: *mut WINBIO_BIR,
    pub SampleSize: usize,
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_2 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_3 {
    pub SubFactor: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_4 {
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_5 {
    pub Identity: WINBIO_IDENTITY,
    pub IsNewTemplate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_6 {
    pub SelectorValue: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_7 {
    pub UnitCount: usize,
    pub UnitSchemaArray: *mut WINBIO_UNIT_SCHEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_7 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_7>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_8 {
    pub StorageCount: usize,
    pub StorageSchemaArray: *mut WINBIO_STORAGE_SCHEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_8 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_8>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_9 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactorCount: usize,
    pub SubFactorArray: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_9 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_9>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_10 {
    pub BspCount: usize,
    pub BspSchemaArray: *mut WINBIO_BSP_SCHEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_10 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_10 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_10>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_11 {
    pub Event: WINBIO_EVENT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_11 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_11 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_11 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_11>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_12 {
    pub PropertyType: u32,
    pub PropertyId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_12 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_12 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_12 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_12 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_12>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_12 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_12 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_13 {
    pub Identity: WINBIO_IDENTITY,
    pub Policy: WINBIO_PROTECTION_POLICY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_13 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_13 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_13 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_13 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_13>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_13 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_13 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_14 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
    pub Ticket: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_14 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_14 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_14 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_14 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_14>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_14 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_14 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_15 {
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_15 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_15 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_15 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_15 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_15>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_15 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_15 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_16 {
    pub ChangeType: u32,
    pub PresenceCount: usize,
    pub PresenceArray: *mut WINBIO_PRESENCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_16 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_16 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_16>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_17 {
    pub ExtendedStatus: WINBIO_EXTENDED_UNIT_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_17 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_17 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_17 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_17 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_17>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_17 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_17 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_18 {
    pub PropertyType: u32,
    pub PropertyId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub PropertyBufferSize: usize,
    pub PropertyBuffer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_18 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_18 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_18 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_18 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_18>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_18 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_18 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_19 {
    pub Match: super::super::Foundation::BOOLEAN,
    pub RejectDetail: u32,
    pub Ticket: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_19 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_19 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_19 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_19 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_19>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_19 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_19 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_ASYNC_RESULT_0_20 {
    pub Match: super::super::Foundation::BOOLEAN,
    pub RejectDetail: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_ASYNC_RESULT_0_20 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_ASYNC_RESULT_0_20 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_ASYNC_RESULT_0_20 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_ASYNC_RESULT_0_20 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ASYNC_RESULT_0_20>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_ASYNC_RESULT_0_20 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_ASYNC_RESULT_0_20 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
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
impl ::core::marker::Copy for WINBIO_BDB_ANSI_381_HEADER {}
impl ::core::clone::Clone for WINBIO_BDB_ANSI_381_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_BDB_ANSI_381_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_BDB_ANSI_381_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_BDB_ANSI_381_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_BDB_ANSI_381_HEADER {}
impl ::core::default::Default for WINBIO_BDB_ANSI_381_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
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
impl ::core::marker::Copy for WINBIO_BDB_ANSI_381_RECORD {}
impl ::core::clone::Clone for WINBIO_BDB_ANSI_381_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_BDB_ANSI_381_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_BDB_ANSI_381_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_BDB_ANSI_381_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_BDB_ANSI_381_RECORD {}
impl ::core::default::Default for WINBIO_BDB_ANSI_381_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_BIR {
    pub HeaderBlock: WINBIO_BIR_DATA,
    pub StandardDataBlock: WINBIO_BIR_DATA,
    pub VendorDataBlock: WINBIO_BIR_DATA,
    pub SignatureBlock: WINBIO_BIR_DATA,
}
impl ::core::marker::Copy for WINBIO_BIR {}
impl ::core::clone::Clone for WINBIO_BIR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_BIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_BIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_BIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_BIR {}
impl ::core::default::Default for WINBIO_BIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_BIR_ALGIN_SIZE: u32 = 8u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_BIR_ALIGN_SIZE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_BIR_DATA {
    pub Size: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for WINBIO_BIR_DATA {}
impl ::core::clone::Clone for WINBIO_BIR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_BIR_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_BIR_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_BIR_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_DATA {}
impl ::core::default::Default for WINBIO_BIR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
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
impl ::core::marker::Copy for WINBIO_BIR_HEADER {}
impl ::core::clone::Clone for WINBIO_BIR_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_BIR_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_BIR_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_BIR_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_HEADER {}
impl ::core::default::Default for WINBIO_BIR_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_BIR_HEADER_0 {
    pub BeginDate: i64,
    pub EndDate: i64,
}
impl ::core::marker::Copy for WINBIO_BIR_HEADER_0 {}
impl ::core::clone::Clone for WINBIO_BIR_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_BIR_HEADER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_BIR_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_BIR_HEADER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_BIR_HEADER_0 {}
impl ::core::default::Default for WINBIO_BIR_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_BLANK_PAYLOAD {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for WINBIO_BLANK_PAYLOAD {}
impl ::core::clone::Clone for WINBIO_BLANK_PAYLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_BLANK_PAYLOAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_BLANK_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_BLANK_PAYLOAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_BLANK_PAYLOAD {}
impl ::core::default::Default for WINBIO_BLANK_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_BSP_SCHEMA {
    pub BiometricFactor: u32,
    pub BspId: ::windows::core::GUID,
    pub Description: [u16; 256],
    pub Vendor: [u16; 256],
    pub Version: WINBIO_VERSION,
}
impl ::core::marker::Copy for WINBIO_BSP_SCHEMA {}
impl ::core::clone::Clone for WINBIO_BSP_SCHEMA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_BSP_SCHEMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_BSP_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_BSP_SCHEMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_BSP_SCHEMA {}
impl ::core::default::Default for WINBIO_BSP_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_CALIBRATION_INFO {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub CalibrationData: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_CALIBRATION_INFO {}
impl ::core::clone::Clone for WINBIO_CALIBRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_CALIBRATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_CALIBRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_CALIBRATION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_CALIBRATION_INFO {}
impl ::core::default::Default for WINBIO_CALIBRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_CAPTURE_DATA {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub SensorStatus: u32,
    pub RejectDetail: u32,
    pub CaptureData: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_CAPTURE_DATA {}
impl ::core::clone::Clone for WINBIO_CAPTURE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_CAPTURE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_CAPTURE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_CAPTURE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_CAPTURE_DATA {}
impl ::core::default::Default for WINBIO_CAPTURE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_CAPTURE_PARAMETERS {
    pub PayloadSize: u32,
    pub Purpose: u8,
    pub Format: WINBIO_REGISTERED_FORMAT,
    pub VendorFormat: ::windows::core::GUID,
    pub Flags: u8,
}
impl ::core::marker::Copy for WINBIO_CAPTURE_PARAMETERS {}
impl ::core::clone::Clone for WINBIO_CAPTURE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_CAPTURE_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_CAPTURE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_CAPTURE_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_CAPTURE_PARAMETERS {}
impl ::core::default::Default for WINBIO_CAPTURE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_COMPONENT = u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_COMPONENT_SENSOR: WINBIO_COMPONENT = 1u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_COMPONENT_ENGINE: WINBIO_COMPONENT = 2u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_COMPONENT_STORAGE: WINBIO_COMPONENT = 3u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_CREDENTIAL_FORMAT = i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_PASSWORD_GENERIC: WINBIO_CREDENTIAL_FORMAT = 1i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_PASSWORD_PACKED: WINBIO_CREDENTIAL_FORMAT = 2i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_PASSWORD_PROTECTED: WINBIO_CREDENTIAL_FORMAT = 3i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_CREDENTIAL_STATE = i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_CREDENTIAL_NOT_SET: WINBIO_CREDENTIAL_STATE = 1i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_CREDENTIAL_SET: WINBIO_CREDENTIAL_STATE = 2i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_CREDENTIAL_TYPE = i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_CREDENTIAL_PASSWORD: WINBIO_CREDENTIAL_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_CREDENTIAL_ALL: WINBIO_CREDENTIAL_TYPE = -1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_DATA {
    pub Size: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for WINBIO_DATA {}
impl ::core::clone::Clone for WINBIO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_DATA {}
impl ::core::default::Default for WINBIO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_DIAGNOSTICS {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub SensorStatus: u32,
    pub VendorDiagnostics: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_DIAGNOSTICS {}
impl ::core::clone::Clone for WINBIO_DIAGNOSTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_DIAGNOSTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_DIAGNOSTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_DIAGNOSTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_DIAGNOSTICS {}
impl ::core::default::Default for WINBIO_DIAGNOSTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    pub PayloadSize: u32,
    pub Purpose: u8,
    pub Format: WINBIO_REGISTERED_FORMAT,
    pub VendorFormat: ::windows::core::GUID,
    pub Flags: u8,
    pub NonceSize: u32,
}
impl ::core::marker::Copy for WINBIO_ENCRYPTED_CAPTURE_PARAMS {}
impl ::core::clone::Clone for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ENCRYPTED_CAPTURE_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_ENCRYPTED_CAPTURE_PARAMS {}
impl ::core::default::Default for WINBIO_ENCRYPTED_CAPTURE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_ENGINE_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::core::GUID,
    pub Attach: PIBIO_ENGINE_ATTACH_FN,
    pub Detach: PIBIO_ENGINE_DETACH_FN,
    pub ClearContext: PIBIO_ENGINE_CLEAR_CONTEXT_FN,
    pub QueryPreferredFormat: PIBIO_ENGINE_QUERY_PREFERRED_FORMAT_FN,
    pub QueryIndexVectorSize: PIBIO_ENGINE_QUERY_INDEX_VECTOR_SIZE_FN,
    pub QueryHashAlgorithms: PIBIO_ENGINE_QUERY_HASH_ALGORITHMS_FN,
    pub SetHashAlgorithm: PIBIO_ENGINE_SET_HASH_ALGORITHM_FN,
    pub QuerySampleHint: PIBIO_ENGINE_QUERY_SAMPLE_HINT_FN,
    pub AcceptSampleData: PIBIO_ENGINE_ACCEPT_SAMPLE_DATA_FN,
    pub ExportEngineData: PIBIO_ENGINE_EXPORT_ENGINE_DATA_FN,
    pub VerifyFeatureSet: PIBIO_ENGINE_VERIFY_FEATURE_SET_FN,
    pub IdentifyFeatureSet: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_FN,
    pub CreateEnrollment: PIBIO_ENGINE_CREATE_ENROLLMENT_FN,
    pub UpdateEnrollment: PIBIO_ENGINE_UPDATE_ENROLLMENT_FN,
    pub GetEnrollmentStatus: PIBIO_ENGINE_GET_ENROLLMENT_STATUS_FN,
    pub GetEnrollmentHash: PIBIO_ENGINE_GET_ENROLLMENT_HASH_FN,
    pub CheckForDuplicate: PIBIO_ENGINE_CHECK_FOR_DUPLICATE_FN,
    pub CommitEnrollment: PIBIO_ENGINE_COMMIT_ENROLLMENT_FN,
    pub DiscardEnrollment: PIBIO_ENGINE_DISCARD_ENROLLMENT_FN,
    pub ControlUnit: PIBIO_ENGINE_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_ENGINE_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_ENGINE_NOTIFY_POWER_CHANGE_FN,
    pub Reserved_1: PIBIO_ENGINE_RESERVED_1_FN,
    pub PipelineInit: PIBIO_ENGINE_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_ENGINE_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_ENGINE_ACTIVATE_FN,
    pub Deactivate: PIBIO_ENGINE_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_ENGINE_QUERY_EXTENDED_INFO_FN,
    pub IdentifyAll: PIBIO_ENGINE_IDENTIFY_ALL_FN,
    pub SetEnrollmentSelector: PIBIO_ENGINE_SET_ENROLLMENT_SELECTOR_FN,
    pub SetEnrollmentParameters: PIBIO_ENGINE_SET_ENROLLMENT_PARAMETERS_FN,
    pub QueryExtendedEnrollmentStatus: PIBIO_ENGINE_QUERY_EXTENDED_ENROLLMENT_STATUS_FN,
    pub RefreshCache: PIBIO_ENGINE_REFRESH_CACHE_FN,
    pub SelectCalibrationFormat: PIBIO_ENGINE_SELECT_CALIBRATION_FORMAT_FN,
    pub QueryCalibrationData: PIBIO_ENGINE_QUERY_CALIBRATION_DATA_FN,
    pub SetAccountPolicy: PIBIO_ENGINE_SET_ACCOUNT_POLICY_FN,
    pub CreateKey: PIBIO_ENGINE_CREATE_KEY_FN,
    pub IdentifyFeatureSetSecure: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_SECURE_FN,
    pub AcceptPrivateSensorTypeInfo: PIBIO_ENGINE_ACCEPT_PRIVATE_SENSOR_TYPE_INFO_FN,
    pub CreateEnrollmentAuthenticated: PIBIO_ENGINE_CREATE_ENROLLMENT_AUTHENTICATED_FN,
    pub IdentifyFeatureSetAuthenticated: PIBIO_ENGINE_IDENTIFY_FEATURE_SET_AUTHENTICATED_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_ENGINE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_ENGINE_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for WINBIO_ENGINE_INTERFACE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for WINBIO_ENGINE_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_ENGINE_INTERFACE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for WINBIO_ENGINE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_ENGINE_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EVENT {
    pub Type: u32,
    pub Parameters: WINBIO_EVENT_0,
}
impl ::core::marker::Copy for WINBIO_EVENT {}
impl ::core::clone::Clone for WINBIO_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EVENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EVENT {}
impl ::core::default::Default for WINBIO_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub union WINBIO_EVENT_0 {
    pub Unclaimed: WINBIO_EVENT_0_2,
    pub UnclaimedIdentify: WINBIO_EVENT_0_1,
    pub Error: WINBIO_EVENT_0_0,
}
impl ::core::marker::Copy for WINBIO_EVENT_0 {}
impl ::core::clone::Clone for WINBIO_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EVENT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EVENT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EVENT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EVENT_0 {}
impl ::core::default::Default for WINBIO_EVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EVENT_0_0 {
    pub ErrorCode: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for WINBIO_EVENT_0_0 {}
impl ::core::clone::Clone for WINBIO_EVENT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EVENT_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EVENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EVENT_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EVENT_0_0 {}
impl ::core::default::Default for WINBIO_EVENT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EVENT_0_1 {
    pub UnitId: u32,
    pub Identity: WINBIO_IDENTITY,
    pub SubFactor: u8,
    pub RejectDetail: u32,
}
impl ::core::marker::Copy for WINBIO_EVENT_0_1 {}
impl ::core::clone::Clone for WINBIO_EVENT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EVENT_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EVENT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EVENT_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EVENT_0_1 {}
impl ::core::default::Default for WINBIO_EVENT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EVENT_0_2 {
    pub UnitId: u32,
    pub RejectDetail: u32,
}
impl ::core::marker::Copy for WINBIO_EVENT_0_2 {}
impl ::core::clone::Clone for WINBIO_EVENT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EVENT_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EVENT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EVENT_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EVENT_0_2 {}
impl ::core::default::Default for WINBIO_EVENT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO {
    pub GenericEngineCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_ENGINE_INFO_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub union WINBIO_EXTENDED_ENGINE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENGINE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENGINE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_ENGINE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_ENGINE_INFO_0_3,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_0_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    pub Null: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_1_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_1 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_1 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0_1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_2_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_2 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_2 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    pub Null: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0_2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    pub Capabilities: u32,
    pub EnrollmentRequirements: WINBIO_EXTENDED_ENGINE_INFO_0_3_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_3 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_3 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    pub Null: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENGINE_INFO_0_3_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_ENGINE_INFO_0_3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    pub Size: usize,
    pub SubFactor: u8,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {}
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {}
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS {
    pub TemplateStatus: ::windows::core::HRESULT,
    pub RejectDetail: u32,
    pub PercentComplete: u32,
    pub Factor: u32,
    pub SubFactor: u8,
    pub Specific: WINBIO_EXTENDED_ENROLLMENT_STATUS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_STATUS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0,
    pub Fingerprint: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1,
    pub Iris: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2,
    pub Voice: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_STATUS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    pub BoundingBox: super::super::Foundation::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    pub AdapterId: ::windows::core::GUID,
    pub Data: [u32; 78],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    pub GeneralSamples: u32,
    pub Center: u32,
    pub TopEdge: u32,
    pub BottomEdge: u32,
    pub LeftEdge: u32,
    pub RightEdge: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_ENROLLMENT_STATUS_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO {
    pub GenericSensorCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_SENSOR_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_SENSOR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_SENSOR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINBIO_EXTENDED_SENSOR_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_SENSOR_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_SENSOR_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_SENSOR_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_SENSOR_INFO_0_3,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_SENSOR_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_SENSOR_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    pub FrameSize: super::super::Foundation::RECT,
    pub FrameOffset: super::super::Foundation::POINT,
    pub MandatoryOrientation: u32,
    pub HardwareInfo: WINBIO_EXTENDED_SENSOR_INFO_0_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_SENSOR_INFO_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    pub ColorSensorId: [u16; 260],
    pub InfraredSensorId: [u16; 260],
    pub InfraredSensorRotationAngle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_SENSOR_INFO_0_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_SENSOR_INFO_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    pub FrameSize: super::super::Foundation::RECT,
    pub FrameOffset: super::super::Foundation::POINT,
    pub MandatoryOrientation: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_SENSOR_INFO_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_EXTENDED_SENSOR_INFO_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_SENSOR_INFO_0_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_EXTENDED_SENSOR_INFO_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_EXTENDED_SENSOR_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO {
    pub GenericStorageCapabilities: u32,
    pub Factor: u32,
    pub Specific: WINBIO_EXTENDED_STORAGE_INFO_0,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_STORAGE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_STORAGE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub union WINBIO_EXTENDED_STORAGE_INFO_0 {
    pub Null: u32,
    pub FacialFeatures: WINBIO_EXTENDED_STORAGE_INFO_0_0,
    pub Fingerprint: WINBIO_EXTENDED_STORAGE_INFO_0_1,
    pub Iris: WINBIO_EXTENDED_STORAGE_INFO_0_2,
    pub Voice: WINBIO_EXTENDED_STORAGE_INFO_0_3,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_STORAGE_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_STORAGE_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0_0 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_STORAGE_INFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_0 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0_1 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_STORAGE_INFO_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_1 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0_2 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_STORAGE_INFO_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_2 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_STORAGE_INFO_0_3 {}
impl ::core::clone::Clone for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_STORAGE_INFO_0_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_STORAGE_INFO_0_3 {}
impl ::core::default::Default for WINBIO_EXTENDED_STORAGE_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_EXTENDED_UNIT_STATUS {
    pub Availability: u32,
    pub ReasonCode: u32,
}
impl ::core::marker::Copy for WINBIO_EXTENDED_UNIT_STATUS {}
impl ::core::clone::Clone for WINBIO_EXTENDED_UNIT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_EXTENDED_UNIT_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_EXTENDED_UNIT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_EXTENDED_UNIT_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_EXTENDED_UNIT_STATUS {}
impl ::core::default::Default for WINBIO_EXTENDED_UNIT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_ADAPTER_INTEGRITY_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860995i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_AUTO_LOGON_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860989i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_BAD_CAPTURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861048i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CALIBRATION_BUFFER_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860975i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860976i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CALIBRATION_BUFFER_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860977i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861052i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CAPTURE_ABORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861050i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CONFIGURATION_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861005i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CRED_PROV_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861008i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CRED_PROV_NO_CREDENTIAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861007i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_CRED_PROV_SECURITY_LOCKOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860985i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861034i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_BAD_INDEX_VECTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861022i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_CANT_CLOSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861037i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_CANT_CREATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861039i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_CANT_ERASE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861036i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_CANT_FIND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861035i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_CANT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861038i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_CORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861030i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_EOF: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861023i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861032i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861031i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_NO_MORE_RECORDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861024i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_NO_RESULTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861025i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_NO_SUCH_RECORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861029i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_READ_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861027i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATABASE_WRITE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861026i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATA_COLLECTION_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861045i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DATA_PROTECTION_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860986i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DEADLOCK_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860992i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DEVICE_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861040i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DEVICE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861002i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861006i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DUPLICATE_ENROLLMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861028i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_DUPLICATE_TEMPLATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861013i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_ENROLLMENT_CANCELED_BY_SUSPEND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860965i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_ENROLLMENT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861049i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_EVENT_MONITOR_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860999i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_FAST_USER_SWITCH_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861001i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INCORRECT_BSP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861020i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INCORRECT_SENSOR_POOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861019i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INCORRECT_SESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860994i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INSECURE_SENSOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860969i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860967i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_BUFFER_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860968i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_CALIBRATION_FORMAT_ARRAY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860980i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_CONTROL_CODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861047i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_DEVICE_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861041i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_KEY_IDENTIFIER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860974i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_OPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861012i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_PROPERTY_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860997i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_PROPERTY_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860998i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_SENSOR_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861017i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_SUBFACTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860981i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_TICKET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860988i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_INVALID_UNIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861054i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_KEY_CREATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860973i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_KEY_IDENTIFIER_BUFFER_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860972i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_LOCK_VIOLATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861014i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_MAX_ERROR_COUNT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860990i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_NOT_ACTIVE_CONSOLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861000i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_NO_CAPTURE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861018i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_NO_MATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861051i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_NO_PREBOOT_IDENTITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860991i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_NO_SUPPORTED_CALIBRATION_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860979i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_POLICY_PROTECTION_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860970i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_PRESENCE_MONITOR_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860982i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_PROPERTY_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860971i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_SAS_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861003i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_SELECTION_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860983i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_SENSOR_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861004i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_SESSION_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861011i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_SESSION_HANDLE_CLOSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860993i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_TICKET_QUOTA_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860987i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_TRUSTLET_INTEGRITY_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860966i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_UNKNOWN_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861053i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_UNSUPPORTED_DATA_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861044i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_UNSUPPORTED_DATA_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861043i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_UNSUPPORTED_FACTOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861055i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_UNSUPPORTED_POOL_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860984i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_UNSUPPORTED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860996i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_UNSUPPORTED_PURPOSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146861042i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_E_UNSUPPORTED_SENSOR_CALIBRATION_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146860978i32);
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_FP_BU_STATE {
    pub SensorAttached: super::super::Foundation::BOOL,
    pub CreationResult: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_FP_BU_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_FP_BU_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_FP_BU_STATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_FP_BU_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_FP_BU_STATE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_FP_BU_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_FP_BU_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_FRAMEWORK_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::core::GUID,
    pub SetUnitStatus: PIBIO_FRAMEWORK_SET_UNIT_STATUS_FN,
    pub VsmStorageAttach: PIBIO_STORAGE_ATTACH_FN,
    pub VsmStorageDetach: PIBIO_STORAGE_DETACH_FN,
    pub VsmStorageClearContext: PIBIO_STORAGE_CLEAR_CONTEXT_FN,
    pub VsmStorageCreateDatabase: PIBIO_STORAGE_CREATE_DATABASE_FN,
    pub VsmStorageOpenDatabase: PIBIO_STORAGE_OPEN_DATABASE_FN,
    pub VsmStorageCloseDatabase: PIBIO_STORAGE_CLOSE_DATABASE_FN,
    pub VsmStorageDeleteRecord: PIBIO_STORAGE_DELETE_RECORD_FN,
    pub VsmStorageNotifyPowerChange: PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN,
    pub VsmStoragePipelineInit: PIBIO_STORAGE_PIPELINE_INIT_FN,
    pub VsmStoragePipelineCleanup: PIBIO_STORAGE_PIPELINE_CLEANUP_FN,
    pub VsmStorageActivate: PIBIO_STORAGE_ACTIVATE_FN,
    pub VsmStorageDeactivate: PIBIO_STORAGE_DEACTIVATE_FN,
    pub VsmStorageQueryExtendedInfo: PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN,
    pub VsmStorageCacheClear: PIBIO_FRAMEWORK_VSM_CACHE_CLEAR_FN,
    pub VsmStorageCacheImportBegin: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_BEGIN_FN,
    pub VsmStorageCacheImportNext: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_NEXT_FN,
    pub VsmStorageCacheImportEnd: PIBIO_FRAMEWORK_VSM_CACHE_IMPORT_END_FN,
    pub VsmStorageCacheExportBegin: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_BEGIN_FN,
    pub VsmStorageCacheExportNext: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_NEXT_FN,
    pub VsmStorageCacheExportEnd: PIBIO_FRAMEWORK_VSM_CACHE_EXPORT_END_FN,
    pub VsmSensorAttach: PIBIO_SENSOR_ATTACH_FN,
    pub VsmSensorDetach: PIBIO_SENSOR_DETACH_FN,
    pub VsmSensorClearContext: PIBIO_SENSOR_CLEAR_CONTEXT_FN,
    pub VsmSensorPushDataToEngine: PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN,
    pub VsmSensorNotifyPowerChange: PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN,
    pub VsmSensorPipelineInit: PIBIO_SENSOR_PIPELINE_INIT_FN,
    pub VsmSensorPipelineCleanup: PIBIO_SENSOR_PIPELINE_CLEANUP_FN,
    pub VsmSensorActivate: PIBIO_SENSOR_ACTIVATE_FN,
    pub VsmSensorDeactivate: PIBIO_SENSOR_DEACTIVATE_FN,
    pub VsmSensorAsyncImportRawBuffer: PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN,
    pub VsmSensorAsyncImportSecureBuffer: PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN,
    pub Reserved1: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_1_FN,
    pub Reserved2: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_2_FN,
    pub Reserved3: PIBIO_FRAMEWORK_VSM_STORAGE_RESERVED_3_FN,
    pub Reserved4: PIBIO_STORAGE_RESERVED_1_FN,
    pub Reserved5: PIBIO_STORAGE_RESERVED_2_FN,
    pub AllocateMemory: PIBIO_FRAMEWORK_ALLOCATE_MEMORY_FN,
    pub FreeMemory: PIBIO_FRAMEWORK_FREE_MEMORY_FN,
    pub GetProperty: PIBIO_FRAMEWORK_GET_PROPERTY_FN,
    pub LockAndValidateSecureBuffer: PIBIO_FRAMEWORK_LOCK_AND_VALIDATE_SECURE_BUFFER_FN,
    pub ReleaseSecureBuffer: PIBIO_FRAMEWORK_RELEASE_SECURE_BUFFER_FN,
    pub QueryAuthorizedEnrollments: PIBIO_FRAMEWORK_VSM_QUERY_AUTHORIZED_ENROLLMENTS_FN,
    pub DecryptSample: PIBIO_FRAMEWORK_VSM_DECRYPT_SAMPLE_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_FRAMEWORK_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_FRAMEWORK_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for WINBIO_FRAMEWORK_INTERFACE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for WINBIO_FRAMEWORK_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_FRAMEWORK_INTERFACE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for WINBIO_FRAMEWORK_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_FRAMEWORK_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_GESTURE_METADATA {
    pub Size: usize,
    pub BiometricType: u32,
    pub MatchType: u32,
    pub ProtectionType: u32,
}
impl ::core::marker::Copy for WINBIO_GESTURE_METADATA {}
impl ::core::clone::Clone for WINBIO_GESTURE_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_GESTURE_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_GESTURE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_GESTURE_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_GESTURE_METADATA {}
impl ::core::default::Default for WINBIO_GESTURE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_GET_INDICATOR {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub IndicatorStatus: u32,
}
impl ::core::marker::Copy for WINBIO_GET_INDICATOR {}
impl ::core::clone::Clone for WINBIO_GET_INDICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_GET_INDICATOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_GET_INDICATOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_GET_INDICATOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_GET_INDICATOR {}
impl ::core::default::Default for WINBIO_GET_INDICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_IDENTITY {
    pub Type: u32,
    pub Value: WINBIO_IDENTITY_0,
}
impl ::core::marker::Copy for WINBIO_IDENTITY {}
impl ::core::clone::Clone for WINBIO_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_IDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_IDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_IDENTITY {}
impl ::core::default::Default for WINBIO_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub union WINBIO_IDENTITY_0 {
    pub Null: u32,
    pub Wildcard: u32,
    pub TemplateGuid: ::windows::core::GUID,
    pub AccountSid: WINBIO_IDENTITY_0_0,
    pub SecureId: [u8; 32],
}
impl ::core::marker::Copy for WINBIO_IDENTITY_0 {}
impl ::core::clone::Clone for WINBIO_IDENTITY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_IDENTITY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_IDENTITY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_IDENTITY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_IDENTITY_0 {}
impl ::core::default::Default for WINBIO_IDENTITY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_IDENTITY_0_0 {
    pub Size: u32,
    pub Data: [u8; 68],
}
impl ::core::marker::Copy for WINBIO_IDENTITY_0_0 {}
impl ::core::clone::Clone for WINBIO_IDENTITY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_IDENTITY_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_IDENTITY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_IDENTITY_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_IDENTITY_0_0 {}
impl ::core::default::Default for WINBIO_IDENTITY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_I_EXTENDED_STATUS_INFORMATION: ::windows::core::HRESULT = ::windows::core::HRESULT(589826i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_I_MORE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(589825i32);
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_MAX_STRING_LEN: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_NOTIFY_WAKE {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub Reason: u32,
}
impl ::core::marker::Copy for WINBIO_NOTIFY_WAKE {}
impl ::core::clone::Clone for WINBIO_NOTIFY_WAKE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_NOTIFY_WAKE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_NOTIFY_WAKE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_NOTIFY_WAKE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_NOTIFY_WAKE {}
impl ::core::default::Default for WINBIO_NOTIFY_WAKE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
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
impl ::core::marker::Copy for WINBIO_PIPELINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_PIPELINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for WINBIO_PIPELINE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for WINBIO_PIPELINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PIPELINE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for WINBIO_PIPELINE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_PIPELINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_POLICY_SOURCE = i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_POLICY_UNKNOWN: WINBIO_POLICY_SOURCE = 0i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_POLICY_DEFAULT: WINBIO_POLICY_SOURCE = 1i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_POLICY_LOCAL: WINBIO_POLICY_SOURCE = 2i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_POLICY_ADMIN: WINBIO_POLICY_SOURCE = 3i32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_POOL = u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_POOL_SYSTEM: WINBIO_POOL = 1u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_POOL_PRIVATE: WINBIO_POOL = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE {
    pub Factor: u32,
    pub SubFactor: u8,
    pub Status: ::windows::core::HRESULT,
    pub RejectDetail: u32,
    pub Identity: WINBIO_IDENTITY,
    pub TrackingId: u64,
    pub Ticket: u64,
    pub Properties: WINBIO_PRESENCE_PROPERTIES,
    pub Authorization: WINBIO_PRESENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_PRESENCE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PRESENCE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE_0 {
    pub Size: u32,
    pub Data: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_PRESENCE_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PRESENCE_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINBIO_PRESENCE_PROPERTIES {
    pub FacialFeatures: WINBIO_PRESENCE_PROPERTIES_0,
    pub Iris: WINBIO_PRESENCE_PROPERTIES_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_PRESENCE_PROPERTIES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PRESENCE_PROPERTIES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE_PROPERTIES_0 {
    pub BoundingBox: super::super::Foundation::RECT,
    pub Distance: i32,
    pub OpaqueEngineData: WINBIO_PRESENCE_PROPERTIES_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_PROPERTIES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_PRESENCE_PROPERTIES_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PRESENCE_PROPERTIES_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE_PROPERTIES_0_0 {
    pub AdapterId: ::windows::core::GUID,
    pub Data: [u32; 78],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_PROPERTIES_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_PRESENCE_PROPERTIES_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PRESENCE_PROPERTIES_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINBIO_PRESENCE_PROPERTIES_1 {
    pub EyeBoundingBox_1: super::super::Foundation::RECT,
    pub EyeBoundingBox_2: super::super::Foundation::RECT,
    pub PupilCenter_1: super::super::Foundation::POINT,
    pub PupilCenter_2: super::super::Foundation::POINT,
    pub Distance: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINBIO_PRESENCE_PROPERTIES_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINBIO_PRESENCE_PROPERTIES_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WINBIO_PRESENCE_PROPERTIES_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINBIO_PRESENCE_PROPERTIES_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PRESENCE_PROPERTIES_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINBIO_PRESENCE_PROPERTIES_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINBIO_PRESENCE_PROPERTIES_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub PrivateSensorTypeInfo: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_PRIVATE_SENSOR_TYPE_INFO {}
impl ::core::clone::Clone for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PRIVATE_SENSOR_TYPE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_PRIVATE_SENSOR_TYPE_INFO {}
impl ::core::default::Default for WINBIO_PRIVATE_SENSOR_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_PROTECTION_POLICY {
    pub Version: u32,
    pub Identity: WINBIO_IDENTITY,
    pub DatabaseId: ::windows::core::GUID,
    pub UserState: u64,
    pub PolicySize: usize,
    pub Policy: [u8; 128],
}
impl ::core::marker::Copy for WINBIO_PROTECTION_POLICY {}
impl ::core::clone::Clone for WINBIO_PROTECTION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_PROTECTION_POLICY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_PROTECTION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_PROTECTION_POLICY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_PROTECTION_POLICY {}
impl ::core::default::Default for WINBIO_PROTECTION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_REGISTERED_FORMAT {
    pub Owner: u16,
    pub Type: u16,
}
impl ::core::marker::Copy for WINBIO_REGISTERED_FORMAT {}
impl ::core::clone::Clone for WINBIO_REGISTERED_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_REGISTERED_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_REGISTERED_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_REGISTERED_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_REGISTERED_FORMAT {}
impl ::core::default::Default for WINBIO_REGISTERED_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_CURVE_FIELD_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_DIGEST_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_ENCRYPTION_BLOCK_SIZE_V1: u32 = 16u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_ENCRYPTION_KEY_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_PRIVATE_KEY_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_PUBLIC_KEY_SIZE_V1: u32 = 65u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_RANDOM_SIZE_V1: u32 = 32u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_SIGNATURE_SIZE_V1: u32 = 64u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SCP_VERSION_1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_SECURE_BUFFER_HEADER_V1 {
    pub Type: u32,
    pub Size: u32,
    pub Flags: u32,
    pub ValidationTag: u64,
}
impl ::core::marker::Copy for WINBIO_SECURE_BUFFER_HEADER_V1 {}
impl ::core::clone::Clone for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_SECURE_BUFFER_HEADER_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_SECURE_BUFFER_HEADER_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_BUFFER_HEADER_V1 {}
impl ::core::default::Default for WINBIO_SECURE_BUFFER_HEADER_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_SECURE_CONNECTION_DATA {
    pub Size: u32,
    pub Version: u16,
    pub Flags: u16,
    pub ModelCertificateSize: u32,
    pub IntermediateCA1Size: u32,
    pub IntermediateCA2Size: u32,
}
impl ::core::marker::Copy for WINBIO_SECURE_CONNECTION_DATA {}
impl ::core::clone::Clone for WINBIO_SECURE_CONNECTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_SECURE_CONNECTION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_CONNECTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_SECURE_CONNECTION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_CONNECTION_DATA {}
impl ::core::default::Default for WINBIO_SECURE_CONNECTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_SECURE_CONNECTION_PARAMS {
    pub PayloadSize: u32,
    pub Version: u16,
    pub Flags: u16,
}
impl ::core::marker::Copy for WINBIO_SECURE_CONNECTION_PARAMS {}
impl ::core::clone::Clone for WINBIO_SECURE_CONNECTION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_SECURE_CONNECTION_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_SECURE_CONNECTION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_SECURE_CONNECTION_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_SECURE_CONNECTION_PARAMS {}
impl ::core::default::Default for WINBIO_SECURE_CONNECTION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_SENSOR_ATTRIBUTES {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
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
impl ::core::marker::Copy for WINBIO_SENSOR_ATTRIBUTES {}
impl ::core::clone::Clone for WINBIO_SENSOR_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_SENSOR_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_SENSOR_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_SENSOR_ATTRIBUTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_SENSOR_ATTRIBUTES {}
impl ::core::default::Default for WINBIO_SENSOR_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_SENSOR_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::core::GUID,
    pub Attach: PIBIO_SENSOR_ATTACH_FN,
    pub Detach: PIBIO_SENSOR_DETACH_FN,
    pub ClearContext: PIBIO_SENSOR_CLEAR_CONTEXT_FN,
    pub QueryStatus: PIBIO_SENSOR_QUERY_STATUS_FN,
    pub Reset: PIBIO_SENSOR_RESET_FN,
    pub SetMode: PIBIO_SENSOR_SET_MODE_FN,
    pub SetIndicatorStatus: PIBIO_SENSOR_SET_INDICATOR_STATUS_FN,
    pub GetIndicatorStatus: PIBIO_SENSOR_GET_INDICATOR_STATUS_FN,
    pub StartCapture: PIBIO_SENSOR_START_CAPTURE_FN,
    pub FinishCapture: PIBIO_SENSOR_FINISH_CAPTURE_FN,
    pub ExportSensorData: PIBIO_SENSOR_EXPORT_SENSOR_DATA_FN,
    pub Cancel: PIBIO_SENSOR_CANCEL_FN,
    pub PushDataToEngine: PIBIO_SENSOR_PUSH_DATA_TO_ENGINE_FN,
    pub ControlUnit: PIBIO_SENSOR_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_SENSOR_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_SENSOR_NOTIFY_POWER_CHANGE_FN,
    pub PipelineInit: PIBIO_SENSOR_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_SENSOR_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_SENSOR_ACTIVATE_FN,
    pub Deactivate: PIBIO_SENSOR_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_SENSOR_QUERY_EXTENDED_INFO_FN,
    pub QueryCalibrationFormats: PIBIO_SENSOR_QUERY_CALIBRATION_FORMATS_FN,
    pub SetCalibrationFormat: PIBIO_SENSOR_SET_CALIBRATION_FORMAT_FN,
    pub AcceptCalibrationData: PIBIO_SENSOR_ACCEPT_CALIBRATION_DATA_FN,
    pub AsyncImportRawBuffer: PIBIO_SENSOR_ASYNC_IMPORT_RAW_BUFFER_FN,
    pub AsyncImportSecureBuffer: PIBIO_SENSOR_ASYNC_IMPORT_SECURE_BUFFER_FN,
    pub QueryPrivateSensorType: PIBIO_SENSOR_QUERY_PRIVATE_SENSOR_TYPE_FN,
    pub ConnectSecure: PIBIO_SENSOR_CONNECT_SECURE_FN,
    pub StartCaptureEx: PIBIO_SENSOR_START_CAPTURE_EX_FN,
    pub StartNotifyWake: PIBIO_SENSOR_START_NOTIFY_WAKE_FN,
    pub FinishNotifyWake: PIBIO_SENSOR_FINISH_NOTIFY_WAKE_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_SENSOR_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_SENSOR_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for WINBIO_SENSOR_INTERFACE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for WINBIO_SENSOR_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_SENSOR_INTERFACE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for WINBIO_SENSOR_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_SENSOR_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub type WINBIO_SETTING_SOURCE = u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SETTING_SOURCE_INVALID: WINBIO_SETTING_SOURCE = 0u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SETTING_SOURCE_DEFAULT: WINBIO_SETTING_SOURCE = 1u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SETTING_SOURCE_LOCAL: WINBIO_SETTING_SOURCE = 3u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_SETTING_SOURCE_POLICY: WINBIO_SETTING_SOURCE = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_SET_INDICATOR {
    pub PayloadSize: u32,
    pub IndicatorStatus: u32,
}
impl ::core::marker::Copy for WINBIO_SET_INDICATOR {}
impl ::core::clone::Clone for WINBIO_SET_INDICATOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_SET_INDICATOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_SET_INDICATOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_SET_INDICATOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_SET_INDICATOR {}
impl ::core::default::Default for WINBIO_SET_INDICATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation', 'Win32_System_IO'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct WINBIO_STORAGE_INTERFACE {
    pub Version: WINBIO_ADAPTER_INTERFACE_VERSION,
    pub Type: u32,
    pub Size: usize,
    pub AdapterId: ::windows::core::GUID,
    pub Attach: PIBIO_STORAGE_ATTACH_FN,
    pub Detach: PIBIO_STORAGE_DETACH_FN,
    pub ClearContext: PIBIO_STORAGE_CLEAR_CONTEXT_FN,
    pub CreateDatabase: PIBIO_STORAGE_CREATE_DATABASE_FN,
    pub EraseDatabase: PIBIO_STORAGE_ERASE_DATABASE_FN,
    pub OpenDatabase: PIBIO_STORAGE_OPEN_DATABASE_FN,
    pub CloseDatabase: PIBIO_STORAGE_CLOSE_DATABASE_FN,
    pub GetDataFormat: PIBIO_STORAGE_GET_DATA_FORMAT_FN,
    pub GetDatabaseSize: PIBIO_STORAGE_GET_DATABASE_SIZE_FN,
    pub AddRecord: PIBIO_STORAGE_ADD_RECORD_FN,
    pub DeleteRecord: PIBIO_STORAGE_DELETE_RECORD_FN,
    pub QueryBySubject: PIBIO_STORAGE_QUERY_BY_SUBJECT_FN,
    pub QueryByContent: PIBIO_STORAGE_QUERY_BY_CONTENT_FN,
    pub GetRecordCount: PIBIO_STORAGE_GET_RECORD_COUNT_FN,
    pub FirstRecord: PIBIO_STORAGE_FIRST_RECORD_FN,
    pub NextRecord: PIBIO_STORAGE_NEXT_RECORD_FN,
    pub GetCurrentRecord: PIBIO_STORAGE_GET_CURRENT_RECORD_FN,
    pub ControlUnit: PIBIO_STORAGE_CONTROL_UNIT_FN,
    pub ControlUnitPrivileged: PIBIO_STORAGE_CONTROL_UNIT_PRIVILEGED_FN,
    pub NotifyPowerChange: PIBIO_STORAGE_NOTIFY_POWER_CHANGE_FN,
    pub PipelineInit: PIBIO_STORAGE_PIPELINE_INIT_FN,
    pub PipelineCleanup: PIBIO_STORAGE_PIPELINE_CLEANUP_FN,
    pub Activate: PIBIO_STORAGE_ACTIVATE_FN,
    pub Deactivate: PIBIO_STORAGE_DEACTIVATE_FN,
    pub QueryExtendedInfo: PIBIO_STORAGE_QUERY_EXTENDED_INFO_FN,
    pub NotifyDatabaseChange: PIBIO_STORAGE_NOTIFY_DATABASE_CHANGE_FN,
    pub Reserved1: PIBIO_STORAGE_RESERVED_1_FN,
    pub Reserved2: PIBIO_STORAGE_RESERVED_2_FN,
    pub UpdateRecordBegin: PIBIO_STORAGE_UPDATE_RECORD_BEGIN_FN,
    pub UpdateRecordCommit: PIBIO_STORAGE_UPDATE_RECORD_COMMIT_FN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for WINBIO_STORAGE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for WINBIO_STORAGE_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for WINBIO_STORAGE_INTERFACE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for WINBIO_STORAGE_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_STORAGE_INTERFACE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for WINBIO_STORAGE_INTERFACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for WINBIO_STORAGE_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
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
impl ::core::marker::Copy for WINBIO_STORAGE_RECORD {}
impl ::core::clone::Clone for WINBIO_STORAGE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_STORAGE_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_STORAGE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_STORAGE_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_STORAGE_RECORD {}
impl ::core::default::Default for WINBIO_STORAGE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_STORAGE_SCHEMA {
    pub BiometricFactor: u32,
    pub DatabaseId: ::windows::core::GUID,
    pub DataFormat: ::windows::core::GUID,
    pub Attributes: u32,
    pub FilePath: [u16; 256],
    pub ConnectionString: [u16; 256],
}
impl ::core::marker::Copy for WINBIO_STORAGE_SCHEMA {}
impl ::core::clone::Clone for WINBIO_STORAGE_SCHEMA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_STORAGE_SCHEMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_STORAGE_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_STORAGE_SCHEMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_STORAGE_SCHEMA {}
impl ::core::default::Default for WINBIO_STORAGE_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_SUPPORTED_ALGORITHMS {
    pub PayloadSize: u32,
    pub WinBioHresult: ::windows::core::HRESULT,
    pub NumberOfAlgorithms: u32,
    pub AlgorithmData: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_SUPPORTED_ALGORITHMS {}
impl ::core::clone::Clone for WINBIO_SUPPORTED_ALGORITHMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_SUPPORTED_ALGORITHMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_SUPPORTED_ALGORITHMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_SUPPORTED_ALGORITHMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_SUPPORTED_ALGORITHMS {}
impl ::core::default::Default for WINBIO_SUPPORTED_ALGORITHMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
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
impl ::core::marker::Copy for WINBIO_UNIT_SCHEMA {}
impl ::core::clone::Clone for WINBIO_UNIT_SCHEMA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_UNIT_SCHEMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_UNIT_SCHEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_UNIT_SCHEMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_UNIT_SCHEMA {}
impl ::core::default::Default for WINBIO_UNIT_SCHEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_UPDATE_FIRMWARE {
    pub PayloadSize: u32,
    pub FirmwareData: WINBIO_DATA,
}
impl ::core::marker::Copy for WINBIO_UPDATE_FIRMWARE {}
impl ::core::clone::Clone for WINBIO_UPDATE_FIRMWARE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_UPDATE_FIRMWARE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_UPDATE_FIRMWARE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_UPDATE_FIRMWARE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_UPDATE_FIRMWARE {}
impl ::core::default::Default for WINBIO_UPDATE_FIRMWARE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub struct WINBIO_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl ::core::marker::Copy for WINBIO_VERSION {}
impl ::core::clone::Clone for WINBIO_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WINBIO_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINBIO_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINBIO_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINBIO_VERSION {}
impl ::core::default::Default for WINBIO_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_WBDI_MAJOR_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
pub const WINBIO_WBDI_MINOR_VERSION: u32 = 0u32;
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioAcquireFocus() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAcquireFocus() -> ::windows::core::HRESULT;
        }
        WinBioAcquireFocus().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncEnumBiometricUnits(frameworkhandle: u32, factor: u32) -> ::windows::core::HRESULT;
        }
        WinBioAsyncEnumBiometricUnits(::core::mem::transmute(frameworkhandle), ::core::mem::transmute(factor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncEnumDatabases(frameworkhandle: u32, factor: u32) -> ::windows::core::HRESULT;
        }
        WinBioAsyncEnumDatabases(::core::mem::transmute(frameworkhandle), ::core::mem::transmute(factor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncEnumServiceProviders(frameworkhandle: u32, factor: u32) -> ::windows::core::HRESULT;
        }
        WinBioAsyncEnumServiceProviders(::core::mem::transmute(frameworkhandle), ::core::mem::transmute(factor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncMonitorFrameworkChanges(frameworkhandle: u32, changetypes: u32) -> ::windows::core::HRESULT;
        }
        WinBioAsyncMonitorFrameworkChanges(::core::mem::transmute(frameworkhandle), ::core::mem::transmute(changetypes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioAsyncOpenFramework<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: Param1, messagecode: u32, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: *const ::core::ffi::c_void, asynchronousopen: Param5) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncOpenFramework(notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: ::windows::core::RawPtr, userdata: *const ::core::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, frameworkhandle: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        WinBioAsyncOpenFramework(::core::mem::transmute(notificationmethod), targetwindow.into_param().abi(), ::core::mem::transmute(messagecode), ::core::mem::transmute(callbackroutine), ::core::mem::transmute(userdata), asynchronousopen.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioAsyncOpenSession<'a, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param11: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::core::GUID, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: Param7, messagecode: u32, callbackroutine: PWINBIO_ASYNC_COMPLETION_CALLBACK, userdata: *const ::core::ffi::c_void, asynchronousopen: Param11) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioAsyncOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::core::GUID, notificationmethod: WINBIO_ASYNC_NOTIFICATION_METHOD, targetwindow: super::super::Foundation::HWND, messagecode: u32, callbackroutine: ::windows::core::RawPtr, userdata: *const ::core::ffi::c_void, asynchronousopen: super::super::Foundation::BOOL, sessionhandle: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        WinBioAsyncOpenSession(::core::mem::transmute(factor), ::core::mem::transmute(pooltype), ::core::mem::transmute(flags), ::core::mem::transmute(unitarray), ::core::mem::transmute(unitcount), ::core::mem::transmute(databaseid), ::core::mem::transmute(notificationmethod), targetwindow.into_param().abi(), ::core::mem::transmute(messagecode), ::core::mem::transmute(callbackroutine), ::core::mem::transmute(userdata), asynchronousopen.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioCancel(sessionhandle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCancel(sessionhandle: u32) -> ::windows::core::HRESULT;
        }
        WinBioCancel(::core::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: *mut u32, sample: *mut *mut WINBIO_BIR, samplesize: *mut usize, rejectdetail: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCaptureSample(sessionhandle: u32, purpose: u8, flags: u8, unitid: *mut u32, sample: *mut *mut WINBIO_BIR, samplesize: *mut usize, rejectdetail: *mut u32) -> ::windows::core::HRESULT;
        }
        WinBioCaptureSample(::core::mem::transmute(sessionhandle), ::core::mem::transmute(purpose), ::core::mem::transmute(flags), ::core::mem::transmute(unitid), ::core::mem::transmute(sample), ::core::mem::transmute(samplesize), ::core::mem::transmute(rejectdetail)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: PWINBIO_CAPTURE_CALLBACK, capturecallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCaptureSampleWithCallback(sessionhandle: u32, purpose: u8, flags: u8, capturecallback: ::windows::core::RawPtr, capturecallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WinBioCaptureSampleWithCallback(::core::mem::transmute(sessionhandle), ::core::mem::transmute(purpose), ::core::mem::transmute(flags), ::core::mem::transmute(capturecallback), ::core::mem::transmute(capturecallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioCloseFramework(frameworkhandle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCloseFramework(frameworkhandle: u32) -> ::windows::core::HRESULT;
        }
        WinBioCloseFramework(::core::mem::transmute(frameworkhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioCloseSession(sessionhandle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioCloseSession(sessionhandle: u32) -> ::windows::core::HRESULT;
        }
        WinBioCloseSession(::core::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioControlUnit(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT;
        }
        WinBioControlUnit(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid), ::core::mem::transmute(component), ::core::mem::transmute(controlcode), ::core::mem::transmute(sendbuffer), ::core::mem::transmute(sendbuffersize), ::core::mem::transmute(receivebuffer), ::core::mem::transmute(receivebuffersize), ::core::mem::transmute(receivedatasize), ::core::mem::transmute(operationstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioControlUnitPrivileged(sessionhandle: u32, unitid: u32, component: WINBIO_COMPONENT, controlcode: u32, sendbuffer: *const u8, sendbuffersize: usize, receivebuffer: *mut u8, receivebuffersize: usize, receivedatasize: *mut usize, operationstatus: *mut u32) -> ::windows::core::HRESULT;
        }
        WinBioControlUnitPrivileged(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid), ::core::mem::transmute(component), ::core::mem::transmute(controlcode), ::core::mem::transmute(sendbuffer), ::core::mem::transmute(sendbuffersize), ::core::mem::transmute(receivebuffer), ::core::mem::transmute(receivebuffersize), ::core::mem::transmute(receivedatasize), ::core::mem::transmute(operationstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioDeleteTemplate(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8) -> ::windows::core::HRESULT;
        }
        WinBioDeleteTemplate(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid), ::core::mem::transmute(identity), ::core::mem::transmute(subfactor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollBegin(sessionhandle: u32, subfactor: u8, unitid: u32) -> ::windows::core::HRESULT;
        }
        WinBioEnrollBegin(::core::mem::transmute(sessionhandle), ::core::mem::transmute(subfactor), ::core::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnrollCapture(sessionhandle: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollCapture(sessionhandle: u32, rejectdetail: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        WinBioEnrollCapture(::core::mem::transmute(sessionhandle), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: PWINBIO_ENROLL_CAPTURE_CALLBACK, enrollcallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollCaptureWithCallback(sessionhandle: u32, enrollcallback: ::windows::core::RawPtr, enrollcallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WinBioEnrollCaptureWithCallback(::core::mem::transmute(sessionhandle), ::core::mem::transmute(enrollcallback), ::core::mem::transmute(enrollcallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnrollCommit(sessionhandle: u32, identity: *mut WINBIO_IDENTITY, isnewtemplate: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollCommit(sessionhandle: u32, identity: *mut WINBIO_IDENTITY, isnewtemplate: *mut u8) -> ::windows::core::HRESULT;
        }
        WinBioEnrollCommit(::core::mem::transmute(sessionhandle), ::core::mem::transmute(identity), ::core::mem::transmute(isnewtemplate)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnrollDiscard(sessionhandle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollDiscard(sessionhandle: u32) -> ::windows::core::HRESULT;
        }
        WinBioEnrollDiscard(::core::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnrollSelect(sessionhandle: u32, selectorvalue: u64) -> ::windows::core::HRESULT;
        }
        WinBioEnrollSelect(::core::mem::transmute(sessionhandle), ::core::mem::transmute(selectorvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnumBiometricUnits(factor: u32, unitschemaarray: *mut *mut WINBIO_UNIT_SCHEMA, unitcount: *mut usize) -> ::windows::core::HRESULT;
        }
        WinBioEnumBiometricUnits(::core::mem::transmute(factor), ::core::mem::transmute(unitschemaarray), ::core::mem::transmute(unitcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnumDatabases(factor: u32, storageschemaarray: *mut *mut WINBIO_STORAGE_SCHEMA, storagecount: *mut usize) -> ::windows::core::HRESULT;
        }
        WinBioEnumDatabases(::core::mem::transmute(factor), ::core::mem::transmute(storageschemaarray), ::core::mem::transmute(storagecount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: *mut usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnumEnrollments(sessionhandle: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactorarray: *mut *mut u8, subfactorcount: *mut usize) -> ::windows::core::HRESULT;
        }
        WinBioEnumEnrollments(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid), ::core::mem::transmute(identity), ::core::mem::transmute(subfactorarray), ::core::mem::transmute(subfactorcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioEnumServiceProviders(factor: u32, bspschemaarray: *mut *mut WINBIO_BSP_SCHEMA, bspcount: *mut usize) -> ::windows::core::HRESULT;
        }
        WinBioEnumServiceProviders(::core::mem::transmute(factor), ::core::mem::transmute(bspschemaarray), ::core::mem::transmute(bspcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioFree(address: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioFree(address: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WinBioFree(::core::mem::transmute(address)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioGetCredentialState<'a, Param0: ::windows::core::IntoParam<'a, WINBIO_IDENTITY>>(identity: Param0, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::core::Result<WINBIO_CREDENTIAL_STATE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetCredentialState(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE, credentialstate: *mut WINBIO_CREDENTIAL_STATE) -> ::windows::core::HRESULT;
        }
        let mut result__: WINBIO_CREDENTIAL_STATE = ::core::mem::zeroed();
        WinBioGetCredentialState(identity.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<WINBIO_CREDENTIAL_STATE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetDomainLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
        }
        WinBioGetDomainLogonSetting(::core::mem::transmute(value), ::core::mem::transmute(source))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetEnabledSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
        }
        WinBioGetEnabledSetting(::core::mem::transmute(value), ::core::mem::transmute(source))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetEnrolledFactors(accountowner: *const WINBIO_IDENTITY, enrolledfactors: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        WinBioGetEnrolledFactors(::core::mem::transmute(accountowner), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetLogonSetting(value: *mut u8, source: *mut WINBIO_SETTING_SOURCE);
        }
        WinBioGetLogonSetting(::core::mem::transmute(value), ::core::mem::transmute(source))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::core::ffi::c_void, propertybuffersize: *mut usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioGetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *mut *mut ::core::ffi::c_void, propertybuffersize: *mut usize) -> ::windows::core::HRESULT;
        }
        WinBioGetProperty(::core::mem::transmute(sessionhandle), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertyid), ::core::mem::transmute(unitid), ::core::mem::transmute(identity), ::core::mem::transmute(subfactor), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioIdentify(sessionhandle: u32, unitid: *mut u32, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioIdentify(sessionhandle: u32, unitid: *mut u32, identity: *mut WINBIO_IDENTITY, subfactor: *mut u8, rejectdetail: *mut u32) -> ::windows::core::HRESULT;
        }
        WinBioIdentify(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid), ::core::mem::transmute(identity), ::core::mem::transmute(subfactor), ::core::mem::transmute(rejectdetail)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: PWINBIO_IDENTIFY_CALLBACK, identifycallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioIdentifyWithCallback(sessionhandle: u32, identifycallback: ::windows::core::RawPtr, identifycallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WinBioIdentifyWithCallback(::core::mem::transmute(sessionhandle), ::core::mem::transmute(identifycallback), ::core::mem::transmute(identifycallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioImproveBegin(sessionhandle: u32, unitid: u32) -> ::windows::core::HRESULT;
        }
        WinBioImproveBegin(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioImproveEnd(sessionhandle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioImproveEnd(sessionhandle: u32) -> ::windows::core::HRESULT;
        }
        WinBioImproveEnd(::core::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioLocateSensor(sessionhandle: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioLocateSensor(sessionhandle: u32, unitid: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        WinBioLocateSensor(::core::mem::transmute(sessionhandle), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: PWINBIO_LOCATE_SENSOR_CALLBACK, locatecallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioLocateSensorWithCallback(sessionhandle: u32, locatecallback: ::windows::core::RawPtr, locatecallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WinBioLocateSensorWithCallback(::core::mem::transmute(sessionhandle), ::core::mem::transmute(locatecallback), ::core::mem::transmute(locatecallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioLockUnit(sessionhandle: u32, unitid: u32) -> ::windows::core::HRESULT;
        }
        WinBioLockUnit(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioLogonIdentifiedUser(sessionhandle: u32) -> ::windows::core::HRESULT;
        }
        WinBioLogonIdentifiedUser(::core::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioMonitorPresence(sessionhandle: u32, unitid: u32) -> ::windows::core::HRESULT;
        }
        WinBioMonitorPresence(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioOpenSession(factor: u32, pooltype: WINBIO_POOL, flags: u32, unitarray: *const u32, unitcount: usize, databaseid: *const ::windows::core::GUID, sessionhandle: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        WinBioOpenSession(::core::mem::transmute(factor), ::core::mem::transmute(pooltype), ::core::mem::transmute(flags), ::core::mem::transmute(unitarray), ::core::mem::transmute(unitcount), ::core::mem::transmute(databaseid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: PWINBIO_EVENT_CALLBACK, eventcallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioRegisterEventMonitor(sessionhandle: u32, eventmask: u32, eventcallback: ::windows::core::RawPtr, eventcallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WinBioRegisterEventMonitor(::core::mem::transmute(sessionhandle), ::core::mem::transmute(eventmask), ::core::mem::transmute(eventcallback), ::core::mem::transmute(eventcallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioReleaseFocus() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioReleaseFocus() -> ::windows::core::HRESULT;
        }
        WinBioReleaseFocus().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioRemoveAllCredentials() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioRemoveAllCredentials() -> ::windows::core::HRESULT;
        }
        WinBioRemoveAllCredentials().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioRemoveAllDomainCredentials() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioRemoveAllDomainCredentials() -> ::windows::core::HRESULT;
        }
        WinBioRemoveAllDomainCredentials().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioRemoveCredential<'a, Param0: ::windows::core::IntoParam<'a, WINBIO_IDENTITY>>(identity: Param0, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioRemoveCredential(identity: WINBIO_IDENTITY, r#type: WINBIO_CREDENTIAL_TYPE) -> ::windows::core::HRESULT;
        }
        WinBioRemoveCredential(identity.into_param().abi(), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: *const u8, credentialsize: usize, format: WINBIO_CREDENTIAL_FORMAT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioSetCredential(r#type: WINBIO_CREDENTIAL_TYPE, credential: *const u8, credentialsize: usize, format: WINBIO_CREDENTIAL_FORMAT) -> ::windows::core::HRESULT;
        }
        WinBioSetCredential(::core::mem::transmute(r#type), ::core::mem::transmute(credential), ::core::mem::transmute(credentialsize), ::core::mem::transmute(format)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *const ::core::ffi::c_void, propertybuffersize: usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioSetProperty(sessionhandle: u32, propertytype: u32, propertyid: u32, unitid: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, propertybuffer: *const ::core::ffi::c_void, propertybuffersize: usize) -> ::windows::core::HRESULT;
        }
        WinBioSetProperty(::core::mem::transmute(sessionhandle), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertyid), ::core::mem::transmute(unitid), ::core::mem::transmute(identity), ::core::mem::transmute(subfactor), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioUnlockUnit(sessionhandle: u32, unitid: u32) -> ::windows::core::HRESULT;
        }
        WinBioUnlockUnit(::core::mem::transmute(sessionhandle), ::core::mem::transmute(unitid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioUnregisterEventMonitor(sessionhandle: u32) -> ::windows::core::HRESULT;
        }
        WinBioUnregisterEventMonitor(::core::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: *mut u32, r#match: *mut u8, rejectdetail: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioVerify(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, unitid: *mut u32, r#match: *mut u8, rejectdetail: *mut u32) -> ::windows::core::HRESULT;
        }
        WinBioVerify(::core::mem::transmute(sessionhandle), ::core::mem::transmute(identity), ::core::mem::transmute(subfactor), ::core::mem::transmute(unitid), ::core::mem::transmute(r#match), ::core::mem::transmute(rejectdetail)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: PWINBIO_VERIFY_CALLBACK, verifycallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioVerifyWithCallback(sessionhandle: u32, identity: *const WINBIO_IDENTITY, subfactor: u8, verifycallback: ::windows::core::RawPtr, verifycallbackcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        WinBioVerifyWithCallback(::core::mem::transmute(sessionhandle), ::core::mem::transmute(identity), ::core::mem::transmute(subfactor), ::core::mem::transmute(verifycallback), ::core::mem::transmute(verifycallbackcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_BiometricFramework'*"]
#[inline]
pub unsafe fn WinBioWait(sessionhandle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinBioWait(sessionhandle: u32) -> ::windows::core::HRESULT;
        }
        WinBioWait(::core::mem::transmute(sessionhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct _WINIBIO_ENGINE_CONTEXT(pub u8);
#[repr(C)]
pub struct _WINIBIO_SENSOR_CONTEXT(pub u8);
#[repr(C)]
pub struct _WINIBIO_STORAGE_CONTEXT(pub u8);
