#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IDummyMBNUCMExt(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionContextEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionManagerEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionProfileEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionProfileManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnConnectionProfileManagerEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnDeviceService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnDeviceServiceStateEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnDeviceServicesContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnDeviceServicesEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnDeviceServicesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnInterface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnInterfaceEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnInterfaceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnInterfaceManagerEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnMultiCarrier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnMultiCarrierEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnPinEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnPinManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnPinManagerEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnRadio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnRadioEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnRegistrationEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnServiceActivation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnServiceActivationEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnSignal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnSignalEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnSms(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnSmsConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnSmsEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnSmsReadMsgPdu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnSmsReadMsgTextCdma(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnSubscriberInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnVendorSpecificEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMbnVendorSpecificOperation(pub *mut ::core::ffi::c_void);
pub struct MBN_ACTIVATION_STATE(i32);
pub struct MBN_AUTH_PROTOCOL(i32);
pub struct MBN_BAND_CLASS(i32);
pub struct MBN_CELLULAR_CLASS(i32);
pub struct MBN_COMPRESSION(i32);
pub struct MBN_CONNECTION_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_CONTEXT(i32);
pub struct MBN_CONTEXT_CONSTANTS(i32);
pub struct MBN_CONTEXT_TYPE(i32);
pub struct MBN_CTRL_CAPS(i32);
pub struct MBN_DATA_CLASS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_DEVICE_SERVICE(i32);
pub struct MBN_DEVICE_SERVICES_INTERFACE_STATE(i32);
pub struct MBN_DEVICE_SERVICE_SESSIONS_STATE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_INTERFACE_CAPS(i32);
pub struct MBN_INTERFACE_CAPS_CONSTANTS(i32);
pub struct MBN_MSG_STATUS(i32);
pub struct MBN_PIN_CONSTANTS(i32);
pub struct MBN_PIN_FORMAT(i32);
pub struct MBN_PIN_INFO(i32);
pub struct MBN_PIN_MODE(i32);
pub struct MBN_PIN_STATE(i32);
pub struct MBN_PIN_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER2(i32);
pub struct MBN_PROVIDER_CONSTANTS(i32);
pub struct MBN_PROVIDER_STATE(i32);
pub struct MBN_RADIO(i32);
pub struct MBN_READY_STATE(i32);
pub struct MBN_REGISTER_MODE(i32);
pub struct MBN_REGISTER_STATE(i32);
pub struct MBN_REGISTRATION_CONSTANTS(i32);
pub struct MBN_SIGNAL_CONSTANTS(i32);
pub struct MBN_SMS_CAPS(i32);
pub struct MBN_SMS_CDMA_ENCODING(i32);
pub struct MBN_SMS_CDMA_LANG(i32);
pub struct MBN_SMS_FILTER(i32);
pub struct MBN_SMS_FLAG(i32);
pub struct MBN_SMS_FORMAT(i32);
pub struct MBN_SMS_STATUS_FLAG(i32);
pub struct MBN_SMS_STATUS_INFO(i32);
pub struct MBN_VOICE_CALL_STATE(i32);
pub struct MBN_VOICE_CLASS(i32);
pub struct MbnConnectionManager(i32);
pub struct MbnConnectionProfileManager(i32);
pub struct MbnDeviceServicesManager(i32);
pub struct MbnInterfaceManager(i32);
pub struct WWAEXT_SMS_CONSTANTS(i32);
pub struct __DummyPinType__(i32);
pub struct __mbnapi_ReferenceRemainingTypes__(i32);
