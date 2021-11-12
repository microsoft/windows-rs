#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CellularClass(pub i32);
impl CellularClass {
    pub const None: CellularClass = CellularClass(0i32);
    pub const Gsm: CellularClass = CellularClass(1i32);
    pub const Cdma: CellularClass = CellularClass(2i32);
}
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
#[repr(transparent)]
pub struct SmsBroadcastType(pub i32);
impl SmsBroadcastType {
    pub const Other: SmsBroadcastType = SmsBroadcastType(0i32);
    pub const CmasPresidential: SmsBroadcastType = SmsBroadcastType(1i32);
    pub const CmasExtreme: SmsBroadcastType = SmsBroadcastType(2i32);
    pub const CmasSevere: SmsBroadcastType = SmsBroadcastType(3i32);
    pub const CmasAmber: SmsBroadcastType = SmsBroadcastType(4i32);
    pub const CmasTest: SmsBroadcastType = SmsBroadcastType(5i32);
    pub const EUAlert1: SmsBroadcastType = SmsBroadcastType(6i32);
    pub const EUAlert2: SmsBroadcastType = SmsBroadcastType(7i32);
    pub const EUAlert3: SmsBroadcastType = SmsBroadcastType(8i32);
    pub const EUAlertAmber: SmsBroadcastType = SmsBroadcastType(9i32);
    pub const EUAlertInfo: SmsBroadcastType = SmsBroadcastType(10i32);
    pub const EtwsEarthquake: SmsBroadcastType = SmsBroadcastType(11i32);
    pub const EtwsTsunami: SmsBroadcastType = SmsBroadcastType(12i32);
    pub const EtwsTsunamiAndEarthquake: SmsBroadcastType = SmsBroadcastType(13i32);
    pub const LatAlertLocal: SmsBroadcastType = SmsBroadcastType(14i32);
}
#[repr(transparent)]
pub struct SmsDataFormat(pub i32);
impl SmsDataFormat {
    pub const Unknown: SmsDataFormat = SmsDataFormat(0i32);
    pub const CdmaSubmit: SmsDataFormat = SmsDataFormat(1i32);
    pub const GsmSubmit: SmsDataFormat = SmsDataFormat(2i32);
    pub const CdmaDeliver: SmsDataFormat = SmsDataFormat(3i32);
    pub const GsmDeliver: SmsDataFormat = SmsDataFormat(4i32);
}
#[repr(transparent)]
pub struct SmsDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsDeviceMessageStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsDeviceStatus(pub i32);
impl SmsDeviceStatus {
    pub const Off: SmsDeviceStatus = SmsDeviceStatus(0i32);
    pub const Ready: SmsDeviceStatus = SmsDeviceStatus(1i32);
    pub const SimNotInserted: SmsDeviceStatus = SmsDeviceStatus(2i32);
    pub const BadSim: SmsDeviceStatus = SmsDeviceStatus(3i32);
    pub const DeviceFailure: SmsDeviceStatus = SmsDeviceStatus(4i32);
    pub const SubscriptionNotActivated: SmsDeviceStatus = SmsDeviceStatus(5i32);
    pub const DeviceLocked: SmsDeviceStatus = SmsDeviceStatus(6i32);
    pub const DeviceBlocked: SmsDeviceStatus = SmsDeviceStatus(7i32);
}
#[repr(transparent)]
pub struct SmsDeviceStatusChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SmsEncodedLength(i32);
#[repr(transparent)]
pub struct SmsEncoding(pub i32);
impl SmsEncoding {
    pub const Unknown: SmsEncoding = SmsEncoding(0i32);
    pub const Optimal: SmsEncoding = SmsEncoding(1i32);
    pub const SevenBitAscii: SmsEncoding = SmsEncoding(2i32);
    pub const Unicode: SmsEncoding = SmsEncoding(3i32);
    pub const GsmSevenBit: SmsEncoding = SmsEncoding(4i32);
    pub const EightBit: SmsEncoding = SmsEncoding(5i32);
    pub const Latin: SmsEncoding = SmsEncoding(6i32);
    pub const Korean: SmsEncoding = SmsEncoding(7i32);
    pub const IA5: SmsEncoding = SmsEncoding(8i32);
    pub const ShiftJis: SmsEncoding = SmsEncoding(9i32);
    pub const LatinHebrew: SmsEncoding = SmsEncoding(10i32);
}
#[repr(transparent)]
pub struct SmsFilterActionType(pub i32);
impl SmsFilterActionType {
    pub const AcceptImmediately: SmsFilterActionType = SmsFilterActionType(0i32);
    pub const Drop: SmsFilterActionType = SmsFilterActionType(1i32);
    pub const Peek: SmsFilterActionType = SmsFilterActionType(2i32);
    pub const Accept: SmsFilterActionType = SmsFilterActionType(3i32);
}
#[repr(transparent)]
pub struct SmsFilterRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsFilterRules(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsGeographicalScope(pub i32);
impl SmsGeographicalScope {
    pub const None: SmsGeographicalScope = SmsGeographicalScope(0i32);
    pub const CellWithImmediateDisplay: SmsGeographicalScope = SmsGeographicalScope(1i32);
    pub const LocationArea: SmsGeographicalScope = SmsGeographicalScope(2i32);
    pub const Plmn: SmsGeographicalScope = SmsGeographicalScope(3i32);
    pub const Cell: SmsGeographicalScope = SmsGeographicalScope(4i32);
}
#[repr(transparent)]
pub struct SmsMessageClass(pub i32);
impl SmsMessageClass {
    pub const None: SmsMessageClass = SmsMessageClass(0i32);
    pub const Class0: SmsMessageClass = SmsMessageClass(1i32);
    pub const Class1: SmsMessageClass = SmsMessageClass(2i32);
    pub const Class2: SmsMessageClass = SmsMessageClass(3i32);
    pub const Class3: SmsMessageClass = SmsMessageClass(4i32);
}
#[repr(transparent)]
pub struct SmsMessageFilter(pub i32);
impl SmsMessageFilter {
    pub const All: SmsMessageFilter = SmsMessageFilter(0i32);
    pub const Unread: SmsMessageFilter = SmsMessageFilter(1i32);
    pub const Read: SmsMessageFilter = SmsMessageFilter(2i32);
    pub const Sent: SmsMessageFilter = SmsMessageFilter(3i32);
    pub const Draft: SmsMessageFilter = SmsMessageFilter(4i32);
}
#[repr(transparent)]
pub struct SmsMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsMessageReceivedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsMessageReceivedTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsMessageRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsMessageType(pub i32);
impl SmsMessageType {
    pub const Binary: SmsMessageType = SmsMessageType(0i32);
    pub const Text: SmsMessageType = SmsMessageType(1i32);
    pub const Wap: SmsMessageType = SmsMessageType(2i32);
    pub const App: SmsMessageType = SmsMessageType(3i32);
    pub const Broadcast: SmsMessageType = SmsMessageType(4i32);
    pub const Voicemail: SmsMessageType = SmsMessageType(5i32);
    pub const Status: SmsMessageType = SmsMessageType(6i32);
}
#[repr(transparent)]
pub struct SmsModemErrorCode(pub i32);
impl SmsModemErrorCode {
    pub const Other: SmsModemErrorCode = SmsModemErrorCode(0i32);
    pub const MessagingNetworkError: SmsModemErrorCode = SmsModemErrorCode(1i32);
    pub const SmsOperationNotSupportedByDevice: SmsModemErrorCode = SmsModemErrorCode(2i32);
    pub const SmsServiceNotSupportedByNetwork: SmsModemErrorCode = SmsModemErrorCode(3i32);
    pub const DeviceFailure: SmsModemErrorCode = SmsModemErrorCode(4i32);
    pub const MessageNotEncodedProperly: SmsModemErrorCode = SmsModemErrorCode(5i32);
    pub const MessageTooLarge: SmsModemErrorCode = SmsModemErrorCode(6i32);
    pub const DeviceNotReady: SmsModemErrorCode = SmsModemErrorCode(7i32);
    pub const NetworkNotReady: SmsModemErrorCode = SmsModemErrorCode(8i32);
    pub const InvalidSmscAddress: SmsModemErrorCode = SmsModemErrorCode(9i32);
    pub const NetworkFailure: SmsModemErrorCode = SmsModemErrorCode(10i32);
    pub const FixedDialingNumberRestricted: SmsModemErrorCode = SmsModemErrorCode(11i32);
}
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
