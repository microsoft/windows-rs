#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IDummyMBNUCMExt(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDummyMBNUCMExt {}
impl ::core::clone::Clone for IDummyMBNUCMExt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnection {}
impl ::core::clone::Clone for IMbnConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionContext {}
impl ::core::clone::Clone for IMbnConnectionContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionContextEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionContextEvents {}
impl ::core::clone::Clone for IMbnConnectionContextEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionEvents {}
impl ::core::clone::Clone for IMbnConnectionEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionManager {}
impl ::core::clone::Clone for IMbnConnectionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionManagerEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionManagerEvents {}
impl ::core::clone::Clone for IMbnConnectionManagerEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionProfile {}
impl ::core::clone::Clone for IMbnConnectionProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionProfileEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionProfileEvents {}
impl ::core::clone::Clone for IMbnConnectionProfileEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionProfileManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionProfileManager {}
impl ::core::clone::Clone for IMbnConnectionProfileManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnConnectionProfileManagerEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnConnectionProfileManagerEvents {}
impl ::core::clone::Clone for IMbnConnectionProfileManagerEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnDeviceService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnDeviceService {}
impl ::core::clone::Clone for IMbnDeviceService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnDeviceServiceStateEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnDeviceServiceStateEvents {}
impl ::core::clone::Clone for IMbnDeviceServiceStateEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnDeviceServicesContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnDeviceServicesContext {}
impl ::core::clone::Clone for IMbnDeviceServicesContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnDeviceServicesEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnDeviceServicesEvents {}
impl ::core::clone::Clone for IMbnDeviceServicesEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnDeviceServicesManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnDeviceServicesManager {}
impl ::core::clone::Clone for IMbnDeviceServicesManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnInterface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnInterface {}
impl ::core::clone::Clone for IMbnInterface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnInterfaceEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnInterfaceEvents {}
impl ::core::clone::Clone for IMbnInterfaceEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnInterfaceManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnInterfaceManager {}
impl ::core::clone::Clone for IMbnInterfaceManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnInterfaceManagerEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnInterfaceManagerEvents {}
impl ::core::clone::Clone for IMbnInterfaceManagerEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnMultiCarrier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnMultiCarrier {}
impl ::core::clone::Clone for IMbnMultiCarrier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnMultiCarrierEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnMultiCarrierEvents {}
impl ::core::clone::Clone for IMbnMultiCarrierEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnPin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnPin {}
impl ::core::clone::Clone for IMbnPin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnPinEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnPinEvents {}
impl ::core::clone::Clone for IMbnPinEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnPinManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnPinManager {}
impl ::core::clone::Clone for IMbnPinManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnPinManagerEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnPinManagerEvents {}
impl ::core::clone::Clone for IMbnPinManagerEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnRadio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnRadio {}
impl ::core::clone::Clone for IMbnRadio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnRadioEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnRadioEvents {}
impl ::core::clone::Clone for IMbnRadioEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnRegistration {}
impl ::core::clone::Clone for IMbnRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnRegistrationEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnRegistrationEvents {}
impl ::core::clone::Clone for IMbnRegistrationEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnServiceActivation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnServiceActivation {}
impl ::core::clone::Clone for IMbnServiceActivation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnServiceActivationEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnServiceActivationEvents {}
impl ::core::clone::Clone for IMbnServiceActivationEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnSignal(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnSignal {}
impl ::core::clone::Clone for IMbnSignal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnSignalEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnSignalEvents {}
impl ::core::clone::Clone for IMbnSignalEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnSms(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnSms {}
impl ::core::clone::Clone for IMbnSms {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnSmsConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnSmsConfiguration {}
impl ::core::clone::Clone for IMbnSmsConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnSmsEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnSmsEvents {}
impl ::core::clone::Clone for IMbnSmsEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnSmsReadMsgPdu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnSmsReadMsgPdu {}
impl ::core::clone::Clone for IMbnSmsReadMsgPdu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnSmsReadMsgTextCdma(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnSmsReadMsgTextCdma {}
impl ::core::clone::Clone for IMbnSmsReadMsgTextCdma {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnSubscriberInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnSubscriberInformation {}
impl ::core::clone::Clone for IMbnSubscriberInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnVendorSpecificEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnVendorSpecificEvents {}
impl ::core::clone::Clone for IMbnVendorSpecificEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMbnVendorSpecificOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMbnVendorSpecificOperation {}
impl ::core::clone::Clone for IMbnVendorSpecificOperation {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MBN_ACTIVATION_STATE_NONE: i32 = 0i32;
pub const MBN_ACTIVATION_STATE_ACTIVATED: i32 = 1i32;
pub const MBN_ACTIVATION_STATE_ACTIVATING: i32 = 2i32;
pub const MBN_ACTIVATION_STATE_DEACTIVATED: i32 = 3i32;
pub const MBN_ACTIVATION_STATE_DEACTIVATING: i32 = 4i32;
pub const MBN_AUTH_PROTOCOL_NONE: i32 = 0i32;
pub const MBN_AUTH_PROTOCOL_PAP: i32 = 1i32;
pub const MBN_AUTH_PROTOCOL_CHAP: i32 = 2i32;
pub const MBN_AUTH_PROTOCOL_MSCHAPV2: i32 = 3i32;
pub const MBN_BAND_CLASS_NONE: i32 = 0i32;
pub const MBN_BAND_CLASS_0: i32 = 1i32;
pub const MBN_BAND_CLASS_I: i32 = 2i32;
pub const MBN_BAND_CLASS_II: i32 = 4i32;
pub const MBN_BAND_CLASS_III: i32 = 8i32;
pub const MBN_BAND_CLASS_IV: i32 = 16i32;
pub const MBN_BAND_CLASS_V: i32 = 32i32;
pub const MBN_BAND_CLASS_VI: i32 = 64i32;
pub const MBN_BAND_CLASS_VII: i32 = 128i32;
pub const MBN_BAND_CLASS_VIII: i32 = 256i32;
pub const MBN_BAND_CLASS_IX: i32 = 512i32;
pub const MBN_BAND_CLASS_X: i32 = 1024i32;
pub const MBN_BAND_CLASS_XI: i32 = 2048i32;
pub const MBN_BAND_CLASS_XII: i32 = 4096i32;
pub const MBN_BAND_CLASS_XIII: i32 = 8192i32;
pub const MBN_BAND_CLASS_XIV: i32 = 16384i32;
pub const MBN_BAND_CLASS_XV: i32 = 32768i32;
pub const MBN_BAND_CLASS_XVI: i32 = 65536i32;
pub const MBN_BAND_CLASS_XVII: i32 = 131072i32;
pub const MBN_BAND_CLASS_CUSTOM: i32 = -2147483648i32;
pub const MBN_CELLULAR_CLASS_NONE: i32 = 0i32;
pub const MBN_CELLULAR_CLASS_GSM: i32 = 1i32;
pub const MBN_CELLULAR_CLASS_CDMA: i32 = 2i32;
pub const MBN_COMPRESSION_NONE: i32 = 0i32;
pub const MBN_COMPRESSION_ENABLE: i32 = 1i32;
pub const MBN_CONNECTION_MODE_PROFILE: i32 = 0i32;
pub const MBN_CONNECTION_MODE_TMP_PROFILE: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_CONTEXT {
    pub contextID: u32,
    pub contextType: MBN_CONTEXT_TYPE,
    pub accessString: super::super::Foundation::BSTR,
    pub userName: super::super::Foundation::BSTR,
    pub password: super::super::Foundation::BSTR,
    pub compression: MBN_COMPRESSION,
    pub authType: MBN_AUTH_PROTOCOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MBN_ACCESSSTRING_LEN: i32 = 100i32;
pub const MBN_USERNAME_LEN: i32 = 255i32;
pub const MBN_PASSWORD_LEN: i32 = 255i32;
pub const MBN_CONTEXT_ID_APPEND: i32 = -1i32;
pub const MBN_CONTEXT_TYPE_NONE: i32 = 0i32;
pub const MBN_CONTEXT_TYPE_INTERNET: i32 = 1i32;
pub const MBN_CONTEXT_TYPE_VPN: i32 = 2i32;
pub const MBN_CONTEXT_TYPE_VOICE: i32 = 3i32;
pub const MBN_CONTEXT_TYPE_VIDEO_SHARE: i32 = 4i32;
pub const MBN_CONTEXT_TYPE_CUSTOM: i32 = 5i32;
pub const MBN_CONTEXT_TYPE_PURCHASE: i32 = 6i32;
pub const MBN_CTRL_CAPS_NONE: i32 = 0i32;
pub const MBN_CTRL_CAPS_REG_MANUAL: i32 = 1i32;
pub const MBN_CTRL_CAPS_HW_RADIO_SWITCH: i32 = 2i32;
pub const MBN_CTRL_CAPS_CDMA_MOBILE_IP: i32 = 4i32;
pub const MBN_CTRL_CAPS_CDMA_SIMPLE_IP: i32 = 8i32;
pub const MBN_CTRL_CAPS_PROTECT_UNIQUEID: i32 = 16i32;
pub const MBN_CTRL_CAPS_MODEL_MULTI_CARRIER: i32 = 32i32;
pub const MBN_CTRL_CAPS_USSD: i32 = 64i32;
pub const MBN_CTRL_CAPS_MULTI_MODE: i32 = 128i32;
pub const MBN_DATA_CLASS_NONE: i32 = 0i32;
pub const MBN_DATA_CLASS_GPRS: i32 = 1i32;
pub const MBN_DATA_CLASS_EDGE: i32 = 2i32;
pub const MBN_DATA_CLASS_UMTS: i32 = 4i32;
pub const MBN_DATA_CLASS_HSDPA: i32 = 8i32;
pub const MBN_DATA_CLASS_HSUPA: i32 = 16i32;
pub const MBN_DATA_CLASS_LTE: i32 = 32i32;
pub const MBN_DATA_CLASS_5G_NSA: i32 = 64i32;
pub const MBN_DATA_CLASS_5G_SA: i32 = 128i32;
pub const MBN_DATA_CLASS_1XRTT: i32 = 65536i32;
pub const MBN_DATA_CLASS_1XEVDO: i32 = 131072i32;
pub const MBN_DATA_CLASS_1XEVDO_REVA: i32 = 262144i32;
pub const MBN_DATA_CLASS_1XEVDV: i32 = 524288i32;
pub const MBN_DATA_CLASS_3XRTT: i32 = 1048576i32;
pub const MBN_DATA_CLASS_1XEVDO_REVB: i32 = 2097152i32;
pub const MBN_DATA_CLASS_UMB: i32 = 4194304i32;
pub const MBN_DATA_CLASS_CUSTOM: i32 = -2147483648i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_DEVICE_SERVICE {
    pub deviceServiceID: super::super::Foundation::BSTR,
    pub dataWriteSupported: i16,
    pub dataReadSupported: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_DEVICE_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_DEVICE_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL: i32 = 0i32;
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL: i32 = 1i32;
pub const MBN_DEVICE_SERVICE_SESSIONS_RESTORED: i32 = 0i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_INTERFACE_CAPS {
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub voiceClass: MBN_VOICE_CLASS,
    pub dataClass: u32,
    pub customDataClass: super::super::Foundation::BSTR,
    pub gsmBandClass: u32,
    pub cdmaBandClass: u32,
    pub customBandClass: super::super::Foundation::BSTR,
    pub smsCaps: u32,
    pub controlCaps: u32,
    pub deviceID: super::super::Foundation::BSTR,
    pub manufacturer: super::super::Foundation::BSTR,
    pub model: super::super::Foundation::BSTR,
    pub firmwareInfo: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_INTERFACE_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_INTERFACE_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MBN_DEVICEID_LEN: i32 = 18i32;
pub const MBN_MANUFACTURER_LEN: i32 = 32i32;
pub const MBN_MODEL_LEN: i32 = 32i32;
pub const MBN_FIRMWARE_LEN: i32 = 32i32;
pub const MBN_MSG_STATUS_NEW: i32 = 0i32;
pub const MBN_MSG_STATUS_OLD: i32 = 1i32;
pub const MBN_MSG_STATUS_DRAFT: i32 = 2i32;
pub const MBN_MSG_STATUS_SENT: i32 = 3i32;
pub const MBN_ATTEMPTS_REMAINING_UNKNOWN: i32 = -1i32;
pub const MBN_PIN_LENGTH_UNKNOWN: i32 = -1i32;
pub const MBN_PIN_FORMAT_NONE: i32 = 0i32;
pub const MBN_PIN_FORMAT_NUMERIC: i32 = 1i32;
pub const MBN_PIN_FORMAT_ALPHANUMERIC: i32 = 2i32;
#[repr(C)]
pub struct MBN_PIN_INFO {
    pub pinState: MBN_PIN_STATE,
    pub pinType: MBN_PIN_TYPE,
    pub attemptsRemaining: u32,
}
impl ::core::marker::Copy for MBN_PIN_INFO {}
impl ::core::clone::Clone for MBN_PIN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MBN_PIN_MODE_ENABLED: i32 = 1i32;
pub const MBN_PIN_MODE_DISABLED: i32 = 2i32;
pub const MBN_PIN_STATE_NONE: i32 = 0i32;
pub const MBN_PIN_STATE_ENTER: i32 = 1i32;
pub const MBN_PIN_STATE_UNBLOCK: i32 = 2i32;
pub const MBN_PIN_TYPE_NONE: i32 = 0i32;
pub const MBN_PIN_TYPE_CUSTOM: i32 = 1i32;
pub const MBN_PIN_TYPE_PIN1: i32 = 2i32;
pub const MBN_PIN_TYPE_PIN2: i32 = 3i32;
pub const MBN_PIN_TYPE_DEVICE_SIM_PIN: i32 = 4i32;
pub const MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN: i32 = 5i32;
pub const MBN_PIN_TYPE_NETWORK_PIN: i32 = 6i32;
pub const MBN_PIN_TYPE_NETWORK_SUBSET_PIN: i32 = 7i32;
pub const MBN_PIN_TYPE_SVC_PROVIDER_PIN: i32 = 8i32;
pub const MBN_PIN_TYPE_CORPORATE_PIN: i32 = 9i32;
pub const MBN_PIN_TYPE_SUBSIDY_LOCK: i32 = 10i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER {
    pub providerID: super::super::Foundation::BSTR,
    pub providerState: u32,
    pub providerName: super::super::Foundation::BSTR,
    pub dataClass: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER2 {
    pub provider: MBN_PROVIDER,
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub signalStrength: u32,
    pub signalError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MBN_PROVIDER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MBN_PROVIDER2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MBN_PROVIDERNAME_LEN: i32 = 20i32;
pub const MBN_PROVIDERID_LEN: i32 = 6i32;
pub const MBN_PROVIDER_STATE_NONE: i32 = 0i32;
pub const MBN_PROVIDER_STATE_HOME: i32 = 1i32;
pub const MBN_PROVIDER_STATE_FORBIDDEN: i32 = 2i32;
pub const MBN_PROVIDER_STATE_PREFERRED: i32 = 4i32;
pub const MBN_PROVIDER_STATE_VISIBLE: i32 = 8i32;
pub const MBN_PROVIDER_STATE_REGISTERED: i32 = 16i32;
pub const MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER: i32 = 32i32;
pub const MBN_RADIO_OFF: i32 = 0i32;
pub const MBN_RADIO_ON: i32 = 1i32;
pub const MBN_READY_STATE_OFF: i32 = 0i32;
pub const MBN_READY_STATE_INITIALIZED: i32 = 1i32;
pub const MBN_READY_STATE_SIM_NOT_INSERTED: i32 = 2i32;
pub const MBN_READY_STATE_BAD_SIM: i32 = 3i32;
pub const MBN_READY_STATE_FAILURE: i32 = 4i32;
pub const MBN_READY_STATE_NOT_ACTIVATED: i32 = 5i32;
pub const MBN_READY_STATE_DEVICE_LOCKED: i32 = 6i32;
pub const MBN_READY_STATE_DEVICE_BLOCKED: i32 = 7i32;
pub const MBN_READY_STATE_NO_ESIM_PROFILE: i32 = 8i32;
pub const MBN_REGISTER_MODE_NONE: i32 = 0i32;
pub const MBN_REGISTER_MODE_AUTOMATIC: i32 = 1i32;
pub const MBN_REGISTER_MODE_MANUAL: i32 = 2i32;
pub const MBN_REGISTER_STATE_NONE: i32 = 0i32;
pub const MBN_REGISTER_STATE_DEREGISTERED: i32 = 1i32;
pub const MBN_REGISTER_STATE_SEARCHING: i32 = 2i32;
pub const MBN_REGISTER_STATE_HOME: i32 = 3i32;
pub const MBN_REGISTER_STATE_ROAMING: i32 = 4i32;
pub const MBN_REGISTER_STATE_PARTNER: i32 = 5i32;
pub const MBN_REGISTER_STATE_DENIED: i32 = 6i32;
pub const MBN_ROAMTEXT_LEN: i32 = 64i32;
pub const MBN_CDMA_DEFAULT_PROVIDER_ID: i32 = 0i32;
pub const MBN_RSSI_DEFAULT: i32 = -1i32;
pub const MBN_RSSI_DISABLE: i32 = 0i32;
pub const MBN_RSSI_UNKNOWN: i32 = 99i32;
pub const MBN_ERROR_RATE_UNKNOWN: i32 = 99i32;
pub const MBN_SMS_CAPS_NONE: i32 = 0i32;
pub const MBN_SMS_CAPS_PDU_RECEIVE: i32 = 1i32;
pub const MBN_SMS_CAPS_PDU_SEND: i32 = 2i32;
pub const MBN_SMS_CAPS_TEXT_RECEIVE: i32 = 4i32;
pub const MBN_SMS_CAPS_TEXT_SEND: i32 = 8i32;
pub const MBN_SMS_CDMA_ENCODING_OCTET: i32 = 0i32;
pub const MBN_SMS_CDMA_ENCODING_EPM: i32 = 1i32;
pub const MBN_SMS_CDMA_ENCODING_7BIT_ASCII: i32 = 2i32;
pub const MBN_SMS_CDMA_ENCODING_IA5: i32 = 3i32;
pub const MBN_SMS_CDMA_ENCODING_UNICODE: i32 = 4i32;
pub const MBN_SMS_CDMA_ENCODING_SHIFT_JIS: i32 = 5i32;
pub const MBN_SMS_CDMA_ENCODING_KOREAN: i32 = 6i32;
pub const MBN_SMS_CDMA_ENCODING_LATIN_HEBREW: i32 = 7i32;
pub const MBN_SMS_CDMA_ENCODING_LATIN: i32 = 8i32;
pub const MBN_SMS_CDMA_ENCODING_GSM_7BIT: i32 = 9i32;
pub const MBN_SMS_CDMA_LANG_NONE: i32 = 0i32;
pub const MBN_SMS_CDMA_LANG_ENGLISH: i32 = 1i32;
pub const MBN_SMS_CDMA_LANG_FRENCH: i32 = 2i32;
pub const MBN_SMS_CDMA_LANG_SPANISH: i32 = 3i32;
pub const MBN_SMS_CDMA_LANG_JAPANESE: i32 = 4i32;
pub const MBN_SMS_CDMA_LANG_KOREAN: i32 = 5i32;
pub const MBN_SMS_CDMA_LANG_CHINESE: i32 = 6i32;
pub const MBN_SMS_CDMA_LANG_HEBREW: i32 = 7i32;
#[repr(C)]
pub struct MBN_SMS_FILTER {
    pub flag: MBN_SMS_FLAG,
    pub messageIndex: u32,
}
impl ::core::marker::Copy for MBN_SMS_FILTER {}
impl ::core::clone::Clone for MBN_SMS_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MBN_SMS_FLAG_ALL: i32 = 0i32;
pub const MBN_SMS_FLAG_INDEX: i32 = 1i32;
pub const MBN_SMS_FLAG_NEW: i32 = 2i32;
pub const MBN_SMS_FLAG_OLD: i32 = 3i32;
pub const MBN_SMS_FLAG_SENT: i32 = 4i32;
pub const MBN_SMS_FLAG_DRAFT: i32 = 5i32;
pub const MBN_SMS_FORMAT_NONE: i32 = 0i32;
pub const MBN_SMS_FORMAT_PDU: i32 = 1i32;
pub const MBN_SMS_FORMAT_TEXT: i32 = 2i32;
pub const MBN_SMS_FLAG_NONE: i32 = 0i32;
pub const MBN_SMS_FLAG_MESSAGE_STORE_FULL: i32 = 1i32;
pub const MBN_SMS_FLAG_NEW_MESSAGE: i32 = 2i32;
#[repr(C)]
pub struct MBN_SMS_STATUS_INFO {
    pub flag: u32,
    pub messageIndex: u32,
}
impl ::core::marker::Copy for MBN_SMS_STATUS_INFO {}
impl ::core::clone::Clone for MBN_SMS_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MBN_VOICE_CALL_STATE_NONE: i32 = 0i32;
pub const MBN_VOICE_CALL_STATE_IN_PROGRESS: i32 = 1i32;
pub const MBN_VOICE_CALL_STATE_HANGUP: i32 = 2i32;
pub const MBN_VOICE_CLASS_NONE: i32 = 0i32;
pub const MBN_VOICE_CLASS_NO_VOICE: i32 = 1i32;
pub const MBN_VOICE_CLASS_SEPARATE_VOICE_DATA: i32 = 2i32;
pub const MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA: i32 = 3i32;
pub const MbnConnectionManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3187597404, data2: 17432, data3: 4573, data4: [144, 237, 0, 28, 37, 124, 207, 241] };
pub const MbnConnectionProfileManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3187597402, data2: 17432, data3: 4573, data4: [144, 237, 0, 28, 37, 124, 207, 241] };
pub const MbnDeviceServicesManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 577362595, data2: 10911, data3: 16741, data4: [165, 1, 206, 0, 166, 247, 167, 91] };
pub const MbnInterfaceManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3187597403, data2: 17432, data3: 4573, data4: [144, 237, 0, 28, 37, 124, 207, 241] };
pub const MBN_MESSAGE_INDEX_NONE: i32 = 0i32;
pub const MBN_CDMA_SHORT_MSG_SIZE_UNKNOWN: i32 = 0i32;
pub const MBN_CDMA_SHORT_MSG_SIZE_MAX: i32 = 160i32;
#[repr(C)]
pub struct __DummyPinType__ {
    pub pinType: u32,
}
impl ::core::marker::Copy for __DummyPinType__ {}
impl ::core::clone::Clone for __DummyPinType__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct __mbnapi_ReferenceRemainingTypes__ {
    pub bandClass: MBN_BAND_CLASS,
    pub contextConstants: MBN_CONTEXT_CONSTANTS,
    pub ctrlCaps: MBN_CTRL_CAPS,
    pub dataClass: MBN_DATA_CLASS,
    pub interfaceCapsConstants: MBN_INTERFACE_CAPS_CONSTANTS,
    pub pinConstants: MBN_PIN_CONSTANTS,
    pub providerConstants: MBN_PROVIDER_CONSTANTS,
    pub providerState: MBN_PROVIDER_STATE,
    pub registrationConstants: MBN_REGISTRATION_CONSTANTS,
    pub signalConstants: MBN_SIGNAL_CONSTANTS,
    pub smsCaps: MBN_SMS_CAPS,
    pub smsConstants: WWAEXT_SMS_CONSTANTS,
    pub wwaextSmsConstants: WWAEXT_SMS_CONSTANTS,
    pub smsStatusFlag: MBN_SMS_STATUS_FLAG,
}
impl ::core::marker::Copy for __mbnapi_ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __mbnapi_ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
