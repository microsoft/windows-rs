#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CellularClass(i32);
#[repr(transparent)]
pub struct DeleteSmsMessageOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeleteSmsMessagesOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GetSmsDeviceOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GetSmsMessageOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GetSmsMessagesOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsAppMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsBinaryMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsBroadcastMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsDevice2Statics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsDeviceMessageStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsDeviceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsFilterRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsFilterRuleFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsFilterRules(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsFilterRulesFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsMessageBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsMessageReceivedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsMessageRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsMessageRegistrationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsReceivedEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsReceivedEventDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsSendMessageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsStatusMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsTextMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsTextMessage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsTextMessageStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsVoicemailMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsWapMessage(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LegacySmsApiContract(i32);
#[repr(transparent)]
pub struct SendSmsMessageOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsAppMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsBinaryMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsBroadcastMessage(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmsBroadcastType(i32);
#[repr(C)]
pub struct SmsDataFormat(i32);
#[repr(transparent)]
pub struct SmsDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsDeviceMessageStore(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmsDeviceStatus(i32);
#[repr(transparent)]
pub struct SmsDeviceStatusChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmsEncodedLength(i32);
#[repr(C)]
pub struct SmsEncoding(i32);
#[repr(C)]
pub struct SmsFilterActionType(i32);
#[repr(transparent)]
pub struct SmsFilterRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsFilterRules(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmsGeographicalScope(i32);
#[repr(C)]
pub struct SmsMessageClass(i32);
#[repr(C)]
pub struct SmsMessageFilter(i32);
#[repr(transparent)]
pub struct SmsMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsMessageReceivedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsMessageReceivedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsMessageRegistration(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmsMessageType(i32);
#[repr(C)]
pub struct SmsModemErrorCode(i32);
#[repr(transparent)]
pub struct SmsReceivedEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsSendMessageResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsStatusMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsTextMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsTextMessage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsVoicemailMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsWapMessage(pub *mut ::core::ffi::c_void);
