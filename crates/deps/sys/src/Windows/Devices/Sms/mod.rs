#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CellularClass(pub i32);
impl CellularClass {
    pub const None: Self = Self(0i32);
    pub const Gsm: Self = Self(1i32);
    pub const Cdma: Self = Self(2i32);
}
impl ::core::marker::Copy for CellularClass {}
impl ::core::clone::Clone for CellularClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeleteSmsMessageOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeleteSmsMessageOperation {}
impl ::core::clone::Clone for DeleteSmsMessageOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeleteSmsMessagesOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeleteSmsMessagesOperation {}
impl ::core::clone::Clone for DeleteSmsMessagesOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GetSmsDeviceOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GetSmsDeviceOperation {}
impl ::core::clone::Clone for GetSmsDeviceOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GetSmsMessageOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GetSmsMessageOperation {}
impl ::core::clone::Clone for GetSmsMessageOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GetSmsMessagesOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GetSmsMessagesOperation {}
impl ::core::clone::Clone for GetSmsMessagesOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsAppMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsAppMessage {}
impl ::core::clone::Clone for ISmsAppMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsBinaryMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsBinaryMessage {}
impl ::core::clone::Clone for ISmsBinaryMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsBroadcastMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsBroadcastMessage {}
impl ::core::clone::Clone for ISmsBroadcastMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsDevice {}
impl ::core::clone::Clone for ISmsDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsDevice2 {}
impl ::core::clone::Clone for ISmsDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsDevice2Statics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsDevice2Statics {}
impl ::core::clone::Clone for ISmsDevice2Statics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsDeviceMessageStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsDeviceMessageStore {}
impl ::core::clone::Clone for ISmsDeviceMessageStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsDeviceStatics {}
impl ::core::clone::Clone for ISmsDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsDeviceStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsDeviceStatics2 {}
impl ::core::clone::Clone for ISmsDeviceStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsFilterRule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsFilterRule {}
impl ::core::clone::Clone for ISmsFilterRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsFilterRuleFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsFilterRuleFactory {}
impl ::core::clone::Clone for ISmsFilterRuleFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsFilterRules(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsFilterRules {}
impl ::core::clone::Clone for ISmsFilterRules {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsFilterRulesFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsFilterRulesFactory {}
impl ::core::clone::Clone for ISmsFilterRulesFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsMessage {}
impl ::core::clone::Clone for ISmsMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsMessageBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsMessageBase {}
impl ::core::clone::Clone for ISmsMessageBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsMessageReceivedEventArgs {}
impl ::core::clone::Clone for ISmsMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsMessageReceivedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsMessageReceivedTriggerDetails {}
impl ::core::clone::Clone for ISmsMessageReceivedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsMessageRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsMessageRegistration {}
impl ::core::clone::Clone for ISmsMessageRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsMessageRegistrationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsMessageRegistrationStatics {}
impl ::core::clone::Clone for ISmsMessageRegistrationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsReceivedEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsReceivedEventDetails {}
impl ::core::clone::Clone for ISmsReceivedEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsReceivedEventDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsReceivedEventDetails2 {}
impl ::core::clone::Clone for ISmsReceivedEventDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsSendMessageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsSendMessageResult {}
impl ::core::clone::Clone for ISmsSendMessageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsStatusMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsStatusMessage {}
impl ::core::clone::Clone for ISmsStatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsTextMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsTextMessage {}
impl ::core::clone::Clone for ISmsTextMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsTextMessage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsTextMessage2 {}
impl ::core::clone::Clone for ISmsTextMessage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsTextMessageStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsTextMessageStatics {}
impl ::core::clone::Clone for ISmsTextMessageStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsVoicemailMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsVoicemailMessage {}
impl ::core::clone::Clone for ISmsVoicemailMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsWapMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsWapMessage {}
impl ::core::clone::Clone for ISmsWapMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SendSmsMessageOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SendSmsMessageOperation {}
impl ::core::clone::Clone for SendSmsMessageOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsAppMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsAppMessage {}
impl ::core::clone::Clone for SmsAppMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsBinaryMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsBinaryMessage {}
impl ::core::clone::Clone for SmsBinaryMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsBroadcastMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsBroadcastMessage {}
impl ::core::clone::Clone for SmsBroadcastMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsBroadcastType(pub i32);
impl SmsBroadcastType {
    pub const Other: Self = Self(0i32);
    pub const CmasPresidential: Self = Self(1i32);
    pub const CmasExtreme: Self = Self(2i32);
    pub const CmasSevere: Self = Self(3i32);
    pub const CmasAmber: Self = Self(4i32);
    pub const CmasTest: Self = Self(5i32);
    pub const EUAlert1: Self = Self(6i32);
    pub const EUAlert2: Self = Self(7i32);
    pub const EUAlert3: Self = Self(8i32);
    pub const EUAlertAmber: Self = Self(9i32);
    pub const EUAlertInfo: Self = Self(10i32);
    pub const EtwsEarthquake: Self = Self(11i32);
    pub const EtwsTsunami: Self = Self(12i32);
    pub const EtwsTsunamiAndEarthquake: Self = Self(13i32);
    pub const LatAlertLocal: Self = Self(14i32);
}
impl ::core::marker::Copy for SmsBroadcastType {}
impl ::core::clone::Clone for SmsBroadcastType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsDataFormat(pub i32);
impl SmsDataFormat {
    pub const Unknown: Self = Self(0i32);
    pub const CdmaSubmit: Self = Self(1i32);
    pub const GsmSubmit: Self = Self(2i32);
    pub const CdmaDeliver: Self = Self(3i32);
    pub const GsmDeliver: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsDataFormat {}
impl ::core::clone::Clone for SmsDataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsDevice {}
impl ::core::clone::Clone for SmsDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsDevice2 {}
impl ::core::clone::Clone for SmsDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsDeviceMessageStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsDeviceMessageStore {}
impl ::core::clone::Clone for SmsDeviceMessageStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsDeviceStatus(pub i32);
impl SmsDeviceStatus {
    pub const Off: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const SubscriptionNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl ::core::marker::Copy for SmsDeviceStatus {}
impl ::core::clone::Clone for SmsDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsDeviceStatusChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsDeviceStatusChangedEventHandler {}
impl ::core::clone::Clone for SmsDeviceStatusChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SmsEncodedLength {
    pub SegmentCount: u32,
    pub CharacterCountLastSegment: u32,
    pub CharactersPerSegment: u32,
    pub ByteCountLastSegment: u32,
    pub BytesPerSegment: u32,
}
impl ::core::marker::Copy for SmsEncodedLength {}
impl ::core::clone::Clone for SmsEncodedLength {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsEncoding(pub i32);
impl SmsEncoding {
    pub const Unknown: Self = Self(0i32);
    pub const Optimal: Self = Self(1i32);
    pub const SevenBitAscii: Self = Self(2i32);
    pub const Unicode: Self = Self(3i32);
    pub const GsmSevenBit: Self = Self(4i32);
    pub const EightBit: Self = Self(5i32);
    pub const Latin: Self = Self(6i32);
    pub const Korean: Self = Self(7i32);
    pub const IA5: Self = Self(8i32);
    pub const ShiftJis: Self = Self(9i32);
    pub const LatinHebrew: Self = Self(10i32);
}
impl ::core::marker::Copy for SmsEncoding {}
impl ::core::clone::Clone for SmsEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsFilterActionType(pub i32);
impl SmsFilterActionType {
    pub const AcceptImmediately: Self = Self(0i32);
    pub const Drop: Self = Self(1i32);
    pub const Peek: Self = Self(2i32);
    pub const Accept: Self = Self(3i32);
}
impl ::core::marker::Copy for SmsFilterActionType {}
impl ::core::clone::Clone for SmsFilterActionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsFilterRule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsFilterRule {}
impl ::core::clone::Clone for SmsFilterRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsFilterRules(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsFilterRules {}
impl ::core::clone::Clone for SmsFilterRules {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsGeographicalScope(pub i32);
impl SmsGeographicalScope {
    pub const None: Self = Self(0i32);
    pub const CellWithImmediateDisplay: Self = Self(1i32);
    pub const LocationArea: Self = Self(2i32);
    pub const Plmn: Self = Self(3i32);
    pub const Cell: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsGeographicalScope {}
impl ::core::clone::Clone for SmsGeographicalScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsMessageClass(pub i32);
impl SmsMessageClass {
    pub const None: Self = Self(0i32);
    pub const Class0: Self = Self(1i32);
    pub const Class1: Self = Self(2i32);
    pub const Class2: Self = Self(3i32);
    pub const Class3: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsMessageClass {}
impl ::core::clone::Clone for SmsMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsMessageFilter(pub i32);
impl SmsMessageFilter {
    pub const All: Self = Self(0i32);
    pub const Unread: Self = Self(1i32);
    pub const Read: Self = Self(2i32);
    pub const Sent: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsMessageFilter {}
impl ::core::clone::Clone for SmsMessageFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsMessageReceivedEventArgs {}
impl ::core::clone::Clone for SmsMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsMessageReceivedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsMessageReceivedEventHandler {}
impl ::core::clone::Clone for SmsMessageReceivedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsMessageReceivedTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsMessageReceivedTriggerDetails {}
impl ::core::clone::Clone for SmsMessageReceivedTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsMessageRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsMessageRegistration {}
impl ::core::clone::Clone for SmsMessageRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsMessageType(pub i32);
impl SmsMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
    pub const Wap: Self = Self(2i32);
    pub const App: Self = Self(3i32);
    pub const Broadcast: Self = Self(4i32);
    pub const Voicemail: Self = Self(5i32);
    pub const Status: Self = Self(6i32);
}
impl ::core::marker::Copy for SmsMessageType {}
impl ::core::clone::Clone for SmsMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsModemErrorCode(pub i32);
impl SmsModemErrorCode {
    pub const Other: Self = Self(0i32);
    pub const MessagingNetworkError: Self = Self(1i32);
    pub const SmsOperationNotSupportedByDevice: Self = Self(2i32);
    pub const SmsServiceNotSupportedByNetwork: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const MessageNotEncodedProperly: Self = Self(5i32);
    pub const MessageTooLarge: Self = Self(6i32);
    pub const DeviceNotReady: Self = Self(7i32);
    pub const NetworkNotReady: Self = Self(8i32);
    pub const InvalidSmscAddress: Self = Self(9i32);
    pub const NetworkFailure: Self = Self(10i32);
    pub const FixedDialingNumberRestricted: Self = Self(11i32);
}
impl ::core::marker::Copy for SmsModemErrorCode {}
impl ::core::clone::Clone for SmsModemErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsReceivedEventDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsReceivedEventDetails {}
impl ::core::clone::Clone for SmsReceivedEventDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsSendMessageResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsSendMessageResult {}
impl ::core::clone::Clone for SmsSendMessageResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsStatusMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsStatusMessage {}
impl ::core::clone::Clone for SmsStatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsTextMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsTextMessage {}
impl ::core::clone::Clone for SmsTextMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsTextMessage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsTextMessage2 {}
impl ::core::clone::Clone for SmsTextMessage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsVoicemailMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsVoicemailMessage {}
impl ::core::clone::Clone for SmsVoicemailMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsWapMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsWapMessage {}
impl ::core::clone::Clone for SmsWapMessage {
    fn clone(&self) -> Self {
        *self
    }
}
